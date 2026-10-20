// common_options.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! The command line options all AtlantisOS programs share.
//!
//! ```text
//!    -debug [flags] – Debugging
//!           -snp    – Set snap environment
//!           -flp    – Set flatpak environment
//!           -lang   – Set fallback language as active
//!    -v, --version  – Show Version Info
//!    -h, --help     – Show this help
//! ```
//!
//! The flags `-snp`, `-flp` and `-lang` only work together with `-debug` (and after it).
//! Every other argument is passed on to GTK.
//!
//! # Usage
//!
//! ```no_run
//! use atlbase::prelude::*;
//!
//! match read_common_options(std::env::args()) {
//!     Err(CliRequest::Version) => {
//!         let environment = current_environment("demo", "/etc/apt/sources.list.d");
//!         println!("{}", version_text("Demo", env!("CARGO_PKG_VERSION"), environment));
//!     }
//!     Err(CliRequest::Help) => print!("Usage: demo\n{COMMON_OPTIONS_HELP}"),
//!     Ok(options) => {
//!         // sets the debug mode and the fake sandbox variables
//!         apply_debug_options(&options, "org.example.Demo");
//!         // start the application with `options.gtk_args`
//!     }
//! }
//! ```

use crate::helper::configuration::application_environment::AppEnvironment;
use std::sync::atomic::{AtomicBool, Ordering};

/// The lines of `--help` that describe the shared options; put them below your own usage
/// line.
pub const COMMON_OPTIONS_HELP: &str = "Options:\n\
     \x20  -debug [flags] – Debugging\n\
     \x20         -snp    – Set snap environment\n\
     \x20         -flp    – Set flatpak environment\n\
     \x20         -lang   – Set fallback language as active\n\
     \x20  -v, --version  – Show Version Info\n\
     \x20  -h, --help     – Show this help\n";

/// The shared options of one program start.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CliOptions {
    /// `-debug`: the program skips its checks for devices and files.
    pub debug: bool,
    /// `-debug -snp`: pretend to run in a Snap sandbox.
    pub fake_snap: bool,
    /// `-debug -flp`: pretend to run in a Flatpak sandbox.
    pub fake_flatpak: bool,
    /// `-debug -lang`: use the English fallback language.
    pub fallback_language: bool,
    /// The arguments for GTK (the program name first).
    pub gtk_args: Vec<String>,
}

/// What the user asked for instead of starting the program.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CliRequest {
    /// `--version`: print the version and exit.
    Version,
    /// `--help`: print the help and exit.
    Help,
}

fn is_debug(arg: &str) -> bool {
    matches!(arg, "-d" | "--d" | "-debug" | "--debug")
}

fn is_version(arg: &str) -> bool {
    matches!(arg, "-v" | "--v" | "-version" | "--version")
}

fn is_help(arg: &str) -> bool {
    matches!(arg, "-h" | "--help")
}

/// Reads the shared options from the arguments (the program name first) without any side
/// effect: nothing is printed and no variable is set.
///
/// A flag that needs `-debug` but comes without it is passed on to GTK, with a warning on
/// `stderr`.
///
/// # Errors
///
/// Returns the [`CliRequest`] if the user asked for the version or for help.
///
/// # Examples
///
/// ```
/// use atlbase::prelude::{CliRequest, read_common_options};
///
/// let args = |list: &[&str]| list.iter().map(|arg| arg.to_string()).collect::<Vec<_>>();
///
/// let options = read_common_options(args(&["demo", "-debug", "-lang", "--gapplication-service"])).unwrap();
/// assert!(options.debug && options.fallback_language);
/// assert_eq!(options.gtk_args, ["demo", "--gapplication-service"]);
///
/// assert_eq!(read_common_options(args(&["demo", "--version"])), Err(CliRequest::Version));
/// ```
pub fn read_common_options(
    args: impl IntoIterator<Item = String>,
) -> Result<CliOptions, CliRequest> {
    let mut args = args.into_iter();
    let mut options = CliOptions {
        gtk_args: args.next().into_iter().collect(), // the program name
        ..CliOptions::default()
    };

    for arg in args {
        if is_debug(&arg) {
            options.debug = true;
        } else if is_version(&arg) {
            return Err(CliRequest::Version);
        } else if is_help(&arg) {
            return Err(CliRequest::Help);
        } else if options.debug && arg == "-snp" {
            options.fake_snap = true;
        } else if options.debug && arg == "-flp" {
            options.fake_flatpak = true;
        } else if options.debug && arg == "-lang" {
            options.fallback_language = true;
        } else {
            if !options.debug && arg.starts_with('-') {
                eprintln!("Warning: '{arg}' can only be used with -debug.");
            }
            options.gtk_args.push(arg);
        }
    }
    Ok(options)
}

/// The text of `--version`, for example `Fastboot-Assistant [Flatpak] 0.9.1`.
///
/// # Examples
///
/// ```
/// use atlbase::prelude::{AppEnvironment, version_text};
///
/// assert_eq!(
///     version_text("Demo", "1.2.0", AppEnvironment::Snap),
///     "Demo [Snap] 1.2.0"
/// );
/// ```
pub fn version_text(name: &str, version: &str, environment: AppEnvironment) -> String {
    format!("{name} [{}] {version}", environment.label())
}

static DEBUG: AtomicBool = AtomicBool::new(false);
static FALLBACK_LANGUAGE: AtomicBool = AtomicBool::new(false);

/// Whether the debug mode is on (`-debug`). The program decides what that means; usually
/// checks for devices and files are skipped.
pub fn debug_mode() -> bool {
    DEBUG.load(Ordering::Relaxed)
}

/// Turns the debug mode on or off.
pub fn set_debug_mode(on: bool) {
    DEBUG.store(on, Ordering::Relaxed);
}

/// Whether `-debug -lang` asked for the English fallback language. Pass it to
/// [`init_language`](crate::helper::language::language::init_language).
pub fn fallback_language() -> bool {
    FALLBACK_LANGUAGE.load(Ordering::Relaxed)
}

/// Applies the options: turns the debug mode on and remembers the fallback language. `-snp`
/// sets `$SNAP` and `-flp` sets `$FLATPAK_ID` (to `flatpak_id`), so the program believes it
/// runs in that sandbox.
///
/// Call it once at the start of `main`, before other threads exist, because it changes the
/// environment of the process.
pub fn apply_debug_options(options: &CliOptions, flatpak_id: &str) {
    set_debug_mode(options.debug);
    FALLBACK_LANGUAGE.store(options.fallback_language, Ordering::Relaxed);

    // SAFETY: documented to be called from `main` before any thread is started.
    unsafe {
        if options.fake_snap {
            std::env::set_var("SNAP", "/tmp/snap-debug");
            println!("[DEBUG] SNAP-Env: $SNAP=/tmp/snap-debug");
        }
        if options.fake_flatpak {
            std::env::set_var("FLATPAK_ID", flatpak_id);
            println!("[DEBUG] Flatpak-Env: $FLATPAK_ID is set");
        }
    }
}
