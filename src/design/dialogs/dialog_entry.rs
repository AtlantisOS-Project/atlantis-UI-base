//! Show a GTK4 dialog with Entry
/**
* dialogs_entry.rs
*
* (C) 2026 AtlantisOS Project
* by @NachtsternBuild
*
* License: GNU GENERAL PUBLIC LICENSE Version 3
*/

use adw::prelude::*;
use adw::{ActionRow, Dialog, HeaderBar, ToolbarView};
use gtk4::{Box as GtkBox, Button, Entry, Label, Orientation};

/// Show a Dialog with Entry
/// ### Usage:
/// 
/// ```rust
/// let btn = gtk4::Button::with_label("Input?");
/// btn.connect_clicked(move |b| {
///    show_entry_dialog(
///        b,
///        "Change Name",
///        "Add new name",
///        "Save",
///        "Exit",
///        "Username:",
///        "e.g. Test",
///        Box::new(|name| {
///            println!("Username {}", name);
///        }),
///    );
/// });
/// ```
pub fn show_entry_dialog<F>(
    parent: &impl IsA<gtk4::Widget>,
    title: &str,
    body: &str,
    ok_label: &str,
    cancel_label: &str,
    entry_label: &str,
    placeholder: &str,
    on_submit: F, // new callack
) -> Dialog
where
    F: Fn(String) + 'static, // F get a string and return nothing
{
    // ui setup
    let content = GtkBox::builder()
        .orientation(Orientation::Vertical)
        .spacing(12)
        .margin_top(18)
        .margin_bottom(18)
        .margin_start(18)
        .margin_end(18)
        .build();

    let body_label = Label::builder()
        .label(body)
        .wrap(true)
        .xalign(0.0)
        .build();
    content.append(&body_label);

    let entry = Entry::builder()
        .placeholder_text(placeholder)
        .hexpand(true)
        .activates_default(true) // enter start standard button
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
        .default_widget(&btn_ok) // connect to activates_default from the entry
        .build();

    // loic and callbacks    
    let dialog_clone = dialog.clone();
    btn_cancel.connect_clicked(move |_| {
        dialog_clone.close();
    });

    // referenz to the the entry and button
    let dialog_clone = dialog.clone();
    let entry_clone = entry.clone();
    

    // pack callback to reference counted
    let on_submit = std::rc::Rc::new(on_submit);
    let on_submit_clone = on_submit.clone();

    btn_ok.connect_clicked(move |_| {
        let input_text = entry_clone.text().to_string();
        on_submit_clone(input_text); // start callback
        dialog_clone.close();
    });

    dialog.present(Some(parent));
    dialog
}
