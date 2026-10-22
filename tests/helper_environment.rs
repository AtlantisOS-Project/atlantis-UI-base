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
        std::env::remove_var("FLATPAK_ID");
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

    // $SNAP counts only if the program lives below it: `cargo run` with the cargo of the rustup
    // Snap inherits $SNAP, and the program is not in that Snap
    unsafe { std::env::remove_var("APPIMAGE") };
    unsafe { std::env::set_var("SNAP", "/snap/my-app/1") };
    assert_eq!(get_execution_environment(), "native");
    let here = std::env::current_exe().unwrap();
    unsafe { std::env::set_var("SNAP", here.parent().unwrap()) };
    assert_eq!(get_execution_environment(), "snap");
    assert_eq!(
        check_application_environment("my-app", dir),
        AppEnvironment::Snap
    );

    // $FLATPAK_ID alone is enough (it is set in a Flatpak, and by `-debug -flp`)
    unsafe {
        std::env::remove_var("SNAP");
        std::env::set_var("FLATPAK_ID", "org.example.App");
    }
    assert_eq!(get_execution_environment(), "flatpak");
    unsafe { std::env::remove_var("FLATPAK_ID") };
}

#[test]
fn environments_have_a_label_and_a_sandbox_flag() {
    assert_eq!(AppEnvironment::Flatpak.label(), "Flatpak");
    assert_eq!(AppEnvironment::Ppa.label(), "Debian Package PPA");
    assert_eq!(AppEnvironment::Unknown.label(), "Unknown package");

    for (environment, sandboxed) in [
        (AppEnvironment::Flatpak, true),
        (AppEnvironment::Snap, true),
        (AppEnvironment::AppImage, false),
        (AppEnvironment::Ppa, false),
        (AppEnvironment::LocalDeb, false),
        (AppEnvironment::Unknown, false),
    ] {
        assert_eq!(environment.is_sandboxed(), sandboxed, "{environment:?}");
    }
}

#[test]
fn the_current_environment_is_determined_once() {
    // the first call decides, whatever the later calls say
    let first = current_environment("my-app", "/does/not/exist");
    assert_eq!(first, current_environment("other", "/etc"));

    // no panic on any system
    let _ = running_in_wsl();
}
