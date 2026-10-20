// page_layout.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! The parts every page of a stack is made of: a centered container, a scrolling list with a
//! button at the bottom, headings, text and pill buttons.

use adw::prelude::*;
use gtk4::{Align, Box as GtkBox, Button, Label, Orientation, PolicyType, ScrolledWindow, pango};

/// The vertical container a menu page starts with: its content is centered, with space
/// around it.
pub fn create_page_container() -> GtkBox {
    GtkBox::builder()
        .orientation(Orientation::Vertical)
        .spacing(12)
        .valign(Align::Center)
        .halign(Align::Fill)
        .hexpand(true)
        .vexpand(true)
        .margin_top(24)
        .margin_bottom(24)
        .margin_start(24)
        .margin_end(24)
        .build()
}

/// The box for the rows of a [`create_scrolling_page`].
pub fn create_rows_box() -> GtkBox {
    GtkBox::builder()
        .orientation(Orientation::Vertical)
        .spacing(6)
        .halign(Align::Center)
        .margin_top(12)
        .build()
}

/// A page with a long list: `rows` scroll, `bottom` (usually a back button) stays at the
/// bottom.
///
/// # Examples
///
/// ```no_run
/// use atlbase::ui_prelude::*;
/// use gtk4::prelude::*;
/// # gtk4::init().unwrap();
///
/// let rows = create_rows_box();
/// rows.append(&create_heading("Devices"));
/// let page = create_scrolling_page(&rows, &create_pill_button("pan-start-symbolic", "Back", false, |_| {}));
/// ```
pub fn create_scrolling_page(rows: &GtkBox, bottom: &Button) -> GtkBox {
    let scrolled = ScrolledWindow::builder()
        .child(rows)
        .hexpand(true)
        .vexpand(true)
        .hscrollbar_policy(PolicyType::Never)
        .build();
    let page = GtkBox::builder()
        .orientation(Orientation::Vertical)
        .spacing(12)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    page.append(&scrolled);
    page.append(bottom);
    page
}

/// A heading (style `title-3`) for a list of rows.
pub fn create_heading(text: &str) -> Label {
    let label = Label::new(Some(text));
    label.add_css_class("title-3");
    label.set_margin_top(12);
    label
}

/// A left-aligned, wrapping label for the text of a page.
pub fn create_text_label(text: &str) -> Label {
    Label::builder()
        .label(text)
        .wrap(true)
        .wrap_mode(pango::WrapMode::WordChar)
        .halign(Align::Start)
        .xalign(0.0)
        .max_width_chars(70)
        .build()
}

/// A standard Libadwaita pill button with an icon. `suggested` marks the main action of a
/// page with the accent color.
pub fn create_pill_button(
    icon: &str,
    label: &str,
    suggested: bool,
    on_click: impl Fn(&Button) + 'static,
) -> Button {
    let content = adw::ButtonContent::builder()
        .icon_name(icon)
        .label(label)
        .build();
    let button = Button::builder()
        .child(&content)
        .halign(Align::Center)
        .css_classes(["pill"])
        .build();
    if suggested {
        button.add_css_class("suggested-action");
    }
    button.connect_clicked(on_click);
    button
}

/// A page with a long form or list that is not wider than `max_width` pixels, however wide the
/// window is: `content` is centered in a [`adw::Clamp`] and scrolls, `bottom` (usually a back
/// button) stays at the bottom.
///
/// Use it for pages made of [`adw::PreferencesGroup`]s, which look best in a narrow column.
///
/// # Examples
///
/// ```no_run
/// use atlbase::ui_prelude::*;
/// use gtk4::prelude::*;
/// # gtk4::init().unwrap();
///
/// let content = gtk4::Box::new(gtk4::Orientation::Vertical, 18);
/// content.append(&adw::PreferencesGroup::builder().title("Devices").build());
/// let back = create_pill_button("pan-start-symbolic", "Back", false, |_| {});
/// let page = create_clamped_scrolling_page(&content, &back, 620);
/// ```
pub fn create_clamped_scrolling_page(
    content: &impl IsA<gtk4::Widget>,
    bottom: &Button,
    max_width: i32,
) -> GtkBox {
    let rows = create_rows_box();
    rows.set_halign(Align::Fill);
    rows.append(
        &adw::Clamp::builder()
            .maximum_size(max_width)
            .child(content)
            .build(),
    );
    create_scrolling_page(&rows, bottom)
}
