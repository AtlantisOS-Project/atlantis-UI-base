// create_label_icon.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! Widgets that combine an icon and a text.
//!
//! The functions of this module create horizontal groups of an icon and a label, which are
//! useful for status information and informative list entries.

use gtk4::prelude::*;
use gtk4::{Align, Box as GtkBox, Image, Label, Orientation};

/// Creates a horizontal box with an icon and a label, aligned to the start.
///
/// Shortcut for [`create_label_icon_position`] with [`Align::Start`].
///
/// # Arguments
///
/// * `icon_name` - The name of the icon in the icon theme, for example
///   `"utilities-terminal"`.
/// * `label_text` - The text next to the icon.
///
/// # Examples
///
/// ```no_run
/// use atlbase::ui_prelude::create_label_icon;
/// use gtk4::prelude::*;
/// # gtk4::init().unwrap();
/// # let vbox = gtk4::Box::new(gtk4::Orientation::Vertical, 6);
///
/// let terminal_info = create_label_icon("utilities-terminal", "Open Terminal");
/// vbox.append(&terminal_info);
/// ```
pub fn create_label_icon(icon_name: &str, label_text: &str) -> GtkBox {
    create_label_icon_position(icon_name, label_text, Align::Start)
}

/// Creates a horizontal box with an icon and a label and places it with `alignment`.
///
/// The alignment applies to the whole group inside the available space, so it can be
/// centered, for example.
///
/// # Arguments
///
/// * `icon_name` - The name of the icon in the icon theme.
/// * `label_text` - The text next to the icon.
/// * `alignment` - The horizontal alignment of the group.
///
/// # Examples
///
/// ```no_run
/// use atlbase::ui_prelude::create_label_icon_position;
/// use gtk4::{Align, prelude::*};
/// # gtk4::init().unwrap();
/// # let vbox = gtk4::Box::new(gtk4::Orientation::Vertical, 6);
///
/// let centered_info = create_label_icon_position("dialog-information", "System ready", Align::Center);
/// vbox.append(&centered_info);
/// ```
pub fn create_label_icon_position(icon_name: &str, label_text: &str, alignment: Align) -> GtkBox {
    let box_container = GtkBox::builder()
        .orientation(Orientation::Horizontal)
        .spacing(5)
        .halign(alignment)
        .build();

    box_container.append(&Image::from_icon_name(icon_name));
    box_container.append(&Label::new(Some(label_text)));

    box_container
}
