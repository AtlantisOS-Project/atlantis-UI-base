//! A dialog that run function
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

// Startet die übergebene Funktion in einem Hintergrund-Thread
fn run_task_thread<F>(task: F, tx: mpsc::Sender<bool>) 
where 
    F: FnOnce() -> bool + Send + 'static 
{
    thread::spawn(move || {
        let success = task();
        // Ergebnis an den Main-Loop senden
        let _ = tx.send(success);
    });
}

/// Zeigt einen Dialog, während eine Rust-Funktion im Hintergrund läuft
///
/// ### Usage:
/// #### Example with code in the call
/// ```rust
/// button.connect_clicked(glib::clone!(@weak window => move |_| {
///    show_task_dialog(
///        &window,
///        "Datenverarbeitung",
///        "Berechne komplexe Daten...",
///        IndicatorType::Spinner,
///        || {
///            // Hier steht dein Rust-Code
///            // Beispiel: Eine schwere Berechnung oder Datei-IO
///            std::thread::sleep(std::time::Duration::from_secs(3));
///            
///            let ergebnis = 42; // Deine Logik hier
///            println!("Berechnung fertig: {}", ergebnis);
///
///            true // Gibt Erfolg (true/false) zurück
///        }
///    );
/// }));
/// ```
///
/// #### Example with function call
///
/// ```rust
/// button.connect_clicked(glib::clone!(@weak window => move |_| {
///    // Falls diese Variablen Strings oder Pfade sind:
///    let p1 = partition1.clone(); 
///    let p2 = partition2.clone();
///    let img = image_name.clone();
///
///    show_task_dialog(
///        &window,
///        "System wird geflasht",
///        "Bitte das Gerät nicht ausschalten...",
///        IndicatorType::ProgressBar,
///        move || { // <--- WICHTIG: 'move' für den Thread-Transfer
///            // Hier wird die eigentliche Arbeit erledigt
///            let result = flash_image(p1, p2, img, optional_flags);
///            
///            // Wenn flash_image() z.B. einen Result zurückgibt, 
///            // wandeln wir das in einen bool für den Dialog um:
///            result.is_ok() 
///        }
///    );
/// }));
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
    // Dialog Erstellung
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

    // Starte die übergebene Rust-Funktion
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
