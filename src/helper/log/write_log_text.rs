// write_log_text.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! Graphical display of the system log inside the application.
//!
//! The module shows the messages that [`init_syslog`](crate::helper::log::write_log::init_syslog)
//! sends to syslog/journald in a window. It embeds a VTE terminal that follows `journalctl`
//! in real time.
//!
//! The module needs the feature `log-viewer` (enabled by default) and VTE
//! (`libvte-2.91-gtk4`) at build and run time.
//!
//! # Packaging
//!
//! `journalctl` must be installed and the user must be allowed to read the journal. Inside a
//! Flatpak sandbox neither is the case, so the window stays empty; hide the log button
//! there (see [`get_execution_environment`](crate::helper::configuration::application_environment::get_execution_environment)).

use crate::gettext;
use adw::prelude::*;
use adw::{ApplicationWindow, HeaderBar, ToolbarView};
use gtk4::{
    Align, Box as GtkBox, Button, Image, Label, ListBox, ListBoxRow, MenuButton, Orientation,
    Popover,
};
use vte4::{Terminal, TerminalExt, TerminalExtManual};

/// Opens a window that follows the syslog entries of an application.
///
/// A `journalctl -f -t <domain>` process is started in an embedded terminal; new entries
/// appear immediately, older ones are not shown. Errors while starting the process are printed
/// to `stderr`.
///
/// # Arguments
///
/// * `app` - The running Libadwaita application.
/// * `domain` - The identifier of the messages, that is the `app_name` passed to
///   [`init_syslog`](crate::helper::log::write_log::init_syslog).
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
        .default_width(800)
        .default_height(600)
        .title(gettext!("Log Viewer"))
        .build();

    let toolbar_view = ToolbarView::new();
    let header_bar = HeaderBar::new();
    let title = Label::new(Some(&gettext!("Log Viewer")));
    header_bar.set_title_widget(Some(&title));
    toolbar_view.add_top_bar(&header_bar);

    let terminal = Terminal::new();
    terminal.set_color_bold(Some(&gtk4::gdk::RGBA::new(1.0, 1.0, 1.0, 1.0)));
    terminal.set_scrollback_lines(1000);
    terminal.feed(b"Starting logging...\r\n");

    // The identifier is an argument of journalctl, not part of a shell command line.
    let argv = [
        "journalctl",
        "-f",
        "-n",
        "0",
        "-t",
        domain,
        "--output=short",
    ];

    terminal.spawn_async(
        vte4::PtyFlags::DEFAULT,
        None, // working directory
        &argv,
        &[], // environment
        glib::SpawnFlags::SEARCH_PATH,
        || {},
        -1,
        None::<&gio::Cancellable>,
        move |result| match result {
            Ok(pid) => println!("Journald-Log started (PID: {:?})", pid),
            Err(e) => eprintln!("Error with starting: {}", e),
        },
    );

    toolbar_view.set_content(Some(&terminal));
    log_window.set_content(Some(&toolbar_view));
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
    let content = GtkBox::new(Orientation::Horizontal, 6);
    content.append(&Image::from_icon_name(icon_name));
    content.append(&Label::new(Some(label)));

    let button = Button::new();
    button.set_child(Some(&content));
    button.set_tooltip_text(Some(label));
    button.set_halign(Align::Start);
    button.add_css_class("pill");
    button
}
