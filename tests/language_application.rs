// language_application.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! An application can ship its own catalog, and it wins over the texts of the library.
//!
//! The catalog is compiled with `msgfmt` at test time; the test is skipped if `msgfmt` or the
//! German glibc locale is missing. One sequential test (global gettext state).

use atlbase::gettext;
use atlbase::prelude::*;
use std::fs;
use std::process::Command;

#[test]
fn the_application_domain_overrides_the_library_domain() {
    let root = tempfile::tempdir().unwrap();
    let po = root.path().join("app.po");
    fs::write(
        &po,
        "msgid \"\"\nmsgstr \"\"\n\"Content-Type: text/plain; charset=UTF-8\\n\"\n\n\
         msgid \"Back\"\nmsgstr \"Retour\"\n\n\
         msgid \"Hello\"\nmsgstr \"Hallo\"\n",
    )
    .unwrap();

    let locale_dir = root.path().join("locale");
    let mo_dir = locale_dir.join("de/LC_MESSAGES");
    fs::create_dir_all(&mo_dir).unwrap();
    let compiled = Command::new("msgfmt")
        .arg(&po)
        .arg("-o")
        .arg(mo_dir.join("test-app.mo"))
        .status();
    if !matches!(compiled, Ok(status) if status.success()) {
        eprintln!("skipped: msgfmt is not available");
        return;
    }

    // SAFETY: this is the only test of the process, so no other thread reads the environment.
    unsafe {
        std::env::set_var("XDG_CACHE_HOME", root.path().join("cache"));
        std::env::set_var("LC_ALL", "de_DE.UTF-8");
        std::env::set_var("LANGUAGE", "de");
        std::env::remove_var("ATL_LOCALEDIR");
    }

    // the directory with the catalog is found because it contains `test-app.mo`
    init_language("test-app", locale_dir.to_str().unwrap(), false);
    assert_eq!(get_language_dir(), locale_dir);

    if gettextrs::setlocale(gettextrs::LocaleCategory::LcAll, "de_DE.UTF-8").is_none() {
        eprintln!("skipped: the locale de_DE.UTF-8 is not installed");
        return;
    }

    assert_eq!(gettext!("Hello"), "Hallo", "text of the application");
    assert_eq!(
        gettext!("Back"),
        "Retour",
        "the application overrides the library"
    );
    assert_eq!(
        gettext!("Next"),
        "Weiter",
        "the rest comes from the library"
    );
}
