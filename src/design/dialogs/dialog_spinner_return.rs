//! Progress indicators and background execution infrastructure for system commands.
//!
//! This module provides synchronous-looking asynchronous orchestration for running
//! sequential system commands (`std::process::Command`) inside a dedicated OS thread,
//! preventing UI degradation or freezing in Libadwaita/GTK 4 applications.
//!
//! While commands execute, a modal, non-closable [`adw::Dialog`] visually communicates
//! activity via an indeterminate [`adw::Spinner`] or a pulsing [`gtk4::ProgressBar`].
//!
//! # Architecture
//!
//! ```text
//!  Main (UI) Thread                      Background Worker Thread
//! ┌────────────────────────┐            ┌────────────────────────┐
//! │  show_spinner_dialog   │            │  run_commands_thread   │
//! │  Instantiates Dialog   │            │                        │
//! │  Spawns Thread ────────┼───────────►│  Loop: Execute Command │
//! │                        │            │        Capture Output  │
//! │  glib::timeout_add     │◄───────────┼──────  Send progress   │
//! │  Polls Receiver Channel│   mpsc     │                        │
//! └────────────────────────┘            └────────────────────────┘
//! ```
//!
//! Once all operations finish—or immediately when any command emits a non-zero exit 
//! status—the channel dispatches historical telemetry back to the main context, closes 
//! the dialog safely, and triggers the dynamic callback.

/**
* dialogs_spinner_return.rs
*
* (C) Copyright 2026 AtlantisOS Project
* by @NachtsternBuild
*
* License: GNU GENERAL PUBLIC LICENSE Version 3
*/

use adw::prelude::*;
use gtk4::{
    glib, 
    ProgressBar, 
    Align, 
    Label, 
    Orientation, 
    Box as GtkBox
};
use adw::{
    Spinner, 
    Dialog
};
use std::process::Command;
use std::thread;
use std::sync::mpsc;
use crate::ui_prelude::IndicatorType;

/// Encapsulates execution metrics and console streams of an evaluated system process.
///
/// This telemetry model tracks standard output buffers and processing results
/// passed from the background worker thread back into the GUI loop.
#[derive(Debug, Clone)]
pub struct CommandResult {
    /// The exact command string representation including joined sub-arguments.
    ///
    /// # Example
    /// `"fastboot flash bootloader boot.img"`
    pub command: String,
    
    /// Collected standard output stream data (`stdout`), safely translated via UTF-8 lossy conversion.
    pub stdout: String,
    
    /// Collected standard error stream data (`stderr`), capturing diagnostics, trace logs, or error text.
    pub stderr: String,
    
    /// Termination status flag. Evaluates to `true` if the process returned an exit code of `0`.
    pub success: bool,
}

/// Dispatches sequential system processes sequentially inside an isolated background thread.
///
/// Execution processing works like a short-circuiting chain. If any atomic command reports a 
/// failure execution profile (or fails to start entirely), downstream commands are dropped, 
/// and the entire analytical stack collected up to that fraction is packaged as an `Err`.
///
/// # Arguments
///
/// * `commands` - Multi-dimensional vector matrix representing command calls with arguments.
/// * `tx` - Asynchronous Multi-Producer Single-Consumer transmission piping boundary.
fn run_commands_thread(commands: Vec<Vec<String>>, tx: mpsc::Sender<Result<Vec<CommandResult>, Vec<CommandResult>>>) {
    thread::spawn(move || {
        let mut results = Vec::new();

        for cmd_args in commands {
            if cmd_args.is_empty() {
                continue;
            }

            let program = &cmd_args[0];
            let args = &cmd_args[1..];
            let cmd_display = cmd_args.join(" ");
            
            let output_res = Command::new(program)
                .args(args)
                .output();

            match output_res {
                Ok(output) => {
                    let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
                    let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
                    let success = output.status.success();

                    results.push(CommandResult {
                        command: cmd_display,
                        stdout,
                        stderr,
                        success,
                    });

                    if !success {
                        let _ = tx.send(Err(results));
                        return;
                    }
                }
                Err(e) => {
                    results.push(CommandResult {
                        command: cmd_display,
                        stdout: String::new(),
                        stderr: format!("Failed to execute process: {}", e),
                        success: false,
                    });
                    let _ = tx.send(Err(results));
                    return;
                }
            }
        }

        let _ = tx.send(Ok(results));
    });
}

/// Renders a non-closable, modal Libadwaita dialog while driving continuous background jobs.
///
/// The dialog configuration parameter enforces `.can_close(false)`, effectively locking input interactions
/// out of the parent frame until execution results are transmitted or thread context disconnects.
///
/// # Arguments
///
/// * `parent` - Reference context to the hosting [`adw::ApplicationWindow`] establishing transient alignment.
/// * `title` - Heading string printed atop the viewport and inside the dialog frame wrapper.
/// * `message` - Explanatory localized label variant communicating instructions or contextual operations to users.
/// * `commands` - Command array sequences mapping programs to discrete parameters (e.g., `vec![vec!["pkexec", "apt", "update"]]`).
/// * `indicator` - Choice variant enforcing either a circular [`IndicatorType::Spinner`] or a sliding linear [`IndicatorType::ProgressBar`].
/// * `on_complete` - Dynamic completion callback closure providing explicit error and success evaluation data trees.
///
/// # Examples
///
/// ```rust
/// show_spinner_dialog_return_output(
///		window, 
///		"System-Update", 
///		"Updating System...", 
///		vec![
///			vec![
///				"ls".to_string(),
///				"./".to_string()
///			],
///			vec![
///				"tree".to_string(),
///				"./".to_string()
///			],
///			vec![
///				"ls".to_string(),
///				"-l".to_string()
///			]
///		], 
///		IndicatorType::Spinner, 
///		Some(Box::new(|res| {
///			match res {
///				Ok(success_list) => {
///    				println!("Success!");
///    				for cmd in success_list {
///        				println!("-> [{}]: {}", cmd.command, cmd.stdout);
///    				}
///				}
///				Err(failed_chain) => {
///    				println!("Abort!");
///   				for cmd in failed_chain {
///   				    if cmd.success {
///   				        println!(" [OK] {}", cmd.command);
///        				} else {
///        				    println!(" [ERROR] {}", cmd.command);
///        				    println!("  Stderr: {}", cmd.stderr);
///	    	    		}
///	    			}	
///				}
///   		}
///		}))
/// );
/// ```
pub fn show_spinner_dialog_return_output(
    parent: &adw::ApplicationWindow,
    title: &str,
    message: &str,
    commands: Vec<Vec<String>>,
    indicator: IndicatorType,
    on_complete: Option<Box<dyn FnOnce(Result<Vec<CommandResult>, Vec<CommandResult>>) + 'static>>,
) {
    // create dialog
    let dialog = Dialog::builder()
        .title(title)
        .content_width(400)
        .can_close(false)
        .build();

    // layout
    let root_box = GtkBox::new(Orientation::Vertical, 18);
    root_box.set_margin_top(24);
    root_box.set_margin_bottom(24);
    root_box.set_margin_start(24);
    root_box.set_margin_end(24);
    root_box.set_halign(Align::Center);

    let label_title = Label::builder()
        .label(title)
        .build();
    
    let label_msg = Label::new(Some(message));

    root_box.append(&label_title);
    root_box.append(&label_msg);
    
    // add indicator
    let spinner = Spinner::builder()
        .halign(Align::Center)
        .valign(Align::Center)
        .width_request(150)
        .height_request(150)
        .build();

    match indicator {
        IndicatorType::Spinner => {
            root_box.append(&spinner);
        }
        IndicatorType::ProgressBar => {
            let progress_bar = ProgressBar::builder()
                .pulse_step(0.1)
                .halign(Align::Center)
                .valign(Align::Center)
                .width_request(150)
                .height_request(50)
                .build();
        
            root_box.append(&progress_bar);
            
            glib::timeout_add_local(std::time::Duration::from_millis(100), move || {
                progress_bar.pulse();
                glib::ControlFlow::Continue
            });
        }
    }

    dialog.set_child(Some(&root_box));
    
    let (tx, rx) = mpsc::channel::<Result<Vec<CommandResult>, Vec<CommandResult>>>();
    run_commands_thread(commands, tx);

    let mut on_complete_opt = on_complete;

    // check the signal from the background thread
    let dialog_to_close = dialog.clone();
    gtk4::glib::timeout_add_local(std::time::Duration::from_millis(25), move || {
        match rx.try_recv() {
            Ok(result) => {
                dialog_to_close.force_close();
                if let Some(cb) = on_complete_opt.take() {
                    cb(result);
                }
                gtk4::glib::ControlFlow::Break
            }
            Err(mpsc::TryRecvError::Empty) => {
                gtk4::glib::ControlFlow::Continue
            }
            Err(mpsc::TryRecvError::Disconnected) => {
                dialog_to_close.force_close();
                if let Some(cb) = on_complete_opt.take() {
                    cb(Err(vec![CommandResult {
                        command: "Unknown".to_string(),
                        stdout: String::new(),
                        stderr: "Thread disconnected unexpectedly.".to_string(),
                        success: false,
                    }]));
                }
                gtk4::glib::ControlFlow::Break
            }
        }
    });
    
    dialog.present(Some(parent));
}
