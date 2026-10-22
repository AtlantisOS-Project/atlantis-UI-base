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
//! the question for one [`Feature`]; the program never has to know why. The answer comes from the
//! [`OsProfile`] of the system and the [`PackageFormat`](super::package_format::PackageFormat) of the package.

use super::os::{Has, OsProfile, os_profile};
use super::package::PackageManager;
use super::package_format::detect_package_format;
use super::program::find_program;

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

/// Whether the program runs in a sandbox (Flatpak or Snap), which has no access to the programs of
/// the host.
pub fn in_sandbox() -> bool {
    detect_package_format(&|key| std::env::var_os(key)).sandboxed
}

/// Whether `feature` is available on a system with `profile`, in a sandbox or not. `has` tells
/// whether a program is installed.
pub(crate) fn available(profile: &OsProfile, sandboxed: bool, has: Has, feature: Feature) -> bool {
    match feature {
        Feature::Elevation => !sandboxed && profile.elevation_program.is_some_and(has),
        Feature::HostTerminal => !sandboxed && (profile.terminal_available)(has),
        Feature::SystemLog => {
            !sandboxed
                && profile
                    .journal_follow
                    .is_some_and(|follow| follow("x").first().is_some_and(|program| has(program)))
        }
        Feature::PackageInstall => {
            available(profile, sandboxed, has, Feature::Elevation)
                && profile
                    .package_managers
                    .iter()
                    .any(|manager: &PackageManager| has(manager.program()))
        }
        Feature::PosixShell => profile.posix_shell,
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
        os_profile(),
        in_sandbox(),
        &|program| find_program(program).is_some(),
        feature,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helper::platform::os::os_profile_for;
    use crate::helper::platform::target::Os;

    fn with(programs: &'static [&'static str]) -> impl Fn(&str) -> bool {
        move |name| programs.contains(&name)
    }

    fn profile(os: Os) -> &'static OsProfile {
        os_profile_for(os).unwrap()
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
            assert!(
                !available(profile(Os::Linux), true, &all, feature),
                "{feature:?}"
            );
            assert!(
                available(profile(Os::Linux), false, &all, feature),
                "{feature:?}"
            );
        }
    }

    #[test]
    fn features_need_their_program() {
        let linux = profile(Os::Linux);
        let none = with(&[]);
        assert!(!available(linux, false, &none, Feature::Elevation));
        assert!(!available(linux, false, &none, Feature::HostTerminal));
        assert!(!available(
            linux,
            false,
            &with(&["pkexec"]),
            Feature::PackageInstall
        ));
        assert!(available(
            linux,
            false,
            &with(&["pkexec", "dnf"]),
            Feature::PackageInstall
        ));
    }

    #[test]
    fn windows_has_a_terminal_but_no_shell_and_no_journal() {
        let all = with(&["powershell", "journalctl"]);
        let windows = profile(Os::Windows);
        assert!(available(windows, false, &all, Feature::HostTerminal));
        assert!(available(windows, false, &all, Feature::Elevation));
        assert!(!available(windows, false, &all, Feature::SystemLog));
        assert!(!available(windows, false, &all, Feature::PosixShell));
        assert!(available(
            profile(Os::MacOs),
            false,
            &all,
            Feature::PosixShell
        ));
    }
}
