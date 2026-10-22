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

use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::time::Duration;

/// How long a connection or a silent server may take.
const TIMEOUT: Duration = Duration::from_secs(30);

/// Downloads `url` into the file `destination` and returns the number of bytes.
///
/// The file is written under a temporary name next to `destination` and renamed when the download
/// is complete, so a failed download never leaves a half file under the real name. The folders above
/// `destination` are created. Redirects are followed. `progress` is called after every block with
/// the bytes so far and the size, if the server tells it.
///
/// # Errors
///
/// Returns an [`io::Error`] if the connection fails, the server answers with an error status (404,
/// …), or the file cannot be written.
pub fn download_file(
    url: &str,
    destination: &Path,
    mut progress: impl FnMut(u64, Option<u64>),
) -> io::Result<u64> {
    let agent: ureq::Agent = ureq::Agent::config_builder()
        // an https address stays https: a redirect to plain http is refused
        .https_only(url.starts_with("https://"))
        .timeout_connect(Some(TIMEOUT))
        .timeout_recv_response(Some(TIMEOUT))
        .timeout_recv_body(Some(TIMEOUT))
        .build()
        .into();
    let response = agent.get(url).call().map_err(io::Error::other)?;
    let total = response
        .headers()
        .get("content-length")
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.parse::<u64>().ok());

    if let Some(parent) = destination.parent() {
        fs::create_dir_all(parent)?;
    }
    let partial = partial_name(destination);
    let result = copy_body(
        response.into_body().into_reader(),
        &partial,
        total,
        &mut progress,
    );
    match result {
        Ok(bytes) => {
            fs::rename(&partial, destination)?;
            Ok(bytes)
        }
        Err(error) => {
            let _ = fs::remove_file(&partial);
            Err(error)
        }
    }
}

fn partial_name(destination: &Path) -> PathBuf {
    let mut name = destination.as_os_str().to_owned();
    name.push(".part");
    PathBuf::from(name)
}

fn copy_body(
    mut reader: impl Read,
    partial: &Path,
    total: Option<u64>,
    progress: &mut impl FnMut(u64, Option<u64>),
) -> io::Result<u64> {
    let mut file = File::create(partial)?;
    let mut buffer = vec![0u8; 64 * 1024];
    let mut done = 0u64;
    loop {
        let read = reader.read(&mut buffer)?;
        if read == 0 {
            break;
        }
        file.write_all(&buffer[..read])?;
        done += read as u64;
        progress(done, total);
    }
    file.flush()?;
    Ok(done)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{BufRead, BufReader};
    use std::net::TcpListener;
    use std::thread;

    /// A server that answers every request with `status` and `body`.
    fn serve(status: &'static str, body: &'static [u8]) -> String {
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
                    "HTTP/1.1 {status}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                    body.len()
                );
                let _ = stream.write_all(head.as_bytes());
                let _ = stream.write_all(body);
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
}
