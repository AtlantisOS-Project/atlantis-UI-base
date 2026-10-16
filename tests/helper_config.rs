// helper_config.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! Tests of the TOML settings (`get_config`, `set_config`).
//!
//! `$XDG_CONFIG_HOME` points to a temporary directory, which is also how Flatpak and Snap
//! redirect the settings of an application. One sequential test, because the environment is
//! changed.

use atlbase::prelude::*;
use std::fs;

#[test]
fn settings_are_stored_below_xdg_config_home() {
    let config_home = tempfile::tempdir().unwrap();
    // SAFETY: this is the only test of the process, so no other thread reads the environment.
    unsafe { std::env::set_var("XDG_CONFIG_HOME", config_home.path()) };

    let app = "atlbase-test-app";
    assert_eq!(get_config(app, "theme"), None, "no file yet");

    set_config(app, "theme", "dark").unwrap();
    set_config(app, "port", "8080").unwrap();
    set_config(app, "theme", "light").unwrap();

    assert_eq!(get_config(app, "theme").as_deref(), Some("light"));
    assert_eq!(get_config(app, "port").as_deref(), Some("8080"));
    assert_eq!(get_config(app, "other"), None);
    assert!(
        config_home.path().join(app).is_dir(),
        "the file lives in the XDG directory"
    );

    // a corrupt file is replaced instead of blocking the application forever
    let file = fs::read_dir(config_home.path().join(app))
        .unwrap()
        .next()
        .unwrap()
        .unwrap()
        .path();
    fs::write(&file, "this is = not [valid toml").unwrap();
    assert_eq!(get_config(app, "theme"), None);
    set_config(app, "theme", "dark").unwrap();
    assert_eq!(get_config(app, "theme").as_deref(), Some("dark"));
}
