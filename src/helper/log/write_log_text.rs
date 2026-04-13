/**
* write_log_text.rs
*
* (C) Copyright 2026 AtlantisOS Project
* by @NachtsternBuild
*
* License: GNU GENERAL PUBLIC LICENSE Version 3
*
*/

use adw::prelude::*;
use adw::{ApplicationWindow, HeaderBar, ToolbarView};
use gtk4::{Box as GtkBox, Button, Label, ListBox, ListBoxRow, MenuButton, Popover, Image, Orientation, Align};
use vte4::Terminal;
use vte4::TerminalExtManual;
use vte4::TerminalExt;
use std::process;
use crate::gettext;

/*
// callback process by started terminal
fn handle_spawn_result(result: Result<glib::Pid, glib::Error>) {
    match result {
        Ok(pid) => println!("Process started, PID={:?}", pid), // {:?} for PID
        Err(err) => eprintln!("Error starting the process: {}", err.message()),
    }
}
*/

/** 
* @brief Create a window, that shows the log
*/
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
    
	// use rgb structure
    terminal.set_color_bold(Some(&gtk4::gdk::RGBA::new(1.0, 1.0, 1.0, 1.0)));
    terminal.set_scrollback_lines(1000);

    let cmd_str = format!("journalctl -f -n 0 -t {} --output=short", domain);
    
    // create the strings
    let bin = "/bin/sh";
    let arg_c = "-c";
    
    // create the array from the strings
    let argv = [bin, arg_c, &cmd_str];

    // Fenster präsentieren
    toolbar_view.set_content(Some(&terminal));
    log_window.set_content(Some(&toolbar_view));
    log_window.present();

    // Spawn-Aufruf
    terminal.spawn_async(
        vte4::PtyFlags::DEFAULT,
        None,
        &argv, // &[&str]
        &[],   // env
        glib::SpawnFlags::DEFAULT,
        || {}, 
        -1,
        None::<&gio::Cancellable>,
        move |result| {
            match result {
                Ok(pid) => println!("Journald-Log started (PID: {:?})", pid),
                Err(e) => eprintln!("Error with starting: {}", e),
            }
        },
    );
}


/** 
* @brief Function that kills the program itself
*/
fn kill_program() {
    process::exit(0); 
}

/**
* @brief Header that create the popover menu
*/
pub fn create_custom_headerbar(app: &adw::Application, domain: &'static str) -> GtkBox {
    let headerbar_box = GtkBox::new(Orientation::Horizontal, 0);
    let menu_button = MenuButton::new();
    let popover = Popover::new();
    let list_box = ListBox::new();

    // Item 1: Show Log 
    let row1 = ListBoxRow::new();
    let hbox1 = GtkBox::new(Orientation::Horizontal, 6);
    let icon1 = Image::from_icon_name("utilities-system-monitor-symbolic");
    let label1 = Label::new(Some(&gettext!("Show Log")));
    
    hbox1.append(&icon1);
    hbox1.append(&label1);
	
	// button 1
    let button1 = Button::new();
    button1.set_child(Some(&hbox1));
    button1.set_tooltip_text(Some(&gettext!("Show Log")));
    button1.set_halign(Align::Start);
    
    let app_clone = app.clone();
    button1.connect_clicked(move |_| {
        show_log_viewer(&app_clone, domain);
    });

    row1.set_child(Some(&button1));
    list_box.append(&row1);

    // Item 2: Exit
    let row2 = ListBoxRow::new();
    let hbox2 = GtkBox::new(Orientation::Horizontal, 6);
    let icon2 = Image::from_icon_name("process-stop");
    let label2 = Label::new(Some(&gettext!("Exit")));

    hbox2.append(&icon2);
    hbox2.append(&label2);
	
	// button 2
    let button2 = Button::new();
    button2.set_child(Some(&hbox2));
    button2.set_tooltip_text(Some(&gettext!("Kill App")));
    button2.set_halign(Align::Start);
    
    button2.connect_clicked(|_| {
        kill_program();
    });

    row2.set_child(Some(&button2));
    list_box.append(&row2);

    // build the popover menu
    popover.set_child(Some(&list_box));
    menu_button.set_popover(Some(&popover));
    headerbar_box.append(&menu_button);

    headerbar_box
}
