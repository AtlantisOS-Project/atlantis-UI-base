// standard_dialogs.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! Ready-made dialogs for common cases.
//!
//! The functions of this module show consistent messages. The four kinds can be told apart at
//! a glance:
//!
//! | Kind | Look |
//! |---|---|
//! | Information (`show_info_*`, [`show_dialog_title`]) | blue circle with an "i", neutral button |
//! | Success ([`show_success_dialog`]) | green circle with a check mark, green heading |
//! | Warning ([`show_warning_dialog`]) | yellow circle with an "!", yellow heading |
//! | Error (`show_error_*`) | red circle with an "!", red heading and red button |
//!
//! The mark in the circle is a letter or a character, not an icon from the icon theme: icon
//! themes draw "information" and "error" very differently (some even as a light bulb), and the
//! accent color of the user may be red. The kinds stay recognizable in every theme.
//!
//! The text of an error can be selected and copied, and a long text scrolls instead of making
//! the dialog huge. The dialogs follow the light/dark style and the accent color.
//!
//! The fixed texts ("Information", "Error", "OK") are translated with
//! [`gettext!`](crate::gettext!). Texts that you pass in are shown as they are: translate
//! them yourself. For a plain dialog without an icon use
//! [`show_alert_dialog`](super::dialog::show_alert_dialog).

use crate::design::theme::load_adw_provider;
use crate::gettext;
use crate::helper::platform::logging::excerpt;
use adw::Dialog;
use adw::prelude::*;
use gtk4::{
    Align, Box as GtkBox, Button, Justification, Label, Orientation, PolicyType, ScrolledWindow,
    Widget, pango,
};
use std::cell::Cell;

/// What a dialog tells the user; it decides the mark and the colors.
#[derive(Clone, Copy)]
enum Kind {
    Info,
    Success,
    Warning,
    Error,
}

impl Kind {
    /// The mark in the circle.
    fn glyph(self) -> &'static str {
        match self {
            Kind::Info => "i",
            Kind::Success => "\u{2713}", // check mark
            Kind::Warning | Kind::Error => "!",
        }
    }

    /// The CSS class of the colored circle around the mark.
    fn badge_class(self) -> &'static str {
        match self {
            Kind::Info => "atl-status-info",
            Kind::Success => "atl-status-success",
            Kind::Warning => "atl-status-warning",
            Kind::Error => "atl-status-error",
        }
    }

    /// The CSS classes of the heading (a color class for everything but information).
    fn heading_classes(self) -> &'static [&'static str] {
        match self {
            Kind::Info => &["title-2"],
            Kind::Success => &["title-2", "success"],
            Kind::Warning => &["title-2", "warning"],
            Kind::Error => &["title-2", "error"],
        }
    }

    /// The CSS classes of the button; only an error has a colored one.
    fn button_classes(self) -> &'static [&'static str] {
        match self {
            Kind::Error => &["pill", "atl-status-error-button"],
            _ => &["pill"],
        }
    }
}

/// The style of the status dialogs; uses only Libadwaita colors.
const STATUS_CSS: &str = r#"
.atl-status-badge {
    border-radius: 999px;
    min-width: 84px;
    min-height: 84px;
}
.atl-status-glyph { font-size: 46px; font-weight: 800; }
.atl-status-info    { background-color: alpha(@blue_3, 0.20);          color: @blue_3; }
.atl-status-success { background-color: alpha(@success_bg_color, 0.20); color: @success_color; }
.atl-status-warning { background-color: alpha(@warning_bg_color, 0.24); color: @warning_color; }
.atl-status-error   { background-color: alpha(@error_bg_color, 0.20);   color: @error_color; }
/* solid red, whatever the theme makes of `destructive-action` */
button.atl-status-error-button {
    background-image: none;
    background-color: @error_bg_color;
    color: @error_fg_color;
}
button.atl-status-error-button:hover {
    background-image: none;
    background-color: shade(@error_bg_color, 1.12);
}
"#;

thread_local! {
    static STATUS_STYLE: Cell<bool> = const { Cell::new(false) };
}

/// Builds a dialog with a big icon, a heading, a text and one button.
fn build_status_dialog(kind: Kind, heading: &str, body: &str, button_label: &str) -> Dialog {
    load_adw_provider::ensure_default_style(&STATUS_STYLE, STATUS_CSS);

    // the colored circle with the icon
    let badge = GtkBox::builder()
        .css_classes(["atl-status-badge", kind.badge_class()])
        .halign(Align::Center)
        .build();
    badge.append(
        &Label::builder()
            .label(kind.glyph())
            .css_classes(["atl-status-glyph"])
            .halign(Align::Center)
            .valign(Align::Center)
            .hexpand(true)
            .vexpand(true)
            .build(),
    );

    let heading_label = Label::builder()
        .label(heading)
        .wrap(true)
        .justify(Justification::Center)
        .css_classes(kind.heading_classes())
        .build();

    // the text can be copied; a long text (for example the output of a command) scrolls
    let body_label = Label::builder()
        .label(body)
        .wrap(true)
        .wrap_mode(pango::WrapMode::WordChar)
        .justify(Justification::Center)
        .max_width_chars(48)
        .selectable(true)
        .build();
    // do not select the whole text when the dialog opens
    body_label.set_focusable(false);
    let body_scroll = ScrolledWindow::builder()
        .child(&body_label)
        .hscrollbar_policy(PolicyType::Never)
        .propagate_natural_height(true)
        .max_content_height(240)
        .build();

    let button = Button::builder()
        .label(button_label)
        .halign(Align::Center)
        .width_request(160)
        .margin_top(12)
        .css_classes(kind.button_classes())
        .build();

    let content = GtkBox::builder()
        .orientation(Orientation::Vertical)
        .spacing(12)
        .margin_top(30)
        .margin_bottom(24)
        .margin_start(24)
        .margin_end(24)
        .build();
    content.append(&badge);
    content.append(&heading_label);
    content.append(&body_scroll);
    content.append(&button);

    let dialog = Dialog::builder()
        .title(heading)
        .content_width(380)
        .child(&content)
        .default_widget(&button)
        .focus_widget(&button)
        .build();

    let weak = dialog.downgrade();
    button.connect_clicked(move |_| {
        if let Some(dialog) = weak.upgrade() {
            dialog.close();
        }
    });
    dialog
}

/// Shows a status dialog over `parent`.
fn show_status_dialog(
    parent: &impl IsA<Widget>,
    kind: Kind,
    heading: &str,
    body: &str,
    button_label: &str,
) {
    // what the user is told is part of the log: a bug report with the log shows the error dialog.
    // Only the beginning: the text can be the output of a command.
    let shown = excerpt(body);
    match kind {
        Kind::Info => log::info!("dialog '{heading}': {shown}"),
        Kind::Success => log::info!("success dialog '{heading}': {shown}"),
        Kind::Warning => log::warn!("warning dialog '{heading}': {shown}"),
        Kind::Error => log::error!("error dialog '{heading}': {shown}"),
    }
    build_status_dialog(kind, heading, body, button_label).present(Some(parent));
}

/// Shows an information dialog.
///
/// The title is "Information" and the button is "OK" (both translated).
///
/// # Arguments
///
/// * `parent` - The widget the dialog is shown over.
/// * `body` - The message.
///
/// # Examples
///
/// ```no_run
/// use atlbase::ui_prelude::show_info_dialog;
/// # gtk4::init().unwrap();
/// # let window = gtk4::Window::new();
///
/// show_info_dialog(&window, "The file was saved.");
/// ```
pub fn show_info_dialog(parent: &impl IsA<Widget>, body: &str) {
    show_status_dialog(
        parent,
        Kind::Info,
        &gettext!("Information"),
        body,
        &gettext!("OK"),
    );
}

/// Shows an information dialog with your own button label.
///
/// Useful when "OK" does not fit, for example "Understood".
///
/// # Arguments
///
/// * `parent` - The widget the dialog is shown over.
/// * `body` - The message.
/// * `button_label` - The label of the button (translate it yourself).
///
/// # Examples
///
/// ```no_run
/// use atlbase::ui_prelude::show_info_button_dialog;
/// # gtk4::init().unwrap();
/// # let window = gtk4::Window::new();
///
/// show_info_button_dialog(&window, "The update takes a few minutes.", "Understood");
/// ```
pub fn show_info_button_dialog(parent: &impl IsA<Widget>, body: &str, button_label: &str) {
    show_status_dialog(
        parent,
        Kind::Info,
        &gettext!("Information"),
        body,
        button_label,
    );
}

/// Shows an information dialog with your own title and the button "OK".
///
/// # Arguments
///
/// * `parent` - The widget the dialog is shown over.
/// * `title` - The heading (translate it yourself).
/// * `body` - The message.
///
/// # Examples
///
/// ```no_run
/// use atlbase::ui_prelude::show_dialog_title;
/// # gtk4::init().unwrap();
/// # let window = gtk4::Window::new();
///
/// show_dialog_title(&window, "Backup", "The backup is complete.");
/// ```
pub fn show_dialog_title(parent: &impl IsA<Widget>, title: &str, body: &str) {
    show_status_dialog(parent, Kind::Info, title, body, &gettext!("OK"));
}

/// Shows a success dialog: a green circle with a check mark.
///
/// The title is "Success" and the button is "OK" (both translated). Use it to confirm that
/// something worked, for example after saving or installing.
///
/// # Arguments
///
/// * `parent` - The widget the dialog is shown over.
/// * `body` - The message.
///
/// # Examples
///
/// ```no_run
/// use atlbase::ui_prelude::show_success_dialog;
/// # gtk4::init().unwrap();
/// # let window = gtk4::Window::new();
///
/// show_success_dialog(&window, "The update was installed.");
/// ```
pub fn show_success_dialog(parent: &impl IsA<Widget>, body: &str) {
    show_status_dialog(
        parent,
        Kind::Success,
        &gettext!("Success"),
        body,
        &gettext!("OK"),
    );
}

/// Shows a warning dialog: a yellow circle with an exclamation mark.
///
/// The title is "Warning" and the button is "OK" (both translated). Use it for something that
/// needs care but is not an error, for example before data is changed.
///
/// # Arguments
///
/// * `parent` - The widget the dialog is shown over.
/// * `body` - The message.
///
/// # Examples
///
/// ```no_run
/// use atlbase::ui_prelude::show_warning_dialog;
/// # gtk4::init().unwrap();
/// # let window = gtk4::Window::new();
///
/// show_warning_dialog(&window, "The battery is low. Plug in the charger.");
/// ```
pub fn show_warning_dialog(parent: &impl IsA<Widget>, body: &str) {
    show_status_dialog(
        parent,
        Kind::Warning,
        &gettext!("Warning"),
        body,
        &gettext!("OK"),
    );
}

/// Shows a standard error dialog.
///
/// The title is "Error" and the button is "OK" (both translated). Use it for general
/// errors of the program.
///
/// # Arguments
///
/// * `parent` - The widget the dialog is shown over.
/// * `body` - The description of the error.
///
/// # Examples
///
/// ```no_run
/// use atlbase::ui_prelude::show_error_dialog;
/// # gtk4::init().unwrap();
/// # let window = gtk4::Window::new();
///
/// show_error_dialog(&window, "The file could not be opened.");
/// ```
pub fn show_error_dialog(parent: &impl IsA<Widget>, body: &str) {
    show_status_dialog(
        parent,
        Kind::Error,
        &gettext!("Error"),
        body,
        &gettext!("OK"),
    );
}

/// Shows an error dialog with your own button label.
///
/// # Arguments
///
/// * `parent` - The widget the dialog is shown over.
/// * `body` - The description of the error.
/// * `button_label` - The label of the button (translate it yourself).
///
/// # Examples
///
/// ```no_run
/// use atlbase::ui_prelude::show_error_button_dialog;
/// # gtk4::init().unwrap();
/// # let window = gtk4::Window::new();
///
/// show_error_button_dialog(&window, "The connection was lost.", "Close");
/// ```
pub fn show_error_button_dialog(parent: &impl IsA<Widget>, body: &str, button_label: &str) {
    show_status_dialog(parent, Kind::Error, &gettext!("Error"), body, button_label);
}

/// Shows an error dialog with a detail in the title.
///
/// The heading is "Error: *title*" (the word "Error" is translated), the button is "OK".
///
/// # Arguments
///
/// * `parent` - The widget the dialog is shown over.
/// * `title` - The detail after "Error:", for example `"Network timeout"`.
/// * `body` - The description of the error.
///
/// # Examples
///
/// ```no_run
/// use atlbase::ui_prelude::show_error_title_dialog;
/// # gtk4::init().unwrap();
/// # let window = gtk4::Window::new();
///
/// // the heading reads "Error: Network timeout"
/// show_error_title_dialog(&window, "Network timeout", "The server did not answer.");
/// ```
pub fn show_error_title_dialog(parent: &impl IsA<Widget>, title: &str, body: &str) {
    show_error_title_button_dialog(parent, title, body, &gettext!("OK"));
}

/// Shows an error dialog with a detail in the title and your own button label.
///
/// The most flexible of the error presets.
///
/// # Arguments
///
/// * `parent` - The widget the dialog is shown over.
/// * `title` - The detail after "Error:".
/// * `body` - The description of the error.
/// * `button_label` - The label of the button (translate it yourself).
///
/// # Examples
///
/// ```no_run
/// use atlbase::ui_prelude::show_error_title_button_dialog;
/// # gtk4::init().unwrap();
/// # let window = gtk4::Window::new();
///
/// show_error_title_button_dialog(&window, "Disk full", "Free some space and try again.", "Close");
/// ```
pub fn show_error_title_button_dialog(
    parent: &impl IsA<Widget>,
    title: &str,
    body: &str,
    button_label: &str,
) {
    let formatted_title = format!("{}: {}", gettext!("Error"), title);
    show_status_dialog(parent, Kind::Error, &formatted_title, body, button_label);
}
