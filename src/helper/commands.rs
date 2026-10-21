// commands.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! System commands and external process control.
//!
//! This module bundles all functions that execute commands at the operating system
//! level: privileged operations through `pkexec`, output capturing, and interaction with
//! the desktop (URLs, terminals).
//!
//! # Packaging
//!
//! Inside a **Flatpak** sandbox, host programs such as `pkexec` and host terminal emulators
//! are not reachable, and **Snap** strict confinement cannot elevate privileges. The
//! functions report such failures on `stderr` instead of panicking. Use
//! [`get_execution_environment`](crate::helper::configuration::application_environment::get_execution_environment)
//! to hide the corresponding buttons.

pub mod command_chain;

pub mod command_exists;

pub mod command_line;

pub mod command_pkexec;

pub mod execute_command;

pub mod open_terminal_desktop;

pub mod open_url;

// Re-exports
pub use command_chain::*;
pub use command_exists::*;
pub use command_line::*;
pub use command_pkexec::*;
pub use execute_command::*;
pub use open_terminal_desktop::*;
pub use open_url::*;
