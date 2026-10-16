// language.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! Internationalization and language support.
//!
//! This module manages the localization of AtlantisOS applications: it initializes the
//! `gettext` system, keeps the text domain and finds the translation files in every
//! package format.

#[allow(clippy::module_inception)]
pub mod language;

// Re-exports
pub use language::*;
