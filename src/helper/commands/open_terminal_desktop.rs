/**
 * open_terminal_desktop.rs
 *
 * (C) 2026 AtlantisOS Project
 * by @NachtsternBuild
 *
 * License: GNU GENERAL PUBLIC LICENSE Version 3
 *
 * @brief Run a command in a new terminal
 *
 * Usage:
 * fn main() {
 *   open_terminal_by_desktop("echo 'Hello Atlantis'; sleep 5");
 * }
 * 
 * Note:
 * default OS is linux
 */

use std::env;
use std::process::Command;
use std::path::Path;

// check if the command already exists
fn command_exists(cmd: &str) -> bool {
    Command::new("sh")
        .arg("-c")
        .arg(format!("command -v {}", cmd))
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

/**
 * @brief Open a terminal and run in this terminal a command
 */
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
            if command_exists(t) {
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
