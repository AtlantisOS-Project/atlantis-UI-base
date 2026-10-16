# Changelog

All notable changes to [`atlbase`](https://crates.io/crates/atlbase) are documented here: new
functions, functions whose behavior or call changed, and functions that were removed (with the
reason and the replacement).

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/). Before 1.0 a
change of the minor version (`0.x`) may break the API, which is why this release is **0.2.0**.
This file is also shown as a web page: [`docs/index.html`](docs/index.html).

## [0.2.0] — unreleased

The version is set in `Cargo.toml`, but 0.2.0 is not on crates.io yet; the latest release there is
0.1.9.

### Upgrade guide

What you have to look at when you update. Everything not listed here keeps working as before.

| Area | Before | Now | What to do |
|---|---|---|---|
| `AppEnvironment` | 5 variants | new variant `AppImage` | Add a match arm, or use `_ =>` |
| `check_application_environment` | ended the process (`exit(1)`) when the APT directory could not be read; a missing directory gave `LocalDeb` | returns `AppEnvironment::Unknown` in both cases and never exits | Handle `Unknown` (for example rpm or Arch systems) |
| `use_adw_provider` | returned `()` | returns the `CssProvider` | Nothing; ignore the value as before |
| `use_adw_provider` (2nd call) | the first stylesheet replaced the second one on a theme change | every call is independent | Nothing; remove workarounds |
| `create_custom_headerbar` | `domain: &'static str`; "Exit" ended the process with `exit(0)` | `domain: &str`; "Exit" calls `Application::quit` | Nothing; shutdown handlers run now |
| `init_language` | looked into `./po` first; panicked when gettext rejected a path | no `./po`; prints a warning instead of panicking | Pass your locale directory as `default_dir` or set `ATL_LOCALEDIR` |
| `gettext!` | needed `gettext-rs` as a direct dependency of your crate | works without it | You may remove `gettext-rs` if you only used it for the macro |
| `delete_files_and_parents` | deleted the files of **every** parent directory on the way up | deletes files only in the given directory | Nothing; this was a data-loss bug |
| `delete_files_in_dir` | no protection at all | refuses system paths, the home directory and its standard folders | Nothing |
| Cargo features | `vte4` always compiled | feature `log-viewer` (default on) | Use `default-features = false` to drop VTE |
| Dependencies | `tokio`, `networkmanager` and `thiserror` always compiled | only with `experimental-api` | Nothing; `libdbus` is no longer needed to build |
| Rust version | not declared | `rust-version = "1.92"` | Use Rust 1.92 or newer (gtk4 0.11 needs it anyway) |
| Version | 0.1.9 | 0.2.0 | Change `atlbase = "0.1"` to `"0.2"` (Cargo does not update across `0.x` on its own) |

### Added

#### New module `design::carousel`

Multi-page guides as a swipeable widget; the content is data (builder or TOML). It comes from the
standalone `adw-guide` crate and is integrated into `design`.

| Item | What it does |
|---|---|
| `Guide::new(title)`, `.page(page)`, `.labels(labels)` | A guide: title, pages, navigation labels |
| `Page::new(title)`, `.subtitle()`, `.icon()` | A page |
| `Page::heading()`, `.paragraph()`, `.markup()`, `.bullets()`, `.steps()`, `.steps_from()`, `.code()`, `.image()`, `.callout()`, `.block()` | The blocks of a page |
| `Step::new(title)`, `.detail()` | A numbered step with a second line |
| `Callout::info()`, `::tip()`, `::warning()`, `::danger()`, `.title()` | Highlighted notes |
| `Image::from_file()`, `::from_resource()`, `::from_texture()`, `.caption()`, `.alt()`, `.max_height()` | Screenshots with caption and alternative text |
| `Labels::default()`, `Labels::localized()` | Navigation texts (English / translated) |
| `GuideView::new()`, `::present_in_stack()`, `::present_dialog()`, `.connect_finished()`, `.page()`, `.n_pages()`, `.set_page()` | The widget |
| `GuideLoader` (feature `carousel-toml`) | `new`, `translate`, `base_dir`, `base_resource`, `image_data_subdir`, `image_dir`, `load_str`, `load_file`, `load_resource` |
| `GuideError` (feature `carousel-toml`) | `Parse`, `Io`, `Resource`, `Invalid` with the position of the problem |
| `page_widget()`, `block_widget()` | Render a page or a block without `GuideView` |
| `GUIDE_CSS` | The default style (overridable with `use_adw_provider`) |

Differences to the standalone `adw-guide` crate: the default style uses the `theme` module of this
library (no second CSS manager); `present_dialog` is always available (no `dialog` feature, libadwaita
1.8 is required anyway); the GObject type is called `AtlGuideView`; `GuideLoader` translates with
`gettext!` when no `translate` function is set; relative image paths also search `$APPDIR/usr/share`
(AppImage).

#### New functions and types in existing modules

| Item | Module | What it does |
|---|---|---|
| `LIB_DOMAIN` | `helper::language` | The text domain of the library (`"atlbase"`); its catalogs are compiled in |
| `translate(msgid)` | `helper::language` | What `gettext!` calls: application domain first, library domain second |
| `AppEnvironment::AppImage` | `helper::configuration` | Detected through `$APPIMAGE`; `get_execution_environment()` returns `"appimage"` |
| `use_adw_provider_with_priority(css, priority)` | `design::theme` | Like `use_adw_provider` with an explicit priority |
| `AboutInfo: Default` | `design::dialogs` | `AboutInfo { app_name: …, ..Default::default() }` |
| `IndicatorType: Debug, Clone, Copy, PartialEq, Eq, Hash` | `design::dialogs` | Was without any derive |
| `CommandResult: PartialEq, Eq` | `design::dialogs` | Comparable in tests |
| `AppEnvironment: Hash` | `helper::configuration` | Usable as a map key |
| `commands_pkexec_spinner`, `commands_pkexec_progressbar` | `ui_prelude` | Existed in `design::utils`, but were missing from the prelude |
| `get_current_domain`, `LIB_DOMAIN` | `prelude` | Added to the prelude |

#### New Cargo features

| Feature | Default | Purpose |
|---|---|---|
| `carousel-toml` | on | `GuideLoader` and `GuideError` (adds the `toml` crate) |
| `log-viewer` | on | `show_log_viewer`, `create_custom_headerbar` (adds VTE) |

#### New files and directories

| Path | Purpose |
|---|---|
| `examples/helper_tools.rs` | Helper functions in a command line program (no window) |
| `examples/dialogs.rs` | Every dialog in one application (language, syslog, CSS) |
| `examples/guide.rs`, `examples/assets/guide.toml` | Guides from TOML and from code |
| `tests/` | Integration tests (see the README) |
| `po/atlbase.pot`, `po/de.po` | Translations of the library (`po/` had no translation before) |
| `docs/PACKAGING.md` | deb, rpm, Flatpak, Snap and AppImage |
| `docs/index.html`, `CHANGELOG.md` | This changelog as a web page and as text |

### Changed

#### Call changed

These signatures changed. All of them accept the old arguments, so existing code compiles unchanged,
except for the return value of `use_adw_provider` in `let () = …` patterns.

| Function | Before | Now |
|---|---|---|
| `show_spinner_dialog` | `parent: &adw::ApplicationWindow` | `parent: &impl IsA<gtk4::Widget>` |
| `show_spinner_dialog_return_output` | `parent: &adw::ApplicationWindow` | `parent: &impl IsA<gtk4::Widget>` |
| `show_task_dialog` | `parent: &adw::ApplicationWindow` | `parent: &impl IsA<gtk4::Widget>` |
| `command_pkexec_spinner`, `commands_pkexec_spinner`, `command_pkexec_progressbar`, `commands_pkexec_progressbar` | `parent: &adw::ApplicationWindow` | `parent: &impl IsA<gtk4::Widget>` |
| `show_file_chooser`, `show_folder_chooser` | `button: &gtk4::Button` | `button: &impl IsA<gtk4::Widget>` |
| `create_custom_headerbar` | `domain: &'static str` | `domain: &str` |
| `use_adw_provider` | `(css: &str)` returns nothing | `(css: &str) -> CssProvider` |

#### Behavior changed

##### `helper::commands`

| Function | Before | Now |
|---|---|---|
| `command_exists_native` | an absolute path counted if it existed; a name counted if a *file* of that name was in `PATH` | the file must be executable (execute bit); directories and plain files give `false` |
| `run_command`, `capture_command_output` | panicked on an empty slice | return an error of kind `InvalidInput` |
| `open_terminal_by_desktop` | used only the terminal of the desktop and did nothing (silently) if it was not installed; MATE used `--`, Xfce used `-e` | tries the terminal of the desktop first, then every known terminal, and uses the first one that is installed; MATE and Xfce use `-x`; errors are printed; macOS quoting is escaped |
| `open_url` | printed a garbled error text ("nicht öffnen") | prints `Could not open '<url>': <reason>` |

##### `helper::configuration`

| Function | Before | Now |
|---|---|---|
| `get_execution_environment` | `"flatpak"`, `"snap"`, `"native"` | also `"appimage"` |
| `check_application_environment` | see the upgrade guide | returns `AppImage` / `Unknown`; never ends the process |
| `set_config` | failed if the file was empty or corrupt | starts a new configuration in that case |

##### `helper::file`

| Function | Before | Now |
|---|---|---|
| `add_line_file` | a new file or a file without a final line break got the next line glued to the last one; the duplicate check compared the untrimmed new line; the backup was made even for a file that had just been created | always writes a line break first if needed; compares trimmed lines; makes the one-time `.bak` only for an existing file |
| `get_config_value` | `KEY = value` (spaces before `=`) was not found; one line that was not valid UTF-8 ended the search with `None` | spaces around `=` are fine; broken lines are skipped |
| `write_to_temp` | wrote at the position the cursor happened to have (after `clear_temp_file` this produced NUL bytes) | always appends |
| `clear_temp_file` | truncated, but left the cursor behind | truncates and resets the cursor (as documented) |
| temporary file | a panic in another thread poisoned the file for good | a poisoned lock is ignored |

##### `helper::filesystem`

| Function | Before | Now |
|---|---|---|
| `delete_files_in_dir` | deleted in any directory | refuses protected directories (see below) |
| `delete_files_and_parents` | deleted the files of every parent directory while climbing; protection was checked after deleting | deletes files only in the given directory, then removes empty directories upwards; protection is checked first |
| `delete_files_with_parent` | continued with the unresolved path when `canonicalize` failed | a path that cannot be resolved is reported and ignored |
| protected paths | `/`, `/bin`, `/sbin`, `/usr`, `/etc`, `/var`, `/proc`, `/sys`, `/dev`, `/lib`, `/boot`, `/root` and below; home and its standard folders (only while climbing) | additionally `/lib32`, `/lib64`, and — the directory itself — `/home`, `/tmp`, `/opt`, `/mnt`, `/media`, `/run`, `/srv`, `/app`, `/snap`; the home check now applies to the start directory too |

##### `helper::language`

| Function | Before | Now |
|---|---|---|
| `init_language` | order: `ATL_LOCALEDIR`, `./po`, `default_dir`, `/usr/share/locale`, Snap (practically never reached, because `/usr/share/locale` exists); the first existing directory wins; `expect` on gettext errors | first directory that contains a catalog wins: `ATL_LOCALEDIR`, Snap/AppImage roots, `default_dir`, `/app/share/locale` (Flatpak), `/usr/share/locale`, `$XDG_DATA_DIRS/locale`; also binds `LIB_DOMAIN`; errors become warnings |
| `gettext!` | `dgettext(<current domain>, text)` | `translate(text)`: application domain, then library domain; empty text stays empty |
| library texts | looked up in the application domain only, and `po/` was empty, so they were never translated | translated (German included) from catalogs compiled into the crate |

##### `helper::log`

| Function | Before | Now |
|---|---|---|
| `init_syslog` | opened a new socket for every message | opens it once and reconnects after a failure |
| `show_log_viewer` | built a shell command (`sh -c "journalctl … -t <domain>"`) | passes `domain` as an argument of `journalctl`; no shell |
| `create_custom_headerbar` | "Exit" button: `exit(0)`, tooltip "Kill App" | `Application::quit()`, tooltip "Exit the application"; texts are translated |

##### `design`

| Function | Before | Now |
|---|---|---|
| `show_file_chooser`, `show_folder_chooser` | printed an error when the user cancelled | cancelling is silent; other errors are printed |
| `show_about_dialog` | headings "Documentation", "Fonts", "Special thanks to" were English only; empty sections were shown | headings are translated; sections without content are left out |
| `show_entry_dialog` | the buttons held strong references to the dialog (memory leak) | weak references |
| `show_spinner_dialog`, `show_spinner_dialog_return_output`, `show_task_dialog` | three copies of the same code with three slightly different looks; the progress bar timer ran forever; `show_task_dialog` put the title into Pango markup (a `&` or `<` broke it) | one implementation: same look (`title-3` heading, spinner 96 px, bar 250 px wide), the timer stops with the dialog, titles are plain text |
| `commands_pkexec_*` | an empty list showed a dialog that closed at once | does nothing (as `command_pkexec_*` did already) |
| `use_adw_provider` | one global provider; a second call made the first stylesheet overwrite the second on every theme change | one provider per call, reloaded independently |

### Removed

**No public function, type or module was removed.** What went away:

| Removed | Why | Replacement |
|---|---|---|
| `./po` lookup in `init_language` | it holds `.po` sources, not `.mo` files, so it hid the real directory whenever the working directory had a `po/` folder; relative to the working directory it was also unsafe | `default_dir` (use `env!("COMPILED_LOCALE_DIR")` while developing) or `ATL_LOCALEDIR` |
| `exit(1)` in `check_application_environment` | a library must not end the application | returns `AppEnvironment::Unknown` |
| `exit(0)` behind the header bar "Exit" button (private `kill_program`) | skipped shutdown handlers and unsaved state | `Application::quit()` |
| `expect(…)` in `init_language` | a broken locale directory crashed the application at start-up | warning on `stderr` |
| need for `gettext-rs` in your own crate | the macro named `gettextrs::dgettext` | the macro uses `$crate::…::translate` |
| dependencies `shlex`, `walkdir`, `libc` | never used | none |
| dependency `lazy_static` | std `LazyLock` does the same | none |
| `tokio`, `networkmanager`, `thiserror` as mandatory dependencies | only the unfinished `system` module uses them; `networkmanager` pulled in `libdbus` | feature `experimental-api` |
| `#[doc(cfg(…))]` attributes | unstable on stable Rust | `#[cfg_attr(docsrs, doc(cfg(…)))]` |

### Known issues and suggestions

These behave as documented. Changing them breaks the API, so they are only suggestions for the next
breaking release (not decided).

| Function | Issue | Suggestion (and what to use meanwhile) |
|---|---|---|
| `delete_directory` | ends the whole process when the directory cannot be removed | return `io::Result<()>`; use `std::fs::remove_dir` meanwhile |
| `get_home_directory` | ends the whole process when `$HOME` is not set | return `Option<PathBuf>`; use `std::env::var_os("HOME")` meanwhile |
| `add_line_file` | takes `create_backup: &bool` | take `bool` |
| `search_file_directory`, `search_string_file` | do nearly the same | merge into one function |
| module `system` (`experimental-api`) | does not compile (`apply_wifi.rs`) | out of scope of this release |

### Fixed

* Deleting the files of parent directories in `delete_files_and_parents` (data loss).
* Lines glued together by `add_line_file`.
* NUL bytes in the temporary file after `clear_temp_file`.
* The translations of the library never worked: `po/` was empty, the locale lookup could not find
  Snap catalogs, and the texts were looked up in the wrong domain.
* `use_adw_provider` replacing earlier stylesheets on a theme change.
* A progress bar timer that never stopped, and broken titles in `show_task_dialog`.
* A reference cycle in `show_entry_dialog`.
* A panic in `run_command(&[])` and `capture_command_output(&[])`.
* Wrong documentation: copy-pasted texts in `create_button_icon*`, a password example that read the
  wrong field, examples that called functions that do not exist, unresolved links.
* Doc examples that did not compile or called functions that do not exist; all 89 examples are
  compiled (and, where possible, run) by `cargo test --doc` now.

### Security

* `show_log_viewer`: no shell command line is built from `domain` any more.
* The deletion helpers protect more directories and check them before deleting anything.
* The `./po` lookup relative to the working directory is gone.
* `open_terminal_by_desktop`: documented that the command is run by a shell; the AppleScript string
  is escaped on macOS.

### Build and packaging

* `include` in `Cargo.toml` lacked `build.rs` and `po/`, so the published crate had no
  translations. It now lists them, plus `examples/`, `tests/` and the documentation.
* `build.rs` no longer fails when `msgfmt` is missing (it warns), so documentation builds work
  without `gettext`. The catalogs are compiled into the crate.
* New `[package.metadata.docs.rs]`, `rust-version`, `keywords` and `categories`; the description now
  says Libadwaita 1.8 (what the code requires).
* Examples that need a feature declare it (`required-features`), and `tests/design_gtk.rs` runs
  without the test harness because GTK needs the main thread.

## [0.1.9] — 2026-06-03

The last release before this changelog. See the
[git history](https://github.com/AtlantisOS-Project/atlantis-UI-base/commits/main) for the changes
of earlier versions.
