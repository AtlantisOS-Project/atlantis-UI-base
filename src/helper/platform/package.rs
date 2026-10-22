// package.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! The package manager of the host.
//!
//! Programs the application needs (`adb`, `curl`, …) can be installed with the package manager
//! of the system. The names of the packages differ between the managers (`p7zip-full` on
//! Debian, `p7zip` on Fedora), so the application keeps its own list per manager; this module
//! finds out which manager is there and how it is called.

use super::os::os_profile;
use super::program::find_program;

/// The directory with the APT sources. A file in it that names the program means the program
/// was installed from a PPA (see [`check_application_environment`](crate::helper::configuration::application_environment::check_application_environment)).
pub const APT_SOURCES_DIR: &str = "/etc/apt/sources.list.d";

/// A package manager.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PackageManager {
    /// Debian, Ubuntu and their relatives (`apt-get`).
    Apt,
    /// Fedora, RHEL (`dnf`).
    Dnf,
    /// Arch Linux (`pacman`).
    Pacman,
    /// openSUSE (`zypper`).
    Zypper,
    /// Windows (`winget`).
    Winget,
    /// macOS (Homebrew, `brew`).
    Homebrew,
}

impl PackageManager {
    /// Every manager, in the order [`detect`](PackageManager::detect) tries them.
    pub const ALL: [PackageManager; 6] = [
        PackageManager::Apt,
        PackageManager::Dnf,
        PackageManager::Pacman,
        PackageManager::Zypper,
        PackageManager::Winget,
        PackageManager::Homebrew,
    ];

    /// The program that installs packages.
    pub const fn program(self) -> &'static str {
        match self {
            PackageManager::Apt => "apt-get",
            PackageManager::Dnf => "dnf",
            PackageManager::Pacman => "pacman",
            PackageManager::Zypper => "zypper",
            PackageManager::Winget => "winget",
            PackageManager::Homebrew => "brew",
        }
    }

    /// Whether installing a package needs administrator rights (Homebrew and winget ask by
    /// themselves).
    pub const fn needs_elevation(self) -> bool {
        !matches!(self, PackageManager::Winget | PackageManager::Homebrew)
    }

    /// The manager of this computer, if it is a known one.
    ///
    /// # Examples
    ///
    /// ```
    /// use atlbase::prelude::PackageManager;
    ///
    /// match PackageManager::detect() {
    ///     Some(manager) => println!("packages are installed with {}", manager.program()),
    ///     None => println!("no known package manager"),
    /// }
    /// ```
    pub fn detect() -> Option<PackageManager> {
        os_profile()
            .package_managers
            .iter()
            .copied()
            .find(|manager| find_program(manager.program()).is_some())
    }

    /// The command line that installs `packages` without asking questions. It needs
    /// administrator rights if [`needs_elevation`](PackageManager::needs_elevation) says so:
    /// put [`elevation_command`](super::elevate::elevation_command) in front of it.
    ///
    /// # Examples
    ///
    /// ```
    /// use atlbase::prelude::PackageManager;
    ///
    /// assert_eq!(
    ///     PackageManager::Apt.install_command(&["adb", "curl"]),
    ///     ["apt-get", "install", "-y", "adb", "curl"]
    /// );
    /// ```
    pub fn install_command(self, packages: &[&str]) -> Vec<String> {
        let head: &[&str] = match self {
            PackageManager::Apt => &["install", "-y"],
            PackageManager::Dnf => &["install", "-y"],
            PackageManager::Pacman => &["-S", "--noconfirm"],
            PackageManager::Zypper => &["--non-interactive", "install"],
            PackageManager::Winget => &["install", "--exact", "--silent"],
            PackageManager::Homebrew => &["install"],
        };
        std::iter::once(self.program())
            .chain(head.iter().copied())
            .chain(packages.iter().copied())
            .map(str::to_string)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn install_commands_name_the_manager_first() {
        for manager in PackageManager::ALL {
            let command = manager.install_command(&["a", "b"]);
            assert_eq!(command[0], manager.program());
            assert_eq!(command[command.len() - 2..], ["a", "b"]);
        }
        assert_eq!(
            PackageManager::Pacman.install_command(&["adb"]),
            ["pacman", "-S", "--noconfirm", "adb"]
        );
    }

    #[test]
    fn only_the_managers_of_this_system_are_detected() {
        if let Some(manager) = PackageManager::detect() {
            assert!(os_profile().package_managers.contains(&manager));
        }
        assert!(PackageManager::Apt.needs_elevation());
        assert!(!PackageManager::Winget.needs_elevation());
    }
}
