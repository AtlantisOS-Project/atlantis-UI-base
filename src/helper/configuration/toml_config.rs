// toml_config.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! TOML configuration files for AtlantisOS applications.
//!
//! The module uses `confy` and `serde` to store simple key-value pairs persistently in the
//! default configuration directory of the user. The file is
//! `<config dir>/<app_name>/default-config.toml`, where the configuration directory follows
//! the XDG rules (`$XDG_CONFIG_HOME`, usually `~/.config`).
//!
//! # Packaging
//!
//! Flatpak and Snap redirect `$XDG_CONFIG_HOME` into the private data directory of the
//! application (`~/.var/app/<id>/config`, `~/snap/<name>/<revision>/.config`). The functions
//! follow that automatically, so the settings of a packaged application never leak into
//! the host configuration.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Data structure of the stored configuration.
///
/// A map of string keys to string values. You do not create it yourself; use [`get_config`]
/// and [`set_config`].
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct AtlConfigs {
    /// Key-value pairs of the configuration.
    data: HashMap<String, String>,
}

/// Reads a value from the configuration file of an application.
///
/// If the file does not exist or cannot be read, an empty configuration is used.
///
/// # Arguments
///
/// * `app_name` - The name of the application; it names the directory of the file.
/// * `key` - The key to look up.
///
/// # Returns
///
/// `Some(value)` if the key exists, otherwise `None`.
///
/// # Examples
///
/// ```no_run
/// use atlbase::prelude::get_config;
///
/// match get_config("my-app", "theme") {
///     Some(value) => println!("Found: {}", value),
///     None => println!("Key not found"),
/// }
/// ```
pub fn get_config(app_name: &str, key: &str) -> Option<String> {
    let cfg: AtlConfigs = confy::load(app_name, None).unwrap_or_default();
    cfg.data.get(key).cloned()
}

/// Saves or updates a value in the configuration file.
///
/// The current content of the file is loaded, the key is inserted or overwritten, and the
/// whole structure is written back. The file is written under a temporary name and moved into
/// place, so a crash never leaves half a file, and only the user can read it (mode `0600` on
/// Unix; a program may keep a token in its settings). A file that cannot be read as TOML is not
/// thrown away: it is moved to `<name>.toml.bad` (an older `.bad` file is replaced), a warning is
/// logged, and the settings start again with the new key.
///
/// Two programs that save at the same moment can still overwrite each other's key (the file is
/// read, changed and written again).
///
/// # Arguments
///
/// * `app_name` - The name of the application; it names the directory of the file.
/// * `key` - The identifier of the setting.
/// * `value` - The value to store.
///
/// # Errors
///
/// Returns a [`confy::ConfyError`] if the directory cannot be created, the file cannot be
/// written (missing permissions) or the data cannot be serialized.
///
/// # Examples
///
/// ```no_run
/// use atlbase::prelude::{get_config, set_config};
///
/// set_config("my-app", "port", "8080").expect("cannot save the setting");
///
/// let port: u16 = get_config("my-app", "port")
///     .and_then(|value| value.parse().ok())
///     .unwrap_or(3000); // default value
/// println!("Port: {}", port);
/// ```
pub fn set_config(app_name: &str, key: &str, value: &str) -> Result<(), confy::ConfyError> {
    let path = confy::get_configuration_file_path(app_name, None)?;

    let mut cfg: AtlConfigs = match confy::load_path(&path) {
        Ok(config) => config,
        Err(confy::ConfyError::GeneralLoadError(_) | confy::ConfyError::BadTomlData(_)) => {
            keep_damaged_file(&path);
            AtlConfigs::default()
        }
        Err(e) => return Err(e),
    };

    cfg.data.insert(key.to_string(), value.to_string());
    store_private(&path, cfg)
}

/// Moves a file that cannot be read to `<name>.bad`, so the settings in it are not lost.
fn keep_damaged_file(path: &Path) {
    let mut bad = path.as_os_str().to_owned();
    bad.push(".bad");
    match fs::rename(path, &bad) {
        Ok(()) => log::warn!(
            "the settings file {} is damaged, it was moved to {}",
            path.display(),
            Path::new(&bad).display()
        ),
        Err(error) => log::warn!(
            "the settings file {} is damaged and cannot be moved away: {error}",
            path.display()
        ),
    }
}

/// Writes `cfg` to `path` through a temporary file in the same folder (created with the mode
/// `0600`) and a rename.
fn store_private(path: &Path, cfg: AtlConfigs) -> Result<(), confy::ConfyError> {
    let dir = path.parent().ok_or_else(|| {
        confy::ConfyError::BadConfigDirectory(format!("{} has no folder", path.display()))
    })?;
    fs::create_dir_all(dir).map_err(confy::ConfyError::DirectoryCreationFailed)?;
    let temp = tempfile::NamedTempFile::new_in(dir)
        .map_err(confy::ConfyError::WriteConfigurationFileError)?;
    confy::store_path(temp.path(), cfg)?;
    temp.persist(path)
        .map_err(|error| confy::ConfyError::WriteConfigurationFileError(error.error))?;
    Ok(())
}
