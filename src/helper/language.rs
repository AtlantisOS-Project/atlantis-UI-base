//! Internationalization and language support.
//!
//! This module manages the localization of AtlantisOS applications. 
//! It provides functions for initializing the `gettext` system, 
//! managing text domains, and automatically determining 
//! the correct paths for translation files.

/// Core logic for language control and path lookup.
pub mod language;

// Re-Export
pub use language::*;
