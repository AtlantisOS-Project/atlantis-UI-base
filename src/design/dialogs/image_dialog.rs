//! # Image Dialog Module
//! This module provides helper functions for displaying media content in
//! Libadwaita dialogs.
/**
* image_dialog.rs
*
* (C) Copyright 2026 AtlantisOS Project
* by @NachtsternBuild
*
* License: GNU GENERAL PUBLIC LICENSE Version 3
*/
use adw::prelude::*;
use adw::{AlertDialog, ResponseAppearance};
use gtk4::{Widget, Box as GtkBox, Orientation, Image, Align, ScrolledWindow};
use std::path::Path;

/// Displays a highly configurable Libadwaita dialog with an image preview.
///
/// This function validates the file path, loads the image, and embeds it in a 
/// [`adw::AlertDialog`]. Specifying `width` and `height` controls 
/// the initial size of the dialog.
///
/// # Arguments
///
/// * `parent` - The parent widget (e.g., the main window) to which the dialog belongs modally.
/// * `title` - The main title of the dialog.
/// * `body` - An explanatory text below the title.
/// * `button_label` - The label of the confirmation button.
/// * `file_path` - The path to the image file on the file system.
/// * `width` - The target width of the image/dialog in pixels.
/// * `height` - The target height of the image/dialog in pixels.
///
/// # Examples
///
/// ```rust
/// use your_crate::show_image_dialog;
///
/// // A medium-sized dialog with a JPG image
/// show_image_dialog(
///     &main_window,
///     "Image Preview",
///     "Would you like to set this image as your wallpaper?",
///     "Select",
///     "/home/user/pictures/wallpaper.jpg",
///     600,
///     400
/// );
/// ```
///
/// # Validation
/// The function internally checks for the following file extensions: `jpg`, `jpeg`, `png`, `svg`, `webp`, `gif`.
/// If the format is not supported or the path is invalid, the function 
/// terminates early without displaying a dialog.
///
/// # UI Structure
/// Within the dialog, the image size is managed via a `GtkScrolledWindow`, 
/// so that the UI remains usable even with large images.
pub fn show_image_dialog(
	parent: &impl IsA<Widget>, 
    title: &str, 
    body: &str, 
    button_label: &str,
    file_path: &str,
    width: i32,
    height: i32,
) {
    // validation of the path
    let path = Path::new(file_path);
    let extension = path.extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_lowercase();
	
	// validation of the file format
    let valid_extensions = ["jpg", "jpeg", "png", "svg", "webp", "gif"];
    if !valid_extensions.contains(&extension.as_str()) {
        return;
    }

    // new container
    let content = GtkBox::builder()
        .orientation(Orientation::Vertical)
        .spacing(12)
        .width_request(width) 
        .build();

    // get the image from the file path
    let image = Image::from_file(file_path);
    
    // set the size of the image
    image.set_pixel_size(width.max(height)); 
    
    image.set_vexpand(true);
    image.set_hexpand(true);
    image.set_halign(Align::Fill);
    image.set_valign(Align::Fill);

    // put everything in a scrollable window to accommodate very large images
    let scroll = ScrolledWindow::builder()
        .child(&image)
        .min_content_height(height)
        .max_content_height(height)
        .propagate_natural_width(true)
        .build();

    content.append(&scroll);

    // dialog setup
    let dialog = AlertDialog::builder()
        .heading(title)
        .body(body)
        .build();
	
	// set the image as additional content
    dialog.set_extra_child(Some(&content));
    dialog.add_response("ok", button_label);
    dialog.set_response_appearance("ok", ResponseAppearance::Suggested);
    dialog.set_close_response("ok");

    dialog.present(Some(parent));
}
