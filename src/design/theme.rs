//! Theme-specific customizations and styling.
//!
//! This module manages the visual appearance of AtlantisOS apps.
//! It enables the loading of custom stylesheets and the 
//! integration of Adwaita-specific design providers.

/// Functions for loading and applying CSS providers for Libadwaita.
pub mod load_adw_provider;

// Re-Export
pub use load_adw_provider::use_adw_provider;
