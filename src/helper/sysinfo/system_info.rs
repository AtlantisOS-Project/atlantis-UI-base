// system_info.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! A short description of the computer: distribution, kernel, processor, graphics card and
//! desktop, and the version of installed Debian packages.
//!
//! [`SystemInfo::collect`] reads a few files (no program is started), which takes a moment: call it
//! in a background thread. The parsers ([`os_release_value`], [`cpu_model`],
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

#[cfg(target_os = "linux")]
use crate::helper::file::get_config_value::config_line_value;
#[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
use crate::helper::platform::target::Os;
use crate::helper::platform::target::Target;
#[cfg(target_os = "linux")]
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
    /// The graphics card (Linux: from `/sys/class/drm`, with the name from the PCI id list of
    /// the system if there is one).
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
    /// Asks the computer. Reads a few files on Linux (no `lspci`, no other program); call it in
    /// the background.
    ///
    /// Linux is described completely. On the other systems the distribution is the name of the
    /// system, the architecture, the language and the desktop come from the program and its
    /// environment, and the rest stays empty. Only the code for the system the program is built
    /// for is compiled.
    pub fn collect() -> Self {
        let env = |name: &str| std::env::var(name).unwrap_or_default();
        let mut info = Self::of_this_system();
        info.architecture = Target::current().arch.name().to_string();
        info.desktop = env("XDG_CURRENT_DESKTOP");
        info.language = language_code(&env("LANG"));
        info.session = env("XDG_SESSION_TYPE");
        info
    }

    #[cfg(target_os = "linux")]
    fn of_this_system() -> Self {
        let os_release = fs::read_to_string("/etc/os-release").unwrap_or_default();
        let cpuinfo = fs::read_to_string("/proc/cpuinfo").unwrap_or_default();
        let kernel = fs::read_to_string("/proc/sys/kernel/osrelease").unwrap_or_default();

        Self {
            distribution: os_release_value(&os_release, "NAME").unwrap_or_default(),
            version: first_word(&os_release_value(&os_release, "VERSION").unwrap_or_default()),
            kernel: kernel.trim().to_string(),
            processor: cpu_model(&cpuinfo).unwrap_or_default(),
            graphics_card: graphics_card().unwrap_or_default(),
            ..SystemInfo::default()
        }
    }

    #[cfg(target_os = "windows")]
    fn of_this_system() -> Self {
        SystemInfo {
            distribution: "Windows".to_string(),
            ..SystemInfo::default()
        }
    }

    #[cfg(target_os = "macos")]
    fn of_this_system() -> Self {
        SystemInfo {
            distribution: "macOS".to_string(),
            ..SystemInfo::default()
        }
    }

    #[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
    fn of_this_system() -> Self {
        SystemInfo {
            distribution: Os::current().name().to_string(),
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
#[cfg(target_os = "linux")]
pub fn os_release_value(text: &str, key: &str) -> Option<String> {
    text.lines().find_map(|line| config_line_value(line, key))
}

/// The vendor of a graphics card by its PCI id, for the systems that have no list of PCI ids.
#[cfg(target_os = "linux")]
fn vendor_name(vendor: &str) -> Option<&'static str> {
    match vendor {
        "1002" => Some("AMD/ATI"),
        "10de" => Some("NVIDIA"),
        "8086" => Some("Intel"),
        "1af4" => Some("Red Hat (virtio)"),
        "15ad" => Some("VMware"),
        "1234" => Some("QEMU"),
        "1ab8" => Some("Parallels"),
        "80ee" => Some("VirtualBox"),
        _ => None,
    }
}

/// The names of a vendor and a device in a list of PCI ids (`pci.ids`: a vendor line `1002  Name`,
/// then device lines `\t1681  Name`), if the list has them.
#[cfg(target_os = "linux")]
fn pci_ids_names(list: &str, vendor: &str, device: &str) -> Option<(String, Option<String>)> {
    let mut vendor_name = None;
    for line in list.lines() {
        if line.starts_with('#') || line.trim().is_empty() {
            continue;
        }
        if !line.starts_with('\t') {
            if vendor_name.is_some() {
                // the next vendor: the device is not listed
                return vendor_name.map(|name| (name, None));
            }
            if let Some((id, name)) = line.split_once("  ")
                && id.eq_ignore_ascii_case(vendor)
            {
                vendor_name = Some(name.trim().to_string());
            }
        } else if vendor_name.is_some()
            && !line.starts_with("\t\t")
            && let Some((id, name)) = line.trim_start().split_once("  ")
            && id.eq_ignore_ascii_case(device)
        {
            return vendor_name.map(|vendor| (vendor, Some(name.trim().to_string())));
        }
    }
    vendor_name.map(|name| (name, None))
}

/// A graphics card as text: with the names of the PCI id list (`pci.ids`) when it is given and
/// has them, otherwise vendor and ids, and always the driver.
#[cfg(target_os = "linux")]
fn describe_graphics_card(
    vendor: &str,
    device: &str,
    driver: Option<&str>,
    pci_ids: Option<&str>,
) -> String {
    let text = match pci_ids.and_then(|list| pci_ids_names(list, vendor, device)) {
        Some((vendor, Some(device))) => format!("{vendor} {device}"),
        Some((vendor, None)) => format!("{vendor} [{device}]"),
        None => match vendor_name(vendor) {
            Some(name) => format!("{name} [{vendor}:{device}]"),
            None => format!("[{vendor}:{device}]"),
        },
    };
    match driver {
        Some(driver) => format!("{text} ({driver})"),
        None => text,
    }
}

/// The first graphics card of the computer: the first device of `/sys/class/drm/card<N>` (the
/// connectors `card<N>-…` are no devices).
#[cfg(target_os = "linux")]
fn graphics_card() -> Option<String> {
    let mut cards: Vec<_> = fs::read_dir("/sys/class/drm")
        .ok()?
        .flatten()
        .filter(|entry| {
            let name = entry.file_name().to_string_lossy().into_owned();
            name.strip_prefix("card")
                .is_some_and(|number| number.chars().all(|c| c.is_ascii_digit()))
        })
        .collect();
    cards.sort_by_key(|entry| entry.file_name());
    let device = cards.first()?.path().join("device");
    let read = |file: &str| {
        fs::read_to_string(device.join(file))
            .ok()
            .map(|text| text.trim().trim_start_matches("0x").to_string())
    };
    let (vendor, id) = (read("vendor")?, read("device")?);
    let driver = fs::read_to_string(device.join("uevent"))
        .ok()
        .and_then(|uevent| {
            uevent
                .lines()
                .find_map(|line| line.strip_prefix("DRIVER=").map(str::to_string))
        });
    let list = [
        "/usr/share/hwdata/pci.ids",
        "/usr/share/misc/pci.ids",
        "/usr/share/pci.ids",
    ]
    .iter()
    .find_map(|path| fs::read_to_string(path).ok());
    Some(describe_graphics_card(
        &vendor,
        &id,
        driver.as_deref(),
        list.as_deref(),
    ))
}

/// The first word of the version (`24.04.1 LTS (Noble Numbat)` becomes `24.04.1`).
#[cfg(target_os = "linux")]
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
#[cfg(target_os = "linux")]
pub fn cpu_model(cpuinfo: &str) -> Option<String> {
    cpuinfo.lines().find_map(|line| {
        let (key, value) = line.split_once(':')?;
        (key.trim() == "model name").then(|| value.trim().to_string())
    })
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

/// The installed version of a Debian package, read from the status file of `dpkg`
/// (`/var/lib/dpkg/status`); no program is started.
///
/// Returns `None` on a system without `dpkg` (Fedora, Arch, a sandbox) and for a package that is
/// not installed.
///
/// # Examples
///
/// ```
/// use atlbase::prelude::package_version;
///
/// assert_eq!(package_version("this-package-does-not-exist-1234"), None);
/// ```
pub fn package_version(package: &str) -> Option<String> {
    #[cfg(target_os = "linux")]
    {
        let file = std::fs::File::open("/var/lib/dpkg/status").ok()?;
        dpkg_status_version(std::io::BufReader::new(file), package)
    }
    #[cfg(not(target_os = "linux"))]
    {
        let _ = package;
        None
    }
}

/// The version of an installed package in the text of a `dpkg` status file: stanzas separated by an
/// empty line, with `Package:`, `Status:` and `Version:`.
#[cfg(target_os = "linux")]
fn dpkg_status_version(status: impl std::io::BufRead, package: &str) -> Option<String> {
    let (mut name, mut installed, mut version) = (false, false, None::<String>);
    for line in status.lines().map_while(Result::ok) {
        if line.is_empty() {
            if name && installed && version.is_some() {
                return version;
            }
            (name, installed, version) = (false, false, None);
        } else if let Some(value) = line.strip_prefix("Package: ") {
            name = value == package;
        } else if let Some(value) = line.strip_prefix("Status: ") {
            installed = value.ends_with(" installed");
        } else if let Some(value) = line.strip_prefix("Version: ") {
            version = Some(value.to_string());
        }
    }
    (name && installed).then_some(version).flatten()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "linux")]
    #[test]
    fn the_version_is_its_first_word() {
        assert_eq!(first_word("24.04.1 LTS (Noble)"), "24.04.1");
        assert_eq!(first_word(""), "");
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn os_release_keeps_the_text_in_quotes_together() {
        let text = "PRETTY_NAME=\"Ubuntu 24.04\"\nVERSION=\"24.04.1 LTS (Noble)\"\n";
        assert_eq!(
            os_release_value(text, "VERSION").as_deref(),
            Some("24.04.1 LTS (Noble)")
        );
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn the_graphics_card_gets_its_name_from_the_pci_list_or_its_ids() {
        let list = "# comment\n1002  Advanced Micro Devices, Inc. [AMD/ATI]\n\t1681  Rembrandt [Radeon 680M]\n\t\t1234 5678  Subsystem\n10de  NVIDIA Corporation\n\t2204  GA102\n";
        assert_eq!(
            describe_graphics_card("1002", "1681", Some("amdgpu"), Some(list)),
            "Advanced Micro Devices, Inc. [AMD/ATI] Rembrandt [Radeon 680M] (amdgpu)"
        );
        assert_eq!(
            describe_graphics_card("10de", "9999", None, Some(list)),
            "NVIDIA Corporation [9999]"
        );
        assert_eq!(
            describe_graphics_card("8086", "46a6", Some("i915"), None),
            "Intel [8086:46a6] (i915)"
        );
        assert_eq!(
            describe_graphics_card("abcd", "0001", None, None),
            "[abcd:0001]"
        );
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn the_status_file_tells_the_installed_version() {
        let status = "Package: a\nStatus: install ok installed\nVersion: 1.0\n\nPackage: b\nStatus: deinstall ok config-files\nVersion: 2.0\n\nPackage: c\nStatus: install ok installed\nVersion: 3.1-2\n";
        let version = |name| dpkg_status_version(status.as_bytes(), name);
        assert_eq!(version("a").as_deref(), Some("1.0"));
        assert_eq!(version("b"), None);
        assert_eq!(version("c").as_deref(), Some("3.1-2"));
        assert_eq!(version("d"), None);
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn missing_values_are_none() {
        assert_eq!(cpu_model("processor : 0\n"), None);
    }

    #[test]
    fn the_language_is_the_code_before_the_underscore() {
        assert_eq!(language_code("C.UTF-8"), "C");
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn this_computer_has_a_kernel() {
        // /proc exists on every Linux system the tests run on
        assert!(!SystemInfo::collect().kernel.is_empty());
    }
}
