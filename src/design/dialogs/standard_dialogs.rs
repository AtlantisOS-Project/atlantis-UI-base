/**
* standard_dialogs.rs
*
* (C) Copyright 2026 AtlantisOS Project
* by @NachtsternBuild
*
* License: GNU GENERAL PUBLIC LICENSE Version 3
*
* @brief Multiple dialogs for special operations
*/

use adw::prelude::*;
use gtk4::Widget;
use crate::design::dialogs::dialog::show_alert_dialog;
use crate::gettext;

// show dialog with standard title
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

// show dialog with individual text button
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

// show dialog with individual title and standard button
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

// show standard error dialog
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

// show error dialog with special button
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

// show a error dialog with extra title
pub fn show_error_title_dialog(
	parent: &impl IsA<Widget>, 
	title: &str, 
	body: &str
) {
    let formatted_title = format!("{}: {}", gettext!("Error"), title);
    show_alert_dialog(
    	parent, 
    	&formatted_title, 
    	body, &gettext!("OK")
    );
}

// show a error dialog with title and spcial button
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
