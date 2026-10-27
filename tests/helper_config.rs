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

    // only the user can read the file (it may hold a token)
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let file = confy_file(config_home.path(), app);
        assert_eq!(
            fs::metadata(file).unwrap().permissions().mode() & 0o777,
            0o600
        );
    }

    // a corrupt file does not block the application forever, and what was in it is kept
    let file = fs::read_dir(config_home.path().join(app))
        .unwrap()
        .next()
        .unwrap()
        .unwrap()
        .path();
    let damaged = "[data]\ntheme = \"light\"\nport = \"80";
    fs::write(&file, damaged).unwrap();
    assert_eq!(get_config(app, "theme"), None);
    set_config(app, "theme", "dark").unwrap();
    assert_eq!(get_config(app, "theme").as_deref(), Some("dark"));
    let mut bad = file.as_os_str().to_owned();
    bad.push(".bad");
    assert_eq!(fs::read_to_string(bad).unwrap(), damaged);

    // no temporary file is left in the folder
    let names: Vec<_> = fs::read_dir(config_home.path().join(app))
        .unwrap()
        .map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned())
        .collect();
    assert_eq!(names.len(), 2, "{names:?}");
}

/// The settings file of `app` below `config_home`.
fn confy_file(config_home: &std::path::Path, app: &str) -> std::path::PathBuf {
    fs::read_dir(config_home.join(app))
        .unwrap()
        .next()
        .unwrap()
        .unwrap()
        .path()
}
