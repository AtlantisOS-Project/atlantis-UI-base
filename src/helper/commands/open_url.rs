// open_url.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! Opens web links and files with the default application.
//!
//! The function abstracts the platform-specific call (`xdg-open`, `open`, `start`).

/// Opens a URL or file path with the default application of the system.
///
/// The system's own mechanism is used (`xdg-open` on Linux, `open` on macOS, `start` on
/// Windows). An empty string does nothing. The result is reported on the console: a debug
/// line on success, an error line on `stderr` if no handler was found or the URL is invalid.
///
/// # Arguments
///
/// * `url` - The target as a string slice, for example `"https://example.org"`.
///
/// # Packaging
///
/// `xdg-open` is part of every Flatpak runtime and forwards the request to the desktop
/// through the *OpenURI* portal; Snap uses the `desktop` interface. No extra permissions
/// are needed in either format.
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

    match open::that(url) {
        Ok(_) => println!("[DEBUG] URL opened successfully: {}", url),
        Err(e) => eprintln!("[ERROR] Could not open '{}': {}", url, e),
    }
}
