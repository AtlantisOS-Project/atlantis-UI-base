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
