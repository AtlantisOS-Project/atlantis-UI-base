//! Provides progress indicators for system commands.
//!
//! This module allows you to run a list of shell commands in the background
//! while displaying a non-closable dialog with a spinner or 
//! a progress bar to the user.
/**
* dialogs_spinner.rs
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

/// Specifies the type of progress indicator in the dialog.
pub enum IndicatorType {
	/// A circular, indefinite loading indicator (Libadwaita Spinner).
    Spinner,
    /// A horizontal bar that indicates activity by moving back and forth (Pulse).
    ProgressBar,
}

/// Executes a list of shell commands in a background thread.
///
/// The commands are executed one after another. As soon as a command fails,
/// the chain is terminated and `false` is sent to the main thread.
///
/// # Platform-specific behavior
/// - **Windows:** Uses `cmd /C` for execution.
/// - **Unix/Linux:** Uses `sh -c` for execution.
fn run_commands_thread(commands: Vec<String>, tx: mpsc::Sender<Result<String, String>>) {
    thread::spawn(move || {
        let mut combined_output = String::new();

        for cmd_str in commands {
            let output_res = if cfg!(target_os = "windows") {
                Command::new("cmd")
                    .arg("/C")
                    .arg(&cmd_str)
                    .output()
            } else {
                Command::new("sh")
                    .arg("-c")
                    .arg(&cmd_str)
                    .output()
            };

            match output_res {
                Ok(output) => {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    
                    combined_output.push_str(&stdout);
                    if !stderr.is_empty() {
                        combined_output.push_str(&format!("\n[Stderr]: {}", stderr));
                    }

                    if !output.status.success() {
                        let _ = tx.send(Err(format!("Command failed: {}\nOutput:\n{}", cmd_str, combined_output)));
                        return;
                    }
                }
                Err(e) => {
                    let _ = tx.send(Err(format!("Failed to execute command '{}': {}", cmd_str, e)));
                    return;
                }
            }
        }

        let _ = tx.send(Ok(combined_output));
    });
}

/// Displays a modal dialog while a list of system commands is being executed.
///
/// The dialog cannot be closed manually by the user (`can_close(false)`).
/// It closes automatically once all commands have completed or an error occurs.
///
/// # Arguments
///
/// * `parent` - The application's main window.
/// * `title` - Title of the dialog.
/// * `message` - Information text for the user.
/// * `commands` - A vector of strings interpreted as shell commands.
/// * `indicator` - The visual style ([IndicatorType]).
/// * `on_complete` - Optional callback: `Some(move |res| { ... })` oder `None`.
///
/// # Usage (Without output):
/// ```rust
/// show_spinner_dialog(
/// 	&window, 
///		"Info", 
/// 	"Please wait...", 
/// 	commands, 
///		IndicatorType::Spinner, 
/// 	None
/// );
/// ```
///
/// # Usage (With Output):
/// ```rust
/// show_spinner_dialog(
///		&window, 
///		"Info", 
///		"Please wait...", 
/// 	commands, 
///		IndicatorType::Spinner, 
///		Some(|res| {
///     	match res {
///         	Ok(out) => println!("Output: {}", out),
///         	Err(err) => eprintln!("Error: {}", err),
///     	}
/// 	})
/// );
/// ```
pub fn show_spinner_dialog(
    parent: &adw::ApplicationWindow,
    title: &str,
    message: &str,
    commands: Vec<String>,
    indicator: IndicatorType,
    on_complete: Option<Box<dyn FnOnce(Result<String, String>) + 'static>>,
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
    
    let (tx, rx) = mpsc::channel::<Result<String, String>>();
    run_commands_thread(commands, tx);

    let mut on_complete_opt = on_complete;

    // check the signal from the background thread
    let dialog_to_close = dialog.clone();
    gtk4::glib::timeout_add_local(std::time::Duration::from_millis(25), move || {
        match rx.try_recv() {
            Ok(result) => {
                dialog_to_close.force_close();
                if let Some(cb) = on_complete_opt.take() {
                    cb(result); // Das Ausführen bleibt identisch
                }
                gtk4::glib::ControlFlow::Break
            }
            Err(mpsc::TryRecvError::Empty) => {
                gtk4::glib::ControlFlow::Continue
            }
            Err(mpsc::TryRecvError::Disconnected) => {
                dialog_to_close.force_close();
                if let Some(cb) = on_complete_opt.take() {
                    cb(Err("Thread disconnected unexpectedly.".to_string()));
                }
                gtk4::glib::ControlFlow::Break
            }
        }
    });
    
    dialog.present(Some(parent));
}
