// log.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! Logging and event display.
//!
//! The module captures system events. It supports writing to the Unix syslog and, with the
//! `log-viewer` feature, the display of the log stream in the application through VTE.

pub mod write_log;

#[cfg(all(feature = "log-viewer", target_os = "linux"))]
#[cfg_attr(docsrs, doc(cfg(feature = "log-viewer")))]
pub mod write_log_text;

// Re-exports
pub use write_log::*;
#[cfg(all(feature = "log-viewer", target_os = "linux"))]
pub use write_log_text::*;
