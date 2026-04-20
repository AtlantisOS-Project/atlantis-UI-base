//! The visual framework of AtlantisOS.
//!
//! This module bundles all components responsible for designing the 
//! graphical user interface. It uses `gtk4` and `libadwaita`
//! to ensure a modern, consistent, and user-friendly design across all 
//! system applications.

/// Tools for selecting files and folders.
pub mod chooser;

/// A variety of dialog boxes for interactions and information.
pub mod dialogs;

/// Management of stylesheets and themes.
pub mod theme;

/// Utility functions for quickly creating UI widgets.
pub mod utils;

// Re-Export
pub use chooser::*;
pub use dialogs::*;
pub use theme::*;
pub use utils::*;
