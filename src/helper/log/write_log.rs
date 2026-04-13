/**
* write_log.rs
*
* (C) Copyright 2026 AtlantisOS Project
* by @NachtsternBuild
*
* License: GNU GENERAL PUBLIC LICENSE Version 3
*
* Usage:
* 
* fn main() {
*	 // init with app_name (LOCAL_DOMAIN)
*	 // const LOCAL_DOMAIN: &str = "atlantis-ui";
*    init_syslog("atlantis-ui").expect("Unable to initialize Syslog");
*    log::info!("UI successful started");
*    log::error!("Critical Error found!");
* }
*/

use syslog::{Facility, Formatter3164};
use log::{LevelFilter, Metadata, Record};
use std::process;

/**
* @brief implement to use syslog for logging
*/
struct SyslogLogger {
    app_name: String,
}

impl log::Log for SyslogLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= log::Level::Debug
    }

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

/**
* @brief Function that init syslog logging
*/
pub fn init_syslog(app_name: &str) -> Result<(), log::SetLoggerError> {
    let logger = SyslogLogger {
        app_name: app_name.to_string(),
    };

    log::set_boxed_logger(Box::new(logger))
        .map(|_| log::set_max_level(LevelFilter::Debug))
}
