// app_icon.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! Finding the icon of the application.
//!
//! The "About" dialog, the window and the task switcher show the icon by its *name*
//! (`org.example.Demo`), which GTK looks up in the icon theme. That works when the icon is
//! installed in `<prefix>/share/icons/hicolor/…` and the prefix is in `$XDG_DATA_DIRS`. It
//! does not work when the program runs from the source tree (`cargo run`), from an AppDir
//! (AppImage) or from a prefix that is not in `$XDG_DATA_DIRS`: the dialog then shows a
//! "missing image" icon. [`ensure_app_icon`] closes these gaps.

use crate::helper::platform::dirs::{AppDirs, DataDirs};
use gtk4::{IconTheme, gdk};
use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// The sizes of the `hicolor` theme; a PNG file goes to the closest one.
const SIZES: [i32; 13] = [16, 22, 24, 32, 36, 48, 64, 72, 96, 128, 192, 256, 512];

/// Folders with icons that a sandbox or a relocated installation uses.
fn icon_dirs() -> Vec<PathBuf> {
    let places = DataDirs::current();
    let mut dirs: Vec<PathBuf> = Vec::new();
    let mut push = |dir: PathBuf| {
        if !dirs.contains(&dir) {
            dirs.push(dir);
        }
    };
    // AppImage: the AppDir; Snap: the root of the snap
    for root in places.app_roots() {
        push(root.join("usr/share/icons"));
    }
    // Flatpak and the usual prefixes, in case $XDG_DATA_DIRS lacks them
    push(PathBuf::from("/app/share/icons"));
    push(PathBuf::from("/usr/local/share/icons"));
    push(PathBuf::from("/usr/share/icons"));
    // <prefix>/bin/<program> with <prefix>/share/icons next to it
    if let Some(prefix) = env::current_exe()
        .ok()
        .and_then(|exe| exe.parent()?.parent().map(Path::to_path_buf))
    {
        push(prefix.join("share/icons"));
    }
    dirs
}

/// The folder in the `hicolor` theme for a file: `scalable` for SVG, otherwise the standard
/// size that is closest to the width of the image.
fn theme_dir(file: &Path) -> (String, &'static str) {
    let extension = file
        .extension()
        .and_then(|extension| extension.to_str())
        .unwrap_or_default()
        .to_ascii_lowercase();
    if extension == "svg" || extension == "svgz" {
        return ("scalable".to_string(), "svg");
    }
    let width = gtk4::gdk_pixbuf::Pixbuf::file_info(file)
        .map(|(_, width, _)| width)
        .unwrap_or(256);
    let size = SIZES
        .iter()
        .min_by_key(|size| (**size - width).abs())
        .copied()
        .unwrap_or(256);
    (format!("{size}x{size}"), "png")
}

/// Copies `file` into a private `hicolor` theme in the cache directory of the user and
/// returns the folder to add to the search path of the icon theme.
fn install_icon_file(icon_name: &str, file: &Path) -> io::Result<PathBuf> {
    let cache = AppDirs::new(crate::helper::language::language::LIB_DOMAIN)
        .map(|dirs| dirs.cache_dir().join("icons"))
        .ok_or_else(|| io::Error::other("no cache directory"))?;

    let (size, extension) = theme_dir(file);
    let target = cache
        .join("hicolor")
        .join(size)
        .join("apps")
        .join(format!("{icon_name}.{extension}"));
    fs::create_dir_all(target.parent().expect("a file in a folder"))?;
    fs::copy(file, &target)?;
    Ok(cache)
}

/// Makes sure that the icon theme finds the icon `icon_name`, and sets it as the icon of all
/// windows.
///
/// The icon is looked up in this order; the first place that has it wins:
///
/// 1. the icon theme as it is (an installed program: `<prefix>/share/icons/hicolor/…` with
///    the prefix in `$XDG_DATA_DIRS`, also in Flatpak),
/// 2. the icon folders of the package formats: `$APPDIR/usr/share/icons` (AppImage),
///    `$SNAP/usr/share/icons` (Snap), `/app/share/icons`, `/usr/share/icons`,
///    `/usr/local/share/icons` and `<prefix>/share/icons` next to the folder of the program,
/// 3. `icon_file`: an SVG or PNG file (for example in the source tree while developing). It
///    is copied into a private icon theme in the cache directory of the user.
///
/// [`show_about_dialog`](crate::design::dialogs::about_dialog::show_about_dialog) calls this
/// function for you with the fields `app_icon` and `icon_file` of
/// [`AboutInfo`](crate::design::dialogs::about_dialog::AboutInfo). Call it yourself once at
/// the start (after GTK was initialized) to give the windows their icon, too.
///
/// # Returns
///
/// `true` if the icon is found now. If not, a warning names the places that were searched and
/// the windows keep the default icon.
///
/// # Examples
///
/// ```no_run
/// use atlbase::ui_prelude::ensure_app_icon;
/// use std::path::Path;
/// # gtk4::init().unwrap();
///
/// // installed: found in the icon theme; `cargo run`: taken from the file
/// ensure_app_icon("org.example.Demo", Some(Path::new("assets/org.example.Demo.svg")));
/// ```
pub fn ensure_app_icon(icon_name: &str, icon_file: Option<&Path>) -> bool {
    let Some(display) = gdk::Display::default() else {
        return false;
    };
    let theme = IconTheme::for_display(&display);

    if !theme.has_icon(icon_name) {
        for dir in icon_dirs().iter().filter(|dir| dir.is_dir()) {
            theme.add_search_path(dir);
            if theme.has_icon(icon_name) {
                break;
            }
        }
    }
    if !theme.has_icon(icon_name)
        && let Some(file) = icon_file.filter(|file| file.is_file())
    {
        match install_icon_file(icon_name, file) {
            Ok(dir) => theme.add_search_path(&dir),
            Err(error) => log::warn!("cannot use the icon file {}: {error}", file.display()),
        }
    }

    let found = theme.has_icon(icon_name);
    if found {
        gtk4::Window::set_default_icon_name(icon_name);
    } else {
        log::warn!(
            "the icon '{icon_name}' was not found in the icon theme, in {:?} or in the icon file",
            icon_dirs()
        );
    }
    found
}
