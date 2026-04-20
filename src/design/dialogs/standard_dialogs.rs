//! Predefined standard dialogs for common use cases.
//!
//! This module provides wrapper functions for [show_alert_dialog] to display consistent 
//! informational and error messages with localized labels.
/**
* standard_dialogs.rs
*
* (C) Copyright 2026 AtlantisOS Project
* by @NachtsternBuild
*
* License: GNU GENERAL PUBLIC LICENSE Version 3
*/

use adw::prelude::*;
use gtk4::Widget;
use crate::design::dialogs::dialog::show_alert_dialog;
use crate::gettext;

/// Displays a standard information dialog.
///
/// Uses the static title "Information" and an "OK" button.
/// Both texts are automatically localized.
///
/// # Arguments
/// * `parent` - The parent widget.
/// * `body` - The message text to display.
pub fn show_info_dialog(
	parent: &impl IsA<Widget>, 
	body: &str
) {
    show_alert_dialog(
    	parent, 
    	&gettext!("Information"), 
    	body, 
    	&gettext!("OK")
    );
}

/// Displays an information dialog with a custom button.
///
/// Useful when the confirmation requires a verb other than "OK" (e.g., "Understood").
///
/// # Arguments
/// * `button_label` - The button label (should ideally already be localized).
pub fn show_info_button_dialog(
	parent: &impl IsA<Widget>, 
	body: &str, 
	button_label: &str
) {
    show_alert_dialog(
    	parent, 
    	&gettext!("Information"), 
    	body, 
    	button_label
    );
}

/// Displays an information dialog with a custom title and a default "OK" button.
pub fn show_dialog_title(
	parent: &impl IsA<Widget>, 
	title: &str, 
	body: &str
) {
    show_alert_dialog(
    	parent, 
    	title, 
    	body, 
    	&gettext!("OK")
    );
}

/// Displays a standard error dialog.
///
/// Uses the localized title "Error" and an "OK" button.
/// This dialog should be used for general program errors.
pub fn show_error_dialog(
	parent: &impl IsA<Widget>, 
	body: &str
) {
    show_alert_dialog(
    	parent, 
    	&gettext!("Error"), 
    	body, 
    	&gettext!("OK")
    );
}

/// Displays an error dialog with a custom button.
pub fn show_error_button_dialog(
	parent: &impl IsA<Widget>, 
	body: &str, 
	button_label: &str
) {
    show_alert_dialog(
    	parent, 
    	&gettext!("Error"), 
    	body, 
    	button_label
    );
}

/// Displays a detailed error dialog with a specific subtitle.
///
/// The title is formatted as "Error: {Your Title}".
///
/// # Example
/// If `title` is "Network Timeout", the heading will appear as "Error: Network Timeout".
pub fn show_error_title_dialog(
	parent: &impl IsA<Widget>, 
	title: &str, 
	body: &str
) {
    let formatted_title = format!("{}: {}", gettext!("Error"), title);
    show_alert_dialog(
    	parent, 
    	&formatted_title, 
    	body, 
    	&gettext!("OK")
    );
}

/// Displays a detailed error dialog with a custom title and a custom button.
///
/// This is the most flexible type of error dialog available in the standard presets.
pub fn show_error_title_button_dialog(
    parent: &impl IsA<Widget>, 
    title: &str, 
    body: &str, 
    button_label: &str
) {
    let formatted_title = format!("{}: {}", gettext!("Error"), title);
    show_alert_dialog(
    	parent, 
    	&formatted_title, 
    	body, 
    	button_label
    );
}
