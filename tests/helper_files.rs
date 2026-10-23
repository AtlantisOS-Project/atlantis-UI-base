// helper_files.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! Tests of the file content helpers (`atlbase::helper::file`).

use atlbase::prelude::*;
use std::fs;

#[test]
fn add_line_file_never_glues_lines_together() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("hosts");
    let path = path.to_str().unwrap();

    // creates the file, with a line break
    add_line_file(false, path, "first").unwrap();
    assert_eq!(fs::read_to_string(path).unwrap(), "first\n");

    // duplicates (even with surrounding spaces) are ignored
    add_line_file(false, path, "first").unwrap();
    add_line_file(false, path, "  first  ").unwrap();
    assert_eq!(fs::read_to_string(path).unwrap(), "first\n");

    // a file without a final line break gets one before the new line
    fs::write(path, "no break at the end").unwrap();
    add_line_file(false, path, "second").unwrap();
    assert_eq!(
        fs::read_to_string(path).unwrap(),
        "no break at the end\nsecond\n"
    );
}

#[test]
fn add_line_file_makes_one_backup_of_an_existing_file() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("config");
    let path_str = path.to_str().unwrap();
    fs::write(&path, "original\n").unwrap();

    add_line_file(true, path_str, "added").unwrap();
    add_line_file(true, path_str, "added later").unwrap();

    let backup = dir.path().join("config.bak");
    assert_eq!(
        fs::read_to_string(backup).unwrap(),
        "original\n",
        "the backup is never overwritten"
    );
    assert_eq!(
        fs::read_to_string(&path).unwrap(),
        "original\nadded\nadded later\n"
    );
}

#[test]
fn config_values_are_parsed_like_in_os_release() {
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("os-release");
    fs::write(
        &file,
        "# comment\nNAME=\"Atlantis OS\"\nVERSION_ID=2026\nSPACED = value \nPORT=80\nPORTAL=1\n",
    )
    .unwrap();

    assert_eq!(
        get_config_value(&file, "NAME").as_deref(),
        Some("Atlantis OS")
    );
    assert_eq!(
        get_config_value(&file, "VERSION_ID").as_deref(),
        Some("2026")
    );
    assert_eq!(get_config_value(&file, "SPACED").as_deref(), Some("value"));
    // a key must not match the beginning of a longer key
    assert_eq!(get_config_value(&file, "PORT").as_deref(), Some("80"));
    assert_eq!(get_config_value(&file, "MISSING"), None);
    assert_eq!(get_config_value(dir.path().join("missing"), "NAME"), None);
}

#[test]
fn config_values_survive_invalid_utf8_lines() {
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("mixed");
    fs::write(&file, b"BROKEN=\xff\xfe\nGOOD=yes\n").unwrap();

    assert_eq!(get_config_value(&file, "GOOD").as_deref(), Some("yes"));
}

#[test]
fn paths_with_parent_components_are_rejected() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(dir.path().join("secret"), "KEY=value\n").unwrap();

    let sneaky = dir.path().join("sub/../secret");
    assert_eq!(get_config_value(sneaky, "KEY"), None);
}

#[test]
fn trimming_and_unquoting() {
    assert_eq!(trim_and_clean("  \"quoted\"  "), "quoted");
    assert_eq!(trim_and_clean("  plain "), "plain");
    assert_eq!(trim_and_clean("\"\""), "");
    assert_eq!(trim_and_clean("\""), "\"");
    assert_eq!(trim_and_clean("\"open"), "\"open");
}

#[test]
fn write_file_creates_the_directories() {
    let root = tempfile::tempdir().unwrap();

    write_file("a.txt", "one", Some(root.path().to_path_buf()), &["x", "y"]).unwrap();
    write_file("a.txt", "two", Some(root.path().to_path_buf()), &["x", "y"]).unwrap();

    let content = fs::read_to_string(root.path().join("x/y/a.txt")).unwrap();
    assert_eq!(content, "two", "an existing file is overwritten");
}
