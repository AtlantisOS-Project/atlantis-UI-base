// download.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! Downloading a file over HTTP or HTTPS, without `curl` or `wget`.
//!
//! The program does the download itself (a small HTTP client with its own root certificates), so a
//! Flatpak, a Snap or an AppImage needs no tool of the system for it, and the result is the same on
//! every system. Needs the feature `download`.
//!
//! # Examples
//!
//! ```no_run
//! use atlbase::prelude::download_file;
//! use std::path::Path;
//!
//! let bytes = download_file(
//!     "https://example.org/tool.zip",
//!     Path::new("/tmp/tool.zip"),
//!     |done, total| println!("{done} of {total:?} bytes"),
//! )
//! .expect("the download failed");
//! println!("{bytes} bytes");
//! ```

use sha2::{Digest, Sha256};
use std::fs::{self, OpenOptions};
use std::io::{self, Read, Write};
use std::net::IpAddr;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{self, RecvTimeoutError, Sender};
use std::thread;
use std::time::Duration;

/// How long the server may take to connect and to answer.
const PHASE_TIMEOUT: Duration = Duration::from_secs(30);

/// How long a download may stay silent: the time without a single new byte. A big file on a slow
/// line is no problem as long as it keeps coming.
const IDLE_TIMEOUT: Duration = Duration::from_secs(30);

/// The most data a download may bring: 16 GiB.
const MAX_DOWNLOAD_BYTES: u64 = 16 * 1024 * 1024 * 1024;

/// What the worker thread tells the caller.
enum Event {
    Progress(u64, Option<u64>),
    Done(io::Result<u64>),
}

/// Downloads `url` into the file `destination` and returns the number of bytes.
///
/// * Only `https://` addresses are downloaded. Plain `http://` is refused, except for the own
///   computer (`localhost`, `127.0.0.1`, `[::1]`), which is handy for tests. An https address stays
///   https: a redirect to plain http is refused. If the program is to run what it downloads, use
///   [`download_file_verified`], which checks a SHA-256 sum.
/// * The file is written under a temporary name next to `destination` and renamed when the
///   download is complete, so a failed download never leaves a half file under the real name. The
///   folders above `destination` are created. At most 16 GiB are accepted.
/// * The download fails when the server sends nothing for 30 seconds; how long the whole
///   download takes does not matter.
/// * `progress` is called after every block with the bytes so far and the size, if the server
///   tells it.
///
/// # Errors
///
/// Returns an [`io::Error`] if the address is not allowed (`InvalidInput`), the connection fails,
/// the server answers with an error status (404, …), the server goes silent (`TimedOut`), the file is
/// too big (`FileTooLarge`), or the file cannot be written.
pub fn download_file(
    url: &str,
    destination: &Path,
    progress: impl FnMut(u64, Option<u64>),
) -> io::Result<u64> {
    download(
        url,
        destination,
        None,
        MAX_DOWNLOAD_BYTES,
        IDLE_TIMEOUT,
        progress,
    )
}

/// Like [`download_file`], but the file must have the SHA-256 sum `sha256` (64 hexadecimal
/// characters, in upper or lower case), as the publisher of the file states it. A file with
/// another sum is deleted and the download fails with `InvalidData`.
///
/// # Examples
///
/// ```no_run
/// use atlbase::prelude::download_file_verified;
/// use std::path::Path;
///
/// download_file_verified(
///     "https://example.org/tool.zip",
///     Path::new("/tmp/tool.zip"),
///     "9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08",
///     |_, _| {},
/// )
/// .expect("the download failed or the file is not the one that was expected");
/// ```
pub fn download_file_verified(
    url: &str,
    destination: &Path,
    sha256: &str,
    progress: impl FnMut(u64, Option<u64>),
) -> io::Result<u64> {
    download(
        url,
        destination,
        Some(sha256),
        MAX_DOWNLOAD_BYTES,
        IDLE_TIMEOUT,
        progress,
    )
}

fn invalid(message: String) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidInput, message)
}

/// The kinds of address that are downloaded.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Scheme {
    Https,
    /// Plain http to the own computer.
    LoopbackHttp,
}

/// Checks the address: https, or http to the own computer, and nothing a parser could read in two
/// ways (backslashes, spaces, control characters).
fn check_url(url: &str) -> io::Result<Scheme> {
    if url
        .chars()
        .any(|c| c == '\\' || c.is_whitespace() || c.is_control())
    {
        return Err(invalid(format!("not a plain address: {url:?}")));
    }
    let Some((scheme, rest)) = url.split_once("://") else {
        return Err(invalid(format!("not an address with a scheme: {url}")));
    };
    match scheme.to_ascii_lowercase().as_str() {
        "https" => Ok(Scheme::Https),
        "http" => {
            // `user@host`: the host is what follows the last `@` of the part before the path
            let authority = rest.split(['/', '?', '#']).next().unwrap_or_default();
            let host_and_port = authority.rsplit('@').next().unwrap_or_default();
            let host = match host_and_port.strip_prefix('[') {
                Some(v6) => v6.split(']').next().unwrap_or_default(),
                None => host_and_port.split(':').next().unwrap_or_default(),
            };
            let loopback = host.eq_ignore_ascii_case("localhost")
                || host.parse::<IpAddr>().is_ok_and(|ip| ip.is_loopback());
            if loopback {
                Ok(Scheme::LoopbackHttp)
            } else {
                Err(invalid(format!("plain http is refused, use https: {url}")))
            }
        }
        _ => Err(invalid(format!("only https is supported: {url}"))),
    }
}

/// The sum as 64 lower case hexadecimal characters.
fn normalize_sha256(sum: &str) -> io::Result<String> {
    let sum = sum.trim().to_ascii_lowercase();
    if sum.len() == 64 && sum.bytes().all(|b| b.is_ascii_hexdigit()) {
        Ok(sum)
    } else {
        Err(invalid(
            "a SHA-256 sum has 64 hexadecimal characters".into(),
        ))
    }
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

/// The download: the transfer runs in a thread of its own, so the caller can give up on a server
/// that goes silent (a read from a socket cannot be interrupted).
fn download(
    url: &str,
    destination: &Path,
    sha256: Option<&str>,
    max_bytes: u64,
    idle: Duration,
    mut progress: impl FnMut(u64, Option<u64>),
) -> io::Result<u64> {
    let scheme = check_url(url)?;
    let expected = sha256.map(normalize_sha256).transpose()?;
    if let Some(parent) = destination.parent() {
        fs::create_dir_all(parent)?;
    }
    let partial = partial_name(destination);
    // a file that is left from before, or that somebody put there, is not ours
    let _ = fs::remove_file(&partial);

    let (sender, receiver) = mpsc::channel();
    let cancel = Arc::new(AtomicBool::new(false));
    {
        let (url, partial, cancel) = (url.to_owned(), partial.clone(), cancel.clone());
        thread::spawn(move || {
            let result = transfer(
                &url, scheme, &partial, expected, max_bytes, &cancel, &sender,
            );
            // after a cancel the caller has deleted the file already, and the name may belong
            // to a newer download
            if result.is_err() && !cancel.load(Ordering::Relaxed) {
                let _ = fs::remove_file(&partial);
            }
            let _ = sender.send(Event::Done(result));
        });
    }

    // connecting and the answer have their own timeouts in the thread
    let mut wait = idle + 2 * PHASE_TIMEOUT;
    loop {
        match receiver.recv_timeout(wait) {
            Ok(Event::Progress(done, total)) => {
                wait = idle;
                progress(done, total);
            }
            Ok(Event::Done(Ok(bytes))) => {
                fs::rename(&partial, destination)?;
                return Ok(bytes);
            }
            Ok(Event::Done(Err(error))) => return Err(error),
            Err(RecvTimeoutError::Timeout) => {
                cancel.store(true, Ordering::Relaxed);
                let _ = fs::remove_file(&partial);
                return Err(io::Error::new(
                    io::ErrorKind::TimedOut,
                    format!("the server sent nothing for {} seconds", wait.as_secs()),
                ));
            }
            Err(RecvTimeoutError::Disconnected) => {
                return Err(io::Error::other("the download stopped unexpectedly"));
            }
        }
    }
}

fn transfer(
    url: &str,
    scheme: Scheme,
    partial: &Path,
    expected: Option<String>,
    max_bytes: u64,
    cancel: &AtomicBool,
    events: &Sender<Event>,
) -> io::Result<u64> {
    let loopback = scheme == Scheme::LoopbackHttp;
    let agent: ureq::Agent = ureq::Agent::config_builder()
        // an https address stays https: a redirect to plain http is refused
        .https_only(!loopback)
        .max_redirects(if loopback { 0 } else { 10 })
        .timeout_connect(Some(PHASE_TIMEOUT))
        .timeout_recv_response(Some(PHASE_TIMEOUT))
        .build()
        .into();
    let response = agent.get(url).call().map_err(io::Error::other)?;
    let total = response
        .headers()
        .get("content-length")
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.parse::<u64>().ok());
    if total.is_some_and(|total| total > max_bytes) {
        return Err(too_big());
    }
    let _ = events.send(Event::Progress(0, total));

    // `take`: a server may send more than it said, or without saying
    let reader = response
        .into_body()
        .into_reader()
        .take(max_bytes.saturating_add(1));
    let (bytes, sum) = copy_body(reader, partial, total, cancel, events)?;
    if bytes > max_bytes {
        return Err(too_big());
    }
    if let Some(expected) = expected
        && sum != expected
    {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("the SHA-256 sum of the download is {sum}, expected {expected}"),
        ));
    }
    Ok(bytes)
}

fn too_big() -> io::Error {
    io::Error::new(
        io::ErrorKind::FileTooLarge,
        "the download is bigger than the limit",
    )
}

fn partial_name(destination: &Path) -> PathBuf {
    let mut name = destination.as_os_str().to_owned();
    name.push(".part");
    PathBuf::from(name)
}

/// Writes the body to `partial` (a new file: never through a link that was there before) and
/// returns the number of bytes and their SHA-256 sum.
fn copy_body(
    mut reader: impl Read,
    partial: &Path,
    total: Option<u64>,
    cancel: &AtomicBool,
    events: &Sender<Event>,
) -> io::Result<(u64, String)> {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(partial)?;
    let mut hasher = Sha256::new();
    let mut buffer = vec![0u8; 64 * 1024];
    let mut done = 0u64;
    loop {
        let read = reader.read(&mut buffer)?;
        if read == 0 {
            break;
        }
        if cancel.load(Ordering::Relaxed) {
            return Err(io::Error::new(io::ErrorKind::Interrupted, "cancelled"));
        }
        file.write_all(&buffer[..read])?;
        hasher.update(&buffer[..read]);
        done += read as u64;
        // nobody is listening any more after a cancel
        let _ = events.send(Event::Progress(done, total));
    }
    file.flush()?;
    Ok((done, hex(&hasher.finalize())))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{BufRead, BufReader};
    use std::net::TcpListener;
    use std::time::Instant;

    /// A server that answers every request with `status` and `body`.
    fn serve(status: &'static str, body: &'static [u8]) -> String {
        serve_slowly(status, body.len(), vec![(body, Duration::ZERO)])
    }

    /// A server that announces `length` bytes and sends the chunks, each followed by a pause.
    fn serve_slowly(
        status: &'static str,
        length: usize,
        chunks: Vec<(&'static [u8], Duration)>,
    ) -> String {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        thread::spawn(move || {
            for stream in listener.incoming().flatten() {
                let mut reader = BufReader::new(stream.try_clone().unwrap());
                let mut line = String::new();
                while reader.read_line(&mut line).is_ok() && line.trim() != "" {
                    line.clear();
                }
                let mut stream = stream;
                let head = format!(
                    "HTTP/1.1 {status}\r\nContent-Length: {length}\r\nConnection: close\r\n\r\n"
                );
                let _ = stream.write_all(head.as_bytes());
                for (chunk, pause) in &chunks {
                    let _ = stream.write_all(chunk);
                    let _ = stream.flush();
                    thread::sleep(*pause);
                }
            }
        });
        format!("http://{address}/file.bin")
    }

    #[test]
    fn a_file_is_downloaded_with_its_progress() {
        let url = serve("200 OK", b"hello download");
        let dir = tempfile::tempdir().unwrap();
        let target = dir.path().join("deep/folder/file.bin");
        let mut last = (0, None);
        let bytes = download_file(&url, &target, |done, total| last = (done, total)).unwrap();
        assert_eq!(bytes, 14);
        assert_eq!(last, (14, Some(14)));
        assert_eq!(fs::read(&target).unwrap(), b"hello download");
        assert!(!partial_name(&target).exists());
    }

    #[test]
    fn an_error_status_is_an_error_and_leaves_nothing() {
        let url = serve("404 Not Found", b"nothing here");
        let dir = tempfile::tempdir().unwrap();
        let target = dir.path().join("file.bin");
        assert!(download_file(&url, &target, |_, _| {}).is_err());
        assert!(!target.exists() && !partial_name(&target).exists());
    }

    #[test]
    fn an_unreachable_server_is_an_error() {
        let dir = tempfile::tempdir().unwrap();
        let url = "http://127.0.0.1:9/never";
        assert!(download_file(url, &dir.path().join("x"), |_, _| {}).is_err());
    }

    #[test]
    fn only_https_and_the_own_computer_are_allowed() {
        for allowed in [
            ("https://example.org/a.zip", Scheme::Https),
            ("HTTPS://EXAMPLE.ORG/a.zip", Scheme::Https),
            ("http://localhost:8080/a", Scheme::LoopbackHttp),
            ("http://127.0.0.1/a", Scheme::LoopbackHttp),
            ("http://[::1]:80/a", Scheme::LoopbackHttp),
            ("HTTP://LOCALHOST/a", Scheme::LoopbackHttp),
        ] {
            assert_eq!(check_url(allowed.0).unwrap(), allowed.1, "{}", allowed.0);
        }
        for refused in [
            "http://example.org/a.zip",
            "HTTP://example.org/a.zip",
            // the host is what follows the `@`
            "http://127.0.0.1@example.org/a",
            "http://localhost.example.org/a",
            "http://127.0.0.1.example.org/a",
            // two ways to read it
            "http://example.org\\@127.0.0.1/a",
            "http://127.0.0.1\\@example.org/a",
            "http://127.0.0.1/a b",
            "ftp://example.org/a",
            "file:///etc/passwd",
            "example.org/a",
            "",
        ] {
            let error = check_url(refused).unwrap_err();
            assert_eq!(error.kind(), io::ErrorKind::InvalidInput, "{refused}");
        }
        let dir = tempfile::tempdir().unwrap();
        let error = download_file("http://example.org/a", &dir.path().join("x"), |_, _| {});
        assert_eq!(error.unwrap_err().kind(), io::ErrorKind::InvalidInput);
    }

    #[test]
    fn a_wrong_checksum_deletes_the_file() {
        let url = serve("200 OK", b"hello download");
        let dir = tempfile::tempdir().unwrap();
        let target = dir.path().join("file.bin");
        let sum = hex(&Sha256::digest(b"hello download"));

        // a bad sum is refused before the download starts
        let error = download_file_verified(&url, &target, "1234", |_, _| {}).unwrap_err();
        assert_eq!(error.kind(), io::ErrorKind::InvalidInput);

        // the right sum, in upper case, too
        let bytes = download_file_verified(&url, &target, &sum.to_uppercase(), |_, _| {}).unwrap();
        assert_eq!(bytes, 14);
        fs::remove_file(&target).unwrap();

        let wrong = "0".repeat(64);
        let error = download_file_verified(&url, &target, &wrong, |_, _| {}).unwrap_err();
        assert_eq!(error.kind(), io::ErrorKind::InvalidData);
        assert!(error.to_string().contains(&sum), "{error}");
        assert!(!target.exists() && !partial_name(&target).exists());
    }

    #[test]
    fn a_download_over_the_limit_is_refused() {
        let url = serve("200 OK", b"hello download");
        let dir = tempfile::tempdir().unwrap();
        let target = dir.path().join("file.bin");
        let idle = Duration::from_secs(5);
        let error = download(&url, &target, None, 5, idle, |_, _| {}).unwrap_err();
        assert_eq!(error.kind(), io::ErrorKind::FileTooLarge);
        assert!(!target.exists() && !partial_name(&target).exists());

        // a server that says less than it sends: the data is cut at the limit
        let url = serve_slowly("200 OK", 3, vec![(&b"hello download"[..], Duration::ZERO)]);
        let error = download(&url, &target, None, 3, idle, |_, _| {});
        // (the HTTP client itself refuses the surplus, or the limit does)
        assert!(
            error.is_err() || fs::read(&target).unwrap().len() <= 3,
            "{error:?}"
        );
        assert!(!partial_name(&target).exists());
    }

    #[test]
    fn a_slow_download_that_keeps_coming_is_not_cut_off() {
        // 8 bytes in 2.4 seconds: longer than the 1 second a silent server may take
        let step = Duration::from_millis(300);
        let chunks: Vec<(&'static [u8], Duration)> = vec![
            (b"s", step),
            (b"l", step),
            (b"o", step),
            (b"w", step),
            (b"l", step),
            (b"o", step),
            (b"a", step),
            (b"d", step),
        ];
        let url = serve_slowly("200 OK", 8, chunks);
        let dir = tempfile::tempdir().unwrap();
        let target = dir.path().join("file.bin");
        let started = Instant::now();
        let bytes = download(&url, &target, None, 1024, Duration::from_secs(1), |_, _| {}).unwrap();
        assert!(started.elapsed() > Duration::from_secs(2));
        assert_eq!(
            (bytes, fs::read(&target).unwrap()),
            (8, b"slowload".to_vec())
        );
    }

    #[test]
    fn a_server_that_goes_silent_is_given_up_on() {
        // 3 bytes, then nothing for a long time
        let url = serve_slowly("200 OK", 100, vec![(&b"abc"[..], Duration::from_secs(20))]);
        let dir = tempfile::tempdir().unwrap();
        let target = dir.path().join("file.bin");
        let started = Instant::now();
        let error =
            download(&url, &target, None, 1024, Duration::from_secs(1), |_, _| {}).unwrap_err();
        assert_eq!(error.kind(), io::ErrorKind::TimedOut, "{error}");
        assert!(started.elapsed() < Duration::from_secs(10));
        assert!(!target.exists() && !partial_name(&target).exists());
    }

    #[cfg(unix)]
    #[test]
    fn a_link_in_the_place_of_the_temporary_file_is_not_followed() {
        let url = serve("200 OK", b"hello download");
        let dir = tempfile::tempdir().unwrap();
        let victim = dir.path().join("victim");
        fs::write(&victim, "keep").unwrap();
        let target = dir.path().join("file.bin");
        std::os::unix::fs::symlink(&victim, partial_name(&target)).unwrap();

        download_file(&url, &target, |_, _| {}).unwrap();
        assert_eq!(fs::read(&victim).unwrap(), b"keep");
        assert_eq!(fs::read(&target).unwrap(), b"hello download");
    }
}
