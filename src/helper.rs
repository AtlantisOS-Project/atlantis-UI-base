// helper.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! The logic half of AtlBase: everything that works without a user interface.
//!
//! The functions are grouped by topic. Most of them are also available through the
//! [`prelude`](crate::prelude).
//!
//! | Module | Purpose |
//! |---|---|
//! | [`commands`] | Run commands, `pkexec`, terminals, URLs |
//! | [`configuration`] | TOML settings and runtime environment detection |
//! | [`file`](mod@file) | Read, write and clean up file contents |
//! | [`filesystem`] | Create, search and delete files and directories |
//! | [`language`] | gettext initialization and locale lookup |
//! | [`log`] | Syslog logger and journal viewer |

pub mod commands;

pub mod configuration;

pub mod file;

pub mod filesystem;

pub mod language;

pub mod log;

// Re-exports
pub use commands::*;
pub use configuration::*;
pub use file::*;
pub use filesystem::*;
pub use language::*;
pub use log::*;
