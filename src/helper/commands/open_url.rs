// open_url.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! Opens web links and files with the default application.
//!
//! The function asks GIO to open the target with the default application of the desktop, so it
//! needs no `xdg-open` and works through the portal in a Flatpak and Snap.

use gio::prelude::FileExt;

/// Opens a URL or file path with the default application of the system.
///
/// GIO does it (the portal *OpenURI* in a Flatpak or Snap, the handlers of the desktop elsewhere;
/// Windows and macOS have their own). A file path without a scheme is turned into a `file:` URI.
/// An empty string does nothing. The result is reported on the console: a debug line on success,
/// an error line on `stderr` if no handler was found or the URL is invalid.
///
/// # Arguments
///
/// * `url` - The target as a string slice, for example `"https://example.org"`.
///
/// # Packaging
///
/// No extra permissions are needed in Flatpak (the *OpenURI* portal) or Snap (the `desktop`
/// interface), and no tool of the system: the program does not start `xdg-open`.
///
/// # Examples
///
/// ```no_run
/// use atlbase::prelude::open_url;
///
/// open_url("https://example.org");
/// ```
pub fn open_url(url: &str) {
    if url.is_empty() {
        return;
    }
    let uri = if url.contains(':') && !is_windows_path(url) {
        url.to_string()
    } else {
        gio::File::for_path(url).uri().to_string()
    };

    match gio::AppInfo::launch_default_for_uri(&uri, None::<&gio::AppLaunchContext>) {
        Ok(()) => println!("[DEBUG] URL opened successfully: {}", url),
        Err(e) => eprintln!("[ERROR] Could not open '{}': {}", url, e),
    }
}

/// `C:\folder`: a drive letter and a colon is a path, not a scheme.
fn is_windows_path(text: &str) -> bool {
    let mut chars = text.chars();
    matches!((chars.next(), chars.next()), (Some(letter), Some(':')) if letter.is_ascii_alphabetic())
}

#[cfg(test)]
mod tests {
    use super::is_windows_path;

    #[test]
    fn a_drive_letter_is_a_path_and_a_scheme_is_not() {
        assert!(is_windows_path("C:\\Users"));
        assert!(!is_windows_path("https://example.org"));
        assert!(!is_windows_path("mailto:me@example.org"));
    }
}
