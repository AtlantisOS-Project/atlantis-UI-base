//! Selection dialogs for the file system.
//!
//! This module bundles specialized dialogs for selecting files and 
//! directories. It abstracts the asynchronous calls from GTK4 and 
//! provides a consistent interface for the entire AtlantisOS user interface.
 
/// Module for the file selection dialog.
pub mod file_chooser;

/// Module for the directory selection dialog.
pub mod folder_chooser;

// Re-Export
pub use file_chooser::show_file_chooser;
pub use folder_chooser::show_folder_chooser;
