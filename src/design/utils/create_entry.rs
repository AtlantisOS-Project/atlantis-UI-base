//! Utility tools for quickly creating input fields.
//!
//! This module provides functions for grouping text and password inputs directly with 
//! a labeled `Label` in a horizontal container.
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

/// Creates a text input field with a preceding label.
///
/// This function combines a `Label` and an `Entry` in a horizontal `GtkBox`.
/// The input field is configured to fill the available horizontal space.
///
/// # Return Value
/// Returns a tuple:
/// 1. `GtkBox`: The container that is inserted into the UI layout (e.g., a `VBox`).
/// 2. `Entry`: The actual input widget for connecting signals or retrieving text.
///
/// # Arguments
/// * `label_text` - The text displayed to the left of the field.
/// * `placeholder` - An optional placeholder text within the field.
///
/// # Usage:
///
/// ```rust
/// Usage:
/// // normal entry
/// let (username_row, username_entry) = create_entry(
///    "Username:", 
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

/// Creates a password-protected input field with a label.
///
/// This function internally uses [create_entry], but configures the `Entry`
/// so that the input remains hidden (default character: `*`).
///
/// # Arguments
/// * `label_text` - The text for the label (e.g., "Password:").
/// * `placeholder` - Optional placeholder text (e.g., "••••••••").
///
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
