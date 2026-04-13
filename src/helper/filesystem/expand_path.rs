/**
 * search_file_directory.rs
 *
 * (C) Copyright 2026 AtlantisOS Project
 * by @NachtsternBuild
 *
 * License: GNU GENERAL PUBLIC LICENSE Version 3
 *
 * @brief Expand '~' to $HOME
 *
 * fn main() {
 *   let local_conf = "~/config/settings.toml";
 *   let expanded = expand_path(local_conf);  
 *   println!("Original: {}", local_conf);
 *   println!("Expanded: {:?}", expanded);
 * }
 */

use std::env;
use std::path::{Path, PathBuf};

/**
 * @brief Expand '~' to $HOME
 */
pub fn expand_path<P: AsRef<Path>>(path: P) -> PathBuf {
    let p = path.as_ref();

    // check if the path starts with '~'
    if !p.starts_with("~") {
        return p.to_path_buf();
    }

    // get the $HOME from the env
    match env::var_os("HOME") {
        None => {
            // if $HOME not set use the path with ~
            p.to_path_buf()
        }
        Some(home) => {
            // remove the ~ and use the rest of the path with everything $HOME
            if let Ok(suffix) = p.strip_prefix("~") {
                let mut full_path = PathBuf::from(home);
                full_path.push(suffix);
                full_path
            } 
            
            else {
                p.to_path_buf()
            }
        }
    }
}
