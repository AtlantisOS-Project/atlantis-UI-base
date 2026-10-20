// status_row.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! A row that shows the result of a check.

use crate::gettext;
use adw::prelude::*;
use gtk4::Image;

/// A row that shows the result of a check: first a spinner and "Checking...", then a symbol
/// (a check mark or a warning) and a text.
///
/// Start the check in the background (see [`run_background`](super::background::run_background)),
/// add [`row`](Self::row) to a [`adw::PreferencesGroup`] and call [`set_result`](Self::set_result)
/// when the check is done.
///
/// # Examples
///
/// ```no_run
/// use adw::prelude::*;
/// use atlbase::ui_prelude::StatusRow;
/// # gtk4::init().unwrap();
///
/// let group = adw::PreferencesGroup::new();
/// let root = StatusRow::new("Root permission:");
/// group.add(&root.row);
/// // ... later, when the check is done:
/// root.set_result(true, "available");
/// ```
#[derive(Clone)]
pub struct StatusRow {
    /// The row; add it to a group.
    pub row: adw::ActionRow,
    spinner: adw::Spinner,
    icon: Image,
}

impl StatusRow {
    /// Creates a row with `title` that shows a spinner until the result is set.
    pub fn new(title: &str) -> Self {
        let spinner = adw::Spinner::new();
        let icon = Image::builder().visible(false).build();
        let row = adw::ActionRow::builder()
            .title(title)
            .subtitle(gettext!("Checking..."))
            .activatable(false)
            .build();
        row.add_suffix(&spinner);
        row.add_suffix(&icon);
        Self { row, spinner, icon }
    }

    /// Shows the result: a green check mark if `ok`, otherwise a yellow warning, and `text`
    /// as the subtitle. It can be called again to show a new result.
    pub fn set_result(&self, ok: bool, text: &str) {
        self.spinner.set_visible(false);
        self.icon.set_visible(true);
        self.icon.set_icon_name(Some(if ok {
            "object-select-symbolic"
        } else {
            "dialog-warning-symbolic"
        }));
        self.icon.remove_css_class("success");
        self.icon.remove_css_class("warning");
        self.icon
            .add_css_class(if ok { "success" } else { "warning" });
        self.row.set_subtitle(text);
    }

    /// Whether the last result was a success; `None` while the check is running.
    pub fn is_ok(&self) -> Option<bool> {
        if !self.icon.is_visible() {
            return None;
        }
        Some(self.icon.has_css_class("success"))
    }
}
