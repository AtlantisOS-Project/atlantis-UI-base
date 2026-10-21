// create_special_button.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! Buttons in the AtlantisOS style.
//!
//! The functions of this module create buttons that follow the AtlantisOS design (pill
//! shape, accent color) and support icons and alignment. All buttons get the CSS classes
//! `pill` and `suggested-action`.

use super::create_label_icon::icon_label_box;
use gtk4::prelude::*;
use gtk4::{Align, Button, Widget};

/// Creates a pill button with a text and a click handler.
///
/// # Arguments
///
/// * `label` - The text on the button.
/// * `callback` - Called when the button is clicked; it receives the button.
///
/// # Examples
///
/// ```no_run
/// use atlbase::ui_prelude::create_button;
/// # gtk4::init().unwrap();
///
/// let button = create_button("Click me", |_| println!("clicked"));
/// ```
pub fn create_button<F>(label: &str, callback: F) -> Button
where
    F: Fn(&Button) + 'static,
{
    let button = Button::with_label(label);
    style_pill(&button, true);
    button.connect_clicked(callback);
    button
}

/// Creates a pill button with an icon, a text and a click handler.
///
/// # Arguments
///
/// * `icon_name` - The name of the icon in the icon theme.
/// * `label_text` - The text next to the icon.
/// * `callback` - Called when the button is clicked; it receives the button.
///
/// # Examples
///
/// ```no_run
/// use atlbase::ui_prelude::create_button_icon;
/// # gtk4::init().unwrap();
///
/// let button = create_button_icon("document-save-symbolic", "Save", |_| println!("saved"));
/// ```
pub fn create_button_icon<F>(icon_name: &str, label_text: &str, callback: F) -> Button
where
    F: Fn(&Button) + 'static,
{
    let button = create_button_icon_no_callback(icon_name, label_text);
    button.connect_clicked(callback);
    button
}

/// Creates a pill button with an icon and a text, without a click handler.
///
/// Useful when the handler is connected later or depends on a condition.
///
/// # Arguments
///
/// * `icon_name` - The name of the icon in the icon theme.
/// * `label_text` - The text next to the icon.
///
/// # Examples
///
/// ```no_run
/// use atlbase::ui_prelude::create_button_icon_no_callback;
/// use gtk4::prelude::*;
/// # gtk4::init().unwrap();
///
/// let button = create_button_icon_no_callback("go-next-symbolic", "Next");
/// button.connect_clicked(|_| println!("next"));
/// ```
pub fn create_button_icon_no_callback(icon_name: &str, label_text: &str) -> Button {
    pill_button(&icon_label_box(&[icon_name], label_text, Align::Fill), true)
}

/// Creates a pill button with up to two icons, a text and a click handler.
///
/// Allows two symbols (for example a status icon and an arrow) in front of the text.
///
/// # Arguments
///
/// * `first_icon` - An optional first icon (`Some("name")` or `None`).
/// * `second_icon` - The main icon.
/// * `label_text` - The text of the button.
/// * `callback` - Called when the button is clicked; it receives the button.
///
/// # Examples
///
/// ```no_run
/// use atlbase::ui_prelude::create_button_two_icon;
/// # gtk4::init().unwrap();
///
/// let button = create_button_two_icon(
///     Some("emblem-ok-symbolic"),
///     "go-next-symbolic",
///     "Continue",
///     |_| println!("continue"),
/// );
/// ```
pub fn create_button_two_icon<F>(
    first_icon: Option<&str>,
    second_icon: &str,
    label_text: &str,
    callback: F,
) -> Button
where
    F: Fn(&Button) + 'static,
{
    let icons: Vec<&str> = first_icon.into_iter().chain([second_icon]).collect();
    let button = pill_button(&icon_label_box(&icons, label_text, Align::Fill), true);
    button.connect_clicked(callback);
    button
}

/// Creates a pill button with an icon and a text whose content has a given alignment.
///
/// The alignment applies to the icon-text group inside the button, for example to center
/// it or to move it to the left edge of a wide button.
///
/// # Arguments
///
/// * `icon_name` - The name of the icon in the icon theme.
/// * `label_text` - The text next to the icon.
/// * `alignment` - The horizontal alignment of the content.
/// * `callback` - Called when the button is clicked; it receives the button.
///
/// # Examples
///
/// ```no_run
/// use atlbase::ui_prelude::create_button_icon_position;
/// use gtk4::Align;
/// # gtk4::init().unwrap();
///
/// let button = create_button_icon_position(
///     "scanner-symbolic",
///     "Scan a QR code",
///     Align::Center,
///     |_| println!("scan"),
/// );
/// ```
pub fn create_button_icon_position<F>(
    icon_name: &str,
    label_text: &str,
    alignment: Align,
    callback: F,
) -> Button
where
    F: Fn(&Button) + 'static,
{
    let button = create_button_icon_no_callback(icon_name, label_text);

    if let Some(child) = button.child() {
        child.set_halign(alignment);
    }

    button.connect_clicked(callback);
    button
}

/// Gives a button the `pill` shape; `suggested` also gives it the accent color.
pub(crate) fn style_pill(button: &Button, suggested: bool) {
    button.add_css_class("pill");
    if suggested {
        button.add_css_class("suggested-action");
    }
}

/// A pill button around `child`.
pub(crate) fn pill_button(child: &impl IsA<Widget>, suggested: bool) -> Button {
    let button = Button::new();
    style_pill(&button, suggested);
    button.set_child(Some(child));
    button
}

/// A centered pill button of Libadwaita ([`adw::ButtonContent`]) with an optional icon.
pub(crate) fn pill_content_button(icon: Option<&str>, label: &str, suggested: bool) -> Button {
    let content = adw::ButtonContent::builder().label(label).build();
    if let Some(icon) = icon {
        content.set_icon_name(icon);
    }
    let button = pill_button(&content, suggested);
    button.set_halign(Align::Center);
    button
}
