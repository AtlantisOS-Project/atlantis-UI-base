// native.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! No format: a deb, an RPM, `cargo install`. The program is installed in the system and finds
//! everything where the system keeps it.

use super::{Lookup, PackageFormat};
use std::path::PathBuf;

fn detect(_var: Lookup) -> bool {
    true
}

fn root(_var: Lookup) -> Option<PathBuf> {
    None
}

/// The program is installed in the system.
pub static FORMAT: PackageFormat = PackageFormat {
    id: "native",
    label: "native package",
    sandboxed: false,
    relocated: false,
    detect,
    root,
    bin_dirs: &[],
    share_dirs: &[],
};
