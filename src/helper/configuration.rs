// configuration.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! Application settings and runtime environment.
//!
//! This module loads and saves settings in TOML format and detects the environment the
//! application runs in (Flatpak, Snap, AppImage, native package). The detection is
//! essential for adapting paths and permissions at run time.

pub mod application_environment;

pub mod toml_config;

// Re-exports
pub use application_environment::*;
pub use toml_config::*;
