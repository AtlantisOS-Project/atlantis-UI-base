// elevate.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! Running a command with administrator rights.
//!
//! Every system has its own way to ask the user for a password and run a command as
//! administrator. [`elevation_command`] puts it in front of a command line:
//!
//! | System | Mechanism |
//! |---|---|
//! | Linux | `pkexec <command>` (Polkit) |
//! | macOS | `osascript -e 'do shell script "<command>" with administrator privileges'` |
//! | Windows | `powershell Start-Process <program> -Verb RunAs -Wait` (the UAC prompt) |
//!
//! Only Linux has been run; the others are covered by tests on the text of the command.
//! A Flatpak or Snap sandbox cannot elevate: ask [`supports`](super::feature::supports) with
//! [`Feature::Elevation`](super::feature::Feature::Elevation) before offering a button.

use super::target::Os;
use crate::helper::commands::command_line::shell_line;

/// The program that asks for the password on `os`, if there is one.
pub(crate) fn elevation_program(os: Os) -> Option<&'static str> {
    match os {
        Os::Linux | Os::Other => Some("pkexec"),
        Os::MacOs => Some("osascript"),
        Os::Windows => Some("powershell"),
    }
}

/// Puts the elevation mechanism of `os` in front of `command`.
///
/// # Returns
///
/// The command line to run, or `None` if `command` is empty. The result is a command line like
/// every other: run it with [`host_command_line`](super::program::host_command_line).
///
/// # Examples
///
/// ```
/// use atlbase::prelude::{Os, elevation_command_for};
///
/// let command = vec!["apt-get".to_string(), "update".to_string()];
/// assert_eq!(
///     elevation_command_for(Os::Linux, &command).unwrap(),
///     ["pkexec", "apt-get", "update"]
/// );
/// let windows = elevation_command_for(Os::Windows, &command).unwrap();
/// assert_eq!(windows[0], "powershell");
/// assert!(windows[3].contains("Start-Process -FilePath 'apt-get' -ArgumentList 'update'"));
/// ```
pub fn elevation_command_for(os: Os, command: &[String]) -> Option<Vec<String>> {
    let (program, args) = command.split_first()?;
    let tool = elevation_program(os)?.to_string();
    Some(match os {
        Os::Linux | Os::Other => std::iter::once(tool)
            .chain(command.iter().cloned())
            .collect(),
        Os::MacOs => {
            // inside an AppleScript string a backslash and a double quote have a meaning
            let script = shell_line(command)
                .replace('\\', "\\\\")
                .replace('"', "\\\"");
            vec![
                tool,
                "-e".to_string(),
                format!("do shell script \"{script}\" with administrator privileges"),
            ]
        }
        Os::Windows => {
            // inside a single quoted PowerShell string only the quote itself has a meaning
            let quote = |word: &str| format!("'{}'", word.replace('\'', "''"));
            let arguments = if args.is_empty() {
                String::new()
            } else {
                format!(
                    " -ArgumentList {}",
                    args.iter()
                        .map(|arg| quote(arg))
                        .collect::<Vec<_>>()
                        .join(",")
                )
            };
            vec![
                tool,
                "-NoProfile".to_string(),
                "-Command".to_string(),
                format!(
                    "$p = Start-Process -FilePath {}{} -Verb RunAs -Wait -PassThru; exit $p.ExitCode",
                    quote(program),
                    arguments
                ),
            ]
        }
    })
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
    elevation_command_for(Os::current(), command)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn line(words: &[&str]) -> Vec<String> {
        words.iter().map(|word| word.to_string()).collect()
    }

    #[test]
    fn linux_puts_pkexec_in_front() {
        assert_eq!(
            elevation_command_for(Os::Linux, &line(&["sh", "-c", "a b"])).unwrap(),
            ["pkexec", "sh", "-c", "a b"]
        );
        assert!(elevation_command_for(Os::Linux, &[]).is_none());
    }

    #[test]
    fn macos_quotes_for_the_shell_and_for_applescript() {
        let command = elevation_command_for(Os::MacOs, &line(&["echo", "say \"hi\""])).unwrap();
        assert_eq!(command[..2], ["osascript", "-e"]);
        assert_eq!(
            command[2],
            "do shell script \"'echo' 'say \\\"hi\\\"'\" with administrator privileges"
        );
    }

    #[test]
    fn windows_quotes_for_powershell() {
        let command =
            elevation_command_for(Os::Windows, &line(&["tool.exe", "it's", "b"])).unwrap();
        assert_eq!(command[..3], ["powershell", "-NoProfile", "-Command"]);
        assert_eq!(
            command[3],
            "$p = Start-Process -FilePath 'tool.exe' -ArgumentList 'it''s','b' -Verb RunAs -Wait -PassThru; exit $p.ExitCode"
        );
        let plain = elevation_command_for(Os::Windows, &line(&["tool.exe"])).unwrap();
        assert!(!plain[3].contains("-ArgumentList"));
    }
}
