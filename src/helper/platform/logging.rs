// logging.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! Where log messages go.
//!
//! The standard macros of the [`log`] crate (`info!`, `warn!`, …) need a logger.
//! [`init_logging`] installs the one that belongs to the system, and **always** a log file:
//!
//! | Where | What |
//! |---|---|
//! | the log file [`AppDirs::log_file`](super::dirs::AppDirs::log_file) | every system and every package format, also a Flatpak, which cannot reach the system log. One line per message: `2026-10-27T11:25:22Z INFO  flash::manager: …`. The file is rotated at 1 MB: the old one stays as `<name>.log.old`. |
//! | the system log (syslog; `journalctl -t <app>`) | Linux and macOS, where it can be reached |
//!
//! The log viewer ([`show_log_viewer`](crate::helper::log::write_log_text::show_log_viewer))
//! shows the file, so it works wherever the program can write its log.
//! [`journal_follow_command`] gives the command that shows the system log as it comes, for
//! systems that have one.

use super::dirs::AppDirs;
use super::os::{os_profile, os_profile_for};
use super::target::Os;
use log::{Level, LevelFilter, Log, Metadata, Record, SetLoggerError};
use std::fmt::Write as _;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

/// The size at which the log file is moved to `<name>.log.old` and a new one is started.
pub const MAX_LOG_BYTES: u64 = 1_000_000;

/// The name of the part of the program that wrote a message, short: the last two parts of the
/// module path (`my_app::helper::flash::manager` becomes `flash::manager`).
///
/// # Examples
///
/// ```
/// use atlbase::prelude::short_target;
///
/// assert_eq!(short_target("my_app::helper::flash::manager"), "flash::manager");
/// assert_eq!(short_target("my_app::main"), "my_app::main");
/// assert_eq!(short_target("main"), "main");
/// ```
pub fn short_target(target: &str) -> &str {
    let mut cut = target.len();
    for _ in 0..2 {
        match target[..cut].rfind("::") {
            Some(at) => cut = at,
            None => return target,
        }
    }
    &target[cut + 2..]
}

/// Makes a message safe for a log. What a program writes into its log can come from outside (the
/// output of a failed command, the name of a device or a file), and the log is shown in a
/// terminal: a control character such as `ESC` could move the cursor, clear lines or set the
/// title there, and a `CR` or a direction mark could hide or turn around what was logged. These
/// characters are written as `\x1b`, `\x9b`, `\u{202e}`, … instead. A line break becomes
/// `newline`; a tab stays.
pub(crate) fn clean_message(text: &str, newline: &str) -> String {
    let mut clean = String::with_capacity(text.len());
    for c in text.chars() {
        match c {
            '\n' => clean.push_str(newline),
            '\t' => clean.push('\t'),
            // C0 and C1 control characters (the 8 bit `CSI` is U+009B)
            c if c.is_control() => {
                let _ = write!(clean, "\\x{:02x}", c as u32);
            }
            // direction marks and overrides
            '\u{061c}'
            | '\u{200e}'
            | '\u{200f}'
            | '\u{202a}'..='\u{202e}'
            | '\u{2066}'..='\u{2069}' => {
                let _ = write!(clean, "\\u{{{:04x}}}", c as u32);
            }
            c => clean.push(c),
        }
    }
    clean
}

/// The beginning of a long text for the log: at most 5 lines and 500 bytes, ended by `…` if
/// something was cut. The output of a command or the text of a dialog can be long, and can hold
/// things that do not belong in a log that is sent around in bug reports.
pub(crate) fn excerpt(text: &str) -> String {
    let lines: Vec<&str> = text.trim().lines().take(5).collect();
    let mut shown = lines.join("\n");
    let cut = shown.len() > 500 || text.trim().lines().nth(5).is_some();
    if shown.len() > 500 {
        shown.truncate(shown.floor_char_boundary(500));
    }
    if cut {
        shown.push('…');
    }
    shown
}

/// The time as `2026-10-27T11:25:22Z` (UTC), from the seconds since 1970. Written by hand: a log
/// needs no calendar library.
///
/// # Examples
///
/// ```
/// use atlbase::prelude::utc_timestamp;
///
/// assert_eq!(utc_timestamp(0), "1970-01-01T00:00:00Z");
/// assert_eq!(utc_timestamp(1_792_000_000), "2026-10-14T17:46:40Z");
/// ```
pub fn utc_timestamp(seconds: u64) -> String {
    let (days, rest) = (seconds / 86_400, seconds % 86_400);
    // the date from the days since 1970 (the algorithm of Howard Hinnant)
    let z = days as i64 + 719_468;
    let era = z.div_euclid(146_097);
    let day_of_era = z.rem_euclid(146_097);
    let year_of_era =
        (day_of_era - day_of_era / 1_460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let month_index = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * month_index + 2) / 5 + 1;
    let month = if month_index < 10 {
        month_index + 3
    } else {
        month_index - 9
    };
    let year = year_of_era + era * 400 + i64::from(month <= 2);
    format!(
        "{year:04}-{month:02}-{day:02}T{:02}:{:02}:{:02}Z",
        rest / 3_600,
        rest % 3_600 / 60,
        rest % 60
    )
}

/// One entry of the log file. A message of several lines continues with an indent, so every entry
/// starts with its time.
fn file_line(seconds: u64, record: &Record) -> String {
    let message = clean_message(&record.args().to_string(), "\n    ");
    format!(
        "{} {:<5} {}: {message}\n",
        utc_timestamp(seconds),
        record.level(),
        short_target(record.target())
    )
}

/// Creates the folders above a log file. A new folder can only be entered by the user (`0700`):
/// the log holds command lines and messages that are nobody else's business.
fn create_private_dir(dir: &Path) -> io::Result<()> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::DirBuilderExt;
        fs::DirBuilder::new()
            .recursive(true)
            .mode(0o700)
            .create(dir)
    }
    #[cfg(not(unix))]
    fs::create_dir_all(dir)
}

/// Takes the rights of group and others from a log file that was created before they were
/// restricted.
fn make_private(file: &File) {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if let Ok(meta) = file.metadata()
            && meta.permissions().mode() & 0o077 != 0
        {
            let _ = file.set_permissions(fs::Permissions::from_mode(0o600));
        }
    }
    #[cfg(not(unix))]
    let _ = file;
}

struct FileState {
    file: File,
    path: PathBuf,
    size: u64,
}

/// A logger that appends the messages to a file (see the module documentation).
struct FileLogger {
    state: Mutex<FileState>,
}

impl FileLogger {
    /// Opens the file (the folders above it are created). A file of [`MAX_LOG_BYTES`] or more is
    /// moved to `<name>.old` first.
    fn open(path: &Path) -> io::Result<FileLogger> {
        if let Some(parent) = path.parent() {
            create_private_dir(parent)?;
        }
        let (file, size) = Self::open_file(path)?;
        Ok(FileLogger {
            state: Mutex::new(FileState {
                file,
                path: path.to_path_buf(),
                size,
            }),
        })
    }

    fn open_file(path: &Path) -> io::Result<(File, u64)> {
        if fs::metadata(path).is_ok_and(|meta| meta.len() >= MAX_LOG_BYTES) {
            let mut old = path.as_os_str().to_owned();
            old.push(".old");
            // an older backup is replaced; a log that cannot be moved is appended to
            let _ = fs::rename(path, old);
        }
        let mut options = OpenOptions::new();
        options.create(true).append(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.mode(0o600);
        }
        let file = options.open(path)?;
        make_private(&file);
        let size = file.metadata().map_or(0, |meta| meta.len());
        Ok((file, size))
    }
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
        let line = file_line(seconds, record);
        let mut state = self.state.lock().unwrap_or_else(|p| p.into_inner());
        if state.size + line.len() as u64 > MAX_LOG_BYTES
            && let Ok((file, size)) = Self::open_file(&state.path.clone())
        {
            state.file = file;
            state.size = size;
        }
        // a log line that cannot be written is lost; the program goes on
        if state.file.write_all(line.as_bytes()).is_ok() {
            state.size += line.len() as u64;
        }
    }

    fn flush(&self) {
        let _ = self
            .state
            .lock()
            .unwrap_or_else(|p| p.into_inner())
            .file
            .flush();
    }
}

/// Passes every message to all of its loggers (the system log and the file).
struct Tee {
    loggers: Vec<Box<dyn Log>>,
}

impl Log for Tee {
    fn enabled(&self, metadata: &Metadata) -> bool {
        self.loggers.iter().any(|logger| logger.enabled(metadata))
    }

    fn log(&self, record: &Record) {
        for logger in &self.loggers {
            logger.log(record);
        }
    }

    fn flush(&self) {
        for logger in &self.loggers {
            logger.flush();
        }
    }
}

/// The log file of an application: `AppDirs::new(app_name).log_file()`, if the system has a folder
/// for it. [`init_logging`] writes it, and the log viewer shows it.
///
/// # Examples
///
/// ```
/// use atlbase::prelude::log_file_of;
///
/// if let Some(path) = log_file_of("my-app") {
///     assert!(path.ends_with("my-app.log"));
/// }
/// ```
pub fn log_file_of(app_name: &str) -> Option<PathBuf> {
    AppDirs::new(app_name).map(|dirs| dirs.log_file())
}

/// Logs to the file `path` only (the folders above it are created, a file of 1 MB or more is moved
/// to `<path>.old`). The level is `Debug`. To log to the system log as well use [`init_logging`].
///
/// # Errors
///
/// Returns a [`log::SetLoggerError`] if a logger has already been registered, and prints a
/// message to `stderr` and returns `Ok` if the file cannot be opened (a program must not stop
/// because it cannot log).
pub fn init_file_logging(path: &Path) -> Result<(), SetLoggerError> {
    let logger = match FileLogger::open(path) {
        Ok(logger) => logger,
        Err(error) => {
            eprintln!(
                "[ERROR] Cannot open the log file {}: {error}",
                path.display()
            );
            return Ok(());
        }
    };
    log::set_boxed_logger(Box::new(logger)).map(|()| log::set_max_level(LevelFilter::Debug))
}

/// Installs the loggers for the application `app_name`: the log file
/// ([`log_file_of`], on every system) and, where the system has one and the program can reach it,
/// the system log (syslog on Linux and macOS). Afterwards `info!()`, `warn!()`, `error!()` and
/// so on work. The level is `Debug`.
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
    let mut loggers: Vec<Box<dyn Log>> = Vec::new();
    #[cfg(unix)]
    if os_profile().syslog {
        loggers.push(Box::new(crate::helper::log::write_log::syslog_logger(
            app_name,
        )));
    }
    if let Some(path) = log_file_of(app_name) {
        match FileLogger::open(&path) {
            Ok(logger) => loggers.push(Box::new(logger)),
            Err(error) => eprintln!(
                "[ERROR] Cannot open the log file {}: {error}",
                path.display()
            ),
        }
    }
    if loggers.is_empty() {
        return Ok(());
    }
    log::set_boxed_logger(Box::new(Tee { loggers }))
        .map(|()| log::set_max_level(LevelFilter::Debug))
}

/// The command that prints the messages of `app_name` in the system log as they come, or `None`
/// where the system has no log to follow (only Linux with `journalctl` has one). It starts with
/// the latest 200 messages.
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

    fn record<'a>(level: Level, args: std::fmt::Arguments<'a>) -> Record<'a> {
        Record::builder()
            .level(level)
            .target("my_app::helper::flash::manager")
            .args(args)
            .build()
    }

    #[test]
    fn the_file_logger_writes_one_entry_per_message() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("deep/app.log");
        let logger = FileLogger::open(&path).unwrap();
        logger.log(&record(Level::Warn, format_args!("disk {}", "full")));
        // below the level: dropped
        logger.log(&record(Level::Trace, format_args!("noise")));
        // a message of several lines is one entry
        logger.log(&record(Level::Error, format_args!("failed\nreason: x")));
        logger.flush();

        let text = fs::read_to_string(&path).unwrap();
        let lines: Vec<_> = text.lines().collect();
        assert_eq!(lines.len(), 3, "{text}");
        // 2026-10-27T11:25:22Z WARN  flash::manager: disk full
        assert!(
            lines[0].ends_with("WARN  flash::manager: disk full"),
            "{text}"
        );
        assert_eq!(lines[0].as_bytes()[10], b'T');
        assert!(lines[1].ends_with("ERROR flash::manager: failed"), "{text}");
        assert_eq!(lines[2], "    reason: x");
    }

    #[test]
    fn a_big_log_file_is_rotated() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("app.log");
        fs::write(&path, vec![b'x'; MAX_LOG_BYTES as usize]).unwrap();

        // at the start of the program
        let logger = FileLogger::open(&path).unwrap();
        logger.log(&record(Level::Info, format_args!("fresh")));
        logger.flush();
        let old = dir.path().join("app.log.old");
        assert_eq!(fs::metadata(&old).unwrap().len(), MAX_LOG_BYTES);
        assert!(fs::read_to_string(&path).unwrap().contains("fresh"));

        // and while it runs: the file never grows much beyond the limit
        let long = "y".repeat(1_000);
        for _ in 0..1_500 {
            logger.log(&record(Level::Info, format_args!("{long}")));
        }
        logger.flush();
        assert!(fs::metadata(&path).unwrap().len() <= MAX_LOG_BYTES);
        assert!(fs::metadata(&old).is_ok());
    }

    #[test]
    fn nothing_a_terminal_would_act_on_reaches_the_log() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("app.log");
        let logger = FileLogger::open(&path).unwrap();
        // what a failed command, or a device that is plugged in, could print
        logger.log(&record(
            Level::Error,
            format_args!("a\u{1b}[2K\u{1b}[1Ab\u{9b}31mc\rd\0e\u{202e}f\tg\nh"),
        ));
        logger.flush();

        let text = fs::read_to_string(&path).unwrap();
        assert!(
            text.chars()
                .all(|c| c == '\n' || c == '\t' || !c.is_control()),
            "{text:?}"
        );
        assert!(!text.contains('\u{202e}'), "{text:?}");
        assert!(
            text.contains("a\\x1b[2K\\x1b[1Ab\\x9b31mc\\x0dd\\x00e\\u{202e}f\tg\n    h"),
            "{text:?}"
        );
    }

    #[test]
    fn messages_are_cleaned_and_excerpts_are_short() {
        assert_eq!(
            clean_message("plain text, äöü €", " | "),
            "plain text, äöü €"
        );
        assert_eq!(clean_message("a\nb", " | "), "a | b");
        assert_eq!(clean_message("\u{7f}", "\n"), "\\x7f");

        assert_eq!(excerpt("  one\ntwo  \n"), "one\ntwo");
        assert_eq!(excerpt("1\n2\n3\n4\n5\n6"), "1\n2\n3\n4\n5…");
        let long = "ä".repeat(400);
        let short = excerpt(&long);
        assert!(short.ends_with('…') && short.len() <= 500 + '…'.len_utf8());
        assert!(long.starts_with(short.trim_end_matches('…')));
    }

    #[cfg(unix)]
    #[test]
    fn the_log_is_private() {
        use std::os::unix::fs::PermissionsExt;
        let mode = |path: &Path| fs::metadata(path).unwrap().permissions().mode() & 0o777;

        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("new/folder/app.log");
        FileLogger::open(&path).unwrap();
        assert_eq!(mode(&path), 0o600);
        assert_eq!(mode(path.parent().unwrap()), 0o700);
        assert_eq!(mode(path.parent().unwrap().parent().unwrap()), 0o700);

        // a log from before the rights were restricted is closed, too
        let old = dir.path().join("old.log");
        fs::write(&old, "x\n").unwrap();
        fs::set_permissions(&old, fs::Permissions::from_mode(0o664)).unwrap();
        FileLogger::open(&old).unwrap();
        assert_eq!(mode(&old), 0o600);
    }

    #[test]
    fn a_tee_reaches_every_logger() {
        let dir = tempfile::tempdir().unwrap();
        let (a, b) = (dir.path().join("a.log"), dir.path().join("b.log"));
        let tee = Tee {
            loggers: vec![
                Box::new(FileLogger::open(&a).unwrap()),
                Box::new(FileLogger::open(&b).unwrap()),
            ],
        };
        tee.log(&record(Level::Info, format_args!("both")));
        tee.flush();
        for path in [a, b] {
            assert!(fs::read_to_string(path).unwrap().contains("both"));
        }
    }

    #[test]
    fn times_and_names_are_short_and_right() {
        // a leap day, the end of a year, the year 2038 and the day the counter ends
        assert_eq!(utc_timestamp(951_782_400), "2000-02-29T00:00:00Z");
        assert_eq!(utc_timestamp(1_704_067_199), "2023-12-31T23:59:59Z");
        assert_eq!(utc_timestamp(2_147_483_648), "2038-01-19T03:14:08Z");
        assert_eq!(short_target("a::b::c"), "b::c");
        assert_eq!(short_target("a::b"), "a::b");
    }

    #[test]
    fn only_linux_follows_the_journal() {
        assert!(journal_follow_command_for(Os::MacOs, "a").is_none());
        assert!(journal_follow_command_for(Os::Linux, "a").is_some());
    }
}
