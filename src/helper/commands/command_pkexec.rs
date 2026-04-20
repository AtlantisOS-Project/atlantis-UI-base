//! Direct execution of privileged commands via `pkexec`.
//!
//! This module provides a lightweight interface for executing system commands with 
//! root privileges, capturing the output (`stdout`), and 
//! handling error conditions (`stderr`) cleanly.
/**
 * command_pkexec.rs
 *
 * (C) 2026 AtlantisOS Project
 * by @NachtsternBuild
 *
 * License: GNU GENERAL PUBLIC LICENSE Version 3
 */

use std::process::Command;

/// Executes a command with elevated privileges via `pkexec` and waits for it to finish.
///
/// This function blocks the current thread until the command (and 
/// user authentication) is complete. It is particularly suitable 
/// for CLI tools or background tasks that do not require direct 
/// UI feedback (such as a spinner).
///
/// # Arguments
/// * `args` - A slice of string references containing the command and its parameters.
///
/// # How it works
/// 1. Starts a new process using the `pkexec` program.
/// 2. Passes the argument list to the process.
/// 3. Captures the complete output after the process terminates.
///
/// # Error Handling
/// - On success, the contents of `stdout` are output with the prefix `[INFO]`.
/// - If the command fails (exit code != 0), `stderr` is printed with the prefix `[ERROR]`.
/// - If `pkexec` itself cannot be started, an error message is printed to the standard error output.
///
/// # Usage:
/// args are list of arguments (e.g. ["apt", "update"])
///
/// ```rust
/// fn main() {
/// command_pkexec(&["ls", "/root"]);
/// }
/// ```
pub fn command_pkexec(args: &[&str]) {
    // create a new process
    let output = Command::new("pkexec")
        .args(args)
        .output(); // run the command and wait for the output

    match output {
        Ok(res) => {
            if res.status.success() {
                let stdout = String::from_utf8_lossy(&res.stdout);
                println!("[INFO] Execution successful: {}", stdout);
            } 
            
            else {
                let stderr = String::from_utf8_lossy(&res.stderr);
                eprintln!("[ERROR] pkexec failed with status {}: {}", res.status, stderr);
            }
        }
        Err(e) => {
            eprintln!("[ERROR] Could not start pkexec: {}", e);
        }
    }
}
