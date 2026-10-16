// write_log.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! System-wide logging via the syslog service.
//!
//! The module implements a logger for the [`log`] crate that forwards the messages to the
//! Unix syslog daemon. That makes centralized error handling and monitoring possible; on
//! systemd systems the messages show up in `journalctl -t <app name>` and can be displayed
//! with `show_log_viewer` (feature `log-viewer`).
//!
//! # Packaging
//!
//! The logger talks to the socket `/dev/log`. Inside a Flatpak sandbox this socket is not
//! reachable; the messages are dropped silently and the application keeps running. Debian,
//! rpm, Snap and AppImage packages can use it without changes.

use log::{Level, LevelFilter, Metadata, Record};
use std::process;
use std::sync::Mutex;
use syslog::{Facility, Formatter3164, Logger, LoggerBackend};

/// Forwards `log` records to syslog.
struct SyslogLogger {
    formatter: Formatter3164,
    /// The connection is opened with the first message and reused; it is dropped after a
    /// failed write so that the next message reconnects (for example after a restart of
    /// the syslog daemon).
    writer: Mutex<Option<Logger<LoggerBackend, Formatter3164>>>,
}

impl log::Log for SyslogLogger {
    /// All levels up to and including `Debug` are logged.
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Debug
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let mut guard = self.writer.lock().unwrap_or_else(|p| p.into_inner());
        if guard.is_none() {
            *guard = syslog::unix(self.formatter.clone()).ok();
        }
        let Some(writer) = guard.as_mut() else {
            return;
        };

        let message = record.args().to_string();
        let sent = match record.level() {
            Level::Error => writer.err(message),
            Level::Warn => writer.warning(message),
            Level::Info => writer.info(message),
            Level::Debug | Level::Trace => writer.debug(message),
        };
        if sent.is_err() {
            *guard = None;
        }
    }

    fn flush(&self) {}
}

/// Initializes syslog logging for the current application.
///
/// Afterwards the standard macros `info!()`, `warn!()`, `error!()` and so on log to the
/// system. The log level is `Debug`.
///
/// # Arguments
///
/// * `app_name` - The identifier under which the messages appear, for example in
///   `/var/log/syslog` or with `journalctl -t app_name`.
///
/// # Errors
///
/// Returns a [`log::SetLoggerError`] if a logger has already been registered for this
/// process.
///
/// # Examples
///
/// ```no_run
/// use atlbase::prelude::init_syslog;
///
/// init_syslog("atlantis-ui").expect("unable to initialize syslog");
/// log::info!("UI successfully started");
/// log::error!("Critical error found!");
/// ```
pub fn init_syslog(app_name: &str) -> Result<(), log::SetLoggerError> {
    let logger = SyslogLogger {
        formatter: Formatter3164 {
            facility: Facility::LOG_USER,
            hostname: None,
            process: app_name.to_string(),
            pid: process::id(),
        },
        writer: Mutex::new(None),
    };

    log::set_boxed_logger(Box::new(logger)).map(|_| log::set_max_level(LevelFilter::Debug))
}
