// language_library.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! The texts of the library are translated without installing anything.
//!
//! The catalogs are compiled into the library; `init_language` writes them to the cache
//! directory of the user (which Flatpak and Snap redirect into the sandbox) and binds them.
//! One sequential test, because the environment and the global gettext state are changed.

use atlbase::gettext;
use atlbase::prelude::*;
use gettextrs::{LocaleCategory, setlocale};
use std::path::Path;

#[test]
fn library_texts_are_translated_from_the_embedded_catalog() {
    let cache = tempfile::tempdir().unwrap();
    // SAFETY: this is the only test of the process, so no other thread reads the environment.
    unsafe {
        std::env::set_var("XDG_CACHE_HOME", cache.path());
        std::env::set_var("LC_ALL", "de_DE.UTF-8");
        std::env::set_var("LANGUAGE", "de");
        std::env::remove_var("ATL_LOCALEDIR");
    }

    // before init_language nothing is translated
    assert_eq!(gettext!("Back"), "Back");

    // the application has no catalog of its own and an unusable locale directory
    init_language("some-app-without-catalog", "/does/not/exist", false);
    assert_eq!(get_current_domain(), "some-app-without-catalog");

    // the glibc locale of the machine decides whether translations can be used at all
    if setlocale(LocaleCategory::LcAll, "de_DE.UTF-8").is_none() {
        eprintln!("skipped: the locale de_DE.UTF-8 is not installed");
        return;
    }
    // (setlocale above only checks; init_language already set the locale from the environment)

    assert_eq!(gettext!("Back"), "Zurück");
    assert_eq!(gettext!("Choose File"), "Datei auswählen");
    assert_eq!(gettext!("Special thanks to"), "Besonderer Dank an");
    // texts nobody translated stay as they are; empty texts stay empty
    assert_eq!(
        gettext!("Some text of the application"),
        "Some text of the application"
    );
    assert_eq!(gettext!(""), "");

    let installed = cache
        .path()
        .join("atlbase")
        .read_dir()
        .expect("the embedded catalogs were written to the cache directory")
        .flatten()
        .any(|entry| {
            Path::new(&entry.path())
                .join("de/LC_MESSAGES")
                .join(format!("{LIB_DOMAIN}.mo"))
                .is_file()
        });
    assert!(
        installed,
        "atlbase.mo was installed below the cache directory"
    );
}
