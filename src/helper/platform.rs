// platform.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! The layer between a program and the operating system.
//!
//! A program that uses this layer never asks "is this Linux?" and never writes `/usr/share`,
//! `$HOME`, `pkexec` or `adb.exe`. It asks the layer, and the layer maps the question to the
//! operating system, the processor architecture and the package format it runs in. That keeps
//! everything that differs between Linux, Windows and macOS in one place.
//!
//! | Module | What it answers |
//! |---|---|
//! | [`target`] | Which operating system and which processor is this? ([`Os`], [`Arch`], [`Target`]) |
//! | [`os`] | Everything that belongs to one operating system, one file per system ([`OsProfile`]) |
//! | [`package_format`] | Everything that belongs to a package format: Flatpak, Snap, AppImage, native ([`PackageFormat`]) |
//! | [`dirs`] | Where do configuration, data, logs and the user's files live? ([`AppDirs`], [`DataDirs`], [`home_dir`]) |
//! | [`program`] | Where is an external program, and how is it started? ([`find_program`], [`host_command`]) |
//! | [`elevate`] | How does a command run with administrator rights? ([`elevation_command`]) |
//! | [`feature`] | What can this system do at all? ([`Feature`], [`supports`]) |
//! | [`package`] | How are programs installed on the host? ([`PackageManager`]) |
//! | [`logging`] | Where do log messages go? ([`init_logging`]) |
//!
//! Other parts of the library use the layer as well: [`run_command`](crate::helper::commands::execute_command::run_command),
//! [`command_pkexec`](crate::helper::commands::command_pkexec::command_pkexec),
//! [`get_config`](crate::helper::configuration::toml_config::get_config),
//! [`init_language`](crate::helper::language::language::init_language) and
//! [`SystemInfo`](crate::helper::sysinfo::system_info::SystemInfo) all ask it where things are.
//!
//! # What is implemented
//!
//! Linux is the platform that is run and tested. For Windows and macOS the profiles are written
//! down and covered by unit tests that run on any system (the tests compile every
//! [`OsProfile`] and ask [`os_profile_for`]), but they have not been run on those systems; the
//! Windows build is checked with `cargo check --target x86_64-pc-windows-gnu`. A mapping that
//! does not exist (for example `pkexec` on Windows) is reported as `None` or `false`, so the
//! program can hide the button instead of failing.
//!
//! # Only what the program needs is compiled
//!
//! The profile of the operating system the program is built for is the only one that is compiled
//! (a Windows program has no Linux code in it), and of the package formats only those whose Cargo
//! feature (`format-flatpak`, `format-snap`, `format-appimage`, `format-native`) is on. A Snap is
//! built with `--no-default-features --features format-snap`. Adding a system or a format is one
//! file and one line in a list: see [`os`] and [`package_format`].
//!
//! # Examples
//!
//! ```
//! use atlbase::prelude::*;
//!
//! // the operating system and the processor
//! let target = Target::current();
//! println!("running on {target}");
//!
//! // a program of the host, started without a console window on Windows
//! let output = host_command("echo").arg("hello").output().unwrap();
//! assert!(output.status.success());
//!
//! // what the program may offer
//! if supports(Feature::Elevation) {
//!     println!("commands can run as administrator");
//! }
//! ```

pub mod dirs;

pub mod elevate;

pub mod feature;

pub mod package_format;

pub mod logging;

pub mod os;

pub mod package;

pub mod program;

pub mod target;

// Re-exports
pub use dirs::*;
pub use elevate::*;
pub use feature::*;
pub use logging::*;
pub use os::*;
pub use package::*;
pub use package_format::*;
pub use program::*;
pub use target::*;
