// feature.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! What the system the program runs on can do.
//!
//! A button that needs `pkexec`, a terminal of the host or the system log must not be shown
//! where there is none: not in a Flatpak or Snap sandbox, not on Windows. [`supports`] answers
//! the question for one [`Feature`]; the program never has to know why.

use super::elevate::elevation_program;
use super::package::PackageManager;
use super::program::find_program;
use super::target::Os;
use crate::helper::commands::open_terminal_desktop::TERMINALS;
use crate::helper::configuration::application_environment::get_execution_environment;

/// Something the host may or may not offer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Feature {
    /// Running a command as administrator ([`elevation_command`](super::elevate::elevation_command)).
    Elevation,
    /// Opening a terminal window on the host and running a command in it.
    HostTerminal,
    /// A system log the program can follow (`journalctl`).
    SystemLog,
    /// Installing programs with the package manager of the host. Needs [`Feature::Elevation`].
    PackageInstall,
    /// A POSIX shell (`sh`, `bash`) for scripts.
    PosixShell,
}

/// Whether the program runs in a Flatpak or Snap sandbox, which has no access to the programs of
/// the host.
pub fn in_sandbox() -> bool {
    matches!(get_execution_environment().as_str(), "flatpak" | "snap")
}

/// Whether `feature` is available on `os`, in a sandbox or not. `has` tells whether a program is
/// installed.
fn available(os: Os, sandboxed: bool, has: &dyn Fn(&str) -> bool, feature: Feature) -> bool {
    match feature {
        Feature::Elevation => !sandboxed && elevation_program(os).is_some_and(has),
        Feature::HostTerminal => match os {
            Os::Windows => !sandboxed,
            Os::MacOs => !sandboxed && has("osascript"),
            Os::Linux | Os::Other => {
                !sandboxed && TERMINALS.iter().any(|(terminal, _)| has(terminal))
            }
        },
        Feature::SystemLog => os == Os::Linux && !sandboxed && has("journalctl"),
        Feature::PackageInstall => {
            available(os, sandboxed, has, Feature::Elevation)
                && PackageManager::ALL
                    .iter()
                    .any(|m| m.os() == os && has(m.program()))
        }
        Feature::PosixShell => os.has_posix_shell(),
    }
}

/// Whether the host offers `feature` to this program.
///
/// # Examples
///
/// ```
/// use atlbase::prelude::{Feature, supports};
///
/// if supports(Feature::Elevation) {
///     println!("show the button that installs a program");
/// }
/// // a shell script can run on every system but Windows
/// assert_eq!(supports(Feature::PosixShell), !cfg!(windows));
/// ```
pub fn supports(feature: Feature) -> bool {
    available(
        Os::current(),
        in_sandbox(),
        &|program| find_program(program).is_some(),
        feature,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn with(programs: &'static [&'static str]) -> impl Fn(&str) -> bool {
        move |name| programs.contains(&name)
    }

    #[test]
    fn a_sandbox_offers_nothing_of_the_host() {
        let all = with(&["pkexec", "journalctl", "xterm", "apt-get"]);
        for feature in [
            Feature::Elevation,
            Feature::HostTerminal,
            Feature::SystemLog,
            Feature::PackageInstall,
        ] {
            assert!(!available(Os::Linux, true, &all, feature), "{feature:?}");
            assert!(available(Os::Linux, false, &all, feature), "{feature:?}");
        }
    }

    #[test]
    fn features_need_their_program() {
        let none = with(&[]);
        assert!(!available(Os::Linux, false, &none, Feature::Elevation));
        assert!(!available(Os::Linux, false, &none, Feature::HostTerminal));
        assert!(!available(
            Os::Linux,
            false,
            &with(&["pkexec"]),
            Feature::PackageInstall
        ));
        assert!(available(
            Os::Linux,
            false,
            &with(&["pkexec", "dnf"]),
            Feature::PackageInstall
        ));
    }

    #[test]
    fn windows_has_a_terminal_but_no_shell_and_no_journal() {
        let all = with(&["powershell", "journalctl"]);
        assert!(available(Os::Windows, false, &all, Feature::HostTerminal));
        assert!(available(Os::Windows, false, &all, Feature::Elevation));
        assert!(!available(Os::Windows, false, &all, Feature::SystemLog));
        assert!(!available(Os::Windows, false, &all, Feature::PosixShell));
        assert!(available(Os::MacOs, false, &all, Feature::PosixShell));
    }
}
