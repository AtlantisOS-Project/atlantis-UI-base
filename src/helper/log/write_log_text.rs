// write_log_text.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! Graphical display of the log inside the application.
//!
//! The window shows the log file that
//! [`init_logging`](crate::helper::platform::logging::init_logging) writes: the latest 300 lines
//! first, then every new line as it is written. The file exists in every package format (a Flatpak
//! cannot reach the system log, the file is its only log), so the viewer works wherever the
//! program can run it. Only if there is no log file the window follows the system log
//! (`journalctl`) instead. An embedded VTE terminal runs `tail` or `journalctl`.
//!
//! The module needs the feature `log-viewer` (enabled by default) and VTE
//! (`libvte-2.91-gtk4`) at build and run time.
//!
//! # Packaging
//!
//! `tail` (coreutils) must be installed, which every distribution and runtime has. For the
//! system log `journalctl` must be installed and the user allowed to read the journal. The
//! GNOME runtime of a Flatpak and the Snap content have no VTE: build those without the feature.

use crate::design::utils::create_label_icon::icon_label_box;
use crate::design::utils::create_special_button::pill_button;
use crate::gettext;
use crate::helper::commands::open_url::open_url;
use crate::helper::platform::logging::{journal_follow_command, log_file_of};
use adw::prelude::*;
use adw::{ApplicationWindow, HeaderBar, ToolbarView};
use gtk4::{Align, Box as GtkBox, Button, ListBox, ListBoxRow, MenuButton, Orientation, Popover};
use std::path::{Path, PathBuf};
use vte4::{Terminal, TerminalExt, TerminalExtManual};

/// How many of the latest lines the viewer shows when it opens.
const FIRST_LINES: &str = "300";

/// Where the viewer takes the log from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum LogSource {
    /// The log file of the program (shown with `tail`).
    File(PathBuf),
    /// The system log (`journalctl`), when there is no log file.
    Journal,
}

/// The command that shows the log, and where it comes from: the log file if it exists, otherwise
/// the system log, otherwise nothing.
pub(crate) fn viewer_command(
    file: Option<&Path>,
    journal: Option<Vec<String>>,
) -> Option<(Vec<String>, LogSource)> {
    if let Some(file) = file.filter(|file| file.is_file()) {
        let command = ["tail", "-n", FIRST_LINES, "-F"]
            .iter()
            .map(|word| word.to_string())
            .chain([file.to_string_lossy().into_owned()])
            .collect();
        return Some((command, LogSource::File(file.to_path_buf())));
    }
    journal.map(|command| (command, LogSource::Journal))
}

/// Opens a window that shows the log of an application.
///
/// The latest 300 lines of the log file of the application (the file that
/// [`init_logging`](crate::helper::platform::logging::init_logging) writes) are shown, and new
/// lines appear as they are written. Without a log file the window follows the system log
/// (`journalctl -t <domain>`) instead. If the command ends or cannot be started, the window says
/// so. The header bar has buttons to show the folder of the log file and to copy its path, for a
/// bug report.
///
/// # Arguments
///
/// * `app` - The running Libadwaita application.
/// * `domain` - The name of the application: the `app_name` passed to
///   [`init_logging`](crate::helper::platform::logging::init_logging).
///
/// # Examples
///
/// ```no_run
/// use atlbase::ui_prelude::*;
/// # gtk4::init().unwrap();
/// # let app = adw::Application::builder().application_id("org.example.Demo").build();
///
/// show_log_viewer(&app, "my-app");
/// ```
pub fn show_log_viewer(app: &adw::Application, domain: &str) {
    let log_window = ApplicationWindow::builder()
        .application(app)
        .default_width(900)
        .default_height(600)
        .title(gettext!("Log Viewer"))
        .build();

    let toolbar_view = ToolbarView::new();
    let header_bar = HeaderBar::new();
    toolbar_view.add_top_bar(&header_bar);

    let terminal = Terminal::new();
    terminal.set_scrollback_lines(10_000);
    terminal.set_color_bold(Some(&gtk4::gdk::RGBA::new(1.0, 1.0, 1.0, 1.0)));
    toolbar_view.set_content(Some(&terminal));
    log_window.set_content(Some(&toolbar_view));

    let file = log_file_of(domain);
    let Some((command, source)) = viewer_command(file.as_deref(), journal_follow_command(domain))
    else {
        terminal
            .feed(format!("{}\r\n", gettext!("There is no log on this system yet.")).as_bytes());
        log_window.present();
        return;
    };

    // the file: buttons for the bug report
    if let LogSource::File(path) = &source {
        let folder = Button::from_icon_name("folder-open-symbolic");
        folder.set_tooltip_text(Some(&gettext!("Show the log file in its folder")));
        let dir = path.parent().map(Path::to_path_buf);
        folder.connect_clicked(move |_| {
            if let Some(dir) = &dir {
                open_url(&dir.to_string_lossy());
            }
        });
        let copy = Button::from_icon_name("edit-copy-symbolic");
        copy.set_tooltip_text(Some(&gettext!("Copy the path of the log file")));
        let text = path.to_string_lossy().into_owned();
        let window = log_window.clone();
        copy.connect_clicked(move |_| window.clipboard().set_text(&text));
        header_bar.pack_start(&folder);
        header_bar.pack_start(&copy);
        header_bar.set_title_widget(Some(&adw::WindowTitle::new(
            &gettext!("Log Viewer"),
            &path.display().to_string(),
        )));
    } else {
        header_bar.set_title_widget(Some(&adw::WindowTitle::new(
            &gettext!("Log Viewer"),
            &gettext!("System log"),
        )));
    }

    // when the command ends, the window says so: an empty black window looks broken
    terminal.connect_child_exited(|terminal, status| {
        let note = gettext!("The command that shows the log ended (status {}).");
        terminal.feed(format!("\r\n[{}]\r\n", note.replace("{}", &status.to_string())).as_bytes());
    });

    // The command is an argument list, not a shell command line.
    let argv: Vec<&str> = command.iter().map(String::as_str).collect();
    log::debug!("log viewer: {}", argv.join(" "));
    let failed = terminal.clone();
    terminal.spawn_async(
        vte4::PtyFlags::DEFAULT,
        None, // working directory
        &argv,
        &[], // environment
        glib::SpawnFlags::SEARCH_PATH,
        || {},
        -1,
        None::<&gio::Cancellable>,
        move |result| {
            if let Err(error) = result {
                log::warn!("the log viewer could not start its command: {error}");
                let note = gettext!("Could not show the log: {}");
                failed.feed(format!("{}\r\n", note.replace("{}", &error.to_string())).as_bytes());
            }
        },
    );

    log_window.present();
}

/// Creates a menu button for a header bar with the entries "Show Log" and "Exit".
///
/// "Show Log" opens [`show_log_viewer`]; "Exit" quits the application with
/// `Application::quit`. Ideal for the main view of an application.
///
/// # Arguments
///
/// * `app` - The running Libadwaita application.
/// * `domain` - The identifier of the messages for the log viewer.
///
/// # Returns
///
/// A [`gtk4::Box`] that can be packed into a header bar.
///
/// # Examples
///
/// ```no_run
/// use adw::prelude::*;
/// use atlbase::ui_prelude::*;
/// # gtk4::init().unwrap();
/// # let app = adw::Application::builder().application_id("org.example.Demo").build();
///
/// let header_bar = adw::HeaderBar::new();
/// header_bar.pack_end(&create_custom_headerbar(&app, "my-app"));
/// ```
pub fn create_custom_headerbar(app: &adw::Application, domain: &str) -> GtkBox {
    let headerbar_box = GtkBox::new(Orientation::Horizontal, 0);
    let menu_button = MenuButton::new();
    let popover = Popover::new();
    let list_box = ListBox::new();

    // entry 1: show the log
    let log_button = menu_entry("utilities-system-monitor-symbolic", &gettext!("Show Log"));
    let (app_clone, domain) = (app.clone(), domain.to_string());
    log_button.connect_clicked(move |_| show_log_viewer(&app_clone, &domain));
    list_box.append(&ListBoxRow::builder().child(&log_button).build());

    // entry 2: quit the application
    let exit_button = menu_entry("process-stop", &gettext!("Exit"));
    exit_button.set_tooltip_text(Some(&gettext!("Exit the application")));
    let app_clone = app.clone();
    exit_button.connect_clicked(move |_| app_clone.quit());
    list_box.append(&ListBoxRow::builder().child(&exit_button).build());

    popover.set_child(Some(&list_box));
    menu_button.set_popover(Some(&popover));
    headerbar_box.append(&menu_button);

    headerbar_box
}

/// A pill button with an icon and a text for the popover menu.
fn menu_entry(icon_name: &str, label: &str) -> Button {
    let button = pill_button(&icon_label_box(&[icon_name], label, Align::Fill), false);
    button.set_tooltip_text(Some(label));
    button.set_halign(Align::Start);
    button
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_log_file_comes_first_then_the_journal_then_nothing() {
        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("app.log");
        let journal = Some(vec!["journalctl".to_string(), "-f".to_string()]);

        // no file yet: the system log
        let (command, source) = viewer_command(Some(&file), journal.clone()).unwrap();
        assert_eq!(source, LogSource::Journal);
        assert_eq!(command[0], "journalctl");

        // the file exists: its latest lines, and then every new one (`-F` follows a rotated file)
        std::fs::write(&file, "x\n").unwrap();
        let (command, source) = viewer_command(Some(&file), journal).unwrap();
        assert_eq!(source, LogSource::File(file.clone()));
        assert_eq!(command[..4], ["tail", "-n", FIRST_LINES, "-F"]);
        assert_eq!(command[4], file.to_string_lossy());

        // neither a file nor a system log
        assert!(viewer_command(Some(&dir.path().join("missing.log")), None).is_none());
        assert!(viewer_command(None, None).is_none());
    }
}
