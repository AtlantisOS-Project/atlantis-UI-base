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

    // the way to a protected folder does not matter: `..`, a link, a link to the home itself
    delete_files_in_dir(&documents.join("../Documents"));
    delete_files_in_dir(&home.path().join(".cache/../Documents"));
    #[cfg(unix)]
    {
        let link = home.path().join("shortcut");
        std::os::unix::fs::symlink(&documents, &link).unwrap();
        delete_files_in_dir(&link);
        delete_files_with_parent(link.to_str().unwrap());
        let to_home = home.path().join("to-home");
        std::os::unix::fs::symlink(home.path(), &to_home).unwrap();
        delete_files_in_dir(&to_home);
        delete_files_and_parents(&to_home, &None);
    }
    assert!(documents.join("thesis.txt").exists());
    assert!(home.path().join("notes.txt").exists());

    // the helpers that delete a whole folder refuse them as well, and delete nothing
    for protected in [
        home.path().to_path_buf(),
        documents.clone(),
        documents.join("../Documents"),
    ] {
        let error = clear_directory_except(&protected, &[]).unwrap_err();
        assert_eq!(error.kind(), std::io::ErrorKind::PermissionDenied);
        let error = remove_dir_all_if_exists(&protected).unwrap_err();
        assert_eq!(error.kind(), std::io::ErrorKind::PermissionDenied);
    }
    assert!(documents.join("thesis.txt").exists());
    assert!(home.path().join("notes.txt").exists());
    // a folder of the application in the home directory is fine
    let work = home.path().join("ROM-Install");
    fs::create_dir(&work).unwrap();
    fs::write(work.join("rom.img"), "x").unwrap();
    clear_directory_except(&work, &[]).unwrap();
    assert!(work.exists() && !work.join("rom.img").exists());
    remove_dir_all_if_exists(&work).unwrap();
    assert!(!work.exists());

    // other folders in the home directory are fine, and the climb stops at the home directory
    let cache = home.path().join(".cache/myapp");
    fs::create_dir_all(&cache).unwrap();
    fs::write(cache.join("data"), "x").unwrap();
    delete_files_with_parent(cache.to_str().unwrap());

    assert!(!cache.exists());
    assert!(!home.path().join(".cache").exists());
    assert!(home.path().exists());
}
