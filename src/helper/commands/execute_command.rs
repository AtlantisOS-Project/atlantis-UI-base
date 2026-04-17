//! Functions to run a shell command
/**
* execute_command.rs
*
* (C) Copyright 2026 AtlantisOS Project
* by @NachtsternBuild
*
* License: GNU GENERAL PUBLIC LICENSE Version 3
*/

use std::process::{Command, Stdio, ExitStatus};
use std::io::{self};

/// Run a command and direct the output to stdout
/// ### Usage:
///
/// ```rust
/// fn main() {
///    let status = run_command(&["echo", "Simuliere apt update..."]);
///    match status {
///        Ok(s) => println!("Ends with code: {}", s),
///        Err(e) => ephemerals!("Error: {}", e),
///    }
/// }
/// ```
pub fn run_command(args: &[&str]) -> io::Result<ExitStatus> {
    let mut child = Command::new(args[0])
        .args(&args[1..])
        .spawn()?;

    child.wait()
}

/// Run a command an capture the stadout of the command
/// ### Usage:
/// 
/// ```rust
/// fn main {
/// match capture_command_output(&["date"]) {
///        Ok(out) => println!("The output was: {}", out.trim()),
///        Err(e) => eprintln!("Error during capture: {}", e),
///		}
/// }
/// ``` 
pub fn capture_command_output(args: &[&str]) -> io::Result<String> {
    let output = Command::new(args[0])
        .args(&args[1..])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()?;

    if output.status.success() {
        let result = String::from_utf8_lossy(&output.stdout).into_owned();
        Ok(result)
    } 
    
    else {
        let error = String::from_utf8_lossy(&output.stderr);
        Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Command failed: {}", error),
        ))
    }
}
