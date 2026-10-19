# User manual

This manual explains how to use AtlBase in a GTK4/Libadwaita project written in Rust.

## Installation

Add the crate to `Cargo.toml`:

```toml
[dependencies]
atlbase = "0.2"
# plus the same gtk4 / libadwaita versions that atlbase uses, if you use them yourself:
gtk4 = { version = "0.11", features = ["v4_20"] }
adw = { version = "0.9", package = "libadwaita", features = ["v1_8"] }
```

**Requirements** (build and run time):

- Rust 1.92 or newer
- GTK 4.20 or newer, Libadwaita 1.8 or newer
- VTE for GTK 4 (`libvte-2.91-gtk4`), only with the feature `log-viewer` (default)
- `gettext` (`msgfmt`) at build time, for the translations of the library

Install the development packages (Debian / Ubuntu):

```bash
sudo apt install build-essential pkg-config gettext libgtk-4-dev libadwaita-1-dev libvte-2.91-gtk4-dev
```

Fedora:

```bash
sudo dnf install gcc pkgconf gettext gtk4-devel libadwaita-devel vte291-gtk4-devel
```

Run the examples from a checkout:

```bash
cargo run --example test_app       # everything in one window
cargo run --example dialogs        # every dialog
cargo run --example guide          # multi-page guide
cargo run --example helper_tools   # helper functions, no window
```

## First application

```rust
use adw::prelude::*;
use atlbase::{gettext, prelude::*, ui_prelude::*};

fn main() -> glib::ExitCode {
    let app = adw::Application::builder()
        .application_id("org.example.Demo")
        .build();

    app.connect_activate(|app| {
        init_syslog("demo").ok();                          // log to syslog / journal
        init_language("demo", "/usr/share/locale", false); // translations
        use_adw_provider("button { border-radius: 12px; }");

        let button = create_button(&gettext!("Hello"), |button| {
            show_info_dialog(button, &gettext!("Hello from AtlBase"));
        });
        let window = adw::ApplicationWindow::builder()
            .application(app)
            .content(&button)
            .build();
        window.present();
    });
    app.run()
}
```

## Translations

1. Wrap every text of your interface in `gettext!("…")`.
2. Collect the texts into `po/<domain>.pot` and translate them in `po/<lang>.po`.
   `xgettext` does not know Rust macros, but this works:

   ```bash
   cat src/*.rs | sed 's/gettext!(/gettext(/g' \
     | xgettext -L C --keyword=gettext --from-code=UTF-8 -o po/demo.pot -
   ```
3. Compile them to `<LOCALEDIR>/<lang>/LC_MESSAGES/<domain>.mo` and call
   `init_language(domain, LOCALEDIR, false)` at start-up.

The texts of the library itself (dialog buttons, the log viewer, the navigation of guides, …)
are translated automatically: the catalogs are part of the library.

## Guides

`design::carousel` shows instructions and tours as a swipeable multi-page widget. Write
the content in a TOML file (see `examples/assets/guide.toml`) and show it:

```rust
let guide = GuideLoader::new().load_resource("/org/example/App/guides/intro.toml")?;
GuideView::present_in_stack(&stack, "intro", &guide, "home");
```

A guide can also do something. A `button` block reports its click to your application, so a
setup assistant can be a guide, too:

```toml
[[pages.blocks]]
button = "Choose folder"
action = "choose-folder"
icon = "folder-open-symbolic"
```

```rust
let view = GuideView::present_in_stack(&stack, "setup", &guide, "home");
view.connect_action(|_view, action| {
    if action == "choose-folder" { /* open a folder chooser */ }
});
```

## Packaging

How to build deb, rpm, Flatpak, Snap and AppImage packages of an application that uses
AtlBase is described in [PACKAGING.md](PACKAGING.md).
