// helper_temp_file.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! Tests of the temporary application file.
//!
//! The file is global to the process, so everything is one sequential test.

use atlbase::prelude::*;

#[test]
fn the_temporary_file_behaves_like_a_log() {
    // starts empty
    assert_eq!(read_from_temp().unwrap(), "");

    // writing appends, also after a read
    write_to_temp("one").unwrap();
    assert_eq!(read_from_temp().unwrap(), "one");
    write_to_temp("\ntwo").unwrap();
    write_to_temp("\nthree").unwrap();
    assert_eq!(read_from_temp().unwrap(), "one\ntwo\nthree");

    // removing a line
    remove_line_from_temp(1).unwrap();
    assert_eq!(read_from_temp().unwrap(), "one\nthree\n");
    let error = remove_line_from_temp(5).unwrap_err();
    assert_eq!(error.kind(), std::io::ErrorKind::InvalidInput);

    // after clearing, the next write starts at the beginning (no NUL bytes in front)
    clear_temp_file().unwrap();
    assert_eq!(read_from_temp().unwrap(), "");
    write_to_temp("four").unwrap();
    clear_temp_file().unwrap();
    write_to_temp("five").unwrap();
    assert_eq!(read_from_temp().unwrap(), "five");
}

/// A program that ends normally, or is killed, leaves nothing of the text in the temporary
/// directory. The test starts itself again as the program (`ATL_TEMP_CHILD`) with a temporary
/// directory of its own.
#[cfg(unix)]
#[test]
fn nothing_stays_behind_when_the_program_ends() {
    const TEST: &str = "nothing_stays_behind_when_the_program_ends";

    if std::env::var_os("ATL_TEMP_CHILD").is_some() {
        write_to_temp("SECRET").unwrap();
        assert_eq!(read_from_temp().unwrap(), "SECRET");
        return;
    }

    let tmp = tempfile::tempdir().unwrap();
    let status = std::process::Command::new(std::env::current_exe().unwrap())
        .args(["--exact", TEST])
        .env("ATL_TEMP_CHILD", "1")
        .env("TMPDIR", tmp.path())
        .status()
        .unwrap();
    assert!(status.success());

    let left: Vec<_> = std::fs::read_dir(tmp.path()).unwrap().flatten().collect();
    assert!(left.is_empty(), "left behind: {left:?}");
}
