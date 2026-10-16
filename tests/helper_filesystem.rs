// helper_filesystem.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! Tests of the file system helpers (`atlbase::helper::filesystem`).

use atlbase::prelude::*;
use std::fs;
use std::path::PathBuf;

#[test]
fn directories_are_created_and_checked() {
    let root = tempfile::tempdir().unwrap();

    let nested = root.path().join("a/b/c");
    make_path(&nested).unwrap();
    assert!(directory_exists(&nested));
    assert!(file_exists(&nested));

    // make_path_dirname creates the parent, never the file itself
    let file = root.path().join("x/y/settings.toml");
    make_path_dirname(&file).unwrap();
    assert!(directory_exists(file.parent().unwrap()));
    assert!(!file_exists(&file));
    make_path_dirname("no-directory-part.toml").unwrap();

    // a file is not a directory
    fs::write(&file, "x").unwrap();
    assert!(file_exists(&file));
    assert!(!directory_exists(&file));

    // the fire-and-forget variant
    create_directory(root.path().join("quiet"));
    assert!(directory_exists(root.path().join("quiet")));
}

#[test]
fn searching_file_names() {
    let root = tempfile::tempdir().unwrap();
    fs::write(root.path().join("atlantis.list"), "").unwrap();

    assert!(search_file_directory(root.path(), "atlantis").unwrap());
    assert!(!search_file_directory(root.path(), "other").unwrap());
    assert!(search_file_directory(root.path().join("missing"), "x").is_err());

    let dir = root.path().to_str().unwrap();
    assert!(search_string_file(dir, "atlantis").unwrap());
    assert!(!search_string_file(dir, "other").unwrap());
    // unlike search_file_directory, a missing directory is no error
    assert!(!search_string_file("/does/not/exist", "x").unwrap());
}

#[test]
fn tilde_expansion() {
    assert_eq!(expand_path("/etc/hosts"), PathBuf::from("/etc/hosts"));
    assert_eq!(expand_path("a/~/b"), PathBuf::from("a/~/b"));
    assert_eq!(expand_path("~user/x"), PathBuf::from("~user/x"));

    if let Some(home) = std::env::var_os("HOME") {
        assert_eq!(expand_path("~/x/y"), PathBuf::from(&home).join("x/y"));
        assert_eq!(expand_path("~"), PathBuf::from(home));
    }
}

#[test]
fn delete_directory_removes_an_empty_directory() {
    let root = tempfile::tempdir().unwrap();
    let dir = root.path().join("empty");
    fs::create_dir(&dir).unwrap();

    delete_directory(&dir);
    assert!(!dir.exists());
}

#[test]
fn deleting_files_only_touches_the_given_directory() {
    let root = tempfile::tempdir().unwrap();
    let cache = root.path().join("projects/old/cache");
    fs::create_dir_all(cache.join("sub")).unwrap();
    fs::write(cache.join("a.log"), "a").unwrap();
    fs::write(cache.join("sub/keep.log"), "keep").unwrap();

    delete_files_in_dir(&cache);

    assert!(
        !cache.join("a.log").exists(),
        "files of the directory are deleted"
    );
    assert!(
        cache.join("sub/keep.log").exists(),
        "subdirectories are untouched"
    );
}

#[test]
fn climbing_up_removes_only_empty_directories_and_spares_foreign_files() {
    let root = tempfile::tempdir().unwrap();
    let project = root.path().join("projects/old");
    let cache = project.join("cache");
    fs::create_dir_all(&cache).unwrap();
    fs::write(cache.join("tmp.log"), "log").unwrap();
    // a file in a parent directory must survive, and so must the directory around it
    fs::write(root.path().join("projects/notes.txt"), "notes").unwrap();

    delete_files_and_parents(&cache, &Some(root.path().to_path_buf()));

    assert!(!cache.exists());
    assert!(!project.exists(), "the empty parent is removed");
    assert!(
        root.path().join("projects/notes.txt").exists(),
        "files above are never deleted"
    );
}

#[test]
fn the_stop_directory_is_never_removed() {
    let root = tempfile::tempdir().unwrap();
    let cache = root.path().join("cache");
    fs::create_dir(&cache).unwrap();

    delete_files_and_parents(&cache, &Some(root.path().to_path_buf()));

    assert!(!cache.exists());
    assert!(root.path().exists());
}

#[test]
fn a_missing_path_is_ignored() {
    delete_files_with_parent("/does/not/exist/at/all");
}
