//! System commands and external process control.
//!
//! This module bundles all functions related to executing commands 
//! at the operating system level. It provides abstractions for 
//! privileged operations, output redirection, and interaction 
//! with desktop standards (URLs, terminals).
 
/// Checks whether a command is available in the system path ($PATH).
pub mod command_exists;

/// Executes commands with administrator privileges via pkexec (Polkit).
pub mod command_pkexec;

/// Basic functions for executing commands and capturing return values.
pub mod execute_command;

/// Open a terminal window based on the desktop environment.
pub mod open_terminal_desktop;

/// Open web links or files with the system's default application.
pub mod open_url;

// Re-Export
pub use command_exists::command_exists_native;
pub use command_pkexec::command_pkexec;
pub use execute_command::{run_command, capture_command_output};
pub use open_terminal_desktop::open_terminal_by_desktop;
pub use open_url::open_url;
