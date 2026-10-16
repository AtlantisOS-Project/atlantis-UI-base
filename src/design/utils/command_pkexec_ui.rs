// command_pkexec_ui.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! Privileged commands with a progress dialog.
//!
//! The functions combine `pkexec` with the progress dialogs of
//! [`show_spinner_dialog`]: the command is started in the background with root privileges
//! (Polkit asks for the password), and a dialog with a spinner or a progress bar blocks the
//! window until the command is done.
//!
//! # Packaging
//!
//! `pkexec` has to be reachable: it works in deb, rpm and AppImage packages, but not inside
//! a Flatpak sandbox and not with Snap strict confinement. In those formats the command fails
//! and the error is shown by the dialog callback; do not offer such actions there.

use crate::design::dialogs::dialogs_spinner::{IndicatorType, show_spinner_dialog};
use gtk4::Widget;
use gtk4::prelude::IsA;

/// Prefixes every non-empty command with `pkexec`; empty commands are dropped.
fn with_pkexec(commands: Vec<Vec<String>>) -> Vec<Vec<String>> {
    commands
        .into_iter()
        .filter(|command| !command.is_empty())
        .map(|command| {
            let mut full_command = vec!["pkexec".to_string()];
            full_command.extend(command);
            full_command
        })
        .collect()
}

/// Runs one privileged command with a spinner dialog.
///
/// The Polkit authentication prompt opens when the dialog appears. An empty `command` does
/// nothing.
///
/// # Arguments
///
/// * `parent` - The window the dialog belongs to.
/// * `command` - The program followed by its arguments (without `pkexec`).
/// * `title` - The title of the dialog.
/// * `text` - The explanation for the user.
///
/// # Examples
///
/// ```no_run
/// use atlbase::ui_prelude::command_pkexec_spinner;
/// # gtk4::init().unwrap();
/// # let window = adw::ApplicationWindow::builder().build();
///
/// command_pkexec_spinner(&window, vec!["fastboot".into(), "reboot".into()], "Rebooting", "Please wait...");
/// ```
pub fn command_pkexec_spinner(
    parent: &impl IsA<Widget>,
    command: Vec<String>,
    title: &str,
    text: &str,
) {
    commands_pkexec_spinner(parent, vec![command], title, text);
}

/// Runs several privileged commands one after another with a spinner dialog.
///
/// Every command is started on its own with `pkexec`. Polkit usually remembers the
/// authentication for a short time, so the user does not have to enter the password for each
/// command. The chain stops at the first command that fails. Empty commands are skipped; if nothing is left, no dialog is shown.
///
/// The commands are not run by a shell, so pipes (`|`), redirections and `&&` do not work.
/// Start the shell explicitly if you need them:
/// `vec!["sh".into(), "-c".into(), "echo 1 > /proc/sys/net/ipv4/ip_forward".into()]`.
///
/// # Arguments
///
/// * `parent` - The window the dialog belongs to.
/// * `commands` - The commands, each as the program followed by its arguments.
/// * `title` - The title of the dialog.
/// * `text` - The explanation for the user.
///
/// # Examples
///
/// ```no_run
/// use atlbase::ui_prelude::commands_pkexec_spinner;
/// # gtk4::init().unwrap();
/// # let window = adw::ApplicationWindow::builder().build();
///
/// commands_pkexec_spinner(
///     &window,
///     vec![
///         vec!["apt".into(), "update".into()],
///         vec!["apt".into(), "upgrade".into(), "-y".into()],
///     ],
///     "Updating",
///     "Please wait...",
/// );
/// ```
pub fn commands_pkexec_spinner(
    parent: &impl IsA<Widget>,
    commands: Vec<Vec<String>>,
    title: &str,
    text: &str,
) {
    let commands = with_pkexec(commands);
    if commands.is_empty() {
        return;
    }
    show_spinner_dialog(parent, title, text, commands, IndicatorType::Spinner, None);
}

/// Runs one privileged command with a progress bar dialog.
///
/// Like [`command_pkexec_spinner`], but with a pulsing progress bar instead of a spinner.
///
/// # Arguments
///
/// * `parent` - The window the dialog belongs to.
/// * `command` - The program followed by its arguments (without `pkexec`).
/// * `title` - The title of the dialog.
/// * `text` - The explanation for the user.
///
/// # Examples
///
/// ```no_run
/// use atlbase::ui_prelude::command_pkexec_progressbar;
/// # gtk4::init().unwrap();
/// # let window = adw::ApplicationWindow::builder().build();
///
/// command_pkexec_progressbar(&window, vec!["apt".into(), "update".into()], "Updating", "Please wait...");
/// ```
pub fn command_pkexec_progressbar(
    parent: &impl IsA<Widget>,
    command: Vec<String>,
    title: &str,
    text: &str,
) {
    commands_pkexec_progressbar(parent, vec![command], title, text);
}

/// Runs several privileged commands one after another with a progress bar dialog.
///
/// Like [`commands_pkexec_spinner`], but with a pulsing progress bar instead of a spinner.
///
/// # Arguments
///
/// * `parent` - The window the dialog belongs to.
/// * `commands` - The commands, each as the program followed by its arguments.
/// * `title` - The title of the dialog.
/// * `text` - The explanation for the user.
///
/// # Examples
///
/// ```no_run
/// use atlbase::ui_prelude::commands_pkexec_progressbar;
/// # gtk4::init().unwrap();
/// # let window = adw::ApplicationWindow::builder().build();
///
/// commands_pkexec_progressbar(
///     &window,
///     vec![vec!["apt".into(), "update".into()], vec!["apt".into(), "autoremove".into()]],
///     "Cleaning up",
///     "Please wait...",
/// );
/// ```
pub fn commands_pkexec_progressbar(
    parent: &impl IsA<Widget>,
    commands: Vec<Vec<String>>,
    title: &str,
    text: &str,
) {
    let commands = with_pkexec(commands);
    if commands.is_empty() {
        return;
    }
    show_spinner_dialog(
        parent,
        title,
        text,
        commands,
        IndicatorType::ProgressBar,
        None,
    );
}

#[cfg(test)]
mod tests {
    use super::with_pkexec;

    #[test]
    fn every_command_gets_pkexec_and_empty_ones_are_dropped() {
        let commands = vec![vec!["ls".to_string(), "-l".to_string()], vec![]];
        assert_eq!(
            with_pkexec(commands),
            vec![vec![
                "pkexec".to_string(),
                "ls".to_string(),
                "-l".to_string()
            ]]
        );
    }
}
