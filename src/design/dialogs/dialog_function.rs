//! Dialog-driven background processes.
//!
//! This module provides a convenient way to execute time-consuming operations
//! in a background thread while displaying a 
//! non-closable dialog with a progress bar to the user.
/**
* dialog_function.rs
*
* (C) Copyright 2026 AtlantisOS Project
* by @NachtsternBuild
*
* License: GNU GENERAL PUBLIC LICENSE Version 3
*/

use adw::prelude::*;
use gtk4::{glib, ProgressBar, Align, Label, Orientation, Box as GtkBox};
use adw::{Spinner, Dialog};
use std::thread;
use std::sync::mpsc;
use crate::design::dialogs::dialogs_spinner::IndicatorType;

/// Runs a function in a dedicated background thread.
///
/// The function's result (`bool`) is returned to the
/// UI's main loop via a channel.
fn run_task_thread<F>(task: F, tx: mpsc::Sender<bool>) 
where 
    F: FnOnce() -> bool + Send + 'static 
{
    thread::spawn(move || {
        let success = task();
        // return sucess to the main loop
        let _ = tx.send(success);
    });
}

/// Displays a modal dialog while a Rust function runs in the background.
///
/// The dialog prevents user interactions with the main window until the 
/// background task is complete. Once the task is finished,
/// the dialog closes automatically.
///
/// # Arguments
///
/// * `parent` - The parent `adw::ApplicationWindow` to which the dialog is docked.
/// * `title` - The dialog's title (displayed in bold within the content).
/// * `message` - An explanatory text for the user.
/// * `indicator` - The type of progress indicator ([IndicatorType::Spinner] or [IndicatorType::ProgressBar]).
/// * `task` - A closure or function that runs in the background. Must be `Send`.
///
/// # Usage:
/// #### Usage with an inline closure:
/// ```rust
/// button.connect_clicked(glib::clone!(@weak window => move |_| {
///    show_task_dialog(
///        &window,
///        "Some Dialog",
///        "Complex data...",
///        IndicatorType::Spinner,
///        || {
///            std::thread::sleep(std::time::Duration::from_secs(3));
///            
///            let solution = 42; 
///            println!("Ready: {}", solution);
///
///            true 
///        }
///    );
/// }));
/// ```
///
/// #### Use with an existing function and variable capture:
///
/// ```rust
/// fn test_function_dialog(val1: &str, val2: &str) -> bool {
///		let status = run_command(&["sleep", "5"]);
///		match status {
///	    	Ok(s) => println!("Ends with code: {}", s),
///        	Err(e) => eprintln!("Error: {}", e),
///    	}
///		println!("Value 1: {}", val1);
///		println!("Value 2: {}", val2);
///		true
///	}
///	
///	let test1 = "Content".to_string();
///	let test2 = "Other Content".to_string();
///	
///	let btn_test_function_dialog = create_special_button::create_button_icon_position(
///        "alacarte-symbolic",
///        "Test Dialog that run function",
///        Align::Center,
///        move |btn| {
///            if let Some(window) = btn.root().and_downcast_ref::<adw::ApplicationWindow>() {
///                let p1 = test1.clone(); 
///                let p2 = test2.clone();
///
///                show_task_dialog(
///                    window, 
///                    "System Update",
///                    "Please wait...",
///                    IndicatorType::ProgressBar, // IndicatorType::Spinner
///                    move || { 
///                        test_function_dialog(&p1, &p2)
///                    }
///                );
///            }
///        }
///    );
/// ```
pub fn show_task_dialog<F>(
    parent: &adw::ApplicationWindow,
    title: &str,
    message: &str,
    indicator: IndicatorType,
    task: F,
) where 
    F: FnOnce() -> bool + Send + 'static 
{
    // create the dialog
    let dialog = Dialog::builder()
        .title(title)
        .content_width(400)
        .can_close(false)
        .build();

    let root_box = GtkBox::new(Orientation::Vertical, 18);
    root_box.set_margin_top(24);
    root_box.set_margin_bottom(24);
    root_box.set_margin_start(24);
    root_box.set_margin_end(24);
    root_box.set_halign(Align::Center);

    let label_title = Label::builder()
        .label(format!("<b>{}</b>", title))
        .use_markup(true)
        .build();
    
    let label_msg = Label::new(Some(message));

    root_box.append(&label_title);
    root_box.append(&label_msg);
    
    match indicator {
        IndicatorType::Spinner => {
            let spinner = Spinner::builder()
                .halign(Align::Center)
                .valign(Align::Center)
                .width_request(64)
                .height_request(64)
                .build();
            root_box.append(&spinner);
        }
        IndicatorType::ProgressBar => {
            let progress_bar = ProgressBar::builder()
                .pulse_step(0.1)
                .halign(Align::Fill)
                .width_request(250)
                .build();
        
            root_box.append(&progress_bar);
            
            glib::timeout_add_local(std::time::Duration::from_millis(100), move || {
                progress_bar.pulse();
                glib::ControlFlow::Continue
            });
        }
    }

    dialog.set_child(Some(&root_box));
    
    let (tx, rx) = mpsc::channel::<bool>();

    // start the function
    run_task_thread(task, tx);

    let dialog_to_close = dialog.clone();
    glib::timeout_add_local(std::time::Duration::from_millis(50), move || {
        match rx.try_recv() {
            Ok(_success) => {
                dialog_to_close.force_close();
                glib::ControlFlow::Break 
            }
            Err(mpsc::TryRecvError::Empty) => glib::ControlFlow::Continue,
            Err(mpsc::TryRecvError::Disconnected) => {
                dialog_to_close.force_close();
                glib::ControlFlow::Break
            }
        }
    });
    
    dialog.present(Some(parent));
}
