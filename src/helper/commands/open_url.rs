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
/// Only the schemes `http`, `https`, `mailto`, `file` and `tel` are opened. Any other scheme
/// (`javascript:`, `ssh:`, the handlers other programs register) is refused with an error line, so
/// an address from outside cannot start a program the application did not think of. A path that
/// begins with a word and a colon (`notes:2026.txt`) counts as an address: write `./notes:2026.txt`.
/// A path like `/home/me/a:b` is a path.
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
    let uri = match target_uri(url) {
        Ok(uri) => uri,
        Err(why) => {
            eprintln!("[ERROR] Could not open '{}': {}", url, why);
            return;
        }
    };

    match gio::AppInfo::launch_default_for_uri(&uri, None::<&gio::AppLaunchContext>) {
        Ok(()) => println!("[DEBUG] URL opened successfully: {}", url),
        Err(e) => eprintln!("[ERROR] Could not open '{}': {}", url, e),
    }
}

/// The schemes that are opened.
const SCHEMES: [&str; 5] = ["http", "https", "mailto", "file", "tel"];

/// The scheme of an address (`https` of `https://example.org`): letters, digits, `+`, `-` and `.`,
/// starting with a letter, and at least two characters (a single letter is a drive).
fn scheme_of(text: &str) -> Option<&str> {
    let (scheme, _) = text.split_once(':')?;
    let mut chars = scheme.chars();
    let first = chars.next()?;
    (scheme.len() >= 2
        && first.is_ascii_alphabetic()
        && chars.all(|c| c.is_ascii_alphanumeric() || "+-.".contains(c)))
    .then_some(scheme)
}

/// The URI to open for `text`: an address of an allowed scheme as it is, a path as a `file:` URI.
fn target_uri(text: &str) -> Result<String, String> {
    match scheme_of(text) {
        _ if is_windows_path(text) => Ok(gio::File::for_path(text).uri().to_string()),
        Some(scheme) if SCHEMES.contains(&scheme.to_ascii_lowercase().as_str()) => {
            Ok(text.to_string())
        }
        Some(scheme) => Err(format!("the scheme `{scheme}:` is not opened")),
        None => Ok(gio::File::for_path(text).uri().to_string()),
    }
}

/// `C:\folder`: a drive letter and a colon is a path, not a scheme.
fn is_windows_path(text: &str) -> bool {
    let mut chars = text.chars();
    matches!((chars.next(), chars.next()), (Some(letter), Some(':')) if letter.is_ascii_alphabetic())
}

#[cfg(test)]
mod tests {
    use super::{is_windows_path, target_uri};

    #[test]
    fn only_known_schemes_are_opened() {
        for address in [
            "https://example.org/a?b=c",
            "HTTP://example.org",
            "mailto:me@example.org",
            "file:///home/me/a.txt",
            "tel:+4930123",
        ] {
            assert_eq!(target_uri(address).unwrap(), address);
        }
        for refused in [
            "javascript:alert(1)",
            "ssh://host",
            "vscode://file/etc/passwd",
            "data:text/html,x",
            "notes:2026.txt",
        ] {
            assert!(target_uri(refused).is_err(), "{refused}");
        }
    }

    #[test]
    fn a_path_is_a_path_even_with_a_colon() {
        for path in ["/home/me/a:b/c", "./notes:2026.txt", "a:b.txt", "/tmp/x"] {
            let uri = target_uri(path).unwrap();
            assert!(uri.starts_with("file://"), "{path} -> {uri}");
        }
    }

    #[test]
    fn a_drive_letter_is_a_path_and_a_scheme_is_not() {
        assert!(is_windows_path("C:\\Users"));
        assert!(!is_windows_path("https://example.org"));
        assert!(!is_windows_path("mailto:me@example.org"));
    }
}
