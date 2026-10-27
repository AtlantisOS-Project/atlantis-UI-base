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
use atlbase::{prelude::command_line, ui_prelude::*};
use std::cell::{Cell, RefCell};
use std::path::PathBuf;
use std::rc::Rc;
use std::time::{Duration, Instant};

fn main() {
    // the private icon theme of `ensure_app_icon` goes to the cache directory
    let cache = tempfile::tempdir().unwrap();
    // SAFETY: nothing else runs yet.
    unsafe { std::env::set_var("XDG_CACHE_HOME", cache.path()) };

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
        ("steps with a huge number", steps_with_a_huge_number),
        ("guide in a stack", guide_in_a_stack),
        ("guide with a missing image", guide_with_missing_image),
        ("guide with a button", guide_with_button),
        ("page layout", page_layout),
        ("button grid", button_grid),
        ("lazy stack pages", lazy_stack_pages),
        ("stack window", stack_window),
        ("form dialog", form_dialog),
        ("pick row", pick_row),
        ("confirm dialog", confirm_dialog),
        ("background work", background_work),
        ("work dialog", work_dialog),
        ("command task", command_task),
        ("app icon", app_icon),
        ("progress dialog results", progress_dialog_results),
        ("choosers", choosers),
        ("buttons", buttons),
        ("dialogs are freed", dialogs_are_freed),
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

    // the handler that reloads a provider on a change of the style goes away with the provider
    let manager = adw::StyleManager::default();
    let notify = glib::subclass::SignalId::lookup("notify", glib::Object::static_type()).unwrap();
    let dark = Some(glib::Quark::from_str("dark"));
    let handlers = || glib::signal::signal_has_handler_pending(&manager, notify, dark, true);
    manager.set_color_scheme(adw::ColorScheme::ForceLight);
    // (the handlers of the two providers above are there)
    assert!(handlers());

    let display = gtk4::gdk::Display::default().unwrap();
    for provider in [&first, &second] {
        gtk4::style_context_remove_provider_for_display(&display, provider);
    }
    drop((first, second));
    // the handlers see on the next change that their providers are gone
    manager.set_color_scheme(adw::ColorScheme::ForceDark);
    manager.set_color_scheme(adw::ColorScheme::ForceLight);
    assert!(
        !handlers(),
        "handlers of providers that are gone stay connected"
    );
    manager.set_color_scheme(adw::ColorScheme::Default);
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

fn steps_with_a_huge_number() {
    // the number comes from a file; counting up from the largest one must not overflow
    let page = Page::new("Steps").steps_from(u32::MAX - 1, ["one", "two", "three"]);
    let view = GuideView::new(&Guide::new("Tour").page(page));
    assert_eq!(view.n_pages(), 1);
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

/// Runs the main loop until `done` is true; fails after ten seconds.
fn wait_until(what: &str, done: impl Fn() -> bool) {
    let context = glib::MainContext::default();
    let start = Instant::now();
    while !done() {
        assert!(start.elapsed() < Duration::from_secs(10), "timeout: {what}");
        context.iteration(false);
        std::thread::sleep(Duration::from_millis(5));
    }
}

/// The title of the dialog that is shown over `window`.
fn dialog_title(window: &adw::Window) -> Option<String> {
    window
        .visible_dialog()
        .map(|dialog| dialog.title().to_string())
}

fn page_layout() {
    let container = create_page_container();
    assert_eq!(container.orientation(), gtk4::Orientation::Vertical);

    let rows = create_rows_box();
    let heading = create_heading("Devices");
    assert!(heading.has_css_class("title-3"));
    rows.append(&heading);
    let text = create_text_label("A long text that wraps");
    assert!(text.wraps() && text.halign() == gtk4::Align::Start);
    rows.append(&text);

    let clicks = Rc::new(Cell::new(0));
    let counter = clicks.clone();
    let main = create_pill_button("go-previous-symbolic", "Back", true, move |_| {
        counter.set(counter.get() + 1)
    });
    assert!(main.has_css_class("pill") && main.has_css_class("suggested-action"));
    main.emit_clicked();
    assert_eq!(clicks.get(), 1);
    let plain = create_pill_button("go-previous-symbolic", "Back", false, |_| {});
    assert!(!plain.has_css_class("suggested-action"));

    let page = create_scrolling_page(&rows, &plain);
    assert_eq!(page.observe_children().n_items(), 2);
    assert!(plain.parent().is_some_and(|parent| parent == page));
}

fn button_grid() {
    let clicked = Rc::new(RefCell::new(Vec::new()));
    let entries: Vec<MenuEntry> = ["One", "Two", "Three"]
        .into_iter()
        .map(|name| {
            let log = clicked.clone();
            MenuEntry::new("go-next-symbolic", name, move |_| {
                log.borrow_mut().push(name)
            })
        })
        .collect();

    let style = MenuGridStyle {
        css_class: "big",
        ..MenuGridStyle::default()
    };
    let grid = create_button_grid(&entries, 2, style);

    // two columns: the third entry is in the second row
    let second = grid.child_at(1, 0).expect("a second column");
    assert!(grid.child_at(0, 1).is_some());
    assert!(grid.child_at(1, 1).is_none());

    let button = second.downcast::<gtk4::Button>().unwrap();
    assert!(button.has_css_class("big"));
    assert_eq!(button.tooltip_text().as_deref(), Some("Two"));
    button.emit_clicked();
    assert_eq!(*clicked.borrow(), ["Two"]);

    // no CSS class and no column do not break anything
    let plain = create_button_grid(&entries, 0, MenuGridStyle::default());
    assert!(plain.child_at(0, 2).is_some(), "0 columns means 1 column");
}

fn page_builder(built: &Rc<Cell<u32>>) -> impl FnOnce() -> gtk4::Widget + use<> {
    let built = built.clone();
    move || {
        built.set(built.get() + 1);
        gtk4::Label::new(Some("page")).upcast()
    }
}

fn lazy_stack_pages() {
    let stack = gtk4::Stack::new();
    let built = Rc::new(Cell::new(0));

    show_stack_page(&stack, "home", false, page_builder(&built));
    show_stack_page(&stack, "home", false, page_builder(&built));
    assert_eq!(built.get(), 1, "a static page is built once");

    show_stack_page(&stack, "live", true, page_builder(&built));
    let first = stack.child_by_name("live").unwrap();
    show_stack_page(&stack, "live", true, page_builder(&built));
    assert_eq!(built.get(), 3, "a live page is built on every visit");
    assert_ne!(first, stack.child_by_name("live").unwrap());
    assert_eq!(stack.visible_child_name().as_deref(), Some("live"));
}

fn stack_window() {
    let app = adw::Application::builder()
        .application_id("org.atlantis.AtlbaseTest")
        .build();
    // windows may only be added after the start-up of the application
    app.register(gio::Cancellable::NONE).unwrap();
    let window = StackWindow::new(&app, "Demo", 640, 480);
    let built = Rc::new(Cell::new(0));

    window.present(|stack| show_stack_page(stack, "home", false, page_builder(&built)));
    assert_eq!(window.stack.visible_child_name().as_deref(), Some("home"));
    assert_eq!(
        window.stack.transition_type(),
        gtk4::StackTransitionType::None,
        "no animation to the first page"
    );
    wait_until("the animation is back", || {
        window.stack.transition_type() == gtk4::StackTransitionType::SlideLeftRight
    });
    window.window.close();
}

fn form_dialog() {
    let window = adw::Window::new();
    let form = FormDialog::new("Backup", "Start");

    let group = form.group(Some("Partition"));
    let partition = create_combo_row("Partition", &["boot".to_string(), "vendor".to_string()]);
    let root = create_switch_row("Root", "Use root access");
    group.add(&partition);
    group.add(&root);
    assert_eq!(selected_index(&partition), 0);
    assert!(!root.is_active());

    let changes = Rc::new(Cell::new(0));
    let counter = changes.clone();
    on_selection_change(&partition, move || counter.set(counter.get() + 1));
    partition.set_selected(1);
    assert_eq!((selected_index(&partition), changes.get()), (1, 1));

    let started = Rc::new(Cell::new(0));
    let counter = started.clone();
    form.on_ok(move || counter.set(counter.get() + 1));
    form.present(&window);
    assert_eq!(dialog_title(&window).as_deref(), Some("Backup"));

    let ok = find_button(form.widget().child().as_ref().unwrap(), "suggested-action")
        .expect("the dialog has a main button");
    assert_eq!(ok.label().as_deref(), Some("Start"));
    form.set_ok_enabled(false);
    assert!(!ok.is_sensitive());
    form.set_ok_enabled(true);
    form.set_ok_visible(false);
    assert!(!ok.is_visible());
    form.set_ok_visible(true);

    ok.emit_clicked();
    assert_eq!(started.get(), 1);
}

fn pick_row() {
    let seen = Rc::new(RefCell::new(Vec::<PathBuf>::new()));
    let log = seen.clone();
    let row = PickRow::new("Image", Pick::File, None, move |paths| {
        *log.borrow_mut() = paths.to_vec()
    });
    assert!(row.path().is_none() && row.paths().is_empty());
    assert_eq!(
        row.row.subtitle().as_deref().unwrap_or_default(),
        "Nothing chosen"
    );

    row.set_paths(vec![PathBuf::from("/a/boot.img")]);
    assert_eq!(row.path(), Some(PathBuf::from("/a/boot.img")));
    assert_eq!(
        row.row.subtitle().as_deref().unwrap_or_default(),
        "/a/boot.img"
    );
    assert_eq!(*seen.borrow(), [PathBuf::from("/a/boot.img")]);

    row.set_paths(
        ["/a/1.img", "/a/2.img", "/a/3.img"]
            .map(PathBuf::from)
            .to_vec(),
    );
    assert_eq!(
        row.row.subtitle().as_deref().unwrap_or_default(),
        "/a/1.img (+2)"
    );
    assert_eq!(row.paths().len(), 3);

    // another kind of choice starts again
    row.set_pick(Pick::Folder, "Folder");
    assert!(row.paths().is_empty() && seen.borrow().is_empty());
    assert_eq!(row.row.title().as_str(), "Folder");
}

fn confirm_dialog() {
    let window = adw::Window::new();
    let confirmed = Rc::new(Cell::new(0));
    let counter = confirmed.clone();
    show_confirm_dialog(&window, "Delete?", "Gone forever.", "Delete", move || {
        counter.set(counter.get() + 1)
    });

    let dialog = window
        .visible_dialog()
        .and_downcast::<adw::AlertDialog>()
        .expect("an alert dialog is shown");
    assert_eq!(dialog.heading().as_deref(), Some("Delete?"));
    assert_eq!(dialog.default_response().as_deref(), Some("cancel"));
    assert_eq!(
        dialog.response_appearance("confirm"),
        adw::ResponseAppearance::Destructive
    );

    dialog.emit_by_name::<()>("response", &[&"cancel"]);
    assert_eq!(confirmed.get(), 0, "cancel does nothing");
    dialog.emit_by_name::<()>("response", &[&"confirm"]);
    assert_eq!(confirmed.get(), 1);
}

fn background_work() {
    let main_thread = std::thread::current().id();
    let result = Rc::new(RefCell::new(None));
    let slot = result.clone();
    run_background(
        move || (6 * 7, std::thread::current().id() != main_thread),
        move |value| {
            // the result arrives on the main thread again
            assert_eq!(std::thread::current().id(), main_thread);
            *slot.borrow_mut() = Some(value);
        },
    );
    wait_until("the background work", || result.borrow().is_some());
    assert_eq!(*result.borrow(), Some((42, true)));
}

fn work_dialog() {
    let window = adw::Window::new();

    show_work_dialog(&window, "Copying", "Please wait...", || {
        Ok("It worked.".into())
    });
    assert_eq!(dialog_title(&window).as_deref(), Some("Copying"));
    wait_until("the success dialog", || {
        dialog_title(&window).as_deref() == Some("Success")
    });

    show_work_dialog(&window, "Copying", "Please wait...", || {
        Err("Disk full".into())
    });
    wait_until("the error dialog", || {
        dialog_title(&window).as_deref() == Some("Error")
    });
}

fn command_task() {
    let window = adw::Window::new();

    // without commands nothing happens
    CommandTask::new("Empty", "...", Vec::new()).run(&window);
    assert!(window.visible_dialog().is_none());

    CommandTask::new(
        "Check",
        "Please wait...",
        vec![command_line("true", ["-x"])],
    )
    .success_message("Done.")
    .run(&window);
    assert_eq!(dialog_title(&window).as_deref(), Some("Check"));
    wait_until("the success dialog", || {
        dialog_title(&window).as_deref() == Some("Success")
    });

    CommandTask::new(
        "Check",
        "Please wait...",
        vec![command_line("sh", ["-c", "echo locked >&2; exit 1"])],
    )
    .explain_errors(|error| {
        error
            .contains("locked")
            .then(|| "It is locked.".to_string())
    })
    .run(&window);
    wait_until("the error dialog", || {
        dialog_title(&window).as_deref() == Some("Error")
    });

    CommandTask::new("Output", "...", vec![command_line("echo", ["hello"])])
        .show_output("Result")
        .run(&window);
    wait_until("the output dialog", || {
        dialog_title(&window).as_deref() == Some("Result")
    });
}

fn has_icon(name: &str) -> bool {
    gtk4::IconTheme::for_display(&gtk4::gdk::Display::default().unwrap()).has_icon(name)
}

const SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="64" height="64"><rect width="64" height="64" fill="#3d6fb0"/></svg>"##;

fn app_icon() {
    let dir = tempfile::tempdir().unwrap();
    let cache = glib::user_cache_dir();

    // an icon that exists nowhere is not found, and nothing breaks
    assert!(!ensure_app_icon("org.atlantis.NoSuchIcon", None));
    assert!(!ensure_app_icon(
        "org.atlantis.NoSuchIcon",
        Some(&dir.path().join("missing.svg"))
    ));

    // from a file in the source tree: an SVG goes to `scalable`
    let svg = dir.path().join("icon.svg");
    std::fs::write(&svg, SVG).unwrap();
    assert!(!has_icon("org.atlantis.FromSvg"));
    assert!(ensure_app_icon("org.atlantis.FromSvg", Some(&svg)));
    assert!(has_icon("org.atlantis.FromSvg"));
    assert!(
        cache
            .join("atlbase/icons/hicolor/scalable/apps/org.atlantis.FromSvg.svg")
            .is_file()
    );

    // a PNG goes to the standard size that is closest to its width
    let png = dir.path().join("icon.png");
    gtk4::gdk_pixbuf::Pixbuf::new(gtk4::gdk_pixbuf::Colorspace::Rgb, true, 8, 60, 60)
        .unwrap()
        .savev(&png, "png", &[])
        .unwrap();
    assert!(ensure_app_icon("org.atlantis.FromPng", Some(&png)));
    assert!(
        cache
            .join("atlbase/icons/hicolor/64x64/apps/org.atlantis.FromPng.png")
            .is_file()
    );

    // an AppDir: <APPDIR>/usr/share/icons/hicolor/…
    let apps = dir
        .path()
        .join("AppDir/usr/share/icons/hicolor/scalable/apps");
    std::fs::create_dir_all(&apps).unwrap();
    std::fs::write(apps.join("org.atlantis.InAppDir.svg"), SVG).unwrap();
    assert!(!has_icon("org.atlantis.InAppDir"));
    // SAFETY: the test runs on the main thread and nothing else reads the variable.
    unsafe { std::env::set_var("APPDIR", dir.path().join("AppDir")) };
    assert!(ensure_app_icon("org.atlantis.InAppDir", None));
    unsafe { std::env::remove_var("APPDIR") };

    // the about dialog does the same with its own fields
    let window = adw::Window::new();
    show_about_dialog(
        &window,
        AboutInfo {
            app_icon: "org.atlantis.InAbout".into(),
            icon_file: Some(svg),
            app_name: "Demo".into(),
            ..AboutInfo::default()
        },
    );
    assert!(window.visible_dialog().is_some());
    assert!(has_icon("org.atlantis.InAbout"));
}

/// Runs `show` with a callback and waits until the callback was called; returns what it got.
fn wait_for_callback<T: 'static>(show: impl FnOnce(Box<dyn FnOnce(T)>)) -> T {
    let slot = Rc::new(RefCell::new(None));
    let writer = slot.clone();
    show(Box::new(move |value| *writer.borrow_mut() = Some(value)));
    wait_until("the callback", || slot.borrow().is_some());
    slot.borrow_mut().take().unwrap()
}

fn progress_dialog_results() {
    let window = adw::Window::new();

    // the text of show_spinner_dialog: stdout and stderr of every command
    let ok = wait_for_callback(|done| {
        show_spinner_dialog(
            &window,
            "Run",
            "...",
            vec![
                command_line("echo", ["one"]),
                command_line("sh", ["-c", "echo err >&2"]),
            ],
            IndicatorType::Spinner,
            Some(Box::new(done)),
        )
    });
    assert_eq!(ok.unwrap(), "one\n\n[Stderr]: err\n");
    assert!(window.visible_dialog().is_none() || dialog_title(&window).as_deref() != Some("Run"));

    // a command that fails: the command and what it printed
    let failed = wait_for_callback(|done| {
        show_spinner_dialog(
            &window,
            "Run",
            "...",
            vec![command_line("sh", ["-c", "echo out; exit 1"])],
            IndicatorType::ProgressBar,
            Some(Box::new(done)),
        )
    });
    assert_eq!(
        failed.unwrap_err(),
        "Command failed: sh -c echo out; exit 1\nOutput:\nout\n"
    );

    // a program that does not exist
    let missing = wait_for_callback(|done| {
        show_spinner_dialog(
            &window,
            "Run",
            "...",
            vec![command_line("no-such-program-xyz", Vec::<String>::new())],
            IndicatorType::Spinner,
            Some(Box::new(done)),
        )
    });
    let error = missing.unwrap_err();
    assert!(
        error.starts_with("Failed to execute command 'no-such-program-xyz': "),
        "{error}"
    );

    // the variant with the result of every command
    let results = wait_for_callback(|done| {
        show_spinner_dialog_return_output(
            &window,
            "Run",
            "...",
            vec![
                command_line("echo", ["a"]),
                command_line("false", Vec::<String>::new()),
            ],
            IndicatorType::Spinner,
            Some(Box::new(done)),
        )
    });
    let results = results.unwrap_err();
    assert_eq!(results.len(), 2);
    assert!(results[0].success && !results[1].success);

    // a task: the dialog closes when it is done
    let done = Rc::new(Cell::new(false));
    let flag = done.clone();
    show_task_dialog(&window, "Task", "...", IndicatorType::Spinner, move || true);
    assert_eq!(dialog_title(&window).as_deref(), Some("Task"));
    wait_until("the task dialog closes", || {
        window.visible_dialog().is_none()
    });
    flag.set(true);
    assert!(done.get());
}

fn choosers() {
    // the dialogs open (they need a window; nothing is chosen, nothing happens)
    let button = gtk4::Button::new();
    let window = gtk4::Window::new();
    window.set_child(Some(&button));
    show_file_chooser(&button, |_| panic!("nothing was chosen"));
    show_files_chooser(&button, |_| panic!("nothing was chosen"));
    show_folder_chooser(&button, |_| panic!("nothing was chosen"));
    show_chooser(
        &button,
        Pick::Folder,
        Some(std::path::Path::new("/tmp")),
        |_| {},
    );
}

fn buttons() {
    // all the icon buttons are built the same way
    let plain = create_button_icon_no_callback("go-next-symbolic", "Next");
    assert!(plain.has_css_class("pill") && plain.has_css_class("suggested-action"));
    let two = create_button_two_icon(
        Some("go-previous-symbolic"),
        "go-next-symbolic",
        "Both",
        |_| {},
    );
    assert!(two.has_css_class("pill") && two.has_css_class("suggested-action"));
    let without_first = create_button_two_icon(None, "go-next-symbolic", "One", |_| {});
    assert!(without_first.child().is_some());

    // the standard pill button of Libadwaita: not suggested unless asked
    let pill = create_pill_button("go-next-symbolic", "Next", false, |_| {});
    assert!(pill.has_css_class("pill") && !pill.has_css_class("suggested-action"));
    assert!(
        create_pill_button("go-next-symbolic", "Next", true, |_| {})
            .has_css_class("suggested-action")
    );

    // the icon and the label next to each other
    let label = create_label_icon("go-next-symbolic", "Text");
    assert_eq!(label.observe_children().n_items(), 2);
}

// ───────────────────────── Memory ─────────────────────────

/// Lets GTK free what is waiting to be freed.
fn settle() {
    let context = glib::MainContext::default();
    for _ in 0..500 {
        if !context.iteration(false) {
            break;
        }
    }
}

/// Every widget in the dialogs of `window`, and the dialogs themselves.
fn weak_widgets_of_dialogs(window: &adw::Window) -> Vec<(String, glib::WeakRef<gtk4::Widget>)> {
    fn walk(widget: &gtk4::Widget, out: &mut Vec<(String, glib::WeakRef<gtk4::Widget>)>) {
        out.push((widget.type_().name().to_string(), widget.downgrade()));
        let mut child = widget.first_child();
        while let Some(current) = child {
            child = current.next_sibling();
            walk(&current, out);
        }
    }
    let mut out = Vec::new();
    let dialogs = window.dialogs();
    for index in 0..dialogs.n_items() {
        if let Some(dialog) = dialogs.item(index).and_downcast::<gtk4::Widget>() {
            walk(&dialog, &mut out);
        }
    }
    out
}

/// Closes the dialogs of `window` and returns the kinds of widgets that are still alive.
fn alive_after_closing(
    window: &adw::Window,
    weak: Vec<(String, glib::WeakRef<gtk4::Widget>)>,
) -> Vec<String> {
    while let Some(dialog) = window.visible_dialog() {
        dialog.force_close();
    }
    settle();
    weak.into_iter()
        .filter(|(_, widget)| widget.upgrade().is_some())
        .map(|(name, _)| name)
        .collect()
}

/// A dialog that is closed must be freed with everything in it. A closure that keeps the widget it
/// is connected to alive (`row.connect_changed(move |_| row.…)`) would leak the whole dialog each
/// time it is opened.
fn dialogs_are_freed() {
    type Show = Box<dyn Fn(&adw::Window)>;
    let cases: Vec<(&str, Show)> = vec![
        ("info", Box::new(|w| show_info_dialog(w, "Saved"))),
        ("error", Box::new(|w| show_error_dialog(w, "Disk full"))),
        ("success", Box::new(|w| show_success_dialog(w, "Installed"))),
        (
            "warning",
            Box::new(|w| show_warning_dialog(w, "Battery low")),
        ),
        (
            "confirm",
            Box::new(|w| show_confirm_dialog(w, "Delete?", "Gone.", "Delete", || {})),
        ),
        (
            "entry",
            Box::new(|w| {
                show_entry_dialog(w, "Name", "Enter:", "OK", "Cancel", "Name:", "x", |_| {});
            }),
        ),
        (
            "custom content",
            Box::new(|w| {
                let content = gtk4::Box::new(gtk4::Orientation::Vertical, 6);
                content.append(&gtk4::Label::new(Some("content")));
                show_custom_content_dialog(w, "Custom", "Body", "OK", &content);
            }),
        ),
        (
            "about",
            Box::new(|w| {
                show_about_dialog(
                    w,
                    AboutInfo {
                        app_name: "Demo".into(),
                        ..AboutInfo::default()
                    },
                )
            }),
        ),
        (
            "form with rows",
            Box::new(|w| {
                let form = FormDialog::new("Form", "Start");
                let group = form.group(Some("Group"));
                let combo = create_combo_row("Choice", &["a".to_string(), "b".to_string()]);
                let switch = create_switch_row("Switch", "Subtitle");
                let pick = PickRow::new("File", Pick::File, None, |_| {});
                // the pattern the documentation asks for: the closure that is connected to a
                // row holds the row weakly
                let ok = form.ok_button();
                let update = Rc::new(glib::clone!(
                    #[weak]
                    combo,
                    #[weak]
                    ok,
                    move || ok.set_sensitive(selected_index(&combo) == 0)
                ));
                on_selection_change(&combo, {
                    let update = update.clone();
                    move || update()
                });
                group.add(&combo);
                group.add(&switch);
                group.add(&pick.row);
                form.on_ok(move || {});
                form.present(w);
            }),
        ),
    ];

    for (name, show) in &cases {
        let window = adw::Window::new();
        show(&window);
        let weak = weak_widgets_of_dialogs(&window);
        assert!(!weak.is_empty(), "{name}: no dialog was shown");
        let alive = alive_after_closing(&window, weak);
        assert!(alive.is_empty(), "{name}: not freed: {alive:?}");
    }

    // the dialogs with a thread behind them show their result when the work is done
    let window = adw::Window::new();
    show_work_dialog(&window, "Copying", "Please wait...", || {
        Ok("It worked.".into())
    });
    wait_until("the success dialog", || {
        dialog_title(&window).as_deref() == Some("Success")
    });
    let weak = weak_widgets_of_dialogs(&window);
    let alive = alive_after_closing(&window, weak);
    assert!(alive.is_empty(), "work dialog: not freed: {alive:?}");

    let window = adw::Window::new();
    CommandTask::new(
        "Check",
        "Please wait...",
        vec![command_line("true", ["-x"])],
    )
    .success_message("Done.")
    .run(&window);
    wait_until("the success dialog", || {
        dialog_title(&window).as_deref() == Some("Success")
    });
    let weak = weak_widgets_of_dialogs(&window);
    let alive = alive_after_closing(&window, weak);
    assert!(alive.is_empty(), "command task: not freed: {alive:?}");
}
