/**
* execute_command.rs
*
* (C) Copyright 2026 AtlantisOS Project
* by @NachtsternBuild
*
* License: GNU GENERAL PUBLIC LICENSE Version 3
*
* Usage:
* fn main() {
*    let status = run_command(&["echo", "Simuliere apt update..."]);
*    match status {
*        Ok(s) => println!("Beendet mit Code: {}", s),
*        Err(e) => ephemerals!("Fehler: {}", e),
*    }
*    match capture_command_output(&["date"]) {
*        Ok(out) => println!("Der Output war: {}", out.trim()),
*        Err(e) => eprintln!("Fehler beim Capturing: {}", e),
*    }
* }
*/

use std::process::{Command, Stdio, ExitStatus};
use std::io::{self};

/**
* @brief Run a command and direct the output to stdout
*/
pub fn run_command(args: &[&str]) -> io::Result<ExitStatus> {
    let mut child = Command::new(args[0])
        .args(&args[1..])
        .spawn()?;

    child.wait()
}

/**
* @brief Run a command an capture the stadout of the command
*/
pub fn capture_command_output(args: &[&str]) -> io::Result<String> {
    let output = Command::new(args[0])
        .args(&args[1..])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()?;

    if output.status.success() {
        let result = String::from_utf8_lossy(&output.stdout).into_owned();
        Ok(result)
    } else {
        let error = String::from_utf8_lossy(&output.stderr);
        Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Command failed: {}", error),
        ))
    }
}
