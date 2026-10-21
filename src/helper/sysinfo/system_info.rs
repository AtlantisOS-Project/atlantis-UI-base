// system_info.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! A short description of the computer: distribution, kernel, processor, graphics card and
//! desktop, and the version of installed Debian packages.
//!
//! [`SystemInfo::collect`] runs `lspci` and reads a few files, which takes a moment: call it
//! in a background thread. The parsers ([`os_release_value`], [`cpu_model`], [`gpu_model`],
//! [`language_code`]) work on text and can be used on their own.
//!
//! # Examples
//!
//! ```
//! use atlbase::prelude::SystemInfo;
//!
//! let info = SystemInfo::collect();
//! println!("{} {} (kernel {})", info.distribution, info.version, info.kernel);
//! ```

use crate::helper::commands::execute_command::capture_command_output;
use crate::helper::file::get_config_value::config_line_value;
use crate::helper::platform::target::{Os, Target};
use std::fs;

/// What is known about the computer. A value that cannot be found is an empty string.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SystemInfo {
    /// The name of the distribution (`NAME` in `/etc/os-release`); `Windows` or `macOS` on
    /// those systems.
    pub distribution: String,
    /// The version of the distribution, without the name (`24.04.1`).
    pub version: String,
    /// The version of the kernel.
    pub kernel: String,
    /// The name of the processor.
    pub processor: String,
    /// The graphics card (needs `lspci`).
    pub graphics_card: String,
    /// The desktop (`$XDG_CURRENT_DESKTOP`).
    pub desktop: String,
    /// The language code of `$LANG` (`de`).
    pub language: String,
    /// `wayland` or `x11` (`$XDG_SESSION_TYPE`).
    pub session: String,
    /// The processor architecture this program was built for (`x86_64`, `aarch64`).
    pub architecture: String,
}

impl SystemInfo {
    /// Asks the computer. Runs `lspci` on Linux; call it in the background.
    ///
    /// Linux is described completely. On the other systems the distribution is the name of the
    /// system, the architecture, the language and the desktop come from the program and its
    /// environment, and the rest stays empty.
    pub fn collect() -> Self {
        let target = Target::current();
        let env = |name: &str| std::env::var(name).unwrap_or_default();
        let mut info = match target.os {
            Os::Linux => Self::collect_linux(),
            Os::Windows => Self::named("Windows"),
            Os::MacOs => Self::named("macOS"),
            Os::Other => Self::named(target.os.name()),
        };
        info.architecture = target.arch.name().to_string();
        info.desktop = env("XDG_CURRENT_DESKTOP");
        info.language = language_code(&env("LANG"));
        info.session = env("XDG_SESSION_TYPE");
        info
    }

    fn named(distribution: &str) -> Self {
        SystemInfo {
            distribution: distribution.to_string(),
            ..SystemInfo::default()
        }
    }

    fn collect_linux() -> Self {
        let os_release = fs::read_to_string("/etc/os-release").unwrap_or_default();
        let cpuinfo = fs::read_to_string("/proc/cpuinfo").unwrap_or_default();
        let kernel = fs::read_to_string("/proc/sys/kernel/osrelease").unwrap_or_default();
        let lspci = capture_command_output(&["lspci"]).unwrap_or_default();

        Self {
            distribution: os_release_value(&os_release, "NAME").unwrap_or_default(),
            version: first_word(&os_release_value(&os_release, "VERSION").unwrap_or_default()),
            kernel: kernel.trim().to_string(),
            processor: cpu_model(&cpuinfo).unwrap_or_default(),
            graphics_card: gpu_model(&lspci).unwrap_or_default(),
            ..SystemInfo::default()
        }
    }
}

/// The value of `key` in a file in the `os-release` format (`NAME="Ubuntu"`).
///
/// # Examples
///
/// ```
/// use atlbase::prelude::os_release_value;
///
/// let text = "NAME=\"Ubuntu\"\nVERSION=\"24.04.1 LTS (Noble Numbat)\"\n";
/// assert_eq!(os_release_value(text, "NAME").as_deref(), Some("Ubuntu"));
/// assert_eq!(os_release_value(text, "ID"), None);
/// ```
pub fn os_release_value(text: &str, key: &str) -> Option<String> {
    text.lines().find_map(|line| config_line_value(line, key))
}

/// The first word of the version (`24.04.1 LTS (Noble Numbat)` becomes `24.04.1`).
fn first_word(text: &str) -> String {
    text.split_whitespace()
        .next()
        .unwrap_or_default()
        .to_string()
}

/// The name of the processor from the text of `/proc/cpuinfo`.
///
/// # Examples
///
/// ```
/// use atlbase::prelude::cpu_model;
///
/// let cpuinfo = "processor\t: 0\nmodel name\t: AMD Ryzen 5 5600\n";
/// assert_eq!(cpu_model(cpuinfo).as_deref(), Some("AMD Ryzen 5 5600"));
/// ```
pub fn cpu_model(cpuinfo: &str) -> Option<String> {
    cpuinfo.lines().find_map(|line| {
        let (key, value) = line.split_once(':')?;
        (key.trim() == "model name").then(|| value.trim().to_string())
    })
}

/// The graphics card from the output of `lspci`: the text after the colon of the first VGA
/// line.
///
/// # Examples
///
/// ```
/// use atlbase::prelude::gpu_model;
///
/// let lspci = "00:00.0 Host bridge: Intel Corporation\n00:02.0 VGA compatible controller: Intel UHD 620\n";
/// assert_eq!(gpu_model(lspci).as_deref(), Some("Intel UHD 620"));
/// ```
pub fn gpu_model(lspci: &str) -> Option<String> {
    lspci
        .lines()
        .find(|line| line.to_lowercase().contains("vga"))
        .and_then(|line| line.split_once(": "))
        .map(|(_, name)| name.trim().to_string())
}

/// The language code of `$LANG` (`de_DE.UTF-8` becomes `de`).
///
/// # Examples
///
/// ```
/// use atlbase::prelude::language_code;
///
/// assert_eq!(language_code("de_DE.UTF-8"), "de");
/// assert_eq!(language_code(""), "");
/// ```
pub fn language_code(lang: &str) -> String {
    lang.split(['_', '.'])
        .next()
        .unwrap_or_default()
        .to_string()
}

/// The installed version of a Debian package, if `dpkg-query` finds it.
///
/// Returns `None` on a system without `dpkg` (Fedora, Arch) and for a package that is not
/// installed.
///
/// # Examples
///
/// ```
/// use atlbase::prelude::package_version;
///
/// assert_eq!(package_version("this-package-does-not-exist-1234"), None);
/// ```
pub fn package_version(package: &str) -> Option<String> {
    let version = capture_command_output(&["dpkg-query", "-W", "-f=${Version}", package]).ok()?;
    let version = version.trim();
    (!version.is_empty()).then(|| version.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_version_is_its_first_word() {
        assert_eq!(first_word("24.04.1 LTS (Noble)"), "24.04.1");
        assert_eq!(first_word(""), "");
    }

    #[test]
    fn os_release_keeps_the_text_in_quotes_together() {
        let text = "PRETTY_NAME=\"Ubuntu 24.04\"\nVERSION=\"24.04.1 LTS (Noble)\"\n";
        assert_eq!(
            os_release_value(text, "VERSION").as_deref(),
            Some("24.04.1 LTS (Noble)")
        );
    }

    #[test]
    fn missing_values_are_none() {
        assert_eq!(cpu_model("processor : 0\n"), None);
        assert_eq!(gpu_model("00:1f.3 Audio device: Intel\n"), None);
        assert_eq!(language_code("C.UTF-8"), "C");
    }

    #[test]
    fn this_computer_has_a_kernel() {
        // /proc exists on every Linux system the tests run on
        assert!(!SystemInfo::collect().kernel.is_empty());
    }
}
