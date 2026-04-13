/**
* get_config_value.rs
*
* (C) Copyright 2026 AtlantisOS Project
* by @NachtsternBuild
*
* License: GNU GENERAL PUBLIC LICENSE Version 3
*
* @brief This get the all values from a config file
* 
* Usage:
* fn main() {
*    let conf_file = "config.conf";
*    if let Some(update_type) = get_config_value(conf_file, "UPDATE_TYPE") {
*        println!("Update Type: {}", update_type);
*    } else {
*        println!("Value not found.");
*    }
* }
*/

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

/** 
* @brief Function that get values from config files
*/
pub fn get_config_value<P: AsRef<Path>>(filename: P, key: &str) -> Option<String> {
    let path = filename.as_ref();

    // path traversal protection
    if path.components().any(|c| c.as_os_str() == "..") {
        eprintln!("Error: Path traversal detected!");
        return None;
    }

    // open file
    let file = File::open(path).map_err(|e| {
        eprintln!("Could not open {:?}: {}", path, e);
        e
    }).ok()?;

    let reader = BufReader::new(file);

    // parse file line by line
    for line_result in reader.lines() {
        let line = line_result.ok()?;
        let trimmed_line = line.trim();

        // check if key exists
        if trimmed_line.starts_with(key) {
            let part_after_key = &trimmed_line[key.len()..];
            
            if part_after_key.starts_with('=') {
                // get value after =
                let mut value = part_after_key[1..].trim();

                // remove " at start and end
                if value.starts_with('"') {
                    value = &value[1..];
                }
                
                if let Some(end_idx) = value.find('"') {
                    value = &value[..end_idx];
                }

                return Some(value.to_string());
            }
        }
    }

    None
}
