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
use crate::ui_prelude::IndicatorType;

/// Repräsentiert das detaillierte Ergebnis eines einzelnen ausgeführten Kommandos.
#[derive(Debug, Clone)]
pub struct CommandResult {
    /// Das ausgeführte Kommando als lesbarer String (z.B. "fastboot reboot").
    pub command: String,
    /// Die Standardausgabe (stdout) des Kommandos.
    pub stdout: String,
    /// Die Fehlerausgabe (stderr) des Kommandos.
    pub stderr: String,
    /// Gibt an, ob das Kommando erfolgreich (Exit-Status 0) beendet wurde.
    pub success: bool,
}

/// Executes a list of structured commands in a background thread.
///
/// The commands are executed one after another. As soon as a command fails,
/// the chain is terminated, and all results gathered up to that point 
/// (including the failed one) are sent back.
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

                    // Wenn ein Kommando fehlschlägt, brechen wir ab und senden die bisherige Liste als Err
                    if !success {
                        let _ = tx.send(Err(results));
                        return;
                    }
                }
                Err(e) => {
                    // Falls das Binary gar nicht erst gestartet werden kann (z.B. nicht gefunden)
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

        // Alle Kommandos waren erfolgreich
        let _ = tx.send(Ok(results));
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
/// * `commands` - A vector of command vectors.
/// * `indicator` - The visual style ([IndicatorType]).
/// * `on_complete` - Optional callback receiving the detailed vector of results.
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
    
    // Channel-Typ an die neue Struktur angepasst
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
                    // Im unwahrscheinlichen Fall eines Absturzes geben wir eine leere Fehlerliste zurück
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
