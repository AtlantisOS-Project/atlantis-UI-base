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

use crate::gettext;
use adw::prelude::*;
use gtk4::{
    Align, Box as GtkBox, Button, Orientation, PolicyType, ScrolledWindow, StringList, Widget, gio,
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
        cancel.connect_clicked({
            let dialog = dialog.clone();
            move |_| {
                dialog.close();
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
        let dialog = self.dialog.clone();
        self.ok.connect_clicked(move |_| {
            dialog.close();
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

/// What a [`PickRow`] lets the user choose.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pick {
    /// One file.
    File,
    /// Several files.
    Files,
    /// A folder.
    Folder,
}

/// What a [`PickRow`] calls when the choice changes.
type Changed = Rc<dyn Fn(&[PathBuf])>;

/// A row that shows what was chosen and has a button to choose files or a folder.
#[derive(Clone)]
pub struct PickRow {
    /// The row; add it to a group.
    pub row: adw::ActionRow,
    pick: Rc<Cell<Pick>>,
    paths: Rc<RefCell<Vec<PathBuf>>>,
    start_in: Option<PathBuf>,
    changed: Changed,
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

        let this = Self {
            row,
            pick: Rc::new(Cell::new(pick)),
            paths: Rc::default(),
            start_in,
            changed: Rc::new(changed),
        };
        button.connect_clicked({
            let this = this.clone();
            move |button| this.choose(button)
        });
        this
    }

    /// What can be chosen now.
    pub fn set_pick(&self, pick: Pick, title: &str) {
        self.pick.set(pick);
        self.row.set_title(title);
        self.set_paths(Vec::new());
    }

    /// What was chosen.
    pub fn paths(&self) -> Vec<PathBuf> {
        self.paths.borrow().clone()
    }

    /// The first thing that was chosen.
    pub fn path(&self) -> Option<PathBuf> {
        self.paths.borrow().first().cloned()
    }

    /// Sets the choice (and tells `changed`).
    pub fn set_paths(&self, paths: Vec<PathBuf>) {
        self.row.set_subtitle(&match paths.as_slice() {
            [] => gettext!("Nothing chosen"),
            [one] => one.display().to_string(),
            [first, ..] => format!("{} (+{})", first.display(), paths.len() - 1),
        });
        *self.paths.borrow_mut() = paths.clone();
        (self.changed)(&paths);
    }

    fn choose(&self, button: &Button) {
        let window = button.root().and_downcast::<gtk4::Window>();
        let dialog = gtk4::FileDialog::new();
        if let Some(dir) = self.start_in.as_ref().filter(|dir| dir.is_dir()) {
            dialog.set_initial_folder(Some(&gio::File::for_path(dir)));
        }

        let this = self.clone();
        let files = move |files: Vec<gio::File>| {
            this.set_paths(files.iter().filter_map(gio::File::path).collect());
        };
        match self.pick.get() {
            Pick::File => {
                dialog.set_title(&gettext!("Choose File"));
                dialog.open(window.as_ref(), gio::Cancellable::NONE, move |result| {
                    if let Ok(file) = result {
                        files(vec![file]);
                    }
                });
            }
            Pick::Files => {
                dialog.set_title(&gettext!("Choose File"));
                dialog.open_multiple(window.as_ref(), gio::Cancellable::NONE, move |result| {
                    if let Ok(list) = result {
                        files(list.iter::<gio::File>().flatten().collect());
                    }
                });
            }
            Pick::Folder => {
                dialog.set_title(&gettext!("Choose folder"));
                dialog.select_folder(window.as_ref(), gio::Cancellable::NONE, move |result| {
                    if let Ok(folder) = result {
                        files(vec![folder]);
                    }
                });
            }
        }
    }
}
