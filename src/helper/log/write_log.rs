//! System-wide logging via the syslog service.
//!
//! This module implements a logger for the `log` crate that forwards messages 
//! directly to the Unix syslog daemon. This enables centralized 
//! error management and monitoring within AtlantisOS.
/**
* write_log.rs
*
* (C) Copyright 2026 AtlantisOS Project
* by @NachtsternBuild
*
* License: GNU GENERAL PUBLIC LICENSE Version 3
*/

use syslog::{Facility, Formatter3164};
use log::{LevelFilter, Metadata, Record};
use std::process;

/// Internal structure for implementing the `log::Log` trait.
/// 
/// Stores the application name to uniquely identify log entries.
struct SyslogLogger {
    app_name: String,
}

impl log::Log for SyslogLogger {
	/// Determines which log levels are processed.
    /// In AtlantisOS, all levels up to and including `Debug` are logged.
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= log::Level::Debug
    }
	
	/// Sends a log message to the syslog daemon.
    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            // create a code formatter
            let formatter = Formatter3164 {
    			facility: Facility::LOG_USER,
    			hostname: None,
    			process: self.app_name.clone(),
    			pid: process::id() as u32, // save process id as u32
			};
			
            if let Ok(mut writer) = syslog::unix(formatter) {
                let message = format!("{}", record.args());
                
                let _ = match record.level() {
                    log::Level::Error => writer.err(message),
                    log::Level::Warn  => writer.warning(message),
                    log::Level::Info  => writer.info(message),
                    log::Level::Debug => writer.debug(message),
                    log::Level::Trace => writer.debug(message),
                };
            }
        }
    }

    fn flush(&self) {}
}

/// Initializes syslog logging for the current application.
///
/// After calling this function, standard macros such as `info!()`, 
/// `warn!()`, or `error!()` can be used to log messages system-wide.
///
/// # Arguments
/// * `app_name` - The name under which the logs should appear in the system (e.g., in `/var/log/syslog`).
///
/// # Error Handling
/// Returns a `SetLoggerError` if a logger has already been registered for this process.
///
/// # Usage:
///
/// ```rust
/// fn main() {
///    init_syslog("atlantis-ui").expect("Unable to initialize Syslog");
///    log::info!("UI successful started");
///    log::error!("Critical Error found!");
/// }
/// ``` 
pub fn init_syslog(app_name: &str) -> Result<(), log::SetLoggerError> {
    let logger = SyslogLogger {
        app_name: app_name.to_string(),
    };

    log::set_boxed_logger(Box::new(logger))
        .map(|_| log::set_max_level(LevelFilter::Debug))
}
