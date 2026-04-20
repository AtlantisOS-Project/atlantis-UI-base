//! Execution of privileged system commands via `pkexec`.
//!
//! This module combines the background execution of commands with the 
//! visual progress indicators from [`crate::design::dialogs::dialogs_spinner::show_spinner_dialog`]. It is 
//! automatically prefixed with the `pkexec` wrapper to request root privileges.
/**
* command_pkexec.rs
*
* (C) Copyright 2026 AtlantisOS Project
* by @NachtsternBuild
*
* License: GNU GENERAL PUBLIC LICENSE Version 3
*/

use crate::design::dialogs::dialogs_spinner::show_spinner_dialog;
use crate::design::dialogs::dialogs_spinner::IndicatorType;

/// Executes a single privileged command with a spinner dialog.
///
/// Opens a modal dialog and triggers the Polkit authentication prompt.
///
/// # Security
/// The command is passed directly to `pkexec`. Ensure that the 
/// `command` string does not contain any unsafe user input.
pub fn command_pkexec_spinner(
    parent: &adw::ApplicationWindow,
    command: &str,
    title: &str,
    text: &str
) {
    let full_command = format!("pkexec {}", command);
    show_spinner_dialog(parent, title, text, vec![full_command], IndicatorType::Spinner);
}

/// Executes multiple commands within a privileged shell using a spinner dialog.
///
/// The commands are passed as a single argument to `sh -c`, which 
/// is in turn launched by `pkexec`. This allows the use of pipes 
/// or logical operators (&&, ||) with root privileges.
///
pub fn commands_pkexec_spinner(
    parent: &adw::ApplicationWindow,
    commands: &str,
    title: &str,
    text: &str
) {
    let full_command = format!("pkexec sh -c \"{}\"", commands);
    show_spinner_dialog(parent, title, text, vec![full_command], IndicatorType::Spinner);
}

/// Executes a single privileged command with a progress indicator (Pulse).
///
/// Equivalent to [command_pkexec_spinner], but uses a `ProgressBar` instead of a spinner.
pub fn command_pkexec_progressbar(
    parent: &adw::ApplicationWindow,
    command: &str,
    title: &str,
    text: &str
) {
    let full_command = format!("pkexec {}", command);
    show_spinner_dialog(parent, title, text, vec![full_command], IndicatorType::ProgressBar);
}

/// Executes multiple privileged commands with a progress indicator (Pulse).
///
/// Equivalent to [commands_pkexec_spinner], but uses a `ProgressBar` instead of a spinner.
pub fn commands_pkexec_progressbar(
    parent: &adw::ApplicationWindow,
    commands: &str,
    title: &str,
    text: &str
) {
    let full_command = format!("pkexec sh -c \"{}\"", commands);
    show_spinner_dialog(parent, title, text, vec![full_command], IndicatorType::ProgressBar);
}
