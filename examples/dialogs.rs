// dialogs.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! A complete application with every dialog of the library.
//!
//! ```text
//! cargo run --example dialogs
//! LANGUAGE=de cargo run --example dialogs   # the dialogs of the library in German
//! ```
//!
//! The program shows the typical start-up of an application: syslog, language, CSS, a
//! header bar with the menu of the library, and one button per dialog.

use adw::prelude::*;
use atlbase::gettext;
use atlbase::prelude::*;
use atlbase::ui_prelude::*;
use gtk4::{Align, Box as GtkBox, Orientation};
use std::path::PathBuf;

const APP_ID: &str = "org.atlantisos.AtlBaseDialogsDemo";

const CUSTOM_CSS: &str = r#"
button.pill { padding: 10px 18px; }
"#;

/// A big button row of the demo.
fn demo_button(
    label: &str,
    icon: &str,
    callback: impl Fn(&gtk4::Button) + 'static,
) -> gtk4::Button {
    create_button_icon_position(icon, label, Align::Center, callback)
}

fn build_ui(app: &adw::Application) {
    // 1. log to syslog (ignore the error if a logger exists already)
    let _ = init_syslog("atlbase-dialogs-demo");
    log::info!("dialogs demo started");

    // 2. translations: this application has no catalog of its own, but the texts of the
    //    library are translated anyway (they are compiled into the library)
    init_language("atlbase-dialogs-demo", env!("COMPILED_LOCALE_DIR"), false);

    // 3. one CSS provider for the whole application
    use_adw_provider(CUSTOM_CSS);

    let grid = gtk4::Grid::builder()
        .column_spacing(12)
        .row_spacing(12)
        .halign(Align::Center)
        .valign(Align::Center)
        .margin_top(24)
        .margin_bottom(24)
        .margin_start(24)
        .margin_end(24)
        .build();

    let buttons = [
        demo_button("Information", "dialog-information-symbolic", |b| {
            show_info_dialog(b, "The file was saved.");
        }),
        demo_button("Error", "dialog-error-symbolic", |b| {
            show_error_title_dialog(b, "Network timeout", "The server did not answer.");
        }),
        demo_button("Success", "emblem-ok-symbolic", |b| {
            show_success_dialog(b, "The update was installed.");
        }),
        demo_button("Warning", "dialog-warning-symbolic", |b| {
            show_warning_dialog(b, "The battery is low. Plug in the charger.");
        }),
        demo_button("Custom content", "view-list-symbolic", |b| {
            let content = GtkBox::new(Orientation::Vertical, 6);
            content.append(&gtk4::Label::new(Some("Any widgets can be placed here.")));
            content.append(&gtk4::ProgressBar::builder().fraction(0.4).build());
            show_custom_content_dialog(
                b,
                "Update",
                "A new version is available.",
                "Install",
                &content,
            );
        }),
        demo_button("Text input", "document-edit-symbolic", |b| {
            let parent = b.clone();
            show_entry_dialog(
                b,
                "Change name",
                "Enter the new name.",
                "Save",
                "Cancel",
                "Name:",
                "e.g. Donald",
                move |name| show_info_dialog(&parent, &format!("Hello {name}!")),
            );
        }),
        demo_button("About", "help-about-symbolic", |b| {
            show_about_dialog(
                b,
                AboutInfo {
                    app_icon: "org.atlantisos.AtlBase".into(),
                    app_name: "AtlBase dialogs".into(),
                    developer_name: "AtlantisOS Project".into(),
                    version: env!("CARGO_PKG_VERSION").into(),
                    comments: "Demo of the dialogs of the library".into(),
                    website: "https://github.com/AtlantisOS-Project/atlantis-UI-base".into(),
                    copyright: "© 2026 AtlantisOS Project".into(),
                    developers: vec!["NachtsternBuild".into()],
                    special_thanks: vec!["The GNOME project".into()],
                    ..AboutInfo::default()
                },
            );
        }),
        demo_button("Image", "image-x-generic-symbolic", |b| {
            let image = concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/examples/assets/treble-check.png"
            );
            show_image_dialog(
                b,
                "Preview",
                "A screenshot from the examples.",
                "Close",
                image,
                480,
                300,
            );
        }),
        demo_button("Choose file", "document-open-symbolic", |b| {
            show_file_chooser(b, {
                let parent = b.clone();
                move |path: PathBuf| show_info_dialog(&parent, &format!("File: {}", path.display()))
            });
        }),
        demo_button("Choose folder", "folder-open-symbolic", |b| {
            show_folder_chooser(b, {
                let parent = b.clone();
                move |path: PathBuf| {
                    show_info_dialog(&parent, &format!("Folder: {}", path.display()))
                }
            });
        }),
        demo_button("Spinner (commands)", "emblem-system-symbolic", |b| {
            let Some(window) = b.root().and_downcast::<adw::ApplicationWindow>() else {
                return;
            };
            let parent = window.clone();
            show_spinner_dialog(
                &window,
                "Working",
                "Please wait...",
                vec![
                    vec!["sleep".into(), "2".into()],
                    vec!["echo".into(), "ready".into()],
                ],
                IndicatorType::Spinner,
                Some(Box::new(move |result| match result {
                    Ok(output) => show_info_dialog(&parent, &format!("Output: {}", output.trim())),
                    Err(error) => show_error_dialog(&parent, &error),
                })),
            );
        }),
        demo_button("Progress bar (results)", "emblem-system-symbolic", |b| {
            let Some(window) = b.root().and_downcast::<adw::ApplicationWindow>() else {
                return;
            };
            let parent = window.clone();
            show_spinner_dialog_return_output(
                &window,
                "Working",
                "Please wait...",
                // the first steps take a while, so the progress bar can be seen; the
                // third command fails and ends the chain
                vec![
                    vec!["sleep".into(), "2".into()],
                    vec!["echo".into(), "step one".into()],
                    vec!["sleep".into(), "1".into()],
                    vec!["ls".into(), "/does/not/exist".into()],
                    vec!["echo".into(), "never reached".into()],
                ],
                IndicatorType::ProgressBar,
                Some(Box::new(move |result| match result {
                    Ok(results) => {
                        show_info_dialog(&parent, &format!("{} commands succeeded", results.len()))
                    }
                    Err(results) => {
                        let failed = results
                            .last()
                            .map(|r| r.command.clone())
                            .unwrap_or_default();
                        show_error_dialog(
                            &parent,
                            &format!("`{failed}` failed after {} commands", results.len()),
                        );
                    }
                })),
            );
        }),
        demo_button("Background task", "system-run-symbolic", |b| {
            let Some(window) = b.root().and_downcast::<adw::ApplicationWindow>() else {
                return;
            };
            show_task_dialog(
                &window,
                "Calculating",
                "Complex data...",
                IndicatorType::Spinner,
                || {
                    std::thread::sleep(std::time::Duration::from_secs(2));
                    true
                },
            );
        }),
    ];

    for (index, button) in buttons.iter().enumerate() {
        grid.attach(button, (index % 2) as i32, (index / 2) as i32, 1, 1);
    }

    // entries with a label
    let (name_row, _name) = create_entry("Name:", Some("Your name"));
    let (password_row, _password) = create_password_entry("Password:", None);
    let row = buttons.len() as i32 / 2 + 1;
    grid.attach(&name_row, 0, row, 2, 1);
    grid.attach(&password_row, 0, row + 1, 2, 1);

    // the header bar with the menu of the library (log viewer and exit)
    let header_bar = adw::HeaderBar::new();
    header_bar.set_title_widget(Some(&gtk4::Label::new(Some(&gettext!("Dialogs demo")))));
    header_bar.pack_end(&create_custom_headerbar(app, "atlbase-dialogs-demo"));

    let toolbar_view = adw::ToolbarView::new();
    toolbar_view.add_top_bar(&header_bar);
    toolbar_view.set_content(Some(&grid));

    let window = adw::ApplicationWindow::builder()
        .application(app)
        .title("AtlBase dialogs")
        .default_width(560)
        .default_height(620)
        .content(&toolbar_view)
        .build();
    window.present();
}

fn main() -> glib::ExitCode {
    let app = adw::Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}
