// open_terminal_desktop.rs
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! Opens a terminal emulator that fits the current desktop environment.
//!
//! The module detects the operating system and desktop (for example GNOME, KDE or WSL),
//! selects an installed terminal emulator and runs a command in it.

use crate::helper::configuration::application_environment::running_in_wsl;
use crate::helper::platform::program::{find_program, program_path};
use crate::helper::platform::target::Os;
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
        let line = terminal_command(
            Os::current(),
            &desktop,
            &|program| find_program(program).is_some(),
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

/// Terminal emulators with the option that introduces the command to run.
pub(crate) const TERMINALS: &[(&str, &str)] = &[
    ("gnome-terminal", "--"),
    ("konsole", "-e"),
    ("lxterminal", "-e"),
    ("mate-terminal", "-x"),
    ("xfce4-terminal", "-x"),
    ("x-terminal-emulator", "-e"),
    ("xterm", "-e"),
];

/// Returns the terminal that belongs to the desktop named in `XDG_CURRENT_DESKTOP`.
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

/// The command line that opens a terminal on `os` and runs `function_command` in it, or `None`
/// if there is no terminal (`has` tells which programs are installed; `desktop` is
/// `XDG_CURRENT_DESKTOP`).
fn terminal_command(
    os: Os,
    desktop: &str,
    has: &dyn Fn(&str) -> bool,
    function_command: &str,
) -> Option<Vec<String>> {
    let words = |line: &[&str]| line.iter().map(|word| word.to_string()).collect::<Vec<_>>();
    match os {
        Os::Windows => Some(words(&[
            "cmd",
            "/C",
            &format!("start cmd.exe /K \"{function_command}\""),
        ])),
        Os::MacOs => {
            // escape the characters that have a meaning inside an AppleScript string
            let escaped = function_command.replace('\\', "\\\\").replace('"', "\\\"");
            Some(words(&[
                "osascript",
                "-e",
                &format!("tell application \"Terminal\" to do script \"{escaped}\""),
            ]))
        }
        Os::Linux | Os::Other => {
            // the terminal of the desktop first, then every other known terminal
            let preferred = preferred_terminal(desktop);
            let (term, option) = TERMINALS
                .iter()
                .filter(|(name, _)| Some(*name) == preferred)
                .chain(
                    TERMINALS
                        .iter()
                        .filter(|(name, _)| Some(*name) != preferred),
                )
                .find(|(name, _)| has(name))?;
            Some(words(&[
                term,
                option,
                "bash",
                "-c",
                &format!("{function_command}; exec bash"),
            ]))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn linux_uses_the_terminal_of_the_desktop_or_the_first_installed() {
        let both = |name: &str| name == "konsole" || name == "xterm";
        assert_eq!(
            terminal_command(Os::Linux, "KDE", &both, "ls").unwrap(),
            ["konsole", "-e", "bash", "-c", "ls; exec bash"]
        );
        assert_eq!(
            terminal_command(Os::Linux, "GNOME", &both, "ls").unwrap()[0],
            "konsole"
        );
        assert_eq!(
            terminal_command(Os::Linux, "", &|name: &str| name == "xterm", "ls").unwrap()[0],
            "xterm"
        );
        assert!(terminal_command(Os::Linux, "KDE", &|_: &str| false, "ls").is_none());
    }

    #[test]
    fn windows_and_macos_have_their_own_terminal() {
        assert_eq!(
            terminal_command(Os::Windows, "", &|_: &str| false, "dir").unwrap(),
            ["cmd", "/C", "start cmd.exe /K \"dir\""]
        );
        let mac = terminal_command(Os::MacOs, "", &|_: &str| false, "echo \"a\"").unwrap();
        assert_eq!(
            mac[2],
            "tell application \"Terminal\" to do script \"echo \\\"a\\\"\""
        );
    }
}
