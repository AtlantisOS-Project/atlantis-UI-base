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
/// # Example
/// ```rust
/// command_pkexec_spinner(&window, vec!["fastboot".into(), "reboot".into()], "Rebooting", "Please wait...");
/// ```
pub fn command_pkexec_spinner(
    parent: &adw::ApplicationWindow,
    command: Vec<String>,
    title: &str,
    text: &str
) {
    if command.is_empty() { return; }

    // Fügt "pkexec" als auszuführendes Programm an den Anfang der Argumente ein
    let mut full_command = vec!["pkexec".to_string()];
    full_command.extend(command);

    show_spinner_dialog(
        parent, 
        title, 
        text, 
        vec![full_command], 
        IndicatorType::Spinner,
        None
    );
}

/// Executes multiple commands sequentially with root privileges using a spinner dialog.
///
/// Jedes Kommando wird sauber separiert übergeben. Polkit cached die Authentifizierung 
/// in der Regel für kurze Zeit, sodass der Nutzer nicht für jeden Befehl neu eingeben muss.
///
/// # Wichtiger Hinweis zu Shell-Features (Pipes, Verknüpfungen wie &&):
/// Da die Befehle direkt ausgeführt werden, interpretieren sie standardmäßig keine Shell-Logik.
/// Wenn du Pipes (`|`) oder `&&` benötigst, übergib die Shell explizit als Befehl, z. B.:
/// `vec!["sh".into(), "-c".into(), "echo 1 > /proc/sys/net/ipv4/ip_forward".into()]`
pub fn commands_pkexec_spinner(
    parent: &adw::ApplicationWindow,
    commands: Vec<Vec<String>>,
    title: &str,
    text: &str
) {
    // Verarbeite alle Befehle und stelle jedem ein "pkexec" voran
    let full_commands: Vec<Vec<String>> = commands
        .into_iter()
        .filter(|cmd| !cmd.is_empty())
        .map(|cmd| {
            let mut pkexec_cmd = vec!["pkexec".to_string()];
            pkexec_cmd.extend(cmd);
            pkexec_cmd
        })
        .collect();

    show_spinner_dialog(
        parent, 
        title, 
        text, 
        full_commands, 
        IndicatorType::Spinner,
        None
    );
}

/// Executes a single privileged command with a progress indicator (Pulse).
///
/// Equivalent to [command_pkexec_spinner], but uses a `ProgressBar` instead of a spinner.
pub fn command_pkexec_progressbar(
    parent: &adw::ApplicationWindow,
    command: Vec<String>,
    title: &str,
    text: &str
) {
    if command.is_empty() { return; }

    let mut full_command = vec!["pkexec".to_string()];
    full_command.extend(command);

    show_spinner_dialog(
        parent, 
        title, 
        text, 
        vec![full_command], 
        IndicatorType::ProgressBar,
        None
    );
}

/// Executes multiple privileged commands with a progress indicator (Pulse).
///
/// Equivalent to [commands_pkexec_spinner], but uses a `ProgressBar` instead of a spinner.
pub fn commands_pkexec_progressbar(
    parent: &adw::ApplicationWindow,
    commands: Vec<Vec<String>>,
    title: &str,
    text: &str
) {
    let full_commands: Vec<Vec<String>> = commands
        .into_iter()
        .filter(|cmd| !cmd.is_empty())
        .map(|cmd| {
            let mut pkexec_cmd = vec!["pkexec".to_string()];
            pkexec_cmd.extend(cmd);
            pkexec_cmd
        })
        .collect();

    show_spinner_dialog(
        parent, 
        title, 
        text, 
        full_commands, 
        IndicatorType::ProgressBar,
        None
    );
}
