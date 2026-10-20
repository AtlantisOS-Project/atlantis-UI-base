// application_environment.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! Identification of the runtime environment of the application.
//!
//! The module detects whether the application runs inside an isolated package (Flatpak,
//! Snap, AppImage) or as a native system package (PPA, local deb). The result is used to
//! adjust paths and permissions at run time.

use crate::helper::filesystem::search_string_name::search_string_file;
use std::env;
use std::path::Path;
use std::sync::OnceLock;

/// The possible runtime environments of the application.
///
/// The enum is returned by [`check_application_environment`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AppEnvironment {
    /// Runs inside a Flatpak sandbox.
    Flatpak,
    /// Runs as a Snap package.
    Snap,
    /// Runs from an AppImage.
    AppImage,
    /// Installed through a registered PPA (Personal Package Archive).
    Ppa,
    /// Installed as a local deb package.
    LocalDeb,
    /// Unknown environment, for example a system without APT (rpm, Arch, …).
    Unknown,
}

impl AppEnvironment {
    /// A readable name of the package kind, for example for `--version`.
    ///
    /// # Examples
    ///
    /// ```
    /// use atlbase::prelude::AppEnvironment;
    ///
    /// assert_eq!(AppEnvironment::Flatpak.label(), "Flatpak");
    /// assert_eq!(AppEnvironment::LocalDeb.label(), "Local Debian Package");
    /// ```
    pub fn label(self) -> &'static str {
        match self {
            AppEnvironment::Flatpak => "Flatpak",
            AppEnvironment::Snap => "Snap",
            AppEnvironment::AppImage => "AppImage",
            AppEnvironment::Ppa => "Debian Package PPA",
            AppEnvironment::LocalDeb => "Local Debian Package",
            AppEnvironment::Unknown => "Unknown package",
        }
    }

    /// Whether the application runs in a sandbox (Flatpak or Snap).
    ///
    /// In a sandbox there is no `pkexec`, no `journalctl` and no terminal of the host.
    pub fn is_sandboxed(self) -> bool {
        matches!(self, AppEnvironment::Flatpak | AppEnvironment::Snap)
    }
}

/// Returns the name of the current runtime environment.
///
/// The check needs only the file system and the environment variables:
///
/// | Result | Condition |
/// |---|---|
/// | `"flatpak"` | the file `/.flatpak-info` exists, or `$FLATPAK_ID` is set |
/// | `"snap"` | `$SNAP` is set |
/// | `"appimage"` | `$APPIMAGE` is set (the AppImage runtime sets it) |
/// | `"native"` | none of the above (deb, rpm, `cargo install`, …) |
///
/// # Examples
///
/// ```
/// use atlbase::prelude::get_execution_environment;
///
/// let env = get_execution_environment();
/// assert!(["flatpak", "snap", "appimage", "native"].contains(&env.as_str()));
/// ```
pub fn get_execution_environment() -> String {
    let name = if Path::new("/.flatpak-info").exists() || env::var_os("FLATPAK_ID").is_some() {
        "flatpak"
    } else if env::var_os("SNAP").is_some() {
        "snap"
    } else if env::var_os("APPIMAGE").is_some() {
        "appimage"
    } else {
        "native"
    };
    name.to_string()
}

/// Analyzes the installation source of the application in detail.
///
/// Sandboxed packages are recognized through [`get_execution_environment`]. A native
/// installation is looked up in the APT source lists: a file in `source_dir` whose name
/// contains `search_string` means the application comes from a PPA, otherwise it was
/// installed from a local deb package.
///
/// # Arguments
///
/// * `search_string` - The name of the application in the source lists, for example
///   `"fastboot-assistant"`.
/// * `source_dir` - The directory with the APT lists, usually `/etc/apt/sources.list.d`.
///
/// # Returns
///
/// The detected [`AppEnvironment`]. [`AppEnvironment::Unknown`] is returned if `source_dir`
/// does not exist (a system without APT) or cannot be read; the reason is printed to
/// `stderr`. The function never terminates the process.
///
/// # Examples
///
/// ```
/// use atlbase::prelude::{AppEnvironment, check_application_environment};
///
/// let environment = check_application_environment("fastboot-assistant", "/etc/apt/sources.list.d");
/// match environment {
///     AppEnvironment::Flatpak => println!("running as Flatpak"),
///     AppEnvironment::Ppa => println!("installed from the PPA"),
///     _ => println!("other environment"),
/// }
/// ```
pub fn check_application_environment(search_string: &str, source_dir: &str) -> AppEnvironment {
    let env_str = get_execution_environment();
    println!("[INFO] The app runs in the environment: {}", env_str);

    match env_str.as_str() {
        "flatpak" => AppEnvironment::Flatpak,
        "snap" => AppEnvironment::Snap,
        "appimage" => AppEnvironment::AppImage,
        _ => {
            if !Path::new(source_dir).is_dir() {
                println!(
                    "[INFO] {} does not exist: not an APT based system",
                    source_dir
                );
                return AppEnvironment::Unknown;
            }
            match search_string_file(source_dir, search_string) {
                Ok(true) => AppEnvironment::Ppa,
                Ok(false) => {
                    println!("[INFO] Running as local deb package.");
                    AppEnvironment::LocalDeb
                }
                Err(e) => {
                    eprintln!(
                        "[ERROR] Cannot check the APT sources in {}: {}",
                        source_dir, e
                    );
                    AppEnvironment::Unknown
                }
            }
        }
    }
}

static CURRENT: OnceLock<AppEnvironment> = OnceLock::new();

/// The environment of the application, determined once and then remembered.
///
/// [`check_application_environment`] reads files and prints to the terminal; this function
/// is meant for code that asks "am I in a sandbox?" in many places. The arguments of the
/// *first* call are used, later calls ignore theirs.
///
/// # Examples
///
/// ```
/// use atlbase::prelude::current_environment;
///
/// let environment = current_environment("fastboot-assistant", "/etc/apt/sources.list.d");
/// println!("running as {}", environment.label());
/// assert_eq!(environment, current_environment("other", "/nonexistent"));
/// ```
pub fn current_environment(search_string: &str, source_dir: &str) -> AppEnvironment {
    *CURRENT.get_or_init(|| check_application_environment(search_string, source_dir))
}

/// Whether the program runs in the Windows Subsystem for Linux (WSL).
///
/// WSL is recognized by the Windows drive `/mnt/c/Users`. In WSL the Windows programs
/// (`adb.exe`, …) are available next to the Linux ones.
pub fn running_in_wsl() -> bool {
    Path::new("/mnt/c/Users").is_dir()
}
