//! Run a command with pkexec in the shell
/**
 * command_pkexec.rs
 *
 * (C) 2026 AtlantisOS Project
 * by @NachtsternBuild
 *
 * License: GNU GENERAL PUBLIC LICENSE Version 3
 */

use std::process::Command;

/// Command with pkexec
/// ### Usage:
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
