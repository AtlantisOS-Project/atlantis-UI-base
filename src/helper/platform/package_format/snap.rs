// snap.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! Snap: the program is mounted at `$SNAP`, a confinement that keeps it away from the host.

use super::{Lookup, PackageFormat};
use std::path::PathBuf;

fn root(var: Lookup) -> Option<PathBuf> {
    var("SNAP")
        .filter(|root| !root.is_empty())
        .map(PathBuf::from)
}

fn detect(var: Lookup) -> bool {
    root(var).is_some()
}

/// Snap.
pub static FORMAT: PackageFormat = PackageFormat {
    id: "snap",
    label: "Snap",
    sandboxed: true,
    relocated: true,
    detect,
    root,
    bin_dirs: &["usr/bin", "bin"],
    share_dirs: &["usr/share", "share"],
};
