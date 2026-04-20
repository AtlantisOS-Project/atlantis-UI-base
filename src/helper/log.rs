//! Logging and event display.
//!
//! This module provides functions for capturing system events. 
//! It supports both writing to the Unix syslog and 
//! graphical display of log streams within the application via VTE.
 
/// Interface to the Unix syslog service.
pub mod write_log;

/// Graphical components for displaying Journald logs.
pub mod write_log_text;

// Re-Export
pub use write_log::init_syslog;
pub use write_log_text::{show_log_viewer, create_custom_headerbar};
