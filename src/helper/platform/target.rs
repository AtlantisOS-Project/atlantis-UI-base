// target.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! The operating system and the processor architecture the program was built for.
//!
//! The values come from the compiler (`cfg!`), not from a guess at run time: a program built
//! for Windows is a Windows program, whatever it is started in (Wine and WSL included). The
//! names are the ones other systems use, so a download URL or a package name can be built from
//! them: [`Arch::name`] is the name of Rust and Flatpak (`x86_64`, `aarch64`),
//! [`Arch::debian_name`] the one of Debian, Ubuntu and Snap (`amd64`, `arm64`).

use std::fmt;

/// An operating system family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Os {
    /// Linux, in every distribution and package format.
    Linux,
    /// Windows.
    Windows,
    /// macOS.
    MacOs,
    /// Anything else (the BSDs, …): treated like a Unix.
    Other,
}

impl Os {
    /// The system this program was built for.
    ///
    /// # Examples
    ///
    /// ```
    /// use atlbase::prelude::Os;
    ///
    /// assert_eq!(Os::current() == Os::Linux, cfg!(target_os = "linux"));
    /// ```
    pub const fn current() -> Os {
        if cfg!(target_os = "linux") {
            Os::Linux
        } else if cfg!(target_os = "windows") {
            Os::Windows
        } else if cfg!(target_os = "macos") {
            Os::MacOs
        } else {
            Os::Other
        }
    }

    /// A short lower case name: `linux`, `windows`, `macos`, `other`.
    pub const fn name(self) -> &'static str {
        match self {
            Os::Linux => "linux",
            Os::Windows => "windows",
            Os::MacOs => "macos",
            Os::Other => "other",
        }
    }

    /// The ending of a program file: `.exe` on Windows, otherwise nothing.
    ///
    /// # Examples
    ///
    /// ```
    /// use atlbase::prelude::Os;
    ///
    /// assert_eq!(Os::Windows.exe_suffix(), ".exe");
    /// assert_eq!(Os::Linux.exe_suffix(), "");
    /// ```
    pub const fn exe_suffix(self) -> &'static str {
        match self {
            Os::Windows => ".exe",
            _ => "",
        }
    }

    /// Whether the system has a POSIX shell (`sh`, `bash`) as its normal command line.
    pub const fn has_posix_shell(self) -> bool {
        !matches!(self, Os::Windows)
    }
}

impl fmt::Display for Os {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

/// A processor architecture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arch {
    /// 64 bit x86 (`amd64`).
    X86_64,
    /// 64 bit ARM (`arm64`).
    Aarch64,
    /// 32 bit x86 (`i386`).
    X86,
    /// 32 bit ARM (`armhf`).
    Arm,
    /// Anything else.
    Other,
}

impl Arch {
    /// The architecture this program was built for.
    ///
    /// # Examples
    ///
    /// ```
    /// use atlbase::prelude::Arch;
    ///
    /// assert_eq!(Arch::current() == Arch::X86_64, cfg!(target_arch = "x86_64"));
    /// ```
    pub const fn current() -> Arch {
        if cfg!(target_arch = "x86_64") {
            Arch::X86_64
        } else if cfg!(target_arch = "aarch64") {
            Arch::Aarch64
        } else if cfg!(target_arch = "x86") {
            Arch::X86
        } else if cfg!(target_arch = "arm") {
            Arch::Arm
        } else {
            Arch::Other
        }
    }

    /// The name of Rust, Flatpak and most downloads: `x86_64`, `aarch64`, `x86`, `arm`.
    pub const fn name(self) -> &'static str {
        match self {
            Arch::X86_64 => "x86_64",
            Arch::Aarch64 => "aarch64",
            Arch::X86 => "x86",
            Arch::Arm => "arm",
            Arch::Other => "unknown",
        }
    }

    /// The name of Debian, Ubuntu and Snap: `amd64`, `arm64`, `i386`, `armhf`.
    ///
    /// # Examples
    ///
    /// ```
    /// use atlbase::prelude::Arch;
    ///
    /// assert_eq!(Arch::X86_64.debian_name(), "amd64");
    /// assert_eq!(Arch::Aarch64.debian_name(), "arm64");
    /// ```
    pub const fn debian_name(self) -> &'static str {
        match self {
            Arch::X86_64 => "amd64",
            Arch::Aarch64 => "arm64",
            Arch::X86 => "i386",
            Arch::Arm => "armhf",
            Arch::Other => "unknown",
        }
    }
}

impl fmt::Display for Arch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

/// An operating system on a processor architecture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Target {
    /// The operating system.
    pub os: Os,
    /// The processor architecture.
    pub arch: Arch,
}

impl Target {
    /// What this program was built for.
    ///
    /// # Examples
    ///
    /// ```
    /// use atlbase::prelude::{Arch, Os, Target};
    ///
    /// let target = Target::current();
    /// assert_eq!(target.os, Os::current());
    /// assert_eq!(target.arch, Arch::current());
    /// assert_eq!(Target { os: Os::Linux, arch: Arch::Aarch64 }.to_string(), "linux-aarch64");
    /// ```
    pub const fn current() -> Target {
        Target {
            os: Os::current(),
            arch: Arch::current(),
        }
    }
}

impl fmt::Display for Target {
    /// `<os>-<arch>`, for example `linux-x86_64` or `windows-aarch64`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", self.os, self.arch)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_follow_the_systems_that_use_them() {
        assert_eq!(Os::MacOs.name(), "macos");
        assert_eq!(Arch::X86.debian_name(), "i386");
        assert_eq!(Arch::Arm.debian_name(), "armhf");
        assert!(Os::Linux.has_posix_shell() && Os::MacOs.has_posix_shell());
        assert!(!Os::Windows.has_posix_shell());
    }

    #[test]
    fn the_current_target_is_known_on_the_systems_we_build_for() {
        assert_ne!(Os::current(), Os::Other);
        assert_ne!(Arch::current(), Arch::Other);
        assert_eq!(
            Target::current().to_string(),
            format!("{}-{}", Os::current(), Arch::current())
        );
    }
}
