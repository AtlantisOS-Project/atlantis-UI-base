// macos.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! macOS. Run only in tests so far: the mappings are text, covered by the unit tests below.

use super::{Has, OsProfile, words};
use crate::helper::commands::command_line::shell_line;
use crate::helper::platform::package::PackageManager;
use crate::helper::platform::target::Os;

/// Inside an AppleScript string a backslash and a double quote have a meaning.
fn escape(text: &str) -> String {
    text.replace('\\', "\\\\").replace('"', "\\\"")
}

/// `osascript -e 'do shell script "<command>" with administrator privileges'`.
fn elevate(command: &[String]) -> Vec<String> {
    words(&[
        "osascript",
        "-e",
        &format!(
            "do shell script \"{}\" with administrator privileges",
            escape(&shell_line(command))
        ),
    ])
}

fn terminal(_has: Has, _desktop: &str, command: &str) -> Option<Vec<String>> {
    Some(words(&[
        "osascript",
        "-e",
        &format!(
            "tell application \"Terminal\" to do script \"{}\"",
            escape(command)
        ),
    ]))
}

fn program_names(name: &str, _pathext: Option<&str>) -> Vec<String> {
    vec![name.to_string()]
}

/// macOS.
pub static PROFILE: OsProfile = OsProfile {
    os: Os::MacOs,
    exe_suffix: "",
    posix_shell: true,
    default_share_dirs: &["/usr/local/share", "/opt/homebrew/share", "/usr/share"],
    critical_paths: &[
        "/System",
        "/Library",
        "/Applications",
        "/bin",
        "/sbin",
        "/usr",
        "/etc",
        "/var",
        "/private",
        "/dev",
        "/cores",
    ],
    protected_dirs: &["/Users", "/Volumes", "/tmp", "/opt"],
    standard_home_folders: &[
        "Desktop",
        "Documents",
        "Downloads",
        "Music",
        "Pictures",
        "Movies",
        "Library",
        "Applications",
        ".config",
    ],
    elevation_program: Some("osascript"),
    elevate,
    terminal,
    terminal_available: |has| has("osascript"),
    package_managers: &[PackageManager::Homebrew],
    journal_follow: None,
    syslog: true,
    program_names,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quotes_for_the_shell_and_for_applescript() {
        let line = elevate(&words(&["echo", "say \"hi\""]));
        assert_eq!(line[..2], ["osascript", "-e"]);
        assert_eq!(
            line[2],
            "do shell script \"'echo' 'say \\\"hi\\\"'\" with administrator privileges"
        );
        assert_eq!(
            terminal(&|_: &str| false, "", "echo \"a\"").unwrap()[2],
            "tell application \"Terminal\" to do script \"echo \\\"a\\\"\""
        );
    }
}
