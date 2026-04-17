//! Function to show a dialog with a Libadwaita spinner
/**
* dialogs_spinner.rs
*
* (C) Copyright 2026 AtlantisOS Project
* by @NachtsternBuild
*
* License: GNU GENERAL PUBLIC LICENSE Version 3
*/

use adw::prelude::*;
use gtk4::{glib, ProgressBar, Align, Label, Orientation, Box as GtkBox};
use adw::{Spinner, Dialog};
use std::process::Command;
use std::thread;
use std::sync::mpsc;

pub enum IndicatorType {
    Spinner,
    ProgressBar,
}

/**
* @brief Function that run shell commands in a background thread and send signal to the gtk main loop
*/
fn run_commands_thread(commands: Vec<String>, tx: mpsc::Sender<bool>) {
    thread::spawn(move || {
        let mut all_success = true;

        for cmd_str in commands {
            let status = if cfg!(target_os = "windows") {
                Command::new("cmd").arg("/C").arg(&cmd_str).status()
            } 
            
            else {
                Command::new("sh").arg("-c").arg(&cmd_str).status()
            };

            if !status.map(|s| s.success()).unwrap_or(false) {
                all_success = false;
                break; // break at error
            }
        }

        // send the output to the main thread
        let _ = tx.send(all_success);
    });
}

/// Function that show a dialog with spinner/progressbar
/// ### Usage:
///
/// ```rust
///  button.connect_clicked(glib::clone!(@weak window => move |_| {
///  	show_spinner_dialog(
///		&window,
///  		"System-Update",
///  		"Please wait...",
///  		vec![
///  			"sleep 2".to_string(), 
///  			"echo 'Ready'".to_string()
///  		],
///  		IndicatorType::ProgressBar // or IndicatorType::Spinner
///  	);
///  }));
/// ```
pub fn show_spinner_dialog(
    parent: &adw::ApplicationWindow,
    title: &str,
    message: &str,
    commands: Vec<String>,
    indicator: IndicatorType,
) {
    // create dialog
    let dialog = Dialog::builder()
        .title(title)
        .content_width(400)
        .can_close(false) // prevent closing
        .build();

    // layout
    let root_box = GtkBox::new(Orientation::Vertical, 18);
    root_box.set_margin_top(24);
    root_box.set_margin_bottom(24);
    root_box.set_margin_start(24);
    root_box.set_margin_end(24);
    root_box.set_halign(Align::Center);

    let label_title = Label::builder()
        .label(title)
        .build();
    
    let label_msg = Label::new(Some(message));

    root_box.append(&label_title);
    root_box.append(&label_msg);
	
	// add indicator
    let spinner = Spinner::builder()
    .halign(Align::Center)
    .valign(Align::Center)
    .width_request(150)
    .height_request(150)
    .build();

	// start the spinner/progressbar by indicator
	match indicator {
    	IndicatorType::Spinner => {
    	    root_box.append(&spinner);
    	}
    	IndicatorType::ProgressBar => {
    	    let progress_bar = ProgressBar::builder()
    	        .pulse_step(0.1)
    	        .halign(Align::Center)
    	        .valign(Align::Center)
    	        .width_request(150)
    	        .height_request(50)
    	        .build();
        
    	    root_box.append(&progress_bar);
			
			// set timeout for animation
    	    glib::timeout_add_local(std::time::Duration::from_millis(100), move || {
    	        progress_bar.pulse();
    	        glib::ControlFlow::Continue
    	    });
    	}
	}

    dialog.set_child(Some(&root_box));
    
    // use the standard rust channel
    let (tx, rx) = mpsc::channel::<bool>();

    // run the command in the background thread
    run_commands_thread(commands, tx);

    // check the signal from the background thread
    let dialog_to_close = dialog.clone();
    gtk4::glib::timeout_add_local(std::time::Duration::from_millis(25), move || {
        // try_recv() does not block
        match rx.try_recv() {
            Ok(_success) => {
                dialog_to_close.force_close(); // force closing the dialog
                gtk4::glib::ControlFlow::Break // stop polling
            }
            Err(mpsc::TryRecvError::Empty) => {
                gtk4::glib::ControlFlow::Continue // Not finished yet; continue checking
            }
            Err(mpsc::TryRecvError::Disconnected) => {
                dialog_to_close.force_close(); // force closing the dialog
                gtk4::glib::ControlFlow::Break
            }
        }
    });
    
    // show dialog
    dialog.present(Some(parent));
}
