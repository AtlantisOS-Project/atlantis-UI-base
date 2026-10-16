// helper_home_protection.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! The deletion helpers must protect the home directory and its standard folders.
//!
//! The test points `$HOME` to a temporary directory, so nothing real is at risk. It lives in
//! its own file (its own process) because it changes the environment.

use atlbase::prelude::*;
use std::fs;

#[test]
fn home_and_standard_folders_are_protected() {
    let home = tempfile::tempdir().unwrap();
    // SAFETY: this is the only test of the process, so no other thread reads the environment.
    unsafe { std::env::set_var("HOME", home.path()) };

    let documents = home.path().join("Documents");
    fs::create_dir(&documents).unwrap();
    fs::write(documents.join("thesis.txt"), "important").unwrap();
    fs::write(home.path().join("notes.txt"), "important").unwrap();

    delete_files_in_dir(&documents);
    delete_files_in_dir(home.path());
    delete_files_with_parent(documents.to_str().unwrap());
    delete_files_with_parent(home.path().to_str().unwrap());

    assert!(documents.join("thesis.txt").exists());
    assert!(home.path().join("notes.txt").exists());

    // other folders in the home directory are fine, and the climb stops at the home directory
    let cache = home.path().join(".cache/myapp");
    fs::create_dir_all(&cache).unwrap();
    fs::write(cache.join("data"), "x").unwrap();
    delete_files_with_parent(cache.to_str().unwrap());

    assert!(!cache.exists());
    assert!(!home.path().join(".cache").exists());
    assert!(home.path().exists());
}
