/**
* delete_directory.rs
*
* (C) 2026 AtlantisOS Project
* by @NachtsternBuild
*
* License: GNU GENERAL PUBLIC LICENSE Version 3
*
* @brief Rmeove a empty directory
*
* Usage:
* fn main() {
*    delete_directory("/tmp/test_dir");
* }
*/

use std::fs;
use std::path::Path;
use std::process;

/** 
* @brief Delete directroy
*/
pub fn delete_directory<P: AsRef<Path>>(path: P) {
    let path_ref = path.as_ref();

    match fs::remove_dir(path_ref) {
        Ok(_) => {
            println!("Directory deleted: {:?}", path_ref);
        }
        Err(e) => {
            eprintln!(
                "Error deleting directory {:?}: {} (Note: Only empty directories can be deleted.)",
                path_ref, e
            );
            process::exit(1);
        }
    }
}
