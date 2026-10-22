// appimage.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! AppImage: the program is mounted at `$APPDIR`; it runs on the host like any program.

use super::{Lookup, PackageFormat};
use std::path::PathBuf;

fn detect(var: Lookup) -> bool {
    var("APPIMAGE").is_some_and(|image| !image.is_empty())
}

fn root(var: Lookup) -> Option<PathBuf> {
    var("APPDIR")
        .filter(|root| !root.is_empty())
        .map(PathBuf::from)
}

/// AppImage.
pub static FORMAT: PackageFormat = PackageFormat {
    id: "appimage",
    label: "AppImage",
    sandboxed: false,
    relocated: true,
    detect,
    root,
    bin_dirs: &["usr/bin"],
    share_dirs: &["usr/share"],
};
