//! Localization and translation management via gettext.
//!
//! This module encapsulates the `gettextrs` bindings and automates the search for 
//! translation files (.mo). It supports various installation layouts 
//! (standard Linux, Snap, local development) and provides thread-safe access 
//! to the current configuration.
/**
* language.rs
*
* (C) Copyright 2026 AtlantisOS Project
* by @NachtsternBuild
*
* License: GNU GENERAL PUBLIC LICENSE Version 3
*/

use gettextrs::{bind_textdomain_codeset, bindtextdomain, setlocale, textdomain, LocaleCategory};
use std::env;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    /// Global variable storing the current path to the locale directory.
    static ref CURRENT_LOCALEDIR: Mutex<PathBuf> = Mutex::new(PathBuf::new());

    /// Global variable for the current text domain (usually the project name).
    static ref CURRENT_DOMAIN: Mutex<String> = Mutex::new(String::new());
}

/// Manually sets the path to the directory containing the translation files.
/// 
/// The path is only applied if it exists in the file system and is a directory.
pub fn set_language_dir(dir: &str) {
    let path = Path::new(dir);
    if path.is_dir() {
        let mut locale_dir = CURRENT_LOCALEDIR.lock().unwrap();
        *locale_dir = path.to_path_buf();
    }
}

/// Returns a copy of the currently set locale directory.
pub fn get_language_dir() -> PathBuf {
    CURRENT_LOCALEDIR.lock().unwrap().clone()
}

/// Returns the currently registered text domain.
pub fn get_current_domain() -> String {
    CURRENT_DOMAIN.lock().unwrap().clone()
}

/// Initializes the gettext system and automatically searches for the best translation directory.
///
/// # How it works
/// The function prioritizes paths in the following order:
/// 1. Environment variable `ATL_LOCALEDIR`
/// 2. Local directory `./po` (for development)
/// 3. The passed `default_dir`
/// 4. System-wide directory `/usr/share/locale`
/// 5. Specific paths within a **Snap sandbox**
///
/// # Arguments
/// * `domain` - The application’s text domain (e.g., "atlantis-shell").
/// * `default_dir` - A fallback directory (often determined via `build.rs`).
/// * `debug_lang` - If `true`, the language is hard-coded to `en_US.UTF-8`.
///
/// # Security
/// The use of `unsafe` for `env::set_var` is necessary in debug mode, 
/// since environment variables cannot be modified in a thread-safe manner.
///
/// # Usage:
///
/// ```rust
/// let test_dir = env!("COMPILED_LOCALE_DIR");
/// language::init_language(language::LIB_DOMAIN, test_dir, false);
/// ```
pub fn init_language(domain: &str, default_dir: &str, debug_lang: bool) {
    let mut d = CURRENT_DOMAIN.lock().unwrap();
    *d = domain.to_string();
    
    // for debugging
    if debug_lang {
    	// Note: Use this only for debugging
		// try setting env
        unsafe {
            env::set_var("LC_ALL", "en_US.UTF-8");
            env::set_var("LANG", "en_US.UTF-8");
        }
        setlocale(LocaleCategory::LcAll, "en_US.UTF-8");
    } 
    
    else {
        setlocale(LocaleCategory::LcAll, "");
    }

    // logic for the paths
    // get path from env (ATL_LOCALEDIR)
    let selected_dir = if let Ok(envdir) = env::var("ATL_LOCALEDIR") {
        if Path::new(&envdir).is_dir() { 
        	Some(envdir) 
        } 
        
        else { 
        	None 
        }
    } 
    
    // use local dir (./po)
    else if Path::new("./po").is_dir() {
        Some("./po".to_string())
    } 
    
    // get the default dir at with init
    else if Path::new(default_dir).is_dir() {
        Some(default_dir.to_string())
    } 
    
    // use /usr/share/locale
    else if Path::new("/usr/share/locale").is_dir() {
    	Some("/usr/share/locale".to_string())
    }
    
    // special case is snap
    else if let Ok(snap_path) = env::var("SNAP").or_else(|_| env::var("SNAP_NAME")) {
        // check if default dir exists
        if Path::new(default_dir).is_dir() { 
            let snap_dir = format!("{}/{}", snap_path, default_dir);
            if Path::new(&snap_dir).is_dir() {  
                Some(snap_dir)  
            }  
            else {  
                None  
            }
        }
        // use $SNAP/usr/share/locale
        else {
            let snap_dir = format!("{}/usr/share/locale", snap_path);
            if Path::new(&snap_dir).is_dir() {  
                Some(snap_dir)  
            }  
            else {  
                None  
            }
        }
    }
    
    // use default dir without everything
    else {
        Some(default_dir.to_string())
    };
	
	// apply selected dir
    if let Some(dir) = selected_dir {
        set_language_dir(&dir);
    }

    let final_dir = get_language_dir();
    let dir_str = final_dir.to_str().unwrap_or(default_dir);

    // run gettext bindings
    bindtextdomain(domain, dir_str).expect("Failed to bind textdomain");
    bind_textdomain_codeset(domain, "UTF-8").expect("Failed to set codeset");
    textdomain(domain).expect("Failed to set textdomain");

    println!("[INFO] Locale init: domain='{}', dir='{}'", domain, dir_str);
}
