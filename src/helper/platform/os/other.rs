// other.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! Other Unix systems (the BSDs, …): the profile of Linux without what is Linux only. A good file
//! to copy when a system is added.

use super::{Has, OsProfile, words};
use crate::helper::platform::target::Os;

fn elevate(command: &[String]) -> Vec<String> {
    std::iter::once("pkexec".to_string())
        .chain(command.iter().cloned())
        .collect()
}

fn terminal(has: Has, _desktop: &str, command: &str) -> Option<Vec<String>> {
    ["x-terminal-emulator", "xterm"]
        .into_iter()
        .find(|name| has(name))
        .map(|term| words(&[term, "-e", "sh", "-c", &format!("{command}; exec sh")]))
}

fn program_names(name: &str, _pathext: Option<&str>) -> Vec<String> {
    vec![name.to_string()]
}

/// Other Unix systems.
pub static PROFILE: OsProfile = OsProfile {
    os: Os::Other,
    exe_suffix: "",
    posix_shell: true,
    default_share_dirs: &["/usr/local/share", "/usr/share"],
    critical_paths: &[
        "/bin", "/sbin", "/usr", "/etc", "/var", "/dev", "/lib", "/boot", "/root",
    ],
    protected_dirs: &["/home", "/tmp", "/opt", "/mnt", "/media", "/run"],
    standard_home_folders: &[
        "Desktop",
        "Documents",
        "Downloads",
        "Music",
        "Pictures",
        "Videos",
    ],
    elevation_program: Some("pkexec"),
    elevate,
    terminal,
    terminal_available: |has| has("x-terminal-emulator") || has("xterm"),
    package_managers: &[],
    journal_follow: None,
    syslog: true,
    program_names,
};
