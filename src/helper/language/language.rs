/**
* language.rs
*
* (C) Copyright 2026 AtlantisOS Project
* by @NachtsternBuild
*
* License: GNU GENERAL PUBLIC LICENSE Version 3
*
* Provides gettext binding and translation setup
*/

use gettextrs::{bind_textdomain_codeset, bindtextdomain, setlocale, textdomain, LocaleCategory};
use std::env;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use lazy_static::lazy_static;

// Die Domain, die wir in der build.rs gesetzt haben
pub const LIB_DOMAIN: &str = env!("LIB_DOMAIN");

lazy_static! {
    // at startup creating a empty path
    static ref CURRENT_LOCALEDIR: Mutex<PathBuf> = Mutex::new(PathBuf::new());
}

// function to set the default langauge dir
pub fn set_language_dir(dir: &str) {
    let path = Path::new(dir);
    if path.is_dir() {
        let mut locale_dir = CURRENT_LOCALEDIR.lock().unwrap();
        *locale_dir = path.to_path_buf();
    }
}

// function that returns the current language dir
pub fn get_language_dir() -> PathBuf {
    CURRENT_LOCALEDIR.lock().unwrap().clone()
}

/**
* @brief Init the gettext system 
*/
pub fn init_language(domain: &str, default_dir: &str, debug_lang: bool) {
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
