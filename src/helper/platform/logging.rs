// logging.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! Where log messages go.
//!
//! The standard macros of the [`log`] crate (`info!`, `warn!`, …) need a logger.
//! [`init_logging`] installs the one that belongs to the system:
//!
//! | System | Destination |
//! |---|---|
//! | Linux, macOS | the system log (syslog; `journalctl -t <app>` on systemd systems) |
//! | Windows | the file [`AppDirs::log_file`](super::dirs::AppDirs::log_file) |
//!
//! [`journal_follow_command`] gives the command that shows the messages as they come, for the
//! log viewer; systems without a system log have none.

use super::dirs::AppDirs;
use super::os::{os_profile, os_profile_for};
use super::target::Os;
use log::{Level, LevelFilter, Log, Metadata, Record, SetLoggerError};
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

/// A logger that appends the messages to a file, one line each:
/// `<seconds since 1970> <LEVEL> <message>`. The file is never rotated or shortened.
// ponytail: no rotation; add it if a program logs a lot
struct FileLogger {
    file: Mutex<File>,
}

impl Log for FileLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Debug
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }
        let seconds = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(0, |time| time.as_secs());
        let mut file = self.file.lock().unwrap_or_else(|p| p.into_inner());
        // a log line that cannot be written is lost; the program goes on
        let _ = writeln!(file, "{seconds} {} {}", record.level(), record.args());
    }

    fn flush(&self) {
        let _ = self.file.lock().unwrap_or_else(|p| p.into_inner()).flush();
    }
}

/// Logs to the file `path` (the folders above it are created). The level is `Debug`.
///
/// # Errors
///
/// Returns a [`log::SetLoggerError`] if a logger has already been registered, and prints a
/// message to `stderr` and returns `Ok` if the file cannot be opened (a program must not stop
/// because it cannot log).
pub fn init_file_logging(path: &Path) -> Result<(), SetLoggerError> {
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    let file = match OpenOptions::new().create(true).append(true).open(path) {
        Ok(file) => file,
        Err(error) => {
            eprintln!(
                "[ERROR] Cannot open the log file {}: {error}",
                path.display()
            );
            return Ok(());
        }
    };
    log::set_boxed_logger(Box::new(FileLogger {
        file: Mutex::new(file),
    }))
    .map(|()| log::set_max_level(LevelFilter::Debug))
}

/// Installs the logger of the system for the application `app_name`: syslog on Linux and
/// macOS, the file [`AppDirs::log_file`](super::dirs::AppDirs::log_file) on Windows. Afterwards `info!()`, `warn!()`,
/// `error!()` and so on work. The level is `Debug`.
///
/// # Errors
///
/// Returns a [`log::SetLoggerError`] if a logger has already been registered for this process.
///
/// # Examples
///
/// ```no_run
/// use atlbase::prelude::init_logging;
///
/// init_logging("my-app").expect("a logger was already installed");
/// log::info!("started");
/// ```
pub fn init_logging(app_name: &str) -> Result<(), SetLoggerError> {
    #[cfg(unix)]
    if os_profile().syslog {
        return crate::helper::log::write_log::init_syslog(app_name);
    }
    match AppDirs::new(app_name) {
        Some(dirs) => init_file_logging(&dirs.log_file()),
        None => Ok(()),
    }
}

/// The command that prints the messages of `app_name` as they come, or `None` where the system
/// has no log to follow (only Linux with `journalctl` has one).
///
/// # Examples
///
/// ```
/// use atlbase::prelude::{Os, journal_follow_command_for};
///
/// if let Some(command) = journal_follow_command_for(Os::current(), "my-app") {
///     assert!(command.contains(&"my-app".to_string()));
/// }
/// ```
pub fn journal_follow_command_for(os: Os, app_name: &str) -> Option<Vec<String>> {
    os_profile_for(os)?
        .journal_follow
        .map(|follow| follow(app_name))
}

/// [`journal_follow_command_for`] for the system the program runs on.
pub fn journal_follow_command(app_name: &str) -> Option<Vec<String>> {
    os_profile().journal_follow.map(|follow| follow(app_name))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_file_logger_appends_lines() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("deep/app.log");
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        let logger = FileLogger {
            file: Mutex::new(
                OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(&path)
                    .unwrap(),
            ),
        };
        logger.log(
            &Record::builder()
                .level(Level::Warn)
                .args(format_args!("disk {}", "full"))
                .build(),
        );
        // below the level: dropped
        logger.log(
            &Record::builder()
                .level(Level::Trace)
                .args(format_args!("noise"))
                .build(),
        );
        logger.flush();
        let text = fs::read_to_string(&path).unwrap();
        assert_eq!(text.lines().count(), 1);
        assert!(text.trim_end().ends_with(" WARN disk full"), "{text}");
    }

    #[test]
    fn only_linux_follows_the_journal() {
        assert!(journal_follow_command_for(Os::MacOs, "a").is_none());
        assert!(journal_follow_command_for(Os::Linux, "a").is_some());
    }
}
