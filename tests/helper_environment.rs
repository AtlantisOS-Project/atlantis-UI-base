// helper_environment.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! Tests of the detection of the runtime environment (Flatpak, Snap, AppImage, deb, …).
//!
//! The environment variables are changed, so everything is one sequential test.

use atlbase::prelude::*;
use std::fs;
use std::path::Path;

#[test]
fn the_environment_is_detected_from_markers() {
    // These tests simulate the markers of the formats; the Flatpak marker is a file in the
    // root directory and cannot be faked.
    if Path::new("/.flatpak-info").exists() {
        assert_eq!(get_execution_environment(), "flatpak");
        assert_eq!(
            check_application_environment("x", "/nonexistent"),
            AppEnvironment::Flatpak
        );
        return;
    }

    // SAFETY: this is the only test of the process, so no other thread reads the environment.
    unsafe {
        std::env::remove_var("SNAP");
        std::env::remove_var("APPIMAGE");
    }
    assert_eq!(get_execution_environment(), "native");

    // deb / rpm: decided by the APT sources
    let sources = tempfile::tempdir().unwrap();
    let dir = sources.path().to_str().unwrap();
    assert_eq!(
        check_application_environment("my-app", dir),
        AppEnvironment::LocalDeb
    );
    fs::write(sources.path().join("my-app-ppa.list"), "").unwrap();
    assert_eq!(
        check_application_environment("my-app", dir),
        AppEnvironment::Ppa
    );
    // a system without APT (rpm, Arch, …) is unknown; it must not terminate the process
    assert_eq!(
        check_application_environment("my-app", "/does/not/exist"),
        AppEnvironment::Unknown
    );

    unsafe { std::env::set_var("APPIMAGE", "/home/user/App.AppImage") };
    assert_eq!(get_execution_environment(), "appimage");
    assert_eq!(
        check_application_environment("my-app", dir),
        AppEnvironment::AppImage
    );

    unsafe { std::env::set_var("SNAP", "/snap/my-app/1") };
    assert_eq!(get_execution_environment(), "snap");
    assert_eq!(
        check_application_environment("my-app", dir),
        AppEnvironment::Snap
    );
}
