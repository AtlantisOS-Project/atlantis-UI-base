// command_pkexec.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! Direct execution of privileged commands via `pkexec`.
//!
//! This module provides a lightweight interface for running a command with root
//! privileges and reporting its output. For a variant with a progress dialog see
//! [`command_pkexec_spinner`](crate::design::utils::command_pkexec_ui::command_pkexec_spinner).

use std::process::Command;

/// Executes a command with elevated privileges via `pkexec` and waits for it to finish.
///
/// The function blocks the current thread until the command (and the Polkit authentication)
/// is complete. It suits command line tools and background threads; in a GTK application use
/// the dialogs from [`design::utils::command_pkexec_ui`](crate::design::utils::command_pkexec_ui)
/// so that the interface stays responsive.
///
/// # Arguments
///
/// * `args` - The command and its parameters, for example `["apt", "update"]`.
///
/// # Output
///
/// * On success, `stdout` of the command is printed with the prefix `[INFO]`.
/// * If the command fails (exit code other than 0), `stderr` is printed with the prefix
///   `[ERROR]`.
/// * If `pkexec` itself cannot be started (for example inside a Flatpak sandbox), the
///   error is printed to `stderr`.
///
/// # Examples
///
/// ```no_run
/// use atlbase::prelude::command_pkexec;
///
/// command_pkexec(&["ls", "/root"]);
/// ```
pub fn command_pkexec(args: &[&str]) {
    let output = Command::new("pkexec").args(args).output();

    match output {
        Ok(res) if res.status.success() => {
            let stdout = String::from_utf8_lossy(&res.stdout);
            println!("[INFO] Execution successful: {}", stdout);
        }
        Ok(res) => {
            let stderr = String::from_utf8_lossy(&res.stderr);
            eprintln!(
                "[ERROR] pkexec failed with status {}: {}",
                res.status, stderr
            );
        }
        Err(e) => {
            eprintln!("[ERROR] Could not start pkexec: {}", e);
        }
    }
}
