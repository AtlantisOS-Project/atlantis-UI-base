// helper_filesystem.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! Tests of the file system helpers (`atlbase::helper::filesystem`).

use atlbase::prelude::*;
use std::fs;
use std::path::{Path, PathBuf};

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
    // a missing directory is an error of kind NotFound: the caller decides what it means
    let missing = search_file_directory(root.path().join("missing"), "x").unwrap_err();
    assert_eq!(missing.kind(), std::io::ErrorKind::NotFound);
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

    delete_directory(&dir).unwrap();
    assert!(!dir.exists());
}

#[test]
fn delete_directory_reports_what_went_wrong_and_does_not_end_the_process() {
    let root = tempfile::tempdir().unwrap();
    // not there
    let error = delete_directory(root.path().join("missing")).unwrap_err();
    assert_eq!(error.kind(), std::io::ErrorKind::NotFound);
    // not empty: nothing is deleted
    let full = root.path().join("full");
    fs::create_dir(&full).unwrap();
    fs::write(full.join("file"), "x").unwrap();
    assert!(delete_directory(&full).is_err());
    assert!(full.join("file").exists());
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

#[test]
fn a_file_is_moved_and_renamed() {
    let root = tempfile::tempdir().unwrap();
    let source = root.path().join("Downloads/lineage-boot.img");
    fs::create_dir_all(source.parent().unwrap()).unwrap();
    fs::write(&source, b"image").unwrap();

    let target = move_file_into(&source, &root.path().join("ROM-Install"), "boot.img").unwrap();
    assert_eq!(target, root.path().join("ROM-Install/boot.img"));
    assert_eq!(fs::read(&target).unwrap(), b"image");
    assert!(!source.exists());

    // moving a file onto itself changes nothing
    assert_eq!(
        move_file_into(&target, target.parent().unwrap(), "boot.img").unwrap(),
        target
    );
    assert_eq!(fs::read(&target).unwrap(), b"image");

    // an existing file with the new name is replaced
    let newer = root.path().join("newer.img");
    fs::write(&newer, b"newer").unwrap();
    move_file_into(&newer, target.parent().unwrap(), "boot.img").unwrap();
    assert_eq!(fs::read(&target).unwrap(), b"newer");

    // a missing source is an error and changes nothing
    assert!(move_file_into(&source, root.path(), "x.img").is_err());
}

#[test]
fn only_a_folder_with_the_right_name_is_touched() {
    let root = tempfile::tempdir().unwrap();

    let other = root.path().join("Documents");
    fs::create_dir(&other).unwrap();
    assert!(require_dir_name(&other, "ROM-Install").is_err());
    assert_eq!(
        require_dir_name(&other, "ROM-Install").unwrap_err().kind(),
        std::io::ErrorKind::PermissionDenied
    );
    assert!(require_dir_name(&root.path().join("ROM-Install"), "ROM-Install").is_ok());
    assert!(require_dir_name(Path::new("/"), "ROM-Install").is_err());
}

#[test]
fn folders_are_removed_and_cleared() {
    let root = tempfile::tempdir().unwrap();
    let dir = root.path().join("work");
    fs::create_dir_all(dir.join("Backup")).unwrap();
    fs::create_dir_all(dir.join("lineage/nested")).unwrap();
    fs::write(dir.join("boot.img"), "x").unwrap();
    fs::write(dir.join("lineage/nested/system.img"), "x").unwrap();
    fs::write(dir.join("Backup/boot.img"), "backup").unwrap();

    clear_directory_except(&dir, &["Backup"]).unwrap();
    assert!(!dir.join("boot.img").exists() && !dir.join("lineage").exists());
    assert_eq!(fs::read(dir.join("Backup/boot.img")).unwrap(), b"backup");

    // a folder that is gone is fine
    clear_directory_except(&root.path().join("gone"), &[]).unwrap();

    remove_dir_all_if_exists(&dir).unwrap();
    assert!(!dir.exists());
    remove_dir_all_if_exists(&dir).unwrap();
}
