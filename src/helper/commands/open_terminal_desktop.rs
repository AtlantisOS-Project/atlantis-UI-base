// open_terminal_desktop.rs
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! Opens a terminal emulator that fits the current desktop environment.
//!
//! The module detects the operating system and desktop (for example GNOME, KDE or WSL),
//! selects an installed terminal emulator and runs a command in it.

use crate::helper::configuration::application_environment::running_in_wsl;
use crate::helper::platform::os::os_profile;
use crate::helper::platform::program::{find_program, program_path};
use std::process::Command;

/// Opens a new terminal window and executes a command in it.
///
/// This is the bridge between a graphical application and the command line. After the
/// command has finished, the terminal stays open (`exec bash`).
///
/// An empty `function_command` does nothing. Errors (no terminal installed, program cannot
/// be started) are printed to `stderr`; the function never panics. Ask
/// [`supports`](crate::helper::platform::feature::supports) with
/// [`Feature::HostTerminal`](crate::helper::platform::feature::Feature::HostTerminal) before
/// offering a button for it.
///
/// # Supported environments
///
/// * **Linux:** prefers the terminal of the running desktop (GNOME, KDE, MATE, Cinnamon,
///   LXDE/LXQt, Xfce) and falls back to `x-terminal-emulator`, `xterm` and others, using
///   the first one that is installed.
/// * **macOS:** controls `Terminal.app` through AppleScript.
/// * **Windows:** uses `cmd.exe /K`.
/// * **WSL:** starts the Windows Terminal from within Linux.
///
/// # Security
///
/// `function_command` is run by a shell (`bash -c`; on WSL even by `sh -c` and `cmd.exe`),
/// so it can do anything the user can. Never pass text that comes from untrusted input.
///
/// # Packaging
///
/// A Flatpak or Snap sandbox cannot start the terminal of the host. The error is reported
/// on `stderr`; offer the command as text to copy instead.
///
/// # Examples
///
/// ```no_run
/// use atlbase::prelude::open_terminal_by_desktop;
///
/// open_terminal_by_desktop("echo 'Hello Atlantis'; sleep 5");
/// ```
pub fn open_terminal_by_desktop(function_command: &str) {
    if function_command.is_empty() {
        return;
    }

    let line = if running_in_wsl() {
        Some(vec![
            "sh".to_string(),
            "-c".to_string(),
            format!(
                "cmd.exe /C start cmd.exe /K \"wsl.exe {}\"",
                function_command
            ),
        ])
    } else {
        let desktop = std::env::var("XDG_CURRENT_DESKTOP").unwrap_or_default();
        let line = (os_profile().terminal)(
            &|program| find_program(program).is_some(),
            &desktop,
            function_command,
        );
        if line.is_none() {
            eprintln!(
                "[ERROR] No supported terminal found for desktop: {}",
                desktop
            );
        }
        line
    };

    let Some(line) = line else {
        return;
    };
    // a terminal is meant to show a window, so this is not `host_command`
    if let Err(error) = Command::new(program_path(&line[0]))
        .args(&line[1..])
        .spawn()
    {
        eprintln!("[ERROR] Could not start {}: {}", line[0], error);
    }
}
