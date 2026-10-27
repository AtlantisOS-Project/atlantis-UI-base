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
use std::io::{self, Read};
use std::process::{Command, ExitStatus, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
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
/// The quoting is the one of the POSIX shells (`sh`, `bash`, `zsh`), so it is for a terminal on
/// Linux and macOS. It does **not** protect on Windows: `cmd.exe` knows no single quotes (and so
/// not on WSL, where the text goes through `cmd.exe` too). Use
/// [`open_terminal_command`](crate::helper::commands::open_terminal_desktop::open_terminal_command),
/// which takes the right measures for every system. Commands that are started directly need no
/// quoting.
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
/// of the returned text. If the program ends but a program that it started keeps its output open
/// (a server that goes to the background), the function waits a quarter of a second for the
/// rest and then returns what it has.
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
    let captured = run_captured(host_command_line(command), Some(limit)).ok()?;
    captured.status?;
    Some(captured.stdout + &captured.stderr)
}

/// The most output of one stream that [`run_captured`] keeps: 8 MiB. The rest is read and thrown
/// away, so the program never stops on a full pipe.
pub(crate) const MAX_CAPTURE: usize = 8 * 1024 * 1024;

/// How long [`run_captured`] waits for output after the program has ended.
const GRACE: Duration = Duration::from_millis(250);

/// What a program printed and how it ended.
pub(crate) struct Captured {
    pub(crate) stdout: String,
    pub(crate) stderr: String,
    /// `None` if the program was stopped because it took too long.
    pub(crate) status: Option<ExitStatus>,
    /// Whether one of the streams was longer than [`MAX_CAPTURE`].
    pub(crate) truncated: bool,
}

/// A pipe that is read to the end in a thread.
struct Drain {
    data: Arc<Mutex<Vec<u8>>>,
    truncated: Arc<AtomicBool>,
    thread: JoinHandle<()>,
}

impl Drain {
    fn start(mut pipe: impl Read + Send + 'static) -> Drain {
        let data = Arc::new(Mutex::new(Vec::new()));
        let truncated = Arc::new(AtomicBool::new(false));
        let thread = {
            let (data, truncated) = (data.clone(), truncated.clone());
            thread::spawn(move || {
                let mut chunk = [0u8; 8192];
                while let Ok(read @ 1..) = pipe.read(&mut chunk) {
                    let mut data = data.lock().unwrap_or_else(|p| p.into_inner());
                    let room = MAX_CAPTURE.saturating_sub(data.len());
                    data.extend_from_slice(&chunk[..read.min(room)]);
                    if read > room {
                        truncated.store(true, Ordering::Relaxed);
                    }
                }
            })
        };
        Drain {
            data,
            truncated,
            thread,
        }
    }

    fn text(&self) -> String {
        let data = self.data.lock().unwrap_or_else(|p| p.into_inner());
        String::from_utf8_lossy(&data).into_owned()
    }
}

/// Runs a program with the input closed and collects its output, at most [`MAX_CAPTURE`] bytes of
/// each stream. After `timeout` (if there is one) the program is killed and `status` is `None`.
///
/// A program that cannot be killed (one that runs with the rights of `root` through `pkexec`)
/// is left alone, and the function returns anyway. A program that was started by the program
/// and keeps the output open is not waited for longer than [`GRACE`].
pub(crate) fn run_captured(
    mut command: Command,
    timeout: Option<Duration>,
) -> io::Result<Captured> {
    let mut child = command
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    let no_pipe = || io::Error::other("the pipe of the program is missing");
    let out = Drain::start(child.stdout.take().ok_or_else(no_pipe)?);
    let err = Drain::start(child.stderr.take().ok_or_else(no_pipe)?);

    let start = Instant::now();
    let status = loop {
        if let Some(status) = child.try_wait()? {
            break Some(status);
        }
        if timeout.is_some_and(|limit| start.elapsed() >= limit) {
            let _ = child.kill();
            let _ = child.try_wait();
            break None;
        }
        // quick to notice a short program, light on the processor for a long one
        let pause = if start.elapsed() < Duration::from_secs(2) {
            10
        } else {
            100
        };
        thread::sleep(Duration::from_millis(pause));
    };

    let until = Instant::now() + GRACE;
    while !(out.thread.is_finished() && err.thread.is_finished()) && Instant::now() < until {
        thread::sleep(Duration::from_millis(5));
    }
    Ok(Captured {
        stdout: out.text(),
        stderr: err.text(),
        status,
        truncated: out.truncated.load(Ordering::Relaxed) || err.truncated.load(Ordering::Relaxed),
    })
}

/// Words that name a secret: the word after one of them (`--password hunter2`, `psk hunter2`) or
/// the value in `name=value` is not written to the log.
const SECRET_NAMES: [&str; 9] = [
    "password",
    "passwd",
    "passphrase",
    "psk",
    "secret",
    "token",
    "apikey",
    "api-key",
    "credential",
];

fn names_a_secret(word: &str) -> bool {
    let word = word.to_ascii_lowercase();
    SECRET_NAMES.iter().any(|name| word.contains(name))
}

/// The command line as one string for the log, with the values of secrets replaced by `***`.
/// Best effort: it knows the usual names (`password`, `psk`, `token`, …), not every program.
///
/// # Examples
///
/// ```text
/// nmcli dev wifi connect Home password hunter2  →  nmcli dev wifi connect Home password ***
/// curl --token=abc https://example.org          →  curl --token=*** https://example.org
/// ```
pub(crate) fn redacted(command: &[impl AsRef<str>]) -> String {
    let mut hide_next = false;
    command
        .iter()
        .map(|word| {
            let word = word.as_ref();
            if std::mem::take(&mut hide_next) {
                return "***".to_string();
            }
            if let Some((name, _)) = word.split_once('=')
                && names_a_secret(name)
            {
                return format!("{name}=***");
            }
            hide_next = names_a_secret(word);
            word.to_string()
        })
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn words(line: &[&str]) -> Vec<String> {
        line.iter().map(|word| word.to_string()).collect()
    }

    #[test]
    fn secrets_do_not_reach_the_log() {
        assert_eq!(
            redacted(&words(&[
                "nmcli", "dev", "wifi", "connect", "Home", "password", "hunter2", "ifname", "wlan0"
            ])),
            "nmcli dev wifi connect Home password *** ifname wlan0"
        );
        assert_eq!(
            redacted(&words(&["curl", "--token=abc", "https://example.org"])),
            "curl --token=*** https://example.org"
        );
        assert_eq!(
            redacted(&words(&["tool", "--API-Key", "k", "x"])),
            "tool --API-Key *** x"
        );
        assert_eq!(
            redacted(&words(&["fastboot", "flash", "boot", "boot.img"])),
            "fastboot flash boot boot.img"
        );
        let none: [&str; 0] = [];
        assert_eq!(redacted(&none), "");
    }

    #[cfg(unix)]
    #[test]
    fn a_program_is_stopped_when_it_takes_too_long() {
        let started = Instant::now();
        let mut sleep = Command::new("sleep");
        sleep.arg("5");
        let captured = run_captured(sleep, Some(Duration::from_millis(100))).unwrap();
        assert!(captured.status.is_none());
        assert!(started.elapsed() < Duration::from_secs(2));
    }

    #[cfg(unix)]
    #[test]
    fn the_limit_holds_when_the_program_has_children_that_keep_the_pipe() {
        // `sh` is killed, but its `sleep` keeps the pipe open for 4 seconds
        let slow = words(&["sh", "-c", "sleep 4; echo late"]);
        let started = Instant::now();
        assert_eq!(
            command_output_timeout(&slow, Duration::from_millis(200)),
            None
        );
        assert!(
            started.elapsed() < Duration::from_secs(2),
            "{:?}",
            started.elapsed()
        );
    }

    #[cfg(unix)]
    #[test]
    fn output_of_a_program_that_left_a_child_behind_is_returned() {
        // `sh` ends at once, its `sleep` holds the pipe: what was printed is not lost
        let background = words(&["sh", "-c", "sleep 4 & echo hi"]);
        let started = Instant::now();
        let text = command_output_timeout(&background, Duration::from_secs(3));
        assert_eq!(text.as_deref(), Some("hi\n"));
        assert!(started.elapsed() < Duration::from_secs(2));
    }

    #[cfg(unix)]
    #[test]
    fn too_much_output_is_cut_and_the_program_is_not_blocked() {
        // 20 MB on the output: more than the pipe holds and more than is kept
        let mut command = Command::new("sh");
        command.args([
            "-c",
            "head -c 20000000 /dev/zero | tr '\\0' x; echo done >&2",
        ]);
        let captured = run_captured(command, Some(Duration::from_secs(30))).unwrap();
        assert!(captured.status.is_some_and(|status| status.success()));
        assert_eq!(captured.stdout.len(), MAX_CAPTURE);
        assert_eq!(captured.stderr, "done\n");
        assert!(captured.truncated);
    }
}
