//! Utility tools for UI development.
//!
//! This module contains handy factory functions for quickly creating 
//! standardized widgets such as input fields, buttons with icons, or 
//! progress bars for privileged commands.
 
/// Creation of text and password input fields.
pub mod create_entry;

/// Combined widgets consisting of labels and icons.
pub mod create_label_icon;

/// Specialized buttons (e.g., with multiple icons or specific positions).
pub mod create_special_button;

/// Visual feedback (spinner/progress bar) for pkexec commands.
pub mod command_pkexec_ui;

// Re-Exports
pub use create_entry::*;
pub use create_label_icon::*;
pub use create_special_button::*;
pub use command_pkexec_ui::*;
