# AtlBase

[![crates.io](https://img.shields.io/crates/v/atlbase.svg)](https://crates.io/crates/atlbase)
[![docs.rs](https://docs.rs/atlbase/badge.svg)](https://docs.rs/atlbase)
[![License: GPL-3.0](https://img.shields.io/badge/license-GPL--3.0-blue.svg)](LICENSE)

Modern GTK4 + Libadwaita foundation library for Rust applications.

AtlBase is a modular Rust library that simplifies the development of Linux desktop applications with
GTK4 and Libadwaita. It provides ready-made dialogs and widgets, multi-page guides, helper
functions for files, commands and settings, gettext translations and logging — and it works the same
in deb, rpm, Flatpak, Snap and AppImage packages.

[crates.io](https://crates.io/crates/atlbase) ·
[API documentation](https://docs.rs/atlbase) ·
[Changelog](CHANGELOG.md) ·
[Packaging guide](docs/PACKAGING.md) ·
[User guide](docs/USER_GUIDE.md)

## Contents

- [Quick start](#quick-start)
- [Features](#features)
- [Installation](#installation)
- [Overview of the modules](#overview-of-the-modules)
- [Localization](#localization)
- [Packaging](#packaging)
- [Building the library](#building-the-library)
- [Example applications](#example-applications)
- [Tests](#tests)
- [Documentation](#documentation)
- [Changelog](#changelog)
- [License](#license)

## Quick start

```toml
[dependencies]
atlbase = "0.2"
adw = { version = "0.9", package = "libadwaita", features = ["v1_8"] }
```

```rust
use adw::prelude::*;
use atlbase::{gettext, prelude::*, ui_prelude::*};

fn main() -> glib::ExitCode {
    let app = adw::Application::builder()
        .application_id("org.example.Demo")
        .build();

    app.connect_activate(|app| {
        init_language("demo", "/usr/share/locale", false); // translations
        use_adw_provider("button { border-radius: 12px; }"); // your own CSS

        let button = create_button(&gettext!("Say hello"), |button| {
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

## Features

- GTK 4.20 + Libadwaita 1.8 integration
- Dialogs: information, success, warning and error (each with its own color and mark), alert, text input, about, image, custom content, spinner and progress
  bar for commands or Rust functions
- Multi-page guides (`carousel`): swipeable instructions built in code or loaded from TOML
- File and folder choosers, widget factories (buttons, entries, icon labels), CSS provider that
  follows light/dark
- Commands: run and capture, `pkexec` with a progress dialog, terminals, URLs
- Files and file system: append lines once, read `KEY=VALUE` files, temporary file, safe deletion
  helpers, path expansion
- Settings in a TOML file (`get_config`, `set_config`), detection of Flatpak / Snap / AppImage / PPA
- Internationalization with gettext; the texts of the library are translated out of the box
- Logging to syslog and a journal viewer window
- Examples and tests for everything

## Installation

### Cargo

```toml
[dependencies]
atlbase = "0.2"
```

| Cargo feature | Default | Description |
|---|---|---|
| `carousel-toml` | yes | Load multi-page guides from TOML files (`GuideLoader`) |
| `log-viewer` | yes | Journal viewer window and header bar menu (needs VTE) |
| `experimental-api` | no | Unfinished `system` module (does not compile yet) |

Without VTE (for example for a Flatpak that uses the GNOME runtime):

```toml
atlbase = { version = "0.2", default-features = false, features = ["carousel-toml"] }
```

### System requirements

Rust 1.92 or newer, GTK 4.20 or newer and Libadwaita 1.8 or newer (development packages).
`gettext` provides `msgfmt`, which compiles the translations of the library (optional, but without
it the texts of the library stay English).

```bash
# Ubuntu / Debian
sudo apt install build-essential pkg-config gettext \
    libgtk-4-dev libadwaita-1-dev libvte-2.91-gtk4-dev

# Fedora
sudo dnf install gcc pkgconf gettext gtk4-devel libadwaita-devel vte291-gtk4-devel

# Arch Linux
sudo pacman -S base-devel gettext gtk4 libadwaita vte4
```

`libvte` is only needed with the feature `log-viewer`. The packages need to be new enough: Ubuntu
25.10, Fedora 43, the GNOME 49 runtime and Arch qualify; older distributions have to use a newer
runtime (Flatpak) or build the libraries themselves.

## Overview of the modules

```text
atlbase
├── prelude          Everything without a window    use atlbase::prelude::*;
├── ui_prelude       Everything with a window       use atlbase::ui_prelude::*;
├── macros           gettext!
├── helper           commands, configuration, file, filesystem, language, log
├── design           carousel, chooser, dialogs, theme, utils
└── system           (feature experimental-api) unfinished
examples/            complete programs
tests/               integration tests
po/                  translations of the library
docs/                guides and the changelog page
```

| Module | Purpose |
|---|---|
| `helper::commands` | `run_command`, `capture_command_output`, `command_pkexec`, `open_terminal_by_desktop`, `open_url`, `command_exists_native` |
| `helper::configuration` | `get_config`, `set_config`, `get_execution_environment`, `check_application_environment` |
| `helper::file` | `add_line_file`, `get_config_value`, `write_file`, `trim_and_clean`, temporary file |
| `helper::filesystem` | create, check, search, expand and delete paths |
| `helper::language` | `init_language`, `translate`, `LIB_DOMAIN` |
| `helper::log` | `init_syslog`, `show_log_viewer`, `create_custom_headerbar` |
| `design::dialogs` | all dialogs |
| `design::carousel` | `Guide`, `Page`, `GuideView`, `GuideLoader` |
| `design::chooser` | `show_file_chooser`, `show_folder_chooser` |
| `design::theme` | `use_adw_provider` |
| `design::utils` | `create_button*`, `create_entry`, `create_label_icon*`, `commands_pkexec_*` |

Every function is listed in [docs/DEVELOPER_DOCS.md](docs/DEVELOPER_DOCS.md) and, with examples, in
the [API documentation](https://docs.rs/atlbase).

### Guides in a few lines

```rust
let guide = GuideLoader::new().load_resource("/org/example/App/guides/intro.toml")?;
GuideView::present_in_stack(&stack, "intro", &guide, "home");
```

Blocks: paragraph, heading, bullet list, numbered steps, code with a copy button, images with
caption and alternative text, and notes (info, tip, warning, danger).

## Localization

Call `init_language(domain, locale_dir, false)` once at start-up and wrap your texts in `gettext!`.
The library finds the translations of your application in the usual places (your `locale_dir`,
`/usr/share/locale`, `/app/share/locale` in a Flatpak, `$SNAP/…` in a Snap, `$APPDIR/…` in an
AppImage, `$XDG_DATA_DIRS`). The texts of the library itself are compiled into the crate (German is
included), so dialogs are translated without installing anything. Text of your application
overrides the text of the library. `ATL_LOCALEDIR=/path` forces a directory.

## Packaging

AtlBase is a normal crate, so it works in every package format. How to build deb, rpm, Flatpak, Snap
and AppImage packages (versions, VTE, translations, what works inside a sandbox) is described in
[docs/PACKAGING.md](docs/PACKAGING.md).

## Building the library

```bash
git clone https://github.com/AtlantisOS-Project/atlantis-UI-base.git
cd atlantis-UI-base

cargo build                 # debug build
cargo build --release       # release build (optimized for size)
cargo build --no-default-features    # without the TOML loader and the log viewer
```

The `[profile.release]` settings of this repository only apply when you build this repository;
applications that use AtlBase choose their own profile.

## Example applications

The programs in `examples/` show how the pieces fit together in a complete application. Every
example builds with `cargo build --examples`.

| Example | What it shows | Needs a window |
|---|---|---|
| `helper_tools` | The helper functions in a command line program; works in a temporary directory | no |
| `dialogs` | Every dialog, language, syslog, CSS and the header bar menu | yes |
| `guide` | Multi-page guides from a TOML file and from code | yes |
| `test_app` | Nearly every function in one big window | yes |

```bash
cargo run --example helper_tools
cargo run --example dialogs
cargo run --example guide                # content from examples/assets/guide.toml
cargo run --example guide -- builder     # the same guide, built in code
cargo run --example guide -- embedded    # embedded text, images from the data directories
cargo run --example test_app

cargo build --release --example dialogs  # optimized build of one example
```

The window examples need a running desktop session (Wayland or X11). Because the build script sets
`COMPILED_LOCALE_DIR`, they find the translations of the library without installing anything:

```bash
LANGUAGE=de cargo run --example dialogs  # German texts (needs the locale de_DE.UTF-8)
```

`dialogs`, `guide` and `test_app` need the default features (they are skipped by
`--no-default-features`).

### Running the examples without a desktop

GTK can draw into a web browser with the Broadway backend. This works on a server or in a
container, and the window shows up in a browser tab:

```bash
gtk4-broadwayd :5 &                       # the display ":5" is served on http://127.0.0.1:8085
GDK_BACKEND=broadway BROADWAY_DISPLAY=:5 cargo run --example dialogs
# open http://127.0.0.1:8085 in a browser (the port is 8080 plus the display number)
```

## Tests

```bash
cargo test                  # everything: unit tests, tests/ and the examples in the documentation
```

| Command | What it runs |
|---|---|
| `cargo test --lib` | Unit tests inside `src/` (locale directory lookup, TOML guides, `pkexec` commands, terminals) |
| `cargo test --tests` | Everything in `tests/` |
| `cargo test --doc` | The code examples of the API documentation (all are compiled, many run) |
| `cargo test --test helper_files` | One file of `tests/`; replace the name with another file |
| `cargo test --test helper_files add_line` | Only the tests whose name contains `add_line` |
| `cargo test --no-default-features --lib --tests` | The same without the optional features |

What `tests/` covers:

| File | Tests |
|---|---|
| `helper_filesystem.rs`, `helper_home_protection.rs` | create, search, expand and delete paths; the home directory is protected |
| `helper_files.rs`, `helper_temp_file.rs` | appending lines, `KEY=VALUE` files, the temporary file |
| `helper_config.rs` | settings in `$XDG_CONFIG_HOME` (also how Flatpak and Snap redirect them) |
| `helper_commands.rs`, `helper_environment.rs` | running commands, detection of Flatpak / Snap / AppImage / deb |
| `language_library.rs`, `language_application.rs` | translations of the library and of an application |
| `carousel_toml.rs`, `design_gtk.rs`, `preludes.rs` | guides, GTK widgets, the exported names |

Good to know:

* Tests never touch your real files: they work in temporary directories and point `$HOME` and
  `$XDG_*` there. Tests that change the environment live in files of their own.
* `language_*` use the German locale if it is installed and skip the translation checks otherwise;
  `language_application` also needs `msgfmt`.
* `design_gtk` has its own `main` because GTK must run on the main thread. It prints
  `skipped, there is no display` and succeeds when there is no display. To run it for real in a
  session without a desktop, use Broadway (see above):

  ```bash
  gtk4-broadwayd :5 &
  GDK_BACKEND=broadway BROADWAY_DISPLAY=:5 cargo test --test design_gtk
  ```

Before you send a change, these should be quiet:

```bash
cargo clippy --all-targets
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps
cargo +nightly doc --no-deps    # with RUSTDOCFLAGS="--cfg docsrs": how docs.rs builds it
cargo package --allow-dirty     # does the crate build from the packaged files?
```

## Documentation

* [API documentation on docs.rs](https://docs.rs/atlbase); locally with `cargo doc --open`
* [docs/USER_GUIDE.md](docs/USER_GUIDE.md) — using the library in your own project
* [docs/DEVELOPER_DOCS.md](docs/DEVELOPER_DOCS.md) — all functions and the conventions of the code
* [docs/MODULES_OVERVIEW.md](docs/MODULES_OVERVIEW.md) — what each module is for
* [docs/PACKAGING.md](docs/PACKAGING.md) — deb, rpm, Flatpak, Snap, AppImage

## Changelog

All changes — new functions, changed calls and behavior, removed items with their replacements —
are listed in [CHANGELOG.md](CHANGELOG.md). The same text is available as a web page:
[`docs/index.html`](docs/index.html) loads the changelog and renders it. For GitHub Pages choose
*Settings → Pages → Deploy from a branch → `main` / `docs`*; locally it can be viewed with
`python3 -m http.server` in the repository root and `http://127.0.0.1:8000/docs/`.

## License

GPL-3.0. See the [LICENSE](LICENSE) file for more information.

## Repository and author

* Repository: <https://github.com/AtlantisOS-Project/atlantis-UI-base>
* Crate: <https://crates.io/crates/atlbase>
* The AtlantisOS Project — primary developer: @NachtsternBuild
