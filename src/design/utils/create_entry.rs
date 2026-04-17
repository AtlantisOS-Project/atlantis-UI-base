//! Function that create an entry
/**
* create_entry.rs
*
* (C) Copyright 2026 AtlantisOS Project
* by @NachtsternBuild
*
* License: GNU GENERAL PUBLIC LICENSE Version 3
*/

use gtk4::prelude::*;
use gtk4::{Box as GtkBox, Entry, Label, Orientation, Align};

/// Function that create a textfield with label at a box
/// ### Usage:
///
/// ```rust
/// Usage:
/// // normal entry
/// let (username_row, username_entry) = create_entry(
///    "USername:", 
///     Some("Enter your username: ")
/// );
/// vbox.append(&username_row);
/// let input = username_entry.text(); 
/// println!("Input: {}", input);
/// ```
pub fn create_entry(label_text: &str, placeholder: Option<&str>) -> (GtkBox, Entry) {
    // create new box
    let box_container = GtkBox::builder()
        .orientation(Orientation::Horizontal)
        .spacing(6)
        .build();

    // create label and add text
    let label = Label::builder()
        .label(label_text)
        .halign(Align::Start)
        .valign(Align::Center)
        .margin_start(8)
        .margin_end(8)
        .build();

    // create entry
    let entry = Entry::builder()
        .hexpand(true)
        .placeholder_text(placeholder.unwrap_or(""))
        .build();

    // add widget to the box
    box_container.append(&label);
    box_container.append(&entry);

    // return the box and the entry
    (box_container, entry)
}

/// Function that create a password entry
/// ### Usage:
///
/// ```rust
/// Usage:
/// // password entry
/// let (password_row, password_entry) = create_password_entry(
///    "Password:", 
///    Some("Enter your password:")
/// );
/// vbox.append(&password_row);
/// let input = username_entry.text(); 
/// println!("Input: {}", input);
/// ```
pub fn create_password_entry(label_text: &str, placeholder: Option<&str>) -> (GtkBox, Entry) {
    // use create_entry to create the entry
    let (box_container, entry) = create_entry(label_text, placeholder);

    // apply settings for the password
    entry.set_visibility(false);
    entry.set_invisible_char(Some('*'));
	
	// return the box
    (box_container, entry)
}
