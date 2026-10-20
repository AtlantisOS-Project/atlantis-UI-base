// pages.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! A program with several pages in a stack: the building blocks for such programs.
//!
//! ```text
//! cargo run --example pages
//! ```
//!
//! * [`StackWindow`] — a window of fixed size with a header bar and a stack.
//! * [`show_stack_page`] — pages that are built when they are first shown (or on every visit).
//! * [`create_button_grid`] — big buttons for the start page.
//! * [`FormDialog`] — a dialog with a drop-down menu and a file row.
//! * [`CommandTask`], [`show_work_dialog`], [`show_confirm_dialog`] — feedback for slow work.
//! * [`StatusRow`] and [`run_background`] — a check that runs without freezing the window.

use adw::prelude::*;
use atlbase::prelude::*;
use atlbase::ui_prelude::*;
use gtk4::{Box as GtkBox, Stack};
use std::time::Duration;

/// Switches to a page when a button is clicked.
fn go(stack: &Stack, page: &'static str) -> impl Fn(&gtk4::Button) + 'static {
    let stack = stack.downgrade();
    move |_| {
        if let Some(stack) = stack.upgrade() {
            show_page(&stack, page);
        }
    }
}

fn show_page(stack: &Stack, page: &'static str) {
    // "checks" shows live data: it is built again on every visit
    show_stack_page(stack, page, page == "checks", || match page {
        "home" => home(stack).upcast(),
        "checks" => checks(stack).upcast(),
        other => unreachable!("no page {other}"),
    });
}

fn home(stack: &Stack) -> GtkBox {
    let parent = stack.clone();
    let entries = [
        MenuEntry::new("system-run-symbolic", "Run a command", move |button| {
            CommandTask::new(
                "Echo",
                "Please wait...",
                vec![command_line("echo", ["Hello from a command"])],
            )
            .show_output("Output")
            .run(button);
        }),
        MenuEntry::new("edit-delete-symbolic", "Delete something", |button| {
            show_confirm_dialog(
                button,
                "Delete everything?",
                "This cannot be undone.",
                "Delete",
                || println!("deleted (not really)"),
            );
        }),
        MenuEntry::new("document-edit-symbolic", "Fill a form", |button| {
            form(button)
        }),
        MenuEntry::new("emblem-ok-symbolic", "System checks", go(&parent, "checks")),
    ];

    let page = create_page_container();
    page.append(&create_button_grid(&entries, 2, MenuGridStyle::default()));
    page
}

fn form(button: &gtk4::Button) {
    let form = FormDialog::new("Backup", "Start");
    let group = form.group(Some("What to save"));
    let partition = create_combo_row("Partition", &["boot".into(), "vendor".into()]);
    let target = PickRow::new("Folder", Pick::Folder, None, |_| {});
    group.add(&partition);
    group.add(&target.row);

    let parent = button.clone();
    form.on_ok(move || {
        let (index, folder) = (selected_index(&partition), target.path());
        show_work_dialog(&parent, "Saving", "Please wait...", move || {
            std::thread::sleep(Duration::from_secs(1));
            match folder {
                Some(folder) => Ok(format!("Saved partition {index} to {}", folder.display())),
                None => Err("Choose a folder first.".to_string()),
            }
        });
    });
    form.present(button);
}

fn checks(stack: &Stack) -> GtkBox {
    let group = adw::PreferencesGroup::builder()
        .title("This computer")
        .build();
    let kernel = StatusRow::new("Kernel");
    let session = StatusRow::new("Graphical session");
    group.add(&kernel.row);
    group.add(&session.row);

    // the slow work runs in a thread; the rows are filled in the main loop afterwards
    run_background(SystemInfo::collect, move |info| {
        kernel.set_result(!info.kernel.is_empty(), &info.kernel);
        session.set_result(!info.session.is_empty(), &info.session);
    });

    let back = create_pill_button("pan-start-symbolic", "Back", false, go(stack, "home"));
    create_clamped_scrolling_page(&group, &back, 620)
}

fn main() -> glib::ExitCode {
    let app = adw::Application::builder()
        .application_id("org.atlantis.PagesExample")
        .build();
    app.connect_activate(|app| {
        let window = StackWindow::new(app, "Pages", 640, 480);
        window.present(|stack| show_page(stack, "home"));
    });
    app.run()
}
