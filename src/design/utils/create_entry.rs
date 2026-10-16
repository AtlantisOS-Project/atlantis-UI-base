// create_entry.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! Input fields with a label.
//!
//! The functions of this module group a text or password input directly with a `Label` in
//! a horizontal container.

use gtk4::prelude::*;
use gtk4::{Align, Box as GtkBox, Entry, Label, Orientation};

/// Creates a text input field with a label in front of it.
///
/// A [`Label`] and an [`Entry`] are placed in a horizontal [`gtk4::Box`]. The field uses all
/// the horizontal space that is left.
///
/// # Arguments
///
/// * `label_text` - The text to the left of the field.
/// * `placeholder` - Optional gray text inside the empty field.
///
/// # Returns
///
/// A tuple of
///
/// 1. the container that is added to the layout, and
/// 2. the [`Entry`] to connect signals to or to read the text from.
///
/// # Examples
///
/// ```no_run
/// use atlbase::ui_prelude::create_entry;
/// use gtk4::prelude::*;
/// # gtk4::init().unwrap();
/// # let vbox = gtk4::Box::new(gtk4::Orientation::Vertical, 6);
///
/// let (username_row, username_entry) = create_entry("Username:", Some("Enter your username"));
/// vbox.append(&username_row);
///
/// username_entry.connect_activate(|entry| println!("Input: {}", entry.text()));
/// ```
pub fn create_entry(label_text: &str, placeholder: Option<&str>) -> (GtkBox, Entry) {
    let box_container = GtkBox::builder()
        .orientation(Orientation::Horizontal)
        .spacing(6)
        .build();

    let label = Label::builder()
        .label(label_text)
        .halign(Align::Start)
        .valign(Align::Center)
        .margin_start(8)
        .margin_end(8)
        .build();

    let entry = Entry::builder()
        .hexpand(true)
        .placeholder_text(placeholder.unwrap_or(""))
        .build();

    box_container.append(&label);
    box_container.append(&entry);

    (box_container, entry)
}

/// Creates a password input field with a label in front of it.
///
/// Works like [`create_entry`], but the typed characters are hidden (shown as `*`).
///
/// # Arguments
///
/// * `label_text` - The text to the left of the field, for example `"Password:"`.
/// * `placeholder` - Optional gray text inside the empty field.
///
/// # Examples
///
/// ```no_run
/// use atlbase::ui_prelude::create_password_entry;
/// use gtk4::prelude::*;
/// # gtk4::init().unwrap();
/// # let vbox = gtk4::Box::new(gtk4::Orientation::Vertical, 6);
///
/// let (password_row, password_entry) = create_password_entry("Password:", None);
/// vbox.append(&password_row);
///
/// let secret = password_entry.text();
/// # let _ = secret;
/// ```
pub fn create_password_entry(label_text: &str, placeholder: Option<&str>) -> (GtkBox, Entry) {
    let (box_container, entry) = create_entry(label_text, placeholder);

    entry.set_visibility(false);
    entry.set_invisible_char(Some('*'));

    (box_container, entry)
}
