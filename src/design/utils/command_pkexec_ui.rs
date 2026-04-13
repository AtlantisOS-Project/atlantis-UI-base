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

// run pkexec command with spinner
pub fn command_pkexec_spinner(
    parent: &adw::ApplicationWindow,
    command: &str,
    title: &str,
    text: &str
) {
    let full_command = format!("pkexec {}", command);
    show_spinner_dialog(parent, title, text, vec![full_command], IndicatorType::Spinner);
}

// run pkexec commands with spinner
pub fn commands_pkexec_spinner(
    parent: &adw::ApplicationWindow,
    commands: &str,
    title: &str,
    text: &str
) {
    let full_command = format!("pkexec sh -c \"{}\"", commands);
    show_spinner_dialog(parent, title, text, vec![full_command], IndicatorType::Spinner);
}

// run pkexec command with progressbar
pub fn command_pkexec_progressbar(
    parent: &adw::ApplicationWindow,
    command: &str,
    title: &str,
    text: &str
) {
    let full_command = format!("pkexec {}", command);
    show_spinner_dialog(parent, title, text, vec![full_command], IndicatorType::ProgressBar);
}

// run pkexec commands with progressbar
pub fn commands_pkexec_progressbar(
    parent: &adw::ApplicationWindow,
    commands: &str,
    title: &str,
    text: &str
) {
    let full_command = format!("pkexec sh -c \"{}\"", commands);
    show_spinner_dialog(parent, title, text, vec![full_command], IndicatorType::ProgressBar);
}
