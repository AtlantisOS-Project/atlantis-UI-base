//! Functions to create special button
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

/// Create a button with label and callback
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

/// Create a button with icon and label
pub fn create_button_icon<F>(icon_name: &str, label_text: &str, callback: F) -> Button
where 
    F: Fn(&Button) + 'static
{
    let button = create_button_icon_no_callback(icon_name, label_text);
    button.connect_clicked(callback);
    button
}

/// Create a button with icon an dno callback
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

/// Create a button with two icons
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

/// Create a button with icon and special position
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
