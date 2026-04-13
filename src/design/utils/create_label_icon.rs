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

/**
* @brief Function that create label with an icon
*/
pub fn create_label_icon(icon_name: &str, label_text: &str) -> GtkBox {
    // standard align start
    create_label_icon_position(
    	icon_name, 
    	label_text, 
    	Align::Start
    )
}

/**
* @brief Function that create a boc with icon label and special position
*/
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
