// open_terminal_desktop.rs
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! Opens a terminal emulator that fits the current desktop environment.
//!
//! The module detects the operating system and desktop (for example GNOME, KDE or WSL),
//! selects an installed terminal emulator and runs a command in it.

use crate::helper::commands::command_line::shell_line;
use crate::helper::configuration::application_environment::running_in_wsl;
use crate::helper::platform::os::os_profile;
use crate::helper::platform::program::{find_program, program_path};
use crate::helper::platform::target::Os;
use std::io;
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
/// so it can do anything the user can. Never pass text that comes from untrusted input. If a
/// part of the command comes from outside (a file name, the name of a device), use
/// [`open_terminal_command`], which takes the program and its arguments as separate words and
/// quotes them for the shell of the system.
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

/// The text for a terminal from a command line. On Windows and WSL the text goes through
/// `cmd.exe`, which has no quoting that holds, so only words of plain characters are taken; on
/// the other systems every word is quoted for a POSIX shell.
fn terminal_text(command: &[String], windows_like: bool) -> io::Result<String> {
    if command.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "no command to run in the terminal",
        ));
    }
    if !windows_like {
        return Ok(shell_line(command));
    }
    let plain = |word: &str| {
        !word.is_empty()
            && word
                .chars()
                .all(|c| c.is_alphanumeric() || " ._-/\\:=@+,".contains(c))
    };
    let mut words = Vec::new();
    for word in command {
        if !plain(word) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("this word cannot be passed to a Windows terminal safely: {word:?}"),
            ));
        }
        words.push(if word.contains(' ') {
            format!("\"{word}\"")
        } else {
            word.clone()
        });
    }
    Ok(words.join(" "))
}

/// Opens a new terminal window and runs a program in it, given as the program and its
/// arguments. Like [`open_terminal_by_desktop`], and the terminal stays open afterwards, but
/// nothing in the words is read as shell syntax: `["ls", "a b; rm -rf ~"]` lists a file with
/// that odd name.
///
/// # Errors
///
/// `InvalidInput` if `command` is empty, or on Windows and WSL if a word has a character that
/// `cmd.exe` could read as syntax (only letters, digits, space and `. _ - / \ : = @ + ,` are
/// accepted there). Errors of the terminal itself are printed to `stderr`, as for
/// [`open_terminal_by_desktop`].
///
/// # Examples
///
/// ```no_run
/// use atlbase::prelude::{command_line, open_terminal_command};
///
/// let file = "my file; with odd characters.txt";
/// open_terminal_command(&command_line("less", [file])).expect("an ordinary command");
/// ```
pub fn open_terminal_command(command: &[String]) -> io::Result<()> {
    let windows_like = running_in_wsl() || os_profile().os == Os::Windows;
    let text = terminal_text(command, windows_like)?;
    open_terminal_by_desktop(&text);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn words(line: &[&str]) -> Vec<String> {
        line.iter().map(|word| word.to_string()).collect()
    }

    #[test]
    fn every_word_is_quoted_for_a_posix_shell() {
        assert_eq!(
            terminal_text(&words(&["echo", "it's; rm -rf ~", "$(id)"]), false).unwrap(),
            r"'echo' 'it'\''s; rm -rf ~' '$(id)'"
        );
        assert!(terminal_text(&[], false).is_err());
    }

    #[test]
    fn windows_takes_plain_words_only() {
        assert_eq!(
            terminal_text(&words(&["adb", "devices", "-l"]), true).unwrap(),
            "adb devices -l"
        );
        assert_eq!(
            terminal_text(&words(&["C:\\Program Files\\tool.exe", "a b"]), true).unwrap(),
            "\"C:\\Program Files\\tool.exe\" \"a b\""
        );
        for evil in [
            "x & calc", "x | y", "x > y", "%PATH%", "^", "\"", "'", "x;y", "$(id)", "`id`", "a\nb",
            "!", "",
        ] {
            let error = terminal_text(&words(&["tool", evil]), true).unwrap_err();
            assert_eq!(error.kind(), io::ErrorKind::InvalidInput, "{evil:?}");
        }
        assert!(terminal_text(&[], true).is_err());
    }
}
