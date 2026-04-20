//! Specialized buttons for the AtlantisOS user interface.
//!
//! This module provides tools for creating buttons that already 
//! comply with the AtlantisOS design guide (pill shape, accent colors) and 
//! support complex layouts such as icons.
/**
* create_special_button.rs
*
* (C) Copyright 2026 AtlantisOS Project
* by @NachtsternBuild
*
* License: GNU GENERAL PUBLIC LICENSE Version 3
*/

use gtk4::prelude::*;
use gtk4::{Align, Box as GtkBox, Button, Image, Label, Orientation};

/// Creates a standard button with text and a callback.
///
/// The button automatically receives the CSS classes `pill` and `suggested-action`.
///
/// # Arguments
/// * `label` - The text on the button.
/// * `callback` - The function that is executed when the button is clicked.
pub fn create_button<F>(label: &str, callback: F) -> Button 
where 
    F: Fn(&Button) + 'static 
{
    let button = Button::with_label(label);
    button.add_css_class("pill");             // pill form
    button.add_css_class("suggested-action");
    button.connect_clicked(callback);
    button
}

/// Creates a button with an icon and text without an initial callback.
///
/// Useful when the callback is to be assigned later or conditionally.
///
/// # Return value
/// A [gtk4::Button] in pill design with a [GtkBox] as a child element.
pub fn create_button_icon<F>(icon_name: &str, label_text: &str, callback: F) -> Button
where 
    F: Fn(&Button) + 'static
{
    let button = create_button_icon_no_callback(icon_name, label_text);
    button.connect_clicked(callback);
    button
}

/// Creates a button with up to two icons, text, and a callback.
///
/// Allows the combination of two symbols (e.g., status icon + arrow) 
/// before the label text.
///
/// # Arguments
/// * `first_icon` - An optional first icon (`Some(“name”)` or `None`).
/// * `second_icon` - The primary icon.
/// * `label_text` - The button text.
pub fn create_button_icon_no_callback(icon_name: &str, label_text: &str) -> Button {
    let button = Button::new();
    button.add_css_class("pill");             // pill form
    button.add_css_class("suggested-action");
    let content_box = GtkBox::builder()
        .orientation(Orientation::Horizontal)
        .spacing(5)
        .build();

    let image = Image::from_icon_name(icon_name);
    let label = Label::new(Some(label_text));

    content_box.append(&image);
    content_box.append(&label);
    
    button.set_child(Some(&content_box));
    button
}

/// Creates a button with up to two icons, text, and a callback.
///
/// Allows the combination of two symbols (e.g., status icon + arrow) 
/// before the label text.
///
/// # Arguments
/// * `first_icon` - An optional first icon (`Some(“name”)` or `None`).
/// * `second_icon` - The primary icon.
/// * `label_text` - The button text.
pub fn create_button_two_icon<F>(
    first_icon: Option<&str>, 
    second_icon: &str, 
    label_text: &str, 
    callback: F
) -> Button
where 
    F: Fn(&Button) + 'static
{
    let button = Button::new();
    button.add_css_class("pill");             // pill form
    button.add_css_class("suggested-action");
    let content_box = GtkBox::builder()
        .orientation(Orientation::Horizontal)
        .spacing(5)
        .build();

    if let Some(icon) = first_icon {
        content_box.append(&Image::from_icon_name(icon));
    }

    content_box.append(&Image::from_icon_name(second_icon));
    content_box.append(&Label::new(Some(label_text)));

    button.set_child(Some(&content_box));
    button.connect_clicked(callback);
    button
}

/// Creates an icon button with specific content alignment.
///
/// Allows you to align the content (icon + text) within the button,
/// for example, centered or to the left.
///
/// # Arguments
/// * `alignment` - The horizontal alignment of the content ([gtk4::Align]).
pub fn create_button_icon_position<F>(
    icon_name: &str, 
    label_text: &str, 
    alignment: Align,
    callback: F
) -> Button
where 
    F: Fn(&Button) + 'static
{
    let button = create_button_icon_no_callback(icon_name, label_text);
    
    // get the box and change the position
    if let Some(child) = button.child() {
        child.set_halign(alignment);
    }

    button.connect_clicked(callback);
    button
}
