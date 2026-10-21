// command_pkexec.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! Direct execution of privileged commands.
//!
//! This module provides a lightweight interface for running a command with administrator
//! rights and reporting its output: `pkexec` on Linux, and the mechanism of the other systems
//! ([`elevation_command`]). For a variant
//! with a progress dialog see
//! [`command_pkexec_spinner`](crate::design::utils::command_pkexec_ui::command_pkexec_spinner).

use crate::helper::platform::elevate::elevation_command;
use crate::helper::platform::program::host_command_line;

/// Executes a command with elevated privileges (`pkexec` on Linux) and waits for it to finish.
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
    let command: Vec<String> = args.iter().map(|arg| arg.to_string()).collect();
    let Some(elevated) = elevation_command(&command) else {
        eprintln!("[ERROR] No command to run with elevated privileges.");
        return;
    };
    let output = host_command_line(&elevated).output();

    match output {
        Ok(res) if res.status.success() => {
            let stdout = String::from_utf8_lossy(&res.stdout);
            println!("[INFO] Execution successful: {}", stdout);
        }
        Ok(res) => {
            let stderr = String::from_utf8_lossy(&res.stderr);
            eprintln!(
                "[ERROR] {} failed with status {}: {}",
                elevated[0], res.status, stderr
            );
        }
        Err(e) => {
            eprintln!("[ERROR] Could not start {}: {}", elevated[0], e);
        }
    }
}
