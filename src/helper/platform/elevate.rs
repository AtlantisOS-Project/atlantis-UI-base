// elevate.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! Running a command with administrator rights.
//!
//! Every system has its own way to ask the user for a password and run a command as
//! administrator; the profile of the system ([`OsProfile::elevate`]) puts it in front of a
//! command line:
//!
//! | System | Mechanism |
//! |---|---|
//! | Linux | `pkexec <command>` (Polkit) |
//! | macOS | `osascript -e 'do shell script "<command>" with administrator privileges'` |
//! | Windows | `powershell Start-Process <program> -Verb RunAs -Wait` (the UAC prompt) |
//!
//! Only Linux has been run; the others are covered by tests on the text of the command. A Flatpak
//! or Snap sandbox cannot elevate: ask [`supports`](super::feature::supports) with
//! [`Feature::Elevation`](super::feature::Feature::Elevation) before offering a button.

use super::os::{OsProfile, os_profile, os_profile_for};
use super::target::Os;

fn elevate_with(profile: &OsProfile, command: &[String]) -> Option<Vec<String>> {
    (!command.is_empty()).then(|| (profile.elevate)(command))
}

/// Puts the elevation mechanism of `os` in front of `command`, if `os` is compiled in (the system
/// the program is built for; in the unit tests every system).
///
/// # Returns
///
/// The command line to run, or `None` if `command` is empty or `os` is not compiled in. The result
/// is a command line like every other: run it with
/// [`host_command_line`](super::program::host_command_line).
///
/// # Examples
///
/// ```
/// use atlbase::prelude::{Os, elevation_command_for};
///
/// let command = vec!["apt-get".to_string(), "update".to_string()];
/// let line = elevation_command_for(Os::current(), &command).unwrap();
/// assert!(line.len() >= 2);
/// ```
pub fn elevation_command_for(os: Os, command: &[String]) -> Option<Vec<String>> {
    elevate_with(os_profile_for(os)?, command)
}

/// [`elevation_command_for`] for the system the program runs on.
///
/// # Examples
///
/// ```
/// use atlbase::prelude::elevation_command;
///
/// let command = elevation_command(&["id".to_string()]).unwrap();
/// assert!(command.len() >= 2);
/// assert!(elevation_command(&[]).is_none());
/// ```
pub fn elevation_command(command: &[String]) -> Option<Vec<String>> {
    elevate_with(os_profile(), command)
}
