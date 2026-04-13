/**
* file_filter.rs
*
* (C) Copyright 2026 AtlantisOS Project
* by @NachtsternBuild
*
* License: GNU GENERAL PUBLIC LICENSE Version 3
*
* Usage:
* GListStore *filter_list = load_file_filters("/path/to/filechooser-filters.conf");
* // now work with the filter
* if let Some(filter_list) = load_file_filters("filechooser-filters.conf") {
*    let model = filter_list.upcast::<gio::ListModel>();
*    file_dialog.set_filters(Some(&model));
* }
*/

use gtk4 as gtk;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use gio::prelude::ListModelExt;

/** 
* @brief Function that load the filter from a config file
* Expected format: name:*.extension,*.ext2
*/
pub fn load_file_filters<P: AsRef<Path>>(config_path: P) -> Option<gio::ListStore> {
    let path = config_path.as_ref();

    // open file
    let file = File::open(path).map_err(|e| {
        eprintln!("Warning: No filter config found: {:?} ({})", path, e);
        e
    }).ok()?;

    //  create gliststore for the filter
    let filter_list = gio::ListStore::new::<gtk::FileFilter>();
    let reader = BufReader::new(file);

    for line_result in reader.lines() {
        let line = line_result.ok()?;
        let trimmed = line.trim();

        // skip empty lines and comments
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        // parse the format
        if let Some((name, patterns_raw)) = trimmed.split_once(':') {
            let filter = gtk::FileFilter::new();
            filter.set_name(Some(name.trim()));

            // cut pattern and add other
            for pattern in patterns_raw.split(',') {
                let p = pattern.trim();
                if !p.is_empty() {
                    filter.add_pattern(p);
                }
            }

            // add the filter to the list
            filter_list.append(&filter);
        }
    }

    // if list is empty
    if filter_list.n_items() == 0 {
        None
    } 
    
    else {
        Some(filter_list)
    }
}
