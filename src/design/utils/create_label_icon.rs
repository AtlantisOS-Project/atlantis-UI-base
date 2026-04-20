//! Utility tools for creating combined icon-label widgets.
//!
//! This module allows you to quickly create horizontal groups consisting of an icon 
//! and text, which is particularly useful for status bars or informative 
//! list sections.
/**
* create_label_icon.rs
*
* (C) Copyright 2026 AtlantisOS Project
* by @NachtsternBuild
*
* License: GNU GENERAL PUBLIC LICENSE Version 3
*
* Usage:
* let terminal_info = create_label_icon("utilities-terminal", "Open Terminal");
* vbox.append(&terminal_info);
* let centered_info = create_label_icon_position(
*    "dialog-information", 
*    "System ready", 
*    gtk4::Align::Center
* );
* vbox.append(&centered_info);
*/

use gtk4::prelude::*;
use gtk4::{Align, Box as GtkBox, Image, Label, Orientation};

/// Creates a horizontal box with an icon and a label (left-aligned).
///
/// This function is a shortcut for [create_label_icon_position] with 
/// the default alignment `Align::Start`.
///
/// # Arguments
/// * `icon_name` - The name of the icon from the system theme (e.g., "utilities-terminal").
/// * `label_text` - The text to be displayed next to the icon.
///
/// Usage:
///
/// ```rust
/// let terminal_info = create_label_icon("utilities-terminal", "Open Terminal");
/// vbox.append(&terminal_info);
/// ```
pub fn create_label_icon(icon_name: &str, label_text: &str) -> GtkBox {
    // standard align start
    create_label_icon_position(
    	icon_name, 
    	label_text, 
    	Align::Start
    )
}

/// Creates a horizontal box with an icon and label, along with specific alignment.
///
/// Allows for precise positioning (e.g., centering) of the entire group 
/// within the available space.
///
/// # Arguments
/// * `icon_name` - The identifier of the icon.
/// * `label_text` - The text to display.
/// * `alignment` - The horizontal alignment of the box ([gtk4::Align]).
///
/// # Return Value
/// Returns a [GtkBox] containing both widgets, which can be inserted directly into the 
/// layout.
///
/// ```rust
/// let centered_info = create_label_icon_position(
///    "dialog-information", 
///    "System ready", 
///    gtk4::Align::Center
/// );
/// vbox.append(&centered_info);
/// ```
pub fn create_label_icon_position(
    icon_name: &str, 
    label_text: &str, 
    alignment: Align
) -> GtkBox {
    // create box
    let box_container = GtkBox::builder()
        .orientation(Orientation::Horizontal)
        .spacing(5)
        .halign(alignment)
        .build();

    // create icon
    let image = Image::from_icon_name(icon_name);

    // create label
    let label = Label::new(Some(label_text));

    // add widget
    box_container.append(&image);
    box_container.append(&label);

    box_container
}
