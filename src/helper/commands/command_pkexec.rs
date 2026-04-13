/**
 * command_pkexec.rs
 *
 * (C) 2026 AtlantisOS Project
 * by @NachtsternBuild
 *
 * License: GNU GENERAL PUBLIC LICENSE Version 3
 *
 * Usage:
 * * command_pkexec():
 * fn main() {
 *  command_pkexec(&["ls", "/root"]);
 * }
 */

use std::process::Command;

/**
 * @brief run a command with pkexec
 * * @param args: list of arguments (e.g. ["apt", "update"])
 */
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

// TODO: UI Varianten
