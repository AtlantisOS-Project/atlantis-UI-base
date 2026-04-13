/**
 * open_url.rs
 *
 * (C) 2026 AtlantisOS Project
 * by @NachtsternBuild
 *
 * License: GNU GENERAL PUBLIC LICENSE Version 3
 *
 * Open a URL on the system
 *
 * Usage:
 * fn main() {
 *  let target = "google.com";  
 *  open_url(target);
 * }
 */
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
