// command_line.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! Command lines as lists of strings.
//!
//! The progress dialogs of the library ([`show_spinner_dialog`] and
//! [`CommandTask`]) run commands that are given as a list of strings: the program followed by
//! its arguments, started without a shell. This module builds such lists
//! ([`command_line`]), quotes them for a shell if a terminal has to run them
//! ([`shell_line`]) and asks a program a question that must be answered in time
//! ([`command_output_timeout`]).
//!
//! [`show_spinner_dialog`]: crate::design::dialogs::dialogs_spinner::show_spinner_dialog
//! [`CommandTask`]: crate::design::dialogs::command_task::CommandTask

use crate::helper::platform::program::host_command_line;
use std::io::Read;
use std::process::Stdio;
use std::thread;
use std::time::{Duration, Instant};

/// Builds a command line: the program followed by its arguments.
///
/// The arguments can be `&str`, `String` or anything else that is `AsRef<str>`.
///
/// # Examples
///
/// ```
/// use atlbase::prelude::command_line;
///
/// assert_eq!(command_line("ls", ["-l", "/tmp"]), ["ls", "-l", "/tmp"]);
///
/// let file = String::from("boot.img");
/// assert_eq!(command_line("fastboot", ["flash", "boot", &file]).len(), 4);
/// ```
pub fn command_line<I, S>(program: &str, args: I) -> Vec<String>
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    std::iter::once(program.to_string())
        .chain(args.into_iter().map(|arg| arg.as_ref().to_string()))
        .collect()
}

/// Puts a command line into one string for a shell: every word is put in single quotes, so
/// spaces and special characters stay what they are.
///
/// Use it for a terminal (`open_terminal_by_desktop`), which hands the text to a shell.
/// Commands that are started directly need no quoting.
///
/// # Examples
///
/// ```
/// use atlbase::prelude::shell_line;
///
/// let line = shell_line(&["echo".to_string(), "it's a test".to_string()]);
/// assert_eq!(line, r"'echo' 'it'\''s a test'");
/// ```
pub fn shell_line(command: &[String]) -> String {
    command
        .iter()
        .map(|word| format!("'{}'", word.replace('\'', "'\\''")))
        .collect::<Vec<_>>()
        .join(" ")
}

/// Reads a pipe to the end in a thread, so a full pipe never stops the program.
fn drain(mut pipe: impl Read + Send + 'static) -> thread::JoinHandle<String> {
    thread::spawn(move || {
        let mut bytes = Vec::new();
        let _ = pipe.read_to_end(&mut bytes);
        String::from_utf8_lossy(&bytes).into_owned()
    })
}

/// Runs a command and returns what it printed (standard output, then standard error), but
/// waits for `limit` at most.
///
/// Use it for questions to a program that may not answer (a device with a locked screen, a
/// tool that waits for a hung server). Call it in a background thread: it blocks until the
/// program ends or the time is up.
///
/// # Returns
///
/// `None` if the program cannot be started, or does not finish within `limit`; it is killed
/// in the second case. The exit status is not checked: an error message of the program is part
/// of the returned text.
///
/// # Examples
///
/// ```
/// use atlbase::prelude::{command_line, command_output_timeout};
/// use std::time::Duration;
///
/// let text = command_output_timeout(&command_line("echo", ["hello"]), Duration::from_secs(5));
/// assert_eq!(text.as_deref(), Some("hello\n"));
///
/// let slow = command_line("sleep", ["5"]);
/// assert_eq!(command_output_timeout(&slow, Duration::from_millis(100)), None);
/// ```
pub fn command_output_timeout(command: &[String], limit: Duration) -> Option<String> {
    command.first()?;
    let mut child = host_command_line(command)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .ok()?;

    let out = drain(child.stdout.take()?);
    let err = drain(child.stderr.take()?);

    let start = Instant::now();
    let finished = loop {
        match child.try_wait() {
            Ok(Some(_)) => break true,
            Ok(None) if start.elapsed() < limit => thread::sleep(Duration::from_millis(20)),
            _ => break false,
        }
    };
    if !finished {
        let _ = child.kill();
        let _ = child.wait();
    }
    let (out, err) = (
        out.join().unwrap_or_default(),
        err.join().unwrap_or_default(),
    );
    finished.then(|| out + &err)
}
