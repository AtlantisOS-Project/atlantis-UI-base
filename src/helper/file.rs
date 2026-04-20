//! Precise manipulation and processing of file contents.
//!
//! While the `filesystem` module handles the structure (directories, existence),
//! this module provides tools for working directly with the contents of files:
//! from writing and appending lines to cleaning up strings.
 
/// Removes unnecessary spaces and control characters from text.
pub mod trim_and_clean;

/// Extracts specific values from structured files (e.g., key-value pairs).
pub mod get_config_value;

/// Functions for securely writing to or overwriting file contents.
pub mod write_file;

/// Allows individual lines to be appended to existing files.
pub mod add_line_file;

// Re-Export
pub use trim_and_clean::trim_and_clean;
pub use get_config_value::get_config_value;
pub use write_file::write_file;
pub use add_line_file::add_line_file;
