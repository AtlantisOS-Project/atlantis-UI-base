// theme.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! Themes and stylesheets.
//!
//! The module manages the visual appearance of AtlantisOS applications: it loads custom
//! stylesheets and keeps them correct when the system switches between the light and the
//! dark style, and chooses how the windows are drawn ([`renderer`]).

pub mod app_icon;

pub mod load_adw_provider;

pub mod renderer;

// Re-exports
pub use app_icon::*;
pub use load_adw_provider::*;
pub use renderer::*;
