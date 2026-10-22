// os.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! Everything that belongs to one operating system, in one file per system.
//!
//! An [`OsProfile`] is the description of an operating system for the platform layer: the folders
//! that must never be deleted, how a command gets administrator rights, how a terminal is opened,
//! which package managers there are, where the log goes. The rest of the library asks the profile
//! and never the system by name.
//!
//! | File | System |
//! |---|---|
//! | `os/linux.rs` | Linux |
//! | `os/windows.rs` | Windows |
//! | `os/macos.rs` | macOS |
//! | `os/other.rs` | the BSDs and other Unix systems |
//!
//! **Only the profile of the system the program is built for is compiled** (`cfg(target_os)`): a
//! Windows program contains no line of Linux code. The unit tests compile all of them, so the
//! mapping of every system is tested on every system.
//!
//! # Adding a system
//!
//! 1. Copy the closest file in `os/` (for example `other.rs`) to `os/<name>.rs` and fill in the
//!    fields of its `PROFILE`.
//! 2. Add a variant to [`Os`] and the `cfg` of the system to [`Os::current`](super::target::Os::current).
//! 3. Add the module and its `cfg` to the `mod` lines above, and the system to [`os_profile`] and
//!    to the unit tests' list in `other_profile`.

use super::package::PackageManager;
use super::target::Os;

#[cfg(any(test, target_os = "linux"))]
mod linux;
#[cfg(any(test, target_os = "macos"))]
mod macos;
#[cfg(any(
    test,
    not(any(target_os = "linux", target_os = "windows", target_os = "macos"))
))]
mod other;
#[cfg(any(test, target_os = "windows"))]
mod windows;

/// Which programs are installed, as a function: `has("pkexec")`.
pub type Has<'a> = &'a dyn Fn(&str) -> bool;

/// The description of an operating system (see the [module documentation](self)).
pub struct OsProfile {
    /// The system.
    pub os: Os,
    /// The ending of a program file: `.exe` on Windows, otherwise nothing.
    pub exe_suffix: &'static str,
    /// Whether the system has a POSIX shell (`sh`, `bash`) as its normal command line.
    pub posix_shell: bool,
    /// The data folders of the system when `$XDG_DATA_DIRS` is not set.
    pub default_share_dirs: &'static [&'static str],
    /// Folders that must never be deleted, nor anything below them.
    pub critical_paths: &'static [&'static str],
    /// Folders that must not be deleted themselves; what is inside is not protected.
    pub protected_dirs: &'static [&'static str],
    /// Folders in the home directory that must not be deleted.
    pub standard_home_folders: &'static [&'static str],
    /// The program that asks for the password, if the system has one (`pkexec`).
    pub elevation_program: Option<&'static str>,
    /// Puts the elevation in front of a command (not empty).
    pub elevate: fn(&[String]) -> Vec<String>,
    /// Opens a terminal and runs a command in it: `terminal(has, desktop, command)`.
    pub terminal: fn(Has, &str, &str) -> Option<Vec<String>>,
    /// Whether a terminal can be opened at all: `terminal_available(has)`.
    pub terminal_available: fn(Has) -> bool,
    /// The package managers of the system, in the order they are tried.
    pub package_managers: &'static [PackageManager],
    /// The command that prints the messages of an application as they come, if there is a system
    /// log to follow.
    pub journal_follow: Option<fn(&str) -> Vec<String>>,
    /// Whether the system has a syslog (otherwise the messages go to a file).
    pub syslog: bool,
    /// The names to try for a program: `program_names(name, pathext)`.
    pub program_names: fn(&str, Option<&str>) -> Vec<String>,
}

/// The profile of the system the program was built for.
///
/// # Examples
///
/// ```
/// use atlbase::prelude::{Os, os_profile};
///
/// assert_eq!(os_profile().os, Os::current());
/// ```
pub fn os_profile() -> &'static OsProfile {
    #[cfg(target_os = "linux")]
    return &linux::PROFILE;
    #[cfg(target_os = "windows")]
    return &windows::PROFILE;
    #[cfg(target_os = "macos")]
    return &macos::PROFILE;
    #[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
    return &other::PROFILE;
}

/// The profile of `os`, if it is compiled in: the system the program is built for, and in the unit
/// tests every system.
///
/// # Examples
///
/// ```
/// use atlbase::prelude::{Os, os_profile_for};
///
/// assert!(os_profile_for(Os::current()).is_some());
/// ```
pub fn os_profile_for(os: Os) -> Option<&'static OsProfile> {
    let current = os_profile();
    if current.os == os {
        Some(current)
    } else {
        other_profile(os)
    }
}

/// The profiles of the other systems: only in the unit tests.
#[cfg(test)]
fn other_profile(os: Os) -> Option<&'static OsProfile> {
    Some(match os {
        Os::Linux => &linux::PROFILE,
        Os::Windows => &windows::PROFILE,
        Os::MacOs => &macos::PROFILE,
        Os::Other => &other::PROFILE,
    })
}

#[cfg(not(test))]
fn other_profile(_os: Os) -> Option<&'static OsProfile> {
    None
}

fn words(line: &[&str]) -> Vec<String> {
    line.iter().map(|word| word.to_string()).collect()
}
