// guide.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! A complete program with a multi-page guide (module `carousel`).
//!
//! ```text
//! cargo run --example guide             # content from examples/assets/guide.toml
//! cargo run --example guide -- builder  # the same content, built in code
//! cargo run --example guide -- embedded # guide embedded in the binary, images installed
//! ```
//!
//! The `embedded` mode behaves like an installed application: the TOML text is part of the
//! binary (in a real application usually a GResource) and the images are looked up in the
//! XDG data directories, which covers deb/rpm (`/usr/share`), Flatpak (`/app/share`),
//! Snap (`$SNAP/usr/share`) and AppImage (`$APPDIR/usr/share`). Try it without installing:
//!
//! ```text
//! mkdir -p /tmp/demo/atlbase-guide-demo
//! cp examples/assets/treble-check.png /tmp/demo/atlbase-guide-demo/
//! XDG_DATA_DIRS=/tmp/demo:/usr/local/share:/usr/share cargo run --example guide -- embedded
//! ```
//!
//! Without the copied image the placeholder "Image not found" appears and a warning lists the
//! places that were searched.

use adw::prelude::*;
use atlbase::gettext;
use atlbase::prelude::*;
use atlbase::ui_prelude::*;
use glib::clone;
use std::path::PathBuf;

/// Your own adjustments: an additional provider that overrides the default style.
const CUSTOM_CSS: &str = r#"
.adw-guide .guide-step-number { background-color: @purple_3; color: white; }
.adw-guide .guide-callout-tip { background-color: alpha(@purple_3, 0.22); }
"#;

#[derive(Clone, Copy)]
enum Mode {
    Toml,
    Builder,
    Embedded,
}

/// The guide, loaded from a TOML file next to which the images lie.
fn guide_from_toml() -> Guide {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/assets/guide.toml");
    GuideLoader::new()
        .load_file(&path)
        .unwrap_or_else(|e| panic!("{e}"))
}

/// The guide as embedded text; the images come from the data directories of the system.
fn guide_embedded() -> Guide {
    GuideLoader::new()
        .image_data_subdir("atlbase-guide-demo")
        .load_str(include_str!("assets/guide.toml"))
        .unwrap_or_else(|e| panic!("{e}"))
}

/// The first two pages again, this time built in code.
fn guide_from_code() -> Guide {
    let treble = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/examples/assets/treble-check.png"
    );

    Guide::new(gettext!("GSI"))
        .labels(Labels::localized())
        .page(
            Page::new(gettext!("Notes"))
                .icon("dialog-information-symbolic")
                .paragraph(gettext!("GSI (Generic System Image) is an Android system image that requires support for Project Treble."))
                .callout(
                    Callout::tip(gettext!("You can check Treble support from a terminal:"))
                        .title(gettext!("Is my device compatible?")),
                )
                .code("adb shell getprop ro.treble.enabled")
                .image(
                    Image::from_file(treble)
                        .alt(gettext!("Terminal showing the output of the Treble check"))
                        .caption(gettext!("Output on a Treble-compatible device: \"true\"")),
                ),
        )
        .page(
            Page::new(gettext!("GSI Flashing"))
                .subtitle(gettext!("Preparation"))
                .steps([
                    Step::new(gettext!("Download a corresponding GSI.")),
                    Step::new(gettext!("Boot your device in Fastboot.")),
                    Step::new(gettext!("Open the bootloader."))
                        .detail(gettext!("Some devices do not support the Fastboot method.")),
                ])
                .callout(Callout::warning(gettext!("All user data will be deleted."))),
        )
}

fn guide(mode: Mode) -> Guide {
    match mode {
        Mode::Toml => guide_from_toml(),
        Mode::Builder => guide_from_code(),
        Mode::Embedded => guide_embedded(),
    }
}

fn build_ui(app: &adw::Application, mode: Mode) {
    // translations of the library (and of the application, if it had its own catalog)
    init_language("atlbase-guide-demo", env!("COMPILED_LOCALE_DIR"), false);

    // the window shows a GtkStack; every guide becomes one page of it
    let stack = gtk4::Stack::builder()
        .transition_type(gtk4::StackTransitionType::SlideLeftRight)
        .build();

    let buttons = gtk4::Box::builder()
        .orientation(gtk4::Orientation::Vertical)
        .spacing(12)
        .halign(gtk4::Align::Center)
        .build();
    let open = create_button("Open the guide", |_| {});
    buttons.append(&open);

    let start = adw::StatusPage::builder()
        .icon_name("help-browser-symbolic")
        .title("Guide demo")
        .description("Module carousel of AtlBase")
        .child(&buttons)
        .build();
    stack.add_named(&start, Some("start"));

    // "Exit" on the first and "Done" on the last page return to the page "start"
    open.connect_clicked(clone!(
        #[weak]
        stack,
        move |_| {
            GuideView::present_in_stack(&stack, "guide", &guide(mode), "start");
        }
    ));

    // a stylesheet of our own wins over the default style of the guide
    buttons.append(&create_button("Use purple accents", |_| {
        use_adw_provider(CUSTOM_CSS);
    }));

    let window = adw::ApplicationWindow::builder()
        .application(app)
        .title("AtlBase guide")
        .default_width(520)
        .default_height(720)
        .build();

    // the same guide in a dialog
    buttons.append(&create_button(
        "Open as dialog",
        clone!(
            #[weak]
            window,
            move |_| {
                GuideView::present_dialog(&guide(mode), &window);
            }
        ),
    ));

    let content = gtk4::Box::builder()
        .orientation(gtk4::Orientation::Vertical)
        .build();
    content.append(&adw::HeaderBar::new());
    content.append(&stack);
    window.set_content(Some(&content));
    window.present();
}

fn main() -> glib::ExitCode {
    let mode = match std::env::args().nth(1).as_deref() {
        Some("builder") => Mode::Builder,
        Some("embedded") => Mode::Embedded,
        _ => Mode::Toml,
    };

    let app = adw::Application::builder()
        .application_id("org.atlantisos.AtlBaseGuideDemo")
        .build();
    app.connect_activate(move |app| build_ui(app, mode));
    // do not pass "builder"/"embedded" on to GApplication, it does not know them
    app.run_with_args::<&str>(&[])
}
