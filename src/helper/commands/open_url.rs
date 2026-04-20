//! Utility for opening web links in the default browser.
//!
//! This function abstracts platform-specific calls to open a URL 
//! or file path using the user's preferred application.
/**
 * open_url.rs
 *
 * (C) 2026 AtlantisOS Project
 * by @NachtsternBuild
 *
 * License: GNU GENERAL PUBLIC LICENSE Version 3
 */

// Opens a specified URL in the system's default web browser.
///
/// The function uses the system's native mechanisms (e.g., `xdg-open` on Linux, 
/// `open` on macOS, or `start` on Windows) to process the request.
///
/// # Arguments
/// * `url` - The target link as a string slice (e.g., "<https://example.org>").
///
/// # Functionality
/// - **Validation:** If the passed string is empty, the function terminates immediately.
/// - **Execution:** Attempts to pass the URL asynchronously to the operating system.
/// - **Logging:** Writes a debug message to the console upon success and, in case of 
///   errors, a detailed error message to `stderr`.
///
/// # Error Handling
/// If no browser is found or the URL has an invalid format, 
/// the operating system's error message is captured and displayed.
///
/// # Usage:
///
/// ```rust	
/// fn main() {
///  let target = "google.com";  
///  open_url(target);
/// }
/// ```
pub fn open_url(url: &str) {
    // check if url is empty
    if url.is_empty() {
        return;
    }

    // open the url
    match open::that(url) {
        Ok(_) => {
            println!("[DEBUG] URL opened successfully: {}", url);
        }
        Err(e) => {
            eprintln!("[ERROR] Could not open '{}' nicht öffnen: {}", url, e);
        }
    }
}
