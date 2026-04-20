//! Management of application configurations and runtime environments.
//!
//! This module provides tools for loading and saving settings in 
//! TOML format, as well as for detecting the environment in which the application is running. 
//! This is essential for path adaptation and package validation.
 
/// Detection and validation of the runtime environment (Snap, PPA, etc.).
pub mod application_environment;

/// Functions for reading and writing configuration files in TOML format.
pub mod toml_config;

// Rexport
pub use application_environment::{AppEnvironment, get_execution_environment, check_application_environment};
pub use toml_config::{get_config, set_config};
