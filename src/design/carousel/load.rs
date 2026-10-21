// load.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! Loads guides from TOML.
//!
//! A guide file looks like this:
//!
//! ```toml
//! title = "GSI"                       # required
//!
//! [labels]                            # optional, any subset
//! back = "Back"
//! exit = "Exit"
//! next = "Next"
//! done = "Done"
//! copy = "Copy"
//! image_missing = "Image not found"
//!
//! [[pages]]
//! title = "Notes"                     # required
//! subtitle = "Optional"
//! icon = "dialog-information-symbolic"
//!
//! [[pages.blocks]]
//! paragraph = "Plain text."
//!
//! [[pages.blocks]]
//! callout = "Text of the note"
//! kind = "tip"                        # info (default) | tip | warning | danger
//! title = "Optional heading"
//!
//! [[pages.blocks]]
//! code = "adb devices"
//!
//! [[pages.blocks]]
//! image = "treble-check.png"          # next to the TOML file, in the installed image
//!                                     # folders (see `GuideLoader::image_data_subdir`),
//!                                     # or "resource:/path.png"
//! caption = "Caption"
//! alt = "Alternative text for screen readers"
//! max_height = 420
//!
//! [[pages.blocks]]
//! steps = ["First step", { title = "Second", detail = "with a second line" }]
//! start = 1                           # optional: number of the first step
//!
//! [[pages.blocks]]
//! button = "Choose folder"            # text of the button
//! action = "choose-folder"            # required: the name the application gets
//! icon = "folder-open-symbolic"       # optional
//! ```
//!
//! Every block has **exactly one** content key: `heading`, `paragraph`, `markup` (Pango
//! markup), `bullets`, `steps`, `code`, `image`, `callout` or `button`. The other keys
//! (`start`, `caption`, `alt`, `max_height`, `kind`, `title`, `action`, `icon`) belong to one
//! specific content key. A button is reported by
//! [`GuideView::connect_action`](super::GuideView::connect_action).
//! Unknown keys are an error, so typos are noticed immediately.
//!
//! Short blocks can be written in one line:
//!
//! ```toml
//! [[pages]]
//! title = "Dirty Flash"
//! blocks = [
//!   { callout = "User data is retained.", kind = "warning" },
//!   { code = "fastboot reboot" },
//! ]
//! ```

use crate::helper::platform::dirs::DataDirs;
use std::fmt;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use gtk4::{gio, glib};
use serde::Deserialize;

use super::model::{ActionButton, Block, Callout, CalloutKind, Guide, Image, Labels, Page, Step};
use crate::gettext;

// ───────────────────────────── Errors ─────────────────────────────

/// An error while loading a guide.
///
/// The `Display` text names the reason and, where possible, the place in the file (line and
/// column, or page and block number).
#[derive(Debug)]
pub enum GuideError {
    /// Syntax error or unknown key; the message contains the line and column.
    Parse(toml::de::Error),
    /// The file could not be read.
    Io {
        /// The file that was read.
        path: PathBuf,
        /// The underlying error.
        source: std::io::Error,
    },
    /// The GResource was not found or is not UTF-8.
    Resource {
        /// The resource path.
        path: String,
        /// What went wrong.
        message: String,
    },
    /// Valid TOML, but the content does not fit the guide format.
    Invalid {
        /// Where the problem is, for example `page 2 ("Flash"), block 3`.
        location: String,
        /// What is wrong there.
        message: String,
    },
}

impl fmt::Display for GuideError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GuideError::Parse(e) => write!(f, "invalid guide file:\n{e}"),
            GuideError::Io { path, source } => {
                write!(f, "cannot read guide {}: {source}", path.display())
            }
            GuideError::Resource { path, message } => {
                write!(f, "cannot load guide resource {path}: {message}")
            }
            GuideError::Invalid { location, message } => {
                write!(f, "invalid guide at {location}: {message}")
            }
        }
    }
}

impl std::error::Error for GuideError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            GuideError::Parse(e) => Some(e),
            GuideError::Io { source, .. } => Some(source),
            _ => None,
        }
    }
}

impl From<toml::de::Error> for GuideError {
    fn from(e: toml::de::Error) -> Self {
        GuideError::Parse(e)
    }
}

// ───────────────────────────── Loader ─────────────────────────────

type TranslateFn = Rc<dyn Fn(&str) -> String>;

#[derive(Clone, Debug)]
enum Base {
    Dir(PathBuf),
    Resource(String),
}

/// Where installed images are searched (in the order in which they were given).
#[derive(Clone, Debug)]
enum ImageRoot {
    /// A fixed folder, for example the `PKGDATADIR` set at build time.
    Dir(PathBuf),
    /// A subfolder below every XDG data directory.
    DataSubdir(PathBuf),
}

/// A possible location of an image.
enum Spot {
    File(PathBuf),
    Resource(String),
}

impl Spot {
    fn exists(&self) -> bool {
        match self {
            Spot::File(path) => path.is_file(),
            Spot::Resource(path) => {
                gio::resources_get_info(path, gio::ResourceLookupFlags::NONE).is_ok()
            }
        }
    }

    fn describe(&self) -> String {
        match self {
            Spot::File(path) => path.display().to_string(),
            Spot::Resource(path) => format!("resource:{path}"),
        }
    }

    fn into_image(self) -> Image {
        match self {
            Spot::File(path) => Image::from_file(path),
            Spot::Resource(path) => Image::from_resource(path),
        }
    }
}

/// Loads [`Guide`]s from TOML.
///
/// Create one loader and reuse it for all guides of the application.
///
/// # Translations
///
/// Every visible text (title, paragraphs, steps, labels, captions, …) is translated; code
/// blocks, icon names and paths are not. The texts in the TOML file are the `msgid`s. By
/// default they are translated with [`gettext!`](crate::gettext!) (application domain
/// first, then the library domain, which already knows the default navigation labels).
/// [`translate`](Self::translate) replaces that with your own function. Empty texts are
/// never translated (`gettext("")` would return the header of the catalog).
///
/// # Image paths
///
/// A relative `image = "picture.png"` is searched in these places; the first hit wins:
///
/// 1. **Next to the guide:** the folder of the TOML file
///    ([`load_file`](Self::load_file)) or the folder in the GResource
///    ([`load_resource`](Self::load_resource)). During development the images are found in
///    the source tree without installing anything.
/// 2. **Installed images** in the folders given with
///    [`image_data_subdir`](Self::image_data_subdir) and [`image_dir`](Self::image_dir), in
///    the order in which they were given.
///
/// If nothing is found, the placeholder "Image not found" is shown and a GLib warning lists
/// all places that were searched. `image = "resource:/org/example/App/picture.png"` always
/// names an absolute GResource path directly, `image = "/abs/path.png"` is a fixed file
/// path.
///
/// # Examples
///
/// ```
/// use atlbase::ui_prelude::{Block, GuideLoader};
///
/// let loader = GuideLoader::new();
/// let guide = loader
///     .load_str(
///         r#"
/// title = "First steps"
///
/// [[pages]]
/// title = "Welcome"
/// blocks = [
///   { paragraph = "Thanks for installing the app." },
///   { callout = "Read this first.", kind = "tip" },
/// ]
/// "#,
///     )
///     .unwrap();
///
/// assert_eq!(guide.title, "First steps");
/// assert!(matches!(guide.pages[0].blocks[0], Block::Paragraph { .. }));
///
/// // mistakes are reported with their position
/// assert!(loader.load_str("title = 1").is_err());
/// ```
///
/// Loading from a GResource of the application:
///
/// ```no_run
/// use atlbase::ui_prelude::GuideLoader;
///
/// let loader = GuideLoader::new();
/// let guide = loader.load_resource("/org/example/App/guides/gsi.toml")?;
/// # Ok::<(), atlbase::ui_prelude::GuideError>(())
/// ```
#[derive(Clone, Default)]
pub struct GuideLoader {
    translate: Option<TranslateFn>,
    base: Option<Base>,
    image_roots: Vec<ImageRoot>,
    /// Only for tests; otherwise the process environment is read.
    env: Option<DataDirs>,
}

impl fmt::Debug for GuideLoader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GuideLoader")
            .field("translate", &self.translate.is_some())
            .field("base", &self.base)
            .field("image_roots", &self.image_roots)
            .finish()
    }
}

impl GuideLoader {
    /// Creates a loader that translates with [`gettext!`](crate::gettext!).
    pub fn new() -> Self {
        Self::default()
    }

    /// Replaces the translation function, for example `|s| my_gettext(s)`.
    ///
    /// It is called for every visible text, but never with an empty string.
    pub fn translate(mut self, f: impl Fn(&str) -> String + 'static) -> Self {
        self.translate = Some(Rc::new(f));
        self
    }

    /// Sets the base folder for relative image paths of [`load_str`](Self::load_str).
    pub fn base_dir(mut self, dir: impl Into<PathBuf>) -> Self {
        self.base = Some(Base::Dir(dir.into()));
        self
    }

    /// Sets the base GResource path for relative image paths of
    /// [`load_str`](Self::load_str), for example when the TOML text is included with
    /// `include_str!`.
    pub fn base_resource(mut self, path: impl Into<String>) -> Self {
        self.base = Some(Base::Resource(path.into()));
        self
    }

    /// Searches installed images below the **XDG data directories**, for example
    /// `image_data_subdir("fastboot-assistant/guides")`. This covers the usual package
    /// targets:
    ///
    /// | Package format | The images are in                                 |
    /// |----------------|---------------------------------------------------|
    /// | deb / rpm      | `/usr/share/fastboot-assistant/guides/`           |
    /// | Flatpak        | `/app/share/fastboot-assistant/guides/`           |
    /// | Snap           | `$SNAP/usr/share/fastboot-assistant/guides/`      |
    /// | AppImage       | `$APPDIR/usr/share/fastboot-assistant/guides/`    |
    /// | local          | `~/.local/share/…`, `/usr/local/share/…`          |
    ///
    /// The search covers `$XDG_DATA_HOME`, `$XDG_DATA_DIRS` (default: `/usr/local/share`,
    /// `/usr/share`), `/app/share` (Flatpak), `$SNAP/usr/share` and `$SNAP/share` (Snap)
    /// and `$APPDIR/usr/share` (AppImage). To test without installing:
    /// `XDG_DATA_DIRS=$PWD/data:/usr/local/share:/usr/share cargo run`.
    ///
    /// May be called several times; the places given first win.
    pub fn image_data_subdir(mut self, subdir: impl Into<PathBuf>) -> Self {
        self.image_roots.push(ImageRoot::DataSubdir(subdir.into()));
        self
    }

    /// Adds a fixed folder for installed images, for example a `PKGDATADIR` set at build
    /// time (`/usr/share/fastboot-assistant/guides`). Inside a **Snap**, `$SNAP` is put in
    /// front as well, because `/usr/share/…` lives below `$SNAP` at run time.
    ///
    /// Also handy for the source tree during development (give it first and it wins over an
    /// old installed version):
    ///
    /// ```
    /// use atlbase::ui_prelude::GuideLoader;
    ///
    /// let mut loader = GuideLoader::new();
    /// #[cfg(debug_assertions)]
    /// {
    ///     loader = loader.image_dir(concat!(env!("CARGO_MANIFEST_DIR"), "/examples/assets"));
    /// }
    /// let loader = loader.image_data_subdir("fastboot-assistant/guides");
    /// # let _ = loader;
    /// ```
    pub fn image_dir(mut self, dir: impl Into<PathBuf>) -> Self {
        self.image_roots.push(ImageRoot::Dir(dir.into()));
        self
    }

    #[cfg(test)]
    fn with_env(mut self, env: DataDirs) -> Self {
        self.env = Some(env);
        self
    }

    /// Loads a guide from a string. Relative image paths use `base_dir`/`base_resource`.
    ///
    /// # Errors
    ///
    /// Returns a [`GuideError`] for syntax errors, unknown keys and blocks that do not fit
    /// the format.
    pub fn load_str(&self, src: &str) -> Result<Guide, GuideError> {
        self.parse(src, self.base.as_ref())
    }

    /// Loads a guide from a file. Relative image paths start at the folder of the file.
    ///
    /// # Errors
    ///
    /// Returns [`GuideError::Io`] if the file cannot be read, otherwise as
    /// [`load_str`](Self::load_str).
    pub fn load_file(&self, path: impl AsRef<Path>) -> Result<Guide, GuideError> {
        let path = path.as_ref();
        let src = std::fs::read_to_string(path).map_err(|source| GuideError::Io {
            path: path.to_owned(),
            source,
        })?;
        let base = Base::Dir(path.parent().map(Path::to_path_buf).unwrap_or_default());
        self.parse(&src, Some(&base))
    }

    /// Loads a guide from a GResource that is part of the application (recommended for
    /// Flatpak, Snap, deb, rpm and AppImage, because nothing has to be installed next to the
    /// binary). Relative image paths start at the folder of the file in the GResource.
    ///
    /// # Errors
    ///
    /// Returns [`GuideError::Resource`] if the resource does not exist or is not UTF-8,
    /// otherwise as [`load_str`](Self::load_str).
    pub fn load_resource(&self, path: &str) -> Result<Guide, GuideError> {
        let err = |message: String| GuideError::Resource {
            path: path.to_owned(),
            message,
        };
        let bytes = gio::resources_lookup_data(path, gio::ResourceLookupFlags::NONE)
            .map_err(|e| err(e.to_string()))?;
        let src = std::str::from_utf8(&bytes).map_err(|e| err(e.to_string()))?;
        let dir = path.rsplit_once('/').map(|(dir, _)| dir).unwrap_or("");
        self.parse(src, Some(&Base::Resource(dir.to_owned())))
    }

    // ── internal ──

    /// All folders for installed images in search order.
    fn image_search_dirs(&self, env: &DataDirs) -> Vec<PathBuf> {
        let mut dirs = Vec::new();
        for root in &self.image_roots {
            match root {
                ImageRoot::Dir(dir) => {
                    dirs.push(dir.clone());
                    // Snap: a path compiled in like /usr/share/app lives below $SNAP at
                    // run time
                    if let (Some(snap), Ok(rel)) = (&env.snap, dir.strip_prefix("/"))
                        && !dir.starts_with(snap)
                    {
                        dirs.push(snap.join(rel));
                    }
                }
                ImageRoot::DataSubdir(sub) => {
                    dirs.extend(env.data_dirs().into_iter().map(|data| data.join(sub)));
                }
            }
        }
        dirs
    }

    fn parse(&self, src: &str, base: Option<&Base>) -> Result<Guide, GuideError> {
        let def: GuideDef = toml::from_str(src)?;

        if def.pages.is_empty() {
            return Err(invalid("guide", "no pages; add at least one `[[pages]]`"));
        }

        let env = self.env.clone().unwrap_or_else(DataDirs::current);
        let ctx = Ctx {
            loader: self,
            base,
            image_dirs: self.image_search_dirs(&env),
            snap: env.snap,
        };

        let mut labels = Labels::default();
        let l = def.labels;
        labels.back = ctx.tr(&l.back.unwrap_or(labels.back));
        labels.exit = ctx.tr(&l.exit.unwrap_or(labels.exit));
        labels.next = ctx.tr(&l.next.unwrap_or(labels.next));
        labels.done = ctx.tr(&l.done.unwrap_or(labels.done));
        labels.copy = ctx.tr(&l.copy.unwrap_or(labels.copy));
        labels.image_missing = ctx.tr(&l.image_missing.unwrap_or(labels.image_missing));

        let mut guide = Guide::new(ctx.tr(&def.title)).labels(labels);

        for (i, page_def) in def.pages.into_iter().enumerate() {
            let page_no = i + 1;
            let page_title = page_def.title.clone();
            let location = |block: Option<usize>| match block {
                Some(b) => format!("page {page_no} (\"{page_title}\"), block {b}"),
                None => format!("page {page_no} (\"{page_title}\")"),
            };

            let mut page = Page::new(ctx.tr(&page_def.title));
            if let Some(subtitle) = &page_def.subtitle {
                page = page.subtitle(ctx.tr(subtitle));
            }
            if let Some(icon) = page_def.icon {
                page = page.icon(icon);
            }
            for (j, block_def) in page_def.blocks.into_iter().enumerate() {
                let block = ctx
                    .block(block_def)
                    .map_err(|message| invalid(&location(Some(j + 1)), &message))?;
                page = page.block(block);
            }
            guide = guide.page(page);
        }
        Ok(guide)
    }
}

fn invalid(location: &str, message: &str) -> GuideError {
    GuideError::Invalid {
        location: location.to_owned(),
        message: message.to_owned(),
    }
}

// ───────────────────── Conversion TOML → model ─────────────────────

struct Ctx<'a> {
    loader: &'a GuideLoader,
    base: Option<&'a Base>,
    /// Folders for installed images, already resolved (XDG, Flatpak, Snap, AppImage)
    image_dirs: Vec<PathBuf>,
    snap: Option<PathBuf>,
}

const CONTENT_KEYS: &str =
    "heading, paragraph, markup, bullets, steps, code, image, callout, button";

impl Ctx<'_> {
    fn tr(&self, text: &str) -> String {
        match &self.loader.translate {
            _ if text.trim().is_empty() => text.to_owned(), // gettext("") returns the PO header
            Some(f) => f(text),
            None => gettext!(text),
        }
    }

    fn block(&self, def: BlockDef) -> Result<Block, String> {
        // exactly one content key
        let present: Vec<&str> = [
            ("heading", def.heading.is_some()),
            ("paragraph", def.paragraph.is_some()),
            ("markup", def.markup.is_some()),
            ("bullets", def.bullets.is_some()),
            ("steps", def.steps.is_some()),
            ("code", def.code.is_some()),
            ("image", def.image.is_some()),
            ("callout", def.callout.is_some()),
            ("button", def.button.is_some()),
        ]
        .into_iter()
        .filter_map(|(name, set)| set.then_some(name))
        .collect();

        let kind = match present.as_slice() {
            [] => return Err(format!("block has no content; use one of: {CONTENT_KEYS}")),
            [one] => *one,
            many => {
                return Err(format!(
                    "block has several content keys ({}); use one block per element",
                    many.join(", ")
                ));
            }
        };

        // the extra keys must fit the content key
        let allowed: &[&str] = match kind {
            "steps" => &["start"],
            "image" => &["caption", "alt", "max_height"],
            "callout" => &["kind", "title"],
            "button" => &["action", "icon"],
            _ => &[],
        };
        for (name, set) in [
            ("action", def.action.is_some()),
            ("icon", def.icon.is_some()),
            ("start", def.start.is_some()),
            ("caption", def.caption.is_some()),
            ("alt", def.alt.is_some()),
            ("max_height", def.max_height.is_some()),
            ("kind", def.kind.is_some()),
            ("title", def.title.is_some()),
        ] {
            if set && !allowed.contains(&name) {
                return Err(format!("`{name}` is not valid in a `{kind}` block"));
            }
        }

        Ok(match kind {
            "heading" => Block::Heading(self.tr(&def.heading.unwrap())),
            "paragraph" => Block::Paragraph {
                text: self.tr(&def.paragraph.unwrap()),
                markup: false,
            },
            "markup" => Block::Paragraph {
                text: self.tr(&def.markup.unwrap()),
                markup: true,
            },
            "bullets" => Block::Bullets(
                def.bullets
                    .unwrap()
                    .iter()
                    .map(|item| self.tr(item))
                    .collect(),
            ),
            "steps" => Block::Steps {
                start: def.start.unwrap_or(1),
                items: def
                    .steps
                    .unwrap()
                    .into_iter()
                    .map(|step| match step {
                        StepDef::Text(title) => Step::new(self.tr(&title)),
                        StepDef::Full(full) => {
                            let step = Step::new(self.tr(&full.title));
                            match full.detail {
                                Some(detail) => step.detail(self.tr(&detail)),
                                None => step,
                            }
                        }
                    })
                    .collect(),
            },
            "code" => Block::Code(def.code.unwrap()),
            "image" => {
                let mut image = self.image(&def.image.unwrap());
                if let Some(caption) = &def.caption {
                    image = image.caption(self.tr(caption));
                }
                if let Some(alt) = &def.alt {
                    image = image.alt(self.tr(alt));
                }
                if let Some(max_height) = def.max_height {
                    image = image.max_height(max_height);
                }
                Block::Image(image)
            }
            "callout" => {
                let mut callout = Callout {
                    kind: match def.kind.unwrap_or(KindDef::Info) {
                        KindDef::Info => CalloutKind::Info,
                        KindDef::Tip => CalloutKind::Tip,
                        KindDef::Warning => CalloutKind::Warning,
                        KindDef::Danger => CalloutKind::Danger,
                    },
                    title: None,
                    text: self.tr(&def.callout.unwrap()),
                };
                if let Some(title) = &def.title {
                    callout = callout.title(self.tr(title));
                }
                Block::Callout(callout)
            }
            "button" => {
                let Some(action) = def.action else {
                    return Err(
                        "a `button` block needs `action` (the name the application gets)".into(),
                    );
                };
                let mut button = ActionButton::new(action, self.tr(&def.button.unwrap()));
                if let Some(icon) = def.icon {
                    button = button.icon(icon);
                }
                Block::Button(button)
            }
            _ => unreachable!("kind comes from the list above"),
        })
    }

    /// `resource:/path` → GResource; otherwise the first place that exists: next to the
    /// guide, then the folders for installed images.
    fn image(&self, spec: &str) -> Image {
        if let Some(rest) = spec.strip_prefix("resource:") {
            // accepts `resource:/a/b.png` and the GIO URI form `resource:///a/b.png`
            return Image::from_resource(format!("/{}", rest.trim_start_matches('/')));
        }

        let path = Path::new(spec);
        let mut candidates: Vec<Spot> = Vec::new();
        if path.is_absolute() {
            candidates.push(Spot::File(path.to_path_buf()));
            // Snap: a path compiled in lives below $SNAP at run time
            if let (Some(snap), Ok(rel)) = (&self.snap, path.strip_prefix("/"))
                && !path.starts_with(snap)
            {
                candidates.push(Spot::File(snap.join(rel)));
            }
        } else {
            candidates.push(match self.base {
                Some(Base::Resource(dir)) => {
                    Spot::Resource(format!("{}/{spec}", dir.trim_end_matches('/')))
                }
                Some(Base::Dir(dir)) => Spot::File(dir.join(spec)),
                None => Spot::File(path.to_path_buf()),
            });
            candidates.extend(self.image_dirs.iter().map(|dir| Spot::File(dir.join(spec))));
        }

        // Only one candidate: nothing to choose, rendering reports a missing image later.
        if candidates.len() == 1 {
            return candidates.remove(0).into_image();
        }

        let chosen = match candidates.iter().position(Spot::exists) {
            Some(index) => index,
            None => {
                let searched: Vec<String> = candidates.iter().map(Spot::describe).collect();
                glib::g_warning!(
                    "atlbase",
                    "image \"{}\" not found; searched: {}",
                    spec,
                    searched.join(", ")
                );
                0
            }
        };
        candidates.swap_remove(chosen).into_image()
    }
}

// ───────────────────────── serde structures ─────────────────────────

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct GuideDef {
    title: String,
    #[serde(default)]
    labels: LabelsDef,
    #[serde(default)]
    pages: Vec<PageDef>,
}

#[derive(Deserialize, Default)]
#[serde(deny_unknown_fields)]
struct LabelsDef {
    back: Option<String>,
    exit: Option<String>,
    next: Option<String>,
    done: Option<String>,
    copy: Option<String>,
    image_missing: Option<String>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct PageDef {
    title: String,
    subtitle: Option<String>,
    icon: Option<String>,
    #[serde(default)]
    blocks: Vec<BlockDef>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct BlockDef {
    // content (exactly one)
    heading: Option<String>,
    paragraph: Option<String>,
    markup: Option<String>,
    bullets: Option<Vec<String>>,
    steps: Option<Vec<StepDef>>,
    code: Option<String>,
    image: Option<String>,
    callout: Option<String>,
    button: Option<String>,
    // extra keys
    action: Option<String>,
    icon: Option<String>,
    start: Option<u32>,
    caption: Option<String>,
    alt: Option<String>,
    max_height: Option<i32>,
    kind: Option<KindDef>,
    title: Option<String>,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum StepDef {
    Text(String),
    Full(StepFull),
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct StepFull {
    title: String,
    detail: Option<String>,
}

#[derive(Deserialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
enum KindDef {
    Info,
    Tip,
    Warning,
    Danger,
}

// ───────────────────────────── Tests ─────────────────────────────

#[cfg(test)]
mod tests {
    use super::super::model::ImageSource;
    use super::*;
    use std::cell::RefCell;

    const SAMPLE: &str = r#"
title = "GSI"

[labels]
next = "Weiter"

[[pages]]
title = "Notes"
icon = "dialog-information-symbolic"

[[pages.blocks]]
paragraph = "GSI is an image."

[[pages.blocks]]
callout = "Needs Treble."
kind = "warning"
title = "Careful"

[[pages.blocks]]
code = "adb devices"

[[pages.blocks]]
image = "treble.png"
caption = "Output"
alt = "Terminal"
max_height = 300

[[pages]]
title = "Flash"
blocks = [
  { steps = ["Boot", { title = "Open the bootloader.", detail = "Not every device." }], start = 3 },
  { bullets = ["a", "b"] },
  { markup = "Tap <b>Start</b>" },
  { heading = "Commands" },
]
"#;

    fn load(src: &str) -> Result<Guide, GuideError> {
        GuideLoader::new().load_str(src)
    }

    fn err_text(src: &str) -> String {
        load(src).expect_err("should fail").to_string()
    }

    #[test]
    fn parses_all_block_kinds() {
        let guide = load(SAMPLE).unwrap();
        assert_eq!(guide.title, "GSI");
        assert_eq!(guide.pages.len(), 2);
        assert_eq!(guide.labels.next, "Weiter");
        assert_eq!(guide.labels.back, "Back", "unset labels keep their default");

        let p1 = &guide.pages[0];
        assert_eq!(p1.icon.as_deref(), Some("dialog-information-symbolic"));
        assert!(matches!(
            &p1.blocks[0],
            Block::Paragraph { markup: false, .. }
        ));
        match &p1.blocks[1] {
            Block::Callout(c) => {
                assert_eq!(c.kind, CalloutKind::Warning);
                assert_eq!(c.title.as_deref(), Some("Careful"));
            }
            other => panic!("expected callout, got {other:?}"),
        }
        assert!(matches!(&p1.blocks[2], Block::Code(c) if c == "adb devices"));
        match &p1.blocks[3] {
            Block::Image(img) => {
                assert_eq!(img.caption.as_deref(), Some("Output"));
                assert_eq!(img.alt.as_deref(), Some("Terminal"));
                assert_eq!(img.max_height, 300);
            }
            other => panic!("expected image, got {other:?}"),
        }

        let p2 = &guide.pages[1];
        match &p2.blocks[0] {
            Block::Steps { start, items } => {
                assert_eq!(*start, 3);
                assert_eq!(items.len(), 2);
                assert_eq!(items[1].detail.as_deref(), Some("Not every device."));
            }
            other => panic!("expected steps, got {other:?}"),
        }
        assert!(matches!(&p2.blocks[1], Block::Bullets(b) if b.len() == 2));
        assert!(matches!(
            &p2.blocks[2],
            Block::Paragraph { markup: true, .. }
        ));
        assert!(matches!(&p2.blocks[3], Block::Heading(h) if h == "Commands"));
    }

    #[test]
    fn translate_hook_skips_code_icons_and_empty_text() {
        let seen = Rc::new(RefCell::new(Vec::<String>::new()));
        let recorder = seen.clone();
        let loader = GuideLoader::new().translate(move |s| {
            recorder.borrow_mut().push(s.to_owned());
            format!("«{s}»")
        });

        let guide = loader
            .load_str(
                r#"
title = "T"
[[pages]]
title = "P"
icon = "dialog-information-symbolic"
blocks = [
  { paragraph = "Hello" },
  { paragraph = "" },
  { code = "fastboot reboot" },
]
"#,
            )
            .unwrap();

        assert_eq!(guide.title, "«T»");
        assert_eq!(guide.pages[0].title, "«P»");
        assert_eq!(
            guide.pages[0].icon.as_deref(),
            Some("dialog-information-symbolic")
        );
        assert!(
            matches!(&guide.pages[0].blocks[0], Block::Paragraph { text, .. } if text == "«Hello»")
        );
        assert!(
            matches!(&guide.pages[0].blocks[1], Block::Paragraph { text, .. } if text.is_empty())
        );
        assert!(matches!(&guide.pages[0].blocks[2], Block::Code(c) if c == "fastboot reboot"));

        let seen = seen.borrow();
        assert!(seen.contains(&"Hello".to_owned()));
        assert!(
            seen.contains(&"Next".to_owned()),
            "default labels are translated too"
        );
        assert!(
            !seen.iter().any(|s| s.is_empty()),
            "empty strings must not reach gettext"
        );
        assert!(
            !seen.iter().any(|s| s.contains("fastboot")),
            "code is never translated"
        );
        assert!(
            !seen.iter().any(|s| s.contains("symbolic")),
            "icon names are never translated"
        );
    }

    #[test]
    fn resolves_image_paths() {
        let src = |image: &str| {
            format!(
                "title = \"T\"\n[[pages]]\ntitle = \"P\"\nblocks = [ {{ image = \"{image}\" }} ]\n"
            )
        };
        let source = |guide: Guide| match &guide.pages[0].blocks[0] {
            Block::Image(img) => img.source.clone(),
            other => panic!("{other:?}"),
        };

        // without a base: unchanged
        let g = load(&src("a.png")).unwrap();
        assert!(matches!(source(g), ImageSource::File(p) if p == Path::new("a.png")));

        // base folder
        let g = GuideLoader::new()
            .base_dir("/guides")
            .load_str(&src("img/a.png"))
            .unwrap();
        assert!(matches!(source(g), ImageSource::File(p) if p == Path::new("/guides/img/a.png")));

        // base resource
        let g = GuideLoader::new()
            .base_resource("/io/x/guides/")
            .load_str(&src("a.png"))
            .unwrap();
        assert!(matches!(source(g), ImageSource::Resource(p) if p == "/io/x/guides/a.png"));

        // explicit: resource:/… and resource:///…
        for spec in ["resource:/io/x/a.png", "resource:///io/x/a.png"] {
            let g = GuideLoader::new()
                .base_dir("/ignored")
                .load_str(&src(spec))
                .unwrap();
            assert!(
                matches!(source(g), ImageSource::Resource(p) if p == "/io/x/a.png"),
                "{spec}"
            );
        }
    }

    #[test]
    fn load_file_uses_the_directory_of_the_file() {
        let dir = std::env::temp_dir().join(format!("atlbase-guide-test-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let file = dir.join("g.toml");
        std::fs::write(
            &file,
            "title = \"T\"\n[[pages]]\ntitle = \"P\"\nblocks = [ { image = \"x.png\" } ]\n",
        )
        .unwrap();

        let guide = GuideLoader::new().load_file(&file).unwrap();
        match &guide.pages[0].blocks[0] {
            Block::Image(img) => {
                assert!(matches!(&img.source, ImageSource::File(p) if *p == dir.join("x.png")))
            }
            other => panic!("{other:?}"),
        }
        std::fs::remove_dir_all(&dir).ok();

        let missing = GuideLoader::new()
            .load_file(dir.join("nope.toml"))
            .unwrap_err();
        assert!(matches!(missing, GuideError::Io { .. }));
    }

    #[test]
    fn typos_are_errors() {
        let e = err_text(
            "title = \"T\"\n[[pages]]\ntitle = \"P\"\n[[pages.blocks]]\nparagraf = \"x\"\n",
        );
        assert!(e.contains("paragraf"), "{e}");

        let e = err_text("title = \"T\"\n[[page]]\ntitle = \"P\"\n");
        assert!(e.contains("page"), "{e}");
    }

    #[test]
    fn block_validation_messages_name_page_and_block() {
        let head = "title = \"T\"\n[[pages]]\ntitle = \"First\"\n[[pages]]\ntitle = \"Second\"\n";

        let e = err_text(&format!(
            "{head}[[pages.blocks]]\nparagraph = \"ok\"\n[[pages.blocks]]\nkind = \"tip\"\n"
        ));
        assert!(e.contains("page 2 (\"Second\"), block 2"), "{e}");
        assert!(e.contains("no content"), "{e}");

        let e = err_text(&format!(
            "{head}[[pages.blocks]]\nparagraph = \"a\"\ncode = \"b\"\n"
        ));
        assert!(
            e.contains("several content keys") && e.contains("paragraph, code"),
            "{e}"
        );

        let e = err_text(&format!(
            "{head}[[pages.blocks]]\nparagraph = \"a\"\ncaption = \"b\"\n"
        ));
        assert!(
            e.contains("`caption` is not valid in a `paragraph` block"),
            "{e}"
        );

        let e = err_text(&format!(
            "{head}[[pages.blocks]]\ncallout = \"a\"\nkind = \"loud\"\n"
        ));
        assert!(
            e.contains("loud") && e.contains("tip"),
            "unknown callout kind should list the options: {e}"
        );
    }

    #[test]
    fn a_button_block_has_an_action_and_translated_text() {
        let loader = GuideLoader::new().translate(|s| format!("<{s}>"));
        let guide = loader
            .load_str(
                "title = \"T\"\n[[pages]]\ntitle = \"P\"\n[[pages.blocks]]\n\
                 button = \"Choose\"\naction = \"choose-folder\"\nicon = \"folder-open-symbolic\"\n",
            )
            .unwrap();
        let Block::Button(button) = &guide.pages[0].blocks[0] else {
            panic!("expected a button");
        };
        assert_eq!(button.label, "<Choose>");
        assert_eq!(
            button.action, "choose-folder",
            "the action name is not translated"
        );
        assert_eq!(button.icon.as_deref(), Some("folder-open-symbolic"));
    }

    #[test]
    fn a_button_without_action_is_an_error() {
        let head = "title = \"T\"\n[[pages]]\ntitle = \"P\"\n[[pages.blocks]]\n";
        let e = err_text(&format!("{head}button = \"Go\"\n"));
        assert!(e.contains("needs `action`"), "{e}");

        let e = err_text(&format!("{head}paragraph = \"x\"\naction = \"go\"\n"));
        assert!(
            e.contains("`action` is not valid in a `paragraph` block"),
            "{e}"
        );
    }

    #[test]
    fn guide_without_pages_is_an_error() {
        let e = err_text("title = \"T\"\n");
        assert!(e.contains("no pages"), "{e}");
    }

    // ───────── installed images: XDG, Flatpak, Snap, AppImage ─────────

    fn scratch(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("atlbase-guide-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn touch(path: &Path) {
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, b"x").unwrap();
    }

    fn guide_with_image(spec: &str) -> String {
        format!("title = \"T\"\n[[pages]]\ntitle = \"P\"\nblocks = [ {{ image = \"{spec}\" }} ]\n")
    }

    /// Loads a guide with one image and returns its resolved source.
    fn resolved(loader: &GuideLoader, spec: &str) -> ImageSource {
        let guide = loader.load_str(&guide_with_image(spec)).unwrap();
        match &guide.pages[0].blocks[0] {
            Block::Image(img) => img.source.clone(),
            other => panic!("{other:?}"),
        }
    }

    fn file(source: ImageSource) -> PathBuf {
        match source {
            ImageSource::File(path) => path,
            other => panic!("expected a file, got {other:?}"),
        }
    }

    fn paths(list: &[&str]) -> Vec<PathBuf> {
        list.iter().map(PathBuf::from).collect()
    }

    #[test]
    fn data_dirs_follow_xdg_and_add_flatpak_and_snap() {
        // user first; duplicates, empty and relative entries are dropped; Flatpak, Snap and
        // AppImage follow
        let env = DataDirs {
            home: Some("/home/u".into()),
            xdg_data_home: None,
            xdg_data_dirs: Some("/opt/a::/usr/share:/opt/a:relative".into()),
            snap: Some("/snap/x/1".into()),
            appdir: Some("/tmp/.mount_x".into()),
            flatpak: true,
            ..DataDirs::default()
        };
        assert_eq!(
            env.data_dirs(),
            paths(&[
                "/home/u/.local/share",
                "/opt/a",
                "/usr/share",
                "/app/share",
                "/snap/x/1/usr/share",
                "/snap/x/1/share",
                "/tmp/.mount_x/usr/share",
            ])
        );

        // XDG_DATA_HOME replaces ~/.local/share
        let env = DataDirs {
            home: Some("/home/u".into()),
            xdg_data_home: Some("/data".into()),
            ..DataDirs::default()
        };
        assert_eq!(env.data_dirs()[0], PathBuf::from("/data"));

        // without XDG_DATA_DIRS the defaults of the specification apply (Debian: /usr/share)
        assert_eq!(
            DataDirs::default().data_dirs(),
            paths(&["/usr/local/share", "/usr/share"])
        );
    }

    #[test]
    fn finds_installed_images_in_xdg_data_dirs() {
        let tmp = scratch("xdg");
        touch(&tmp.join("share/myapp/guides/a.png"));
        let env = DataDirs {
            xdg_data_dirs: Some(format!("/nonexistent:{}", tmp.join("share").display()).into()),
            ..DataDirs::default()
        };
        let loader = GuideLoader::new()
            .image_data_subdir("myapp/guides")
            .with_env(env);

        // guide from a GResource/string, image only installed (Debian: /usr/share/myapp/guides)
        let found = file(resolved(&loader, "a.png"));
        assert_eq!(found, tmp.join("share/myapp/guides/a.png"));

        // subfolders in the image path are kept
        touch(&tmp.join("share/myapp/guides/img/b.png"));
        let found = file(resolved(&loader, "img/b.png"));
        assert_eq!(found, tmp.join("share/myapp/guides/img/b.png"));
        std::fs::remove_dir_all(&tmp).ok();
    }

    #[test]
    fn flatpak_and_snap_locations_are_searched_without_xdg_data_dirs() {
        let tmp = scratch("snap");
        touch(&tmp.join("usr/share/myapp/a.png"));
        let env = DataDirs {
            // this is how some Snaps look: $SNAP/usr/share is missing in XDG_DATA_DIRS
            xdg_data_dirs: Some("/nonexistent".into()),
            snap: Some(tmp.clone()),
            ..DataDirs::default()
        };
        let loader = GuideLoader::new().image_data_subdir("myapp").with_env(env);
        assert_eq!(
            file(resolved(&loader, "a.png")),
            tmp.join("usr/share/myapp/a.png")
        );
        std::fs::remove_dir_all(&tmp).ok();
    }

    #[test]
    fn fixed_dirs_and_absolute_paths_get_the_snap_prefix() {
        let tmp = scratch("pkgdatadir");
        touch(&tmp.join("usr/share/myapp/a.png"));
        let env = DataDirs {
            snap: Some(tmp.clone()),
            ..DataDirs::default()
        };

        // PKGDATADIR was compiled in as /usr/share/myapp, but it lives below $SNAP
        let loader = GuideLoader::new()
            .image_dir("/usr/share/myapp")
            .with_env(env.clone());
        assert_eq!(
            file(resolved(&loader, "a.png")),
            tmp.join("usr/share/myapp/a.png")
        );

        // the same for an absolute path directly in the TOML file
        let loader = GuideLoader::new().with_env(env);
        assert_eq!(
            file(resolved(&loader, "/usr/share/myapp/a.png")),
            tmp.join("usr/share/myapp/a.png")
        );
        std::fs::remove_dir_all(&tmp).ok();
    }

    #[test]
    fn image_next_to_the_guide_wins_over_installed_copy() {
        let tmp = scratch("beside");
        touch(&tmp.join("src/guides/a.png"));
        touch(&tmp.join("share/myapp/a.png"));
        let env = DataDirs {
            xdg_data_dirs: Some(tmp.join("share").into_os_string()),
            ..DataDirs::default()
        };

        // development: an old installed version must not hide the image in the source tree
        let loader = GuideLoader::new()
            .base_dir(tmp.join("src/guides"))
            .image_data_subdir("myapp")
            .with_env(env.clone());
        assert_eq!(
            file(resolved(&loader, "a.png")),
            tmp.join("src/guides/a.png")
        );

        // release: nothing next to the guide (for example in the GResource) → installed copy
        let loader = GuideLoader::new()
            .base_resource("/io/x/guides")
            .image_data_subdir("myapp")
            .with_env(env);
        assert_eq!(
            file(resolved(&loader, "a.png")),
            tmp.join("share/myapp/a.png")
        );
        std::fs::remove_dir_all(&tmp).ok();
    }

    #[test]
    fn first_configured_location_wins() {
        let tmp = scratch("order");
        touch(&tmp.join("dev/a.png"));
        touch(&tmp.join("share/myapp/a.png"));
        let env = DataDirs {
            xdg_data_dirs: Some(tmp.join("share").into_os_string()),
            ..DataDirs::default()
        };

        let dev_first = GuideLoader::new()
            .image_dir(tmp.join("dev"))
            .image_data_subdir("myapp")
            .with_env(env.clone());
        assert_eq!(file(resolved(&dev_first, "a.png")), tmp.join("dev/a.png"));

        let installed_first = GuideLoader::new()
            .image_data_subdir("myapp")
            .image_dir(tmp.join("dev"))
            .with_env(env);
        assert_eq!(
            file(resolved(&installed_first, "a.png")),
            tmp.join("share/myapp/a.png")
        );
        std::fs::remove_dir_all(&tmp).ok();
    }

    #[test]
    fn missing_image_falls_back_to_the_path_next_to_the_guide() {
        let env = DataDirs {
            xdg_data_dirs: Some("/nonexistent".into()),
            ..DataDirs::default()
        };
        let loader = GuideLoader::new()
            .base_dir("/guides")
            .image_data_subdir("myapp")
            .with_env(env);
        // nothing found: the loader does not fail, rendering shows the placeholder
        assert_eq!(
            file(resolved(&loader, "gone.png")),
            PathBuf::from("/guides/gone.png")
        );
    }

    #[test]
    fn explicit_resource_paths_are_never_searched() {
        let loader = GuideLoader::new().image_data_subdir("myapp");
        assert!(matches!(
            resolved(&loader, "resource:/io/x/a.png"),
            ImageSource::Resource(p) if p == "/io/x/a.png"
        ));
    }
}
