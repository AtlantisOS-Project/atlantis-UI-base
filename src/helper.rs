//! The logic hub of the AtlBase library.
//!
//! This module bundles all the low-level helper functions required for the operation 
//! of AtlantisOS. The functions are organized into 
//! submodules by topic to ensure a clear separation between system commands, 
//! file access, and configuration.

/// Execution and control of system commands.
pub mod commands;

/// Management of TOML configurations and environment checks.
pub mod configuration;

/// Manipulation of file contents (reading, writing, appending lines).
pub mod file;

/// Management of the directory structure (searching, deleting, creating).
pub mod filesystem;

/// Internationalization and gettext integration.
pub mod language;

/// System logging via Syslog and Journald viewer.
pub mod log;

// Re-Export
pub use commands::*;
pub use configuration::*;
pub use file::*;
pub use filesystem::*;
pub use language::*;
pub use log::*;
