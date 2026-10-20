// menu_grid.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! A grid of big buttons for a start page or a menu.
//!
//! Every button has an icon and a text. A text that is too long for the column is cut with
//! "…" (the tooltip shows it whole). How many characters fit is measured in the font that is
//! used, when the button is shown: the grid never makes the window wider, whether the font is
//! narrow (Ubuntu) or wide (the GNOME platform of Flatpak and Snap).

use crate::design::utils::create_special_button::create_button_icon_position;
use gtk4::prelude::*;
use gtk4::{Align, Button, Grid, Label, Widget, pango};
use std::rc::Rc;

/// The click handler of a menu entry.
pub type MenuAction = Rc<dyn Fn(&Button)>;

/// One button of a [`create_button_grid`]: an icon, a text and what happens when it is clicked.
#[derive(Clone)]
pub struct MenuEntry {
    /// The name of a symbolic icon.
    pub icon: &'static str,
    /// The translated text of the button.
    pub label: String,
    /// What the button does.
    pub action: MenuAction,
}

impl MenuEntry {
    /// A button that runs `action`.
    pub fn new(
        icon: &'static str,
        label: impl Into<String>,
        action: impl Fn(&Button) + 'static,
    ) -> Self {
        Self {
            icon,
            label: label.into(),
            action: Rc::new(action),
        }
    }
}

/// How a [`create_button_grid`] looks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MenuGridStyle {
    /// The width in pixels the grid has, in all columns together.
    pub width: i32,
    /// The space a button needs besides its text: padding, border, icon and the gap to the
    /// icon. Depends on your CSS.
    pub chrome: i32,
    /// An additional CSS class for the buttons (for example your own "nav-button" style);
    /// empty for none.
    pub css_class: &'static str,
}

impl Default for MenuGridStyle {
    fn default() -> Self {
        Self {
            width: 776,
            chrome: 78,
            css_class: "",
        }
    }
}

/// The space between the buttons.
const SPACING: i32 = 10;

/// Finds the first label inside a widget.
fn find_label(widget: &Widget) -> Option<Label> {
    if let Some(label) = widget.downcast_ref::<Label>() {
        return Some(label.clone());
    }
    let mut child = widget.first_child();
    while let Some(current) = child {
        if let Some(label) = find_label(&current) {
            return Some(label);
        }
        child = current.next_sibling();
    }
    None
}

/// Gives a button the width of its column and cuts a long label with "…".
fn fit_label(button: &Button, text: &str, columns: usize, style: MenuGridStyle) {
    button.set_tooltip_text(Some(text));
    button.set_hexpand(true);
    let Some(label) = find_label(button.upcast_ref()) else {
        return;
    };
    label.set_ellipsize(pango::EllipsizeMode::End);
    // until the font is known a short text is assumed to fit
    let wanted = text.chars().count();
    label.set_width_chars(wanted.min(12) as i32);

    let column = (style.width - SPACING * (columns as i32 - 1)) / columns as i32;
    label.connect_map(move |label| {
        let digits = label
            .create_pango_layout(Some("0123456789"))
            .pixel_size()
            .0
            .max(10);
        let fits = ((column - style.chrome) * 10 / digits).max(6);
        label.set_width_chars(wanted.min(fits as usize) as i32);
    });
}

/// A grid of big buttons with `columns` columns, one for each entry.
///
/// # Examples
///
/// ```no_run
/// use atlbase::ui_prelude::*;
/// # gtk4::init().unwrap();
///
/// let grid = create_button_grid(
///     &[
///         MenuEntry::new("phone-symbolic", "Devices", |_| println!("devices")),
///         MenuEntry::new("system-reboot-symbolic", "Reboot", |_| println!("reboot")),
///     ],
///     2,
///     MenuGridStyle::default(),
/// );
/// ```
pub fn create_button_grid(entries: &[MenuEntry], columns: usize, style: MenuGridStyle) -> Grid {
    let columns = columns.max(1);
    let grid = Grid::builder()
        .column_spacing(SPACING)
        .row_spacing(SPACING)
        .halign(Align::Fill)
        .row_homogeneous(true)
        .column_homogeneous(true)
        .build();

    for (index, entry) in entries.iter().enumerate() {
        let action = entry.action.clone();
        let button =
            create_button_icon_position(entry.icon, &entry.label, Align::Center, move |button| {
                action(button)
            });
        if !style.css_class.is_empty() {
            button.add_css_class(style.css_class);
        }
        fit_label(&button, &entry.label, columns, style);
        grid.attach(
            &button,
            (index % columns) as i32,
            (index / columns) as i32,
            1,
            1,
        );
    }
    grid
}
