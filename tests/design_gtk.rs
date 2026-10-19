// design_gtk.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! Smoke tests of the widgets that need GTK (`atlbase::design`).
//!
//! GTK may only be used on the main thread, so this test has its own `main` instead of the
//! test harness (`harness = false` in `Cargo.toml`). Without a display (for example in a CI
//! container) the test prints a note and succeeds; use `xvfb-run` or a Wayland session to run
//! it for real.

use adw::prelude::*;
use atlbase::ui_prelude::*;
use std::cell::Cell;
use std::rc::Rc;

fn main() {
    if adw::init().is_err() {
        println!("design_gtk: skipped, there is no display");
        return;
    }

    let checks: &[(&str, fn())] = &[
        ("entry factories", entry_factories),
        ("button factories", button_factories),
        ("css provider", css_provider),
        ("status dialogs", status_dialogs),
        ("guide navigation", guide_navigation),
        ("guide in a stack", guide_in_a_stack),
        ("guide with a missing image", guide_with_missing_image),
        ("guide with a button", guide_with_button),
    ];

    for (name, check) in checks {
        check();
        println!("design_gtk: {name} ... ok");
    }
}

fn entry_factories() {
    let (row, entry) = create_entry("Name:", Some("your name"));
    assert_eq!(entry.placeholder_text().as_deref(), Some("your name"));
    assert!(
        gtk4::prelude::EntryExt::is_visible(&entry),
        "the text of a normal entry is shown"
    );
    assert!(row.first_child().is_some());

    let (_, password) = create_password_entry("Password:", None);
    assert!(
        !gtk4::prelude::EntryExt::is_visible(&password),
        "the text of a password is hidden"
    );
}

fn button_factories() {
    let clicks = Rc::new(Cell::new(0));
    let counter = clicks.clone();
    let button = create_button_icon("go-next-symbolic", "Next", move |_| {
        counter.set(counter.get() + 1)
    });
    assert!(button.has_css_class("pill") && button.has_css_class("suggested-action"));

    button.emit_clicked();
    button.emit_clicked();
    assert_eq!(clicks.get(), 2);

    let plain = create_button_icon_no_callback("go-next-symbolic", "Next");
    assert!(plain.child().is_some());

    let centered =
        create_button_icon_position("go-next-symbolic", "Next", gtk4::Align::Center, |_| {});
    assert_eq!(centered.child().unwrap().halign(), gtk4::Align::Center);

    let _ = create_label_icon("dialog-information", "Info");
}

fn css_provider() {
    // every call creates its own provider
    let first = use_adw_provider("label { color: red; }");
    let second = use_adw_provider("label { color: blue; }");
    assert_ne!(first, second);
}

fn status_dialogs() {
    // an adw::Window can host dialogs without a running application
    let window = adw::Window::new();

    for show in [
        |w: &adw::Window| show_info_dialog(w, "Saved"),
        |w: &adw::Window| show_error_dialog(w, "Disk full"),
        |w: &adw::Window| show_error_title_dialog(w, "Network", "Timeout"),
        |w: &adw::Window| show_dialog_title(w, "Backup", "Done"),
        |w: &adw::Window| show_success_dialog(w, "Installed"),
        |w: &adw::Window| show_warning_dialog(w, "Battery low"),
    ] {
        show(&window);
        let dialog = window.visible_dialog().expect("the dialog is shown");
        // a long text must not break the dialog
        assert!(!dialog.title().is_empty());
        dialog.force_close();
        assert!(window.visible_dialog().is_none());
    }

    show_error_dialog(&window, &"a very long error text\n".repeat(200));
    assert!(window.visible_dialog().is_some());
}

fn sample_guide() -> Guide {
    Guide::new("Tour")
        .page(Page::new("One").paragraph("first"))
        .page(Page::new("Two").paragraph("second"))
        .page(Page::new("Three").paragraph("third"))
}

fn guide_navigation() {
    let view = GuideView::new(&sample_guide());
    assert_eq!(view.n_pages(), 3);
    assert_eq!(view.page(), 0);
    assert!(view.has_css_class("adw-guide"));

    let finished = Rc::new(Cell::new(0));
    let counter = finished.clone();
    view.connect_finished(move |_| counter.set(counter.get() + 1));

    view.set_page(2, false);
    assert_eq!(view.page(), 2);
    // an index beyond the end is limited to the last page
    view.set_page(99, false);
    assert_eq!(view.page(), 2);

    // "Done" on the last page ends the guide
    view.emit_by_name::<()>("finished", &[]);
    assert_eq!(finished.get(), 1);
}

fn guide_in_a_stack() {
    let stack = gtk4::Stack::new();
    stack.add_named(&gtk4::Label::new(Some("home")), Some("home"));

    let view = GuideView::present_in_stack(&stack, "tour", &sample_guide(), "home");
    assert_eq!(stack.visible_child_name().as_deref(), Some("tour"));

    // finishing the guide returns to the page that was asked for
    view.emit_by_name::<()>("finished", &[]);
    assert_eq!(stack.visible_child_name().as_deref(), Some("home"));

    // presenting again replaces the old guide
    GuideView::present_in_stack(&stack, "tour", &sample_guide(), "home");
    assert_eq!(stack.pages().n_items(), 2);
}

fn guide_with_missing_image() {
    // a missing image must show a placeholder instead of failing
    let page = Page::new("Images").image(Image::from_file("/does/not/exist.png").alt("missing"));
    let widget = page_widget(&page, &Labels::default());
    assert!(widget.first_child().is_some());
}

/// The first button below `widget` that has the CSS class `class`.
fn find_button(widget: &gtk4::Widget, class: &str) -> Option<gtk4::Button> {
    if let Some(button) = widget.downcast_ref::<gtk4::Button>()
        && button.has_css_class(class)
    {
        return Some(button.clone());
    }
    let mut child = widget.first_child();
    while let Some(current) = child {
        if let Some(found) = find_button(&current, class) {
            return Some(found);
        }
        child = current.next_sibling();
    }
    None
}

fn guide_with_button() {
    let guide =
        Guide::new("Setup").page(Page::new("Folder").button("choose-folder", "Choose folder"));
    let view = GuideView::new(&guide);

    let actions = Rc::new(std::cell::RefCell::new(Vec::new()));
    let log = actions.clone();
    view.connect_action(move |_, action| log.borrow_mut().push(action.to_string()));

    let button = find_button(view.upcast_ref(), "pill").expect("the page has a pill button");
    button.emit_clicked();
    button.emit_clicked();
    assert_eq!(*actions.borrow(), ["choose-folder", "choose-folder"]);

    // outside of a GuideView a click does nothing
    let alone = block_widget(
        &Block::Button(ActionButton::new("x", "X")),
        &Labels::default(),
    );
    find_button(&alone, "pill").unwrap().emit_clicked();
}
