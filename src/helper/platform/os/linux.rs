// linux.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! Linux.

use super::{Has, OsProfile, words};
use crate::helper::platform::package::PackageManager;
use crate::helper::platform::target::Os;

/// Terminal emulators with the option that introduces the command to run.
const TERMINALS: &[(&str, &str)] = &[
    ("gnome-terminal", "--"),
    ("konsole", "-e"),
    ("lxterminal", "-e"),
    ("mate-terminal", "-x"),
    ("xfce4-terminal", "-x"),
    ("x-terminal-emulator", "-e"),
    ("xterm", "-e"),
];

/// The terminal that belongs to the desktop named in `XDG_CURRENT_DESKTOP`.
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

fn terminal(has: Has, desktop: &str, command: &str) -> Option<Vec<String>> {
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
        &format!("{command}; exec bash"),
    ]))
}

fn elevate(command: &[String]) -> Vec<String> {
    std::iter::once("pkexec".to_string())
        .chain(command.iter().cloned())
        .collect()
}

fn journal_follow(app_name: &str) -> Vec<String> {
    words(&[
        "journalctl",
        "-f",
        "-n",
        "0",
        "-t",
        app_name,
        "--output=short",
    ])
}

fn program_names(name: &str, _pathext: Option<&str>) -> Vec<String> {
    vec![name.to_string()]
}

/// Linux.
pub static PROFILE: OsProfile = OsProfile {
    os: Os::Linux,
    exe_suffix: "",
    posix_shell: true,
    default_share_dirs: &["/usr/local/share", "/usr/share"],
    critical_paths: &[
        "/bin", "/sbin", "/usr", "/etc", "/var", "/proc", "/sys", "/dev", "/lib", "/lib32",
        "/lib64", "/boot", "/root",
    ],
    protected_dirs: &[
        "/home", "/tmp", "/opt", "/mnt", "/media", "/run", "/srv", "/app", "/snap",
    ],
    standard_home_folders: &[
        "Desktop",
        "Documents",
        "Downloads",
        "Music",
        "Pictures",
        "Videos",
        "Public",
        "Templates",
        ".config",
        ".local",
    ],
    elevation_program: Some("pkexec"),
    elevate,
    terminal,
    terminal_available: |has| TERMINALS.iter().any(|(name, _)| has(name)),
    package_managers: &[
        PackageManager::Apt,
        PackageManager::Dnf,
        PackageManager::Pacman,
        PackageManager::Zypper,
    ],
    journal_follow: Some(journal_follow),
    syslog: true,
    program_names,
};

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
    fn uses_the_terminal_of_the_desktop_or_the_first_installed() {
        let both = |name: &str| name == "konsole" || name == "xterm";
        assert_eq!(
            terminal(&both, "KDE", "ls").unwrap(),
            ["konsole", "-e", "bash", "-c", "ls; exec bash"]
        );
        assert_eq!(terminal(&both, "GNOME", "ls").unwrap()[0], "konsole");
        assert_eq!(
            terminal(&|name: &str| name == "xterm", "", "ls").unwrap()[0],
            "xterm"
        );
        assert!(terminal(&|_: &str| false, "KDE", "ls").is_none());
        assert!(!(PROFILE.terminal_available)(&|_: &str| false));
    }

    #[test]
    fn pkexec_goes_in_front_and_the_journal_is_followed() {
        let command = vec!["sh".to_string(), "-c".to_string(), "a b".to_string()];
        assert_eq!(elevate(&command), ["pkexec", "sh", "-c", "a b"]);
        assert_eq!(journal_follow("app")[0], "journalctl");
    }
}
