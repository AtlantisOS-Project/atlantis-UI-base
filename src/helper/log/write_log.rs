// write_log.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! System-wide logging via the syslog service (on Unix; Windows logs to a file, see
//! [`init_logging`](crate::helper::platform::logging::init_logging)).
//!
//! The module implements a logger for the [`log`] crate that forwards the messages to the
//! Unix syslog daemon. That makes centralized error handling and monitoring possible; on
//! systemd systems the messages show up in `journalctl -t <app name>`.
//! [`init_logging`](crate::helper::platform::logging::init_logging) uses this logger next to a
//! log file, and the log viewer (`show_log_viewer`, feature `log-viewer`) shows that file.
//!
//! # Packaging
//!
//! The logger talks to the socket `/dev/log`. Inside a Flatpak sandbox this socket is not
//! reachable; the messages are dropped silently and the application keeps running. Debian,
//! rpm, Snap and AppImage packages can use it without changes.

#[cfg(unix)]
use log::{Level, LevelFilter, Metadata, Record};
#[cfg(unix)]
use std::process;
#[cfg(unix)]
use std::sync::Mutex;
#[cfg(unix)]
use syslog::{Facility, Formatter3164, Logger, LoggerBackend};

/// Forwards `log` records to syslog.
#[cfg(unix)]
pub(crate) struct SyslogLogger {
    formatter: Formatter3164,
    /// The connection is opened with the first message and reused; it is dropped after a
    /// failed write so that the next message reconnects (for example after a restart of
    /// the syslog daemon).
    writer: Mutex<Option<Logger<LoggerBackend, Formatter3164>>>,
}

#[cfg(unix)]
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

        // the part of the program in front of the message: `flash::manager: failed`
        let message = format!(
            "{}: {}",
            crate::helper::platform::logging::short_target(record.target()),
            // one line, and nothing a terminal would act on
            crate::helper::platform::logging::clean_message(&record.args().to_string(), "\\n")
        );
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

/// The logger of syslog for `app_name`; [`init_logging`](crate::helper::platform::logging::init_logging)
/// puts it next to the log file.
#[cfg(unix)]
pub(crate) fn syslog_logger(app_name: &str) -> SyslogLogger {
    SyslogLogger {
        formatter: Formatter3164 {
            facility: Facility::LOG_USER,
            hostname: None,
            process: app_name.to_string(),
            pid: process::id(),
        },
        writer: Mutex::new(None),
    }
}

/// Initializes syslog logging for the current application, **only** syslog. Most programs want
/// [`init_logging`](crate::helper::platform::logging::init_logging), which also writes the log
/// file (the only log of a Flatpak, which cannot reach syslog).
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
#[cfg(unix)]
pub fn init_syslog(app_name: &str) -> Result<(), log::SetLoggerError> {
    log::set_boxed_logger(Box::new(syslog_logger(app_name)))
        .map(|_| log::set_max_level(LevelFilter::Debug))
}

/// On systems without syslog (Windows) the messages go to the log file of the application
/// ([`AppDirs::log_file`](crate::helper::platform::dirs::AppDirs::log_file)); see
/// [`init_logging`](crate::helper::platform::logging::init_logging), which picks the right
/// destination for every system.
#[cfg(not(unix))]
pub fn init_syslog(app_name: &str) -> Result<(), log::SetLoggerError> {
    crate::helper::platform::logging::init_logging(app_name)
}
