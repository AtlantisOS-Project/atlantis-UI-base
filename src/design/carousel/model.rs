// model.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! The data structures of a guide.
//!
//! They describe *what* a guide says, not *how* it looks. That makes the content easy to
//! change, to test and to move to other sources (TOML, JSON, …).

use crate::gettext;
use gtk4::gdk;
use std::path::PathBuf;

/// The captions of the navigation.
///
/// The default is English. Use [`Labels::localized`] to get the translated texts.
#[derive(Clone, Debug)]
pub struct Labels {
    /// Left button from the second page on.
    pub back: String,
    /// Left button on the first page (leaves the guide).
    pub exit: String,
    /// Right button up to the second to last page.
    pub next: String,
    /// Right button on the last page (leaves the guide).
    pub done: String,
    /// Tooltip of the copy button of code blocks.
    pub copy: String,
    /// Text shown when an image could not be loaded.
    pub image_missing: String,
}

impl Default for Labels {
    fn default() -> Self {
        Self {
            back: "Back".into(),
            exit: "Exit".into(),
            next: "Next".into(),
            done: "Done".into(),
            copy: "Copy".into(),
            image_missing: "Image not found".into(),
        }
    }
}

impl Labels {
    /// The labels, translated with [`gettext!`](crate::gettext!).
    ///
    /// The library comes with translations of these six texts, so they are translated
    /// without any work once [`init_language`](crate::helper::language::language::init_language)
    /// was called.
    ///
    /// # Examples
    ///
    /// ```
    /// use atlbase::ui_prelude::{Guide, Labels};
    ///
    /// let guide = Guide::new("Tour").labels(Labels::localized());
    /// // without a language set up, the English texts are kept
    /// assert_eq!(guide.labels.next, "Next");
    /// ```
    pub fn localized() -> Self {
        Self {
            back: gettext!("Back"),
            exit: gettext!("Exit"),
            next: gettext!("Next"),
            done: gettext!("Done"),
            copy: gettext!("Copy"),
            image_missing: gettext!("Image not found"),
        }
    }
}

/// A complete guide: a title and ordered pages.
///
/// Build it in code with the methods below or load it from TOML with
/// `GuideLoader`, then show it with [`GuideView`](super::GuideView).
///
/// # Examples
///
/// ```
/// use atlbase::ui_prelude::{Callout, Guide, Page};
///
/// let guide = Guide::new("First steps").page(
///     Page::new("Welcome")
///         .paragraph("Thanks for installing the app.")
///         .callout(Callout::tip("You can leave the tour at any time."))
///         .code("app --help"),
/// );
/// assert_eq!(guide.pages.len(), 1);
/// ```
#[derive(Clone, Debug)]
pub struct Guide {
    /// The title of the guide (shown in the dialog variant).
    pub title: String,
    /// The pages in the order of display.
    pub pages: Vec<Page>,
    /// The captions of the navigation.
    pub labels: Labels,
}

impl Guide {
    /// Creates an empty guide with the default (English) labels.
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            pages: Vec::new(),
            labels: Labels::default(),
        }
    }

    /// Appends a page.
    pub fn page(mut self, page: Page) -> Self {
        self.pages.push(page);
        self
    }

    /// Replaces the captions of the navigation.
    pub fn labels(mut self, labels: Labels) -> Self {
        self.labels = labels;
        self
    }
}

/// One page of a guide.
#[derive(Clone, Debug)]
pub struct Page {
    /// The title at the top of the page.
    pub title: String,
    /// An optional line below the title.
    pub subtitle: Option<String>,
    /// The name of a symbolic icon, for example `"dialog-information-symbolic"`.
    pub icon: Option<String>,
    /// The content blocks in the order of display.
    pub blocks: Vec<Block>,
}

impl Page {
    /// Creates an empty page.
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            subtitle: None,
            icon: None,
            blocks: Vec::new(),
        }
    }

    /// Sets the line below the title.
    pub fn subtitle(mut self, text: impl Into<String>) -> Self {
        self.subtitle = Some(text.into());
        self
    }

    /// Sets the icon next to the title, for example `"dialog-information-symbolic"`.
    pub fn icon(mut self, icon_name: impl Into<String>) -> Self {
        self.icon = Some(icon_name.into());
        self
    }

    /// Appends any [`Block`]. The methods below are shortcuts for this one.
    pub fn block(mut self, block: Block) -> Self {
        self.blocks.push(block);
        self
    }

    /// Appends a heading.
    pub fn heading(self, text: impl Into<String>) -> Self {
        self.block(Block::Heading(text.into()))
    }

    /// Appends a normal paragraph (plain text, no markup).
    pub fn paragraph(self, text: impl Into<String>) -> Self {
        self.block(Block::Paragraph {
            text: text.into(),
            markup: false,
        })
    }

    /// Appends a paragraph with Pango markup, for example `"Tap <b>Start</b>"`.
    pub fn markup(self, text: impl Into<String>) -> Self {
        self.block(Block::Paragraph {
            text: text.into(),
            markup: true,
        })
    }

    /// Appends a bullet list.
    pub fn bullets<I, S>(self, items: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.block(Block::Bullets(items.into_iter().map(Into::into).collect()))
    }

    /// Appends numbered steps; the count starts at 1.
    pub fn steps<I, S>(self, items: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<Step>,
    {
        self.steps_from(1, items)
    }

    /// Appends numbered steps with a start number of your own (handy when a list is
    /// interrupted by code blocks or images).
    pub fn steps_from<I, S>(self, start: u32, items: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<Step>,
    {
        self.block(Block::Steps {
            start,
            items: items.into_iter().map(Into::into).collect(),
        })
    }

    /// Appends a command or code with a "copy" button.
    pub fn code(self, text: impl Into<String>) -> Self {
        self.block(Block::Code(text.into()))
    }

    /// Appends an image.
    pub fn image(self, image: Image) -> Self {
        self.block(Block::Image(image))
    }

    /// Appends a highlighted note.
    pub fn callout(self, callout: Callout) -> Self {
        self.block(Block::Callout(callout))
    }

    /// Appends a button that reports a click to the application, see [`ActionButton`].
    pub fn button(self, action: impl Into<String>, label: impl Into<String>) -> Self {
        self.block(Block::Button(ActionButton::new(action, label)))
    }
}

/// The building blocks of a page.
#[derive(Clone, Debug)]
pub enum Block {
    /// A heading.
    Heading(String),
    /// A paragraph; `markup` tells whether `text` contains Pango markup.
    Paragraph {
        /// The text of the paragraph.
        text: String,
        /// `true` if `text` is Pango markup.
        markup: bool,
    },
    /// A bullet list.
    Bullets(Vec<String>),
    /// Numbered steps.
    Steps {
        /// The number of the first step.
        start: u32,
        /// The steps.
        items: Vec<Step>,
    },
    /// A command or code with a "copy" button.
    Code(String),
    /// A screenshot or illustration.
    Image(Image),
    /// A highlighted note.
    Callout(Callout),
    /// A button that reports a click to the application.
    Button(ActionButton),
}

/// A button on a page. A click is reported by the
/// [`action` signal of the GuideView](super::GuideView::connect_action) with the `action`
/// name, so a guide can do more than explain: choose a folder, start a check, …
///
/// # Examples
///
/// ```
/// use atlbase::ui_prelude::{ActionButton, Guide, Page};
///
/// let button = ActionButton::new("choose-folder", "Choose folder").icon("folder-open-symbolic");
/// let guide = Guide::new("Setup").page(Page::new("Folder").block(button.into()));
/// assert_eq!(guide.pages[0].blocks.len(), 1);
/// ```
#[derive(Clone, Debug)]
pub struct ActionButton {
    /// The name the application gets when the button is clicked.
    pub action: String,
    /// The text of the button.
    pub label: String,
    /// The name of a symbolic icon in front of the text.
    pub icon: Option<String>,
}

impl ActionButton {
    /// Creates a button.
    pub fn new(action: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            action: action.into(),
            label: label.into(),
            icon: None,
        }
    }

    /// Sets the icon, for example `"folder-open-symbolic"`.
    pub fn icon(mut self, icon_name: impl Into<String>) -> Self {
        self.icon = Some(icon_name.into());
        self
    }
}

impl From<ActionButton> for Block {
    fn from(button: ActionButton) -> Self {
        Block::Button(button)
    }
}

/// A numbered step with an optional second line.
#[derive(Clone, Debug)]
pub struct Step {
    /// The text of the step.
    pub title: String,
    /// An optional second line with details.
    pub detail: Option<String>,
}

impl Step {
    /// Creates a step without detail.
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            detail: None,
        }
    }

    /// Sets the second line.
    pub fn detail(mut self, text: impl Into<String>) -> Self {
        self.detail = Some(text.into());
        self
    }
}

impl From<&str> for Step {
    fn from(s: &str) -> Self {
        Step::new(s)
    }
}

impl From<String> for Step {
    fn from(s: String) -> Self {
        Step::new(s)
    }
}

/// The kind of a [`Callout`]; it sets the color and the icon.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CalloutKind {
    /// Neutral information.
    Info,
    /// A useful tip.
    Tip,
    /// Something that needs care.
    Warning,
    /// Something that can cause damage or data loss.
    Danger,
}

/// A highlighted note box.
#[derive(Clone, Debug)]
pub struct Callout {
    /// The kind of the note.
    pub kind: CalloutKind,
    /// An optional heading.
    pub title: Option<String>,
    /// The text of the note.
    pub text: String,
}

impl Callout {
    fn with(kind: CalloutKind, text: impl Into<String>) -> Self {
        Self {
            kind,
            title: None,
            text: text.into(),
        }
    }

    /// Creates a neutral note.
    pub fn info(text: impl Into<String>) -> Self {
        Self::with(CalloutKind::Info, text)
    }

    /// Creates a tip.
    pub fn tip(text: impl Into<String>) -> Self {
        Self::with(CalloutKind::Tip, text)
    }

    /// Creates a warning.
    pub fn warning(text: impl Into<String>) -> Self {
        Self::with(CalloutKind::Warning, text)
    }

    /// Creates a danger note.
    pub fn danger(text: impl Into<String>) -> Self {
        Self::with(CalloutKind::Danger, text)
    }

    /// Sets a heading for the note.
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }
}

/// Where the data of an [`Image`] comes from.
#[derive(Clone, Debug)]
pub enum ImageSource {
    /// A file in the file system.
    File(PathBuf),
    /// A path in a GResource that is part of the application (recommended for Flatpak,
    /// Snap, deb, rpm and AppImage: nothing has to be installed next to the binary).
    Resource(String),
    /// An already loaded texture.
    Texture(gdk::Texture),
}

/// A screenshot or illustration.
#[derive(Clone, Debug)]
pub struct Image {
    /// Where the image comes from.
    pub source: ImageSource,
    /// An optional caption below the image.
    pub caption: Option<String>,
    /// Alternative text for screen readers — please always set it.
    pub alt: Option<String>,
    /// The maximum display height in pixels (prevents huge portrait screenshots).
    pub max_height: i32,
}

impl Image {
    fn with(source: ImageSource) -> Self {
        Self {
            source,
            caption: None,
            alt: None,
            max_height: 420,
        }
    }

    /// An image from a file.
    pub fn from_file(path: impl Into<PathBuf>) -> Self {
        Self::with(ImageSource::File(path.into()))
    }

    /// An image from a GResource path, for example `"/org/example/App/guides/step1.png"`.
    pub fn from_resource(path: impl Into<String>) -> Self {
        Self::with(ImageSource::Resource(path.into()))
    }

    /// An image from a texture that is already loaded.
    pub fn from_texture(texture: gdk::Texture) -> Self {
        Self::with(ImageSource::Texture(texture))
    }

    /// Sets the caption below the image.
    pub fn caption(mut self, text: impl Into<String>) -> Self {
        self.caption = Some(text.into());
        self
    }

    /// Sets the alternative text for screen readers.
    pub fn alt(mut self, text: impl Into<String>) -> Self {
        self.alt = Some(text.into());
        self
    }

    /// Sets the maximum display height in pixels (default 420).
    pub fn max_height(mut self, pixels: i32) -> Self {
        self.max_height = pixels;
        self
    }
}
