//! A collection of dialog components for AtlantisOS.
//!
//! This module provides various graphical interaction elements based on 
//! `libadwaita` and `gtk4`. It ranges from simple informational messages to 
//! complex task dialogs with progress indicators.

/// Basic alert dialogs for confirmations.
pub mod dialog;

/// The standardized "About" dialog for app information.
pub mod about_dialog;

/// Dialogs with input fields for user interactions.
pub mod dialog_entry;

/// Visual feedback via loading spinners.
pub mod dialogs_spinner;

/// Pre-built standard dialogs (Info, Error, Warning).
pub mod standard_dialogs;

/// Dialogs for executing and monitoring background tasks.
pub mod dialog_function;

// Re-Exports 
pub use standard_dialogs::*;
pub use about_dialog::show_about_dialog;
pub use dialog::show_alert_dialog; 
