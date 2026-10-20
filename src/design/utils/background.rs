// background.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! Slow work without a frozen window.

/// Runs `work` in a background thread and then `done` with its result in the main loop.
///
/// A command that takes a moment (asking a device, reading a big file) must not run in the
/// main loop: the window would not react until it is finished. Do the slow part in `work`
/// and show the result in `done`. `work` runs on another thread, so it cannot touch widgets;
/// `done` runs on the main thread and can.
///
/// Call it from the main thread, because it needs the main loop of GTK. If `work` panics,
/// the error is logged and `done` is not called.
///
/// # Examples
///
/// ```no_run
/// use gtk4::prelude::*;
/// use atlbase::ui_prelude::run_background;
/// # gtk4::init().unwrap();
/// # let label = gtk4::Label::new(None);
///
/// run_background(
///     || std::fs::read_to_string("/proc/version").unwrap_or_default(),
///     move |text| label.set_text(&text),
/// );
/// ```
pub fn run_background<T: Send + 'static>(
    work: impl FnOnce() -> T + Send + 'static,
    done: impl FnOnce(T) + 'static,
) {
    glib::spawn_future_local(async move {
        match gio::spawn_blocking(work).await {
            Ok(result) => done(result),
            Err(_) => log::error!("a background job panicked"),
        }
    });
}
