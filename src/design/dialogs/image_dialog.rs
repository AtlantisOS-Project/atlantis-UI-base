// image_dialog.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! A dialog that shows an image.
//!
//! The module shows pictures in a Libadwaita dialog, for example a preview before a
//! wallpaper is set.

use adw::prelude::*;
use adw::{AlertDialog, ResponseAppearance};
use gtk4::{Align, Box as GtkBox, Image, Orientation, ScrolledWindow, Widget};
use std::path::Path;

/// The file extensions that [`show_image_dialog`] accepts.
const VALID_EXTENSIONS: &[&str] = &["jpg", "jpeg", "png", "svg", "webp", "gif"];

/// Shows a dialog with a preview of an image.
///
/// The image is embedded in an [`adw::AlertDialog`]. `width` and `height` set the initial
/// size of the preview; the image is placed in a scrolled window, so the dialog stays usable
/// with very large images.
///
/// The extension of the file is checked first: only `jpg`, `jpeg`, `png`, `svg`, `webp` and
/// `gif` (in any case) are accepted. For any other file the function returns **without
/// showing a dialog**.
///
/// # Arguments
///
/// * `parent` - The widget (for example the main window) the dialog is shown over.
/// * `title` - The heading of the dialog.
/// * `body` - A text below the heading.
/// * `button_label` - The label of the confirm button.
/// * `file_path` - The path of the image file.
/// * `width` - The width of the preview in pixels.
/// * `height` - The height of the preview in pixels.
///
/// # Examples
///
/// ```no_run
/// use atlbase::ui_prelude::show_image_dialog;
/// # gtk4::init().unwrap();
/// # let main_window = gtk4::Window::new();
///
/// show_image_dialog(
///     &main_window,
///     "Image preview",
///     "Do you want to set this image as your wallpaper?",
///     "Select",
///     "/home/user/pictures/wallpaper.jpg",
///     600,
///     400,
/// );
/// ```
pub fn show_image_dialog(
    parent: &impl IsA<Widget>,
    title: &str,
    body: &str,
    button_label: &str,
    file_path: &str,
    width: i32,
    height: i32,
) {
    let extension = Path::new(file_path)
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("")
        .to_lowercase();
    if !VALID_EXTENSIONS.contains(&extension.as_str()) {
        return;
    }

    let content = GtkBox::builder()
        .orientation(Orientation::Vertical)
        .spacing(12)
        .width_request(width)
        .build();

    let image = Image::from_file(file_path);
    image.set_pixel_size(width.max(height));
    image.set_vexpand(true);
    image.set_hexpand(true);
    image.set_halign(Align::Fill);
    image.set_valign(Align::Fill);

    let scroll = ScrolledWindow::builder()
        .child(&image)
        .min_content_height(height)
        .max_content_height(height)
        .propagate_natural_width(true)
        .build();
    content.append(&scroll);

    let dialog = AlertDialog::builder().heading(title).body(body).build();
    dialog.set_extra_child(Some(&content));
    dialog.add_response("ok", button_label);
    dialog.set_response_appearance("ok", ResponseAppearance::Suggested);
    dialog.set_close_response("ok");

    dialog.present(Some(parent));
}
