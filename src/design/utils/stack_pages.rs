// stack_pages.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! Pages in a [`gtk4::Stack`] that are built when they are needed.

use gtk4::Stack;

/// Shows the page `id` of a stack. A page that does not exist yet is built with `build`
/// first ("lazy loading"), so the start of the program does not build every page.
///
/// `rebuild` is for a page with live data (a device list, the system information): the old
/// page is thrown away and `build` creates it again on every visit.
///
/// # Examples
///
/// ```no_run
/// use atlbase::ui_prelude::show_stack_page;
/// # gtk4::init().unwrap();
/// let stack = gtk4::Stack::new();
///
/// // built once
/// show_stack_page(&stack, "home", false, || gtk4::Label::new(Some("Home")).into());
/// // built on every visit
/// show_stack_page(&stack, "clock", true, || gtk4::Label::new(Some("12:00")).into());
/// assert_eq!(stack.visible_child_name().as_deref(), Some("clock"));
/// ```
pub fn show_stack_page(
    stack: &Stack,
    id: &str,
    rebuild: bool,
    build: impl FnOnce() -> gtk4::Widget,
) {
    if rebuild && let Some(old) = stack.child_by_name(id) {
        stack.remove(&old);
    }
    if stack.child_by_name(id).is_none() {
        log::debug!("load page: {id}");
        stack.add_named(&build(), Some(id));
    }
    stack.set_visible_child_name(id);
}
