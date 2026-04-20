//! Automated terminal interaction for various desktop environments.
//!
//! This module detects the current operating system and desktop environment (e.g., GNOME, KDE, WSL),
//! selects the appropriate terminal emulator, and executes a specific command within it.
/**
 * open_terminal_desktop.rs
 *
 * (C) 2026 AtlantisOS Project
 * by @NachtsternBuild
 *
 * License: GNU GENERAL PUBLIC LICENSE Version 3
 */

use std::env;
use std::path::Path;
use std::process::Command;
use crate::prelude::command_exists_native;

/// Opens a new terminal window and executes a command in it.
///
/// This function is the core component for CLI interactions within the GUI. 
/// It ensures that the terminal remains open after execution (`exec bash`).
///
/// # Supported Environments
/// - **Linux:** Detects GNOME, KDE, MATE, Cinnamon, LXDE/LXQT, and provides fallbacks such as `xterm`.
/// - **macOS:** Uses AppleScript (`osascript`) to control Terminal.app.
/// - **Windows:** Uses `cmd.exe /K`.
/// - **WSL:** Detects a Windows Subsystem environment and launches the Windows Terminal from within Linux.
///
/// # Usage
///
/// ```rust
/// fn main() {
///   open_terminal_by_desktop("echo 'Hello Atlantis'; sleep 5");
/// }
/// ```
pub fn open_terminal_by_desktop(function_command: &str) {
    if function_command.is_empty() {
        return;
    }

    // check if this run via WSL
    if Path::new("/mnt/c/Users").exists() {
        let wsl_cmd = format!("cmd.exe /C start cmd.exe /K \"wsl.exe {}\"", function_command);
        let _ = Command::new("sh").arg("-c").arg(wsl_cmd).spawn();
        return;
    }

    // run the OS based command
    spawn_terminal(function_command);
}

// for windows
#[cfg(target_os = "windows")]
fn spawn_terminal(func_cmd: &str) {
    let cmd = format!("start cmd.exe /K \"{}\"", func_cmd);
    let _ = Command::new("cmd").arg("/C").arg(cmd).spawn();
}

// for macos
#[cfg(target_os = "macos")]
fn spawn_terminal(func_cmd: &str) {
    let script = format!("tell application \"Terminal\" to do script \"{}\"", func_cmd);
    let _ = Command::new("osascript").arg("-e").arg(script).spawn();
}

// all other os
#[cfg(not(any(target_os = "windows", target_os = "macos")))]
fn spawn_terminal(func_cmd: &str) {
	// try XDG to get the desktop
    let desktop = env::var("XDG_CURRENT_DESKTOP").unwrap_or_default().to_uppercase();
    
    // get the terminal env
    let (term, args) = if desktop.contains("GNOME") || desktop.contains("MATE") || desktop.contains("X-CINNAMON") {
        (if desktop.contains("MATE") { 
        	"mate-terminal" 
        } 
        
        else { 
        	"gnome-terminal" 
        }, "--")
    } 
    
    else if desktop.contains("KDE") {
        ("konsole", "-e")
    } 
    
    else if desktop.contains("LXDE") || desktop.contains("LXQT") {
        ("lxterminal", "-e")
    } 
    
    else {
        // fallback logic
        let fallbacks = vec![
            ("gnome-terminal", "--"),
            ("x-terminal-emulator", "-e"),
            ("konsole", "-e"),
            ("xfce4-terminal", "-e"),
            ("xterm", "-e"),
        ];

        let mut found = None;
        for (t, a) in fallbacks {
            if command_exists_native(t) {
                found = Some((t, a));
                break;
            }
        }
		
		// found unsupported env
        match found {
            Some(f) => f,
            None => {
                eprintln!("[ERROR] No supported terminal found for desktop: {}", desktop);
                return;
            }
        }
    };

    // run the command
    let full_cmd = format!("{}; exec bash", func_cmd);
    let _ = Command::new(term)
        .arg(args)
        .arg("bash")
        .arg("-c")
        .arg(full_cmd)
        .spawn();
}
