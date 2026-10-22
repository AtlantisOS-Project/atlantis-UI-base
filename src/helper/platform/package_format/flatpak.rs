// flatpak.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! Flatpak: the program lives in `/app`, a sandbox that keeps it away from the host.

use super::{Lookup, PackageFormat};
use std::path::{Path, PathBuf};

fn detect(var: Lookup) -> bool {
    Path::new("/.flatpak-info").exists() || var("FLATPAK_ID").is_some_and(|id| !id.is_empty())
}

fn root(var: Lookup) -> Option<PathBuf> {
    detect(var).then(|| PathBuf::from("/app"))
}

/// Flatpak.
pub static FORMAT: PackageFormat = PackageFormat {
    id: "flatpak",
    label: "Flatpak",
    sandboxed: true,
    relocated: false,
    detect,
    root,
    bin_dirs: &["bin"],
    share_dirs: &["share"],
};
