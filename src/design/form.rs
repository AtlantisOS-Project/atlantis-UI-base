// form.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! Dialogs with a form.
//!
//! A [`FormDialog`] is a dialog with the buttons "Cancel" and an action ("Start", "Flash", …)
//! and rows in between: drop-down menus ([`create_combo_row`]), switches
//! ([`create_switch_row`]), text fields and the choice of files and folders ([`PickRow`]).
//! The program fills the rows and reads them when the user presses the action button.
//!
//! The titles you pass in are shown as they are: translate them yourself. The fixed texts
//! ("Cancel", "Choose…", "Nothing chosen") are translated by the library.

use crate::design::chooser::choose::show_chooser;
use crate::gettext;
use adw::prelude::*;
use gtk4::{
    Align, Box as GtkBox, Button, Orientation, PolicyType, ScrolledWindow, StringList, Widget,
};
use std::cell::{Cell, RefCell};
use std::path::PathBuf;
use std::rc::Rc;

/// A dialog with a form.
///
/// # Examples
///
/// ```no_run
/// use adw::prelude::*;
/// use atlbase::ui_prelude::*;
/// # gtk4::init().unwrap();
/// # let window = gtk4::Window::new();
///
/// let form = FormDialog::new("Backup", "Start");
/// let group = form.group(Some("Partition"));
/// let partition = create_combo_row("Partition", &["boot".to_string(), "vendor".to_string()]);
/// group.add(&partition);
///
/// form.on_ok(move || println!("chosen: {}", selected_index(&partition)));
/// form.present(&window);
/// ```
pub struct FormDialog {
    dialog: adw::Dialog,
    ok: Button,
    body: GtkBox,
}

impl FormDialog {
    /// Creates the dialog; `ok_label` is the text of the main button.
    pub fn new(title: &str, ok_label: &str) -> Self {
        let cancel = Button::with_label(&gettext!("Cancel"));
        let ok = Button::builder()
            .label(ok_label)
            .css_classes(["suggested-action"])
            .build();

        let header = adw::HeaderBar::builder()
            .show_start_title_buttons(false)
            .show_end_title_buttons(false)
            .build();
        header.pack_start(&cancel);
        header.pack_end(&ok);

        let body = GtkBox::builder()
            .orientation(Orientation::Vertical)
            .spacing(18)
            .margin_top(12)
            .margin_bottom(24)
            .margin_start(18)
            .margin_end(18)
            .build();
        let scrolled = ScrolledWindow::builder()
            .hscrollbar_policy(PolicyType::Never)
            .propagate_natural_height(true)
            .max_content_height(440)
            .child(&body)
            .build();

        let toolbar = adw::ToolbarView::new();
        toolbar.add_top_bar(&header);
        toolbar.set_content(Some(&scrolled));

        let dialog = adw::Dialog::builder()
            .title(title)
            .content_width(520)
            .follows_content_size(true)
            .child(&toolbar)
            .build();
        // weak: the button is part of the dialog, a strong reference would keep both alive
        cancel.connect_clicked({
            let dialog = dialog.downgrade();
            move |_| {
                if let Some(dialog) = dialog.upgrade() {
                    dialog.close();
                }
            }
        });

        Self { dialog, ok, body }
    }

    /// Adds a group of rows (with an optional title) and returns it; add the rows to it.
    pub fn group(&self, title: Option<&str>) -> adw::PreferencesGroup {
        let group = adw::PreferencesGroup::new();
        if let Some(title) = title {
            group.set_title(title);
        }
        self.body.append(&group);
        group
    }

    /// The main button, for a closure that enables or hides it. A closure that is connected to
    /// a row of the form must not keep the row (or the `FormDialog`) alive: capture the button
    /// from here, and the rows with `glib::clone!(#[weak] row, …)`.
    pub fn ok_button(&self) -> Button {
        self.ok.clone()
    }

    /// Whether the main button can be pressed.
    pub fn set_ok_enabled(&self, enabled: bool) {
        self.ok.set_sensitive(enabled);
    }

    /// Shows or hides the main button (a form whose rows have buttons of their own).
    pub fn set_ok_visible(&self, visible: bool) {
        self.ok.set_visible(visible);
    }

    /// Runs `action` when the main button is pressed. The dialog closes first.
    pub fn on_ok(&self, action: impl Fn() + 'static) {
        // weak: the button is part of the dialog, a strong reference would keep both alive
        let dialog = self.dialog.downgrade();
        self.ok.connect_clicked(move |_| {
            if let Some(dialog) = dialog.upgrade() {
                dialog.close();
            }
            action();
        });
    }

    /// The dialog, for example as the parent of other dialogs.
    pub fn widget(&self) -> &adw::Dialog {
        &self.dialog
    }

    /// Shows the dialog over `parent`.
    pub fn present(&self, parent: &impl IsA<Widget>) {
        self.dialog.present(Some(parent));
    }
}

/// A row with a drop-down menu.
pub fn create_combo_row(title: &str, items: &[String]) -> adw::ComboRow {
    let items: Vec<&str> = items.iter().map(String::as_str).collect();
    adw::ComboRow::builder()
        .title(title)
        .model(&StringList::new(&items))
        .build()
}

/// The index of the chosen entry of a menu.
pub fn selected_index(row: &adw::ComboRow) -> usize {
    row.selected() as usize
}

/// A row with a switch.
pub fn create_switch_row(title: &str, subtitle: &str) -> adw::SwitchRow {
    adw::SwitchRow::builder()
        .title(title)
        .subtitle(subtitle)
        .build()
}

/// Calls `update` now and whenever the choice of the menu changes.
pub fn on_selection_change(row: &adw::ComboRow, update: impl Fn() + 'static) {
    row.connect_selected_notify(move |_| update());
}

pub use crate::design::chooser::choose::Pick;

/// What a [`PickRow`] calls when the choice changes.
type Changed = Rc<dyn Fn(&[PathBuf])>;

/// What a [`PickRow`] remembers. Shared by the row and the handler of its button.
struct PickState {
    pick: Cell<Pick>,
    paths: RefCell<Vec<PathBuf>>,
    start_in: Option<PathBuf>,
    changed: Changed,
}

impl PickState {
    /// Shows `paths` in `row`, remembers them and tells `changed`.
    fn apply(&self, row: &adw::ActionRow, paths: Vec<PathBuf>) {
        row.set_subtitle(&match paths.as_slice() {
            [] => gettext!("Nothing chosen"),
            [one] => one.display().to_string(),
            [first, ..] => format!("{} (+{})", first.display(), paths.len() - 1),
        });
        *self.paths.borrow_mut() = paths.clone();
        (self.changed)(&paths);
    }
}

/// A row that shows what was chosen and has a button to choose files or a folder.
#[derive(Clone)]
pub struct PickRow {
    /// The row; add it to a group.
    pub row: adw::ActionRow,
    state: Rc<PickState>,
}

impl PickRow {
    /// Creates the row. `changed` is called with the new choice; `start_in` is the folder the
    /// file chooser opens in.
    pub fn new(
        title: &str,
        pick: Pick,
        start_in: Option<PathBuf>,
        changed: impl Fn(&[PathBuf]) + 'static,
    ) -> Self {
        let row = adw::ActionRow::builder()
            .title(title)
            .subtitle(gettext!("Nothing chosen"))
            .subtitle_lines(3)
            .build();
        let button = Button::builder()
            .label(gettext!("Choose…"))
            .valign(Align::Center)
            .build();
        row.add_suffix(&button);
        row.set_activatable_widget(Some(&button));

        let state = Rc::new(PickState {
            pick: Cell::new(pick),
            paths: RefCell::default(),
            start_in,
            changed: Rc::new(changed),
        });
        // The button is part of the row: a strong reference to the row in its handler would
        // keep the row (and the dialog it is in) alive for ever.
        button.connect_clicked({
            let (row, state) = (row.downgrade(), state.clone());
            move |button| {
                let Some(row) = row.upgrade() else {
                    return;
                };
                let state = state.clone();
                show_chooser(
                    button,
                    state.pick.get(),
                    state.start_in.clone().as_deref(),
                    move |paths| state.apply(&row, paths),
                );
            }
        });
        Self { row, state }
    }

    /// What can be chosen now.
    pub fn set_pick(&self, pick: Pick, title: &str) {
        self.state.pick.set(pick);
        self.row.set_title(title);
        self.set_paths(Vec::new());
    }

    /// What was chosen.
    pub fn paths(&self) -> Vec<PathBuf> {
        self.state.paths.borrow().clone()
    }

    /// The first thing that was chosen.
    pub fn path(&self) -> Option<PathBuf> {
        self.state.paths.borrow().first().cloned()
    }

    /// Sets the choice (and tells `changed`).
    pub fn set_paths(&self, paths: Vec<PathBuf>) {
        self.state.apply(&self.row, paths);
    }
}
