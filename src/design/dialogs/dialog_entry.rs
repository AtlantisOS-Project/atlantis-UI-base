// dialog_entry.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! A Libadwaita dialog for text input.
//!
//! The module provides a standard dialog with a text field that fits the AtlantisOS design.

use adw::prelude::*;
use adw::{ActionRow, Dialog, HeaderBar, ToolbarView};
use glib::clone;
use gtk4::{Box as GtkBox, Button, Entry, Label, Orientation};

/// Shows a dialog with an input field ([`gtk4::Entry`]).
///
/// The dialog is built on an [`adw::ToolbarView`] and uses the "pill" style for the buttons
/// and the field.
///
/// * The input is confirmed with the `ok_label` button or the Enter key; then `on_submit`
///   is called with the text and the dialog closes.
/// * The `cancel_label` button closes the dialog without calling `on_submit`.
///
/// # Arguments
///
/// * `parent` - The widget the dialog is shown over.
/// * `title` - The title in the header bar.
/// * `body` - An explanation above the input field.
/// * `ok_label` - The text of the confirm button, for example `"Save"`.
/// * `cancel_label` - The text of the cancel button, for example `"Cancel"`.
/// * `entry_label` - The title of the row that contains the field.
/// * `placeholder` - Gray text inside the empty field.
/// * `on_submit` - Receives the entered text.
///
/// # Returns
///
/// The [`adw::Dialog`], in case you want to connect more signals.
///
/// # Examples
///
/// ```no_run
/// use atlbase::ui_prelude::show_entry_dialog;
/// use gtk4::prelude::*;
/// # gtk4::init().unwrap();
///
/// let button = gtk4::Button::with_label("Rename");
/// button.connect_clicked(|button| {
///     show_entry_dialog(
///         button,
///         "Change name",
///         "Enter the new name.",
///         "Save",
///         "Cancel",
///         "Username:",
///         "e.g. Test",
///         |name| println!("Username: {}", name),
///     );
/// });
/// ```
#[allow(clippy::too_many_arguments)]
pub fn show_entry_dialog<F>(
    parent: &impl IsA<gtk4::Widget>,
    title: &str,
    body: &str,
    ok_label: &str,
    cancel_label: &str,
    entry_label: &str,
    placeholder: &str,
    on_submit: F,
) -> Dialog
where
    F: Fn(String) + 'static,
{
    let content = GtkBox::builder()
        .orientation(Orientation::Vertical)
        .spacing(12)
        .margin_top(18)
        .margin_bottom(18)
        .margin_start(18)
        .margin_end(18)
        .build();

    content.append(&Label::builder().label(body).wrap(true).xalign(0.0).build());

    let entry = Entry::builder()
        .placeholder_text(placeholder)
        .hexpand(true)
        .activates_default(true) // Enter presses the default button
        .build();
    entry.add_css_class("pill");

    let row = ActionRow::builder()
        .title(entry_label)
        .activatable_widget(&entry)
        .build();
    row.add_suffix(&entry);
    content.append(&row);

    let toolbar_view = ToolbarView::new();
    toolbar_view.set_content(Some(&content));

    let header_bar = HeaderBar::builder()
        .title_widget(&adw::WindowTitle::new(title, ""))
        .build();

    let btn_cancel = Button::with_label(cancel_label);
    btn_cancel.add_css_class("destructive-action");
    btn_cancel.add_css_class("pill");
    header_bar.pack_start(&btn_cancel);

    let btn_ok = Button::with_label(ok_label);
    btn_ok.add_css_class("suggested-action");
    btn_ok.add_css_class("pill");
    header_bar.pack_end(&btn_ok);

    toolbar_view.add_top_bar(&header_bar);

    let dialog = Dialog::builder()
        .child(&toolbar_view)
        .content_width(400)
        .default_widget(&btn_ok) // the target of `activates_default`
        .build();

    // weak references: the buttons live inside the dialog, a strong one would be a cycle
    btn_cancel.connect_clicked(clone!(
        #[weak]
        dialog,
        move |_| {
            dialog.close();
        }
    ));

    btn_ok.connect_clicked(clone!(
        #[weak]
        dialog,
        move |_| {
            on_submit(entry.text().to_string());
            dialog.close();
        }
    ));

    dialog.present(Some(parent));
    dialog
}
