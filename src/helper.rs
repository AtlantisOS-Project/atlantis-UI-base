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
//! | [`cli`] | The command line options all programs share (`-debug`, `--version`) |
//! | [`commands`] | Run commands, `pkexec`, terminals, URLs, command lines |
//! | [`configuration`] | TOML settings and runtime environment detection |
//! | [`file`](mod@file) | Read, write and clean up file contents |
//! | [`filesystem`] | Create, search and delete files and directories |
//! | [`language`] | gettext initialization and locale lookup |
//! | [`log`] | Syslog logger and journal viewer |
//! | [`platform`] | The layer between a program and the operating system: folders, programs, rights, logging |
//! | [`sysinfo`] | Distribution, kernel, processor and package versions |
//! | `archive` | Recognize and unpack archives (feature `archive`) |

pub mod cli;

pub mod commands;

pub mod configuration;

pub mod file;

pub mod filesystem;

pub mod language;

pub mod log;

pub mod platform;

pub mod sysinfo;

#[cfg(feature = "archive")]
#[cfg_attr(docsrs, doc(cfg(feature = "archive")))]
pub mod archive;

// Re-exports
pub use cli::*;
pub use commands::*;
pub use configuration::*;
pub use file::*;
pub use filesystem::*;
pub use language::*;
pub use log::*;
pub use platform::*;
pub use sysinfo::*;

#[cfg(feature = "archive")]
pub use archive::*;
