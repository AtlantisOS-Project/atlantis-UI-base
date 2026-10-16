// open_terminal_desktop.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! Opens a terminal emulator that fits the current desktop environment.
//!
//! The module detects the operating system and desktop (for example GNOME, KDE or WSL),
//! selects an installed terminal emulator and runs a command in it.

use crate::helper::commands::command_exists::command_exists_native;
use std::path::Path;
use std::process::Command;

/// Opens a new terminal window and executes a command in it.
///
/// This is the bridge between a graphical application and the command line. After the
/// command has finished, the terminal stays open (`exec bash`).
///
/// An empty `function_command` does nothing. Errors (no terminal installed, program cannot
/// be started) are printed to `stderr`; the function never panics.
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

    // WSL: the Windows drives are mounted below /mnt
    if Path::new("/mnt/c/Users").exists() {
        let wsl_cmd = format!(
            "cmd.exe /C start cmd.exe /K \"wsl.exe {}\"",
            function_command
        );
        let _ = Command::new("sh").arg("-c").arg(wsl_cmd).spawn();
        return;
    }

    spawn_terminal(function_command);
}

#[cfg(target_os = "windows")]
fn spawn_terminal(func_cmd: &str) {
    let cmd = format!("start cmd.exe /K \"{}\"", func_cmd);
    let _ = Command::new("cmd").arg("/C").arg(cmd).spawn();
}

#[cfg(target_os = "macos")]
fn spawn_terminal(func_cmd: &str) {
    // escape the characters that have a meaning inside an AppleScript string
    let escaped = func_cmd.replace('\\', "\\\\").replace('"', "\\\"");
    let script = format!("tell application \"Terminal\" to do script \"{}\"", escaped);
    let _ = Command::new("osascript").arg("-e").arg(script).spawn();
}

/// Terminal emulators with the option that introduces the command to run.
#[cfg(not(any(target_os = "windows", target_os = "macos")))]
const TERMINALS: &[(&str, &str)] = &[
    ("gnome-terminal", "--"),
    ("konsole", "-e"),
    ("lxterminal", "-e"),
    ("mate-terminal", "-x"),
    ("xfce4-terminal", "-x"),
    ("x-terminal-emulator", "-e"),
    ("xterm", "-e"),
];

/// Returns the terminal that belongs to the desktop named in `XDG_CURRENT_DESKTOP`.
#[cfg(not(any(target_os = "windows", target_os = "macos")))]
fn preferred_terminal(desktop: &str) -> Option<&'static str> {
    let desktop = desktop.to_uppercase();
    if desktop.contains("MATE") {
        Some("mate-terminal")
    } else if desktop.contains("GNOME") || desktop.contains("CINNAMON") {
        Some("gnome-terminal")
    } else if desktop.contains("KDE") {
        Some("konsole")
    } else if desktop.contains("LXDE") || desktop.contains("LXQT") {
        Some("lxterminal")
    } else if desktop.contains("XFCE") {
        Some("xfce4-terminal")
    } else {
        None
    }
}

#[cfg(not(any(target_os = "windows", target_os = "macos")))]
fn spawn_terminal(func_cmd: &str) {
    let desktop = std::env::var("XDG_CURRENT_DESKTOP").unwrap_or_default();
    let preferred = preferred_terminal(&desktop);

    // the terminal of the desktop first, then every other known terminal
    let mut candidates = TERMINALS
        .iter()
        .filter(|(name, _)| Some(*name) == preferred)
        .chain(
            TERMINALS
                .iter()
                .filter(|(name, _)| Some(*name) != preferred),
        );

    let Some((term, option)) = candidates.find(|(name, _)| command_exists_native(name)) else {
        eprintln!(
            "[ERROR] No supported terminal found for desktop: {}",
            desktop
        );
        return;
    };

    let full_cmd = format!("{}; exec bash", func_cmd);
    let result = Command::new(term)
        .arg(option)
        .arg("bash")
        .arg("-c")
        .arg(full_cmd)
        .spawn();

    if let Err(e) = result {
        eprintln!("[ERROR] Could not start {}: {}", term, e);
    }
}

#[cfg(all(test, not(any(target_os = "windows", target_os = "macos"))))]
mod tests {
    use super::preferred_terminal;

    #[test]
    fn terminal_follows_the_desktop() {
        assert_eq!(preferred_terminal("GNOME"), Some("gnome-terminal"));
        assert_eq!(preferred_terminal("ubuntu:GNOME"), Some("gnome-terminal"));
        assert_eq!(preferred_terminal("X-Cinnamon"), Some("gnome-terminal"));
        assert_eq!(preferred_terminal("MATE"), Some("mate-terminal"));
        assert_eq!(preferred_terminal("KDE"), Some("konsole"));
        assert_eq!(preferred_terminal("LXQt"), Some("lxterminal"));
        assert_eq!(preferred_terminal(""), None);
    }
}
