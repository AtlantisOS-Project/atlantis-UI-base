# Developer documentation

All public functions and types, grouped by module. The complete documentation with examples is
generated from the source: `cargo doc --open` (or docs.rs).

* Everything without a window: `use atlbase::prelude::*;`
* Everything with a window: `use atlbase::ui_prelude::*;`
* Translations: `use atlbase::gettext;`

## Conventions

* **Documentation:** every public item has a summary line, a description and, where it makes
  sense, the sections `# Arguments`, `# Errors`, `# Panics` and `# Examples`. Examples are
  compiled by `cargo test --doc`; examples that need a window or change the system are marked
  `no_run`.
* **Sandboxes:** modules that depend on the host (`pkexec`, terminals, `journalctl`) have a
  `# Packaging` section that tells what happens in Flatpak and Snap.
* **Errors:** functions return `io::Result`, `Result` or `Option`; "fire and forget" helpers (for
  example `create_directory`) print their result to the console instead. No function of the
  library ends the process: the caller decides what a failure means.
* **Tests:** unit tests sit next to the code, integration tests in `tests/`, programs that show
  how to use the library in `examples/`.

## `helper::cli`

The command line options all AtlantisOS programs share: `-debug` (with the flags `-snp`, `-flp`, `-lang`), `--version`, `--help`.

| Item | Purpose |
|---|---|
| `read_common_options(args) -> Result<CliOptions, CliRequest>` | Read the options without side effects; `Err` is a request for the version or for help |
| `CliOptions { debug, fake_snap, fake_flatpak, fallback_language, gtk_args }` | The result; `gtk_args` goes to `Application::run_with_args` |
| `apply_debug_options(&options, flatpak_id)` | Turn the debug mode on and fake `$SNAP` / `$FLATPAK_ID` (call it at the start of `main`) |
| `debug_mode()`, `set_debug_mode(on)`, `fallback_language()` | The state of the process |
| `version_text(name, version, environment) -> String` | `Demo [Flatpak] 1.0.0` |
| `COMMON_OPTIONS_HELP` | The option lines of `--help` |

## `helper::commands`

| Function | Purpose |
|---|---|
| `command_exists_native(command: &str) -> bool` | Is the program installed (`PATH` or absolute path, execute bit)? |
| `command_pkexec(args: &[&str])` | Run a command as root with `pkexec`, print the result |
| `run_command(args: &[&str]) -> io::Result<ExitStatus>` | Run a command, output goes to the console |
| `capture_command_output(args: &[&str]) -> io::Result<String>` | Run a command, return its `stdout` |
| `open_terminal_by_desktop(function_command: &str)` | Open the terminal of the desktop and run a command |
| `open_url(url: &str)` | Open a URL or file with the default application |
| `command_line(program, args) -> Vec<String>` | Build a command line for the progress dialogs (`&str` and `String` arguments) |
| `shell_line(command: &[String]) -> String` | Quote a command line for a shell (a terminal) |
| `run_command_chain(commands) -> Result<Vec<CommandResult>, Vec<CommandResult>>` | Run commands one after another (no shell); stops at the first failure. The worker of every progress dialog |
| `command_output_timeout(command: &[String], limit) -> Option<String>` | Output of a command (stdout, then stderr); `None` if it cannot be started or is too slow (it is killed) |

## `helper::configuration`

| Item | Purpose |
|---|---|
| `get_config(app_name, key) -> Option<String>` | Read a setting from `<config dir>/<app>/default-config.toml` |
| `set_config(app_name, key, value) -> Result<(), ConfyError>` | Store a setting |
| `enum AppEnvironment { Flatpak, Snap, AppImage, Ppa, LocalDeb, Unknown }` | Where the application was installed from |
| `get_execution_environment() -> String` | `"flatpak"`, `"snap"`, `"appimage"` or `"native"` |
| `check_application_environment(search_string, source_dir) -> AppEnvironment` | Detect the installation source (APT sources) |
| `current_environment(search_string, source_dir) -> AppEnvironment` | The same, determined once and remembered |
| `AppEnvironment::label()`, `.is_sandboxed()` | A readable name; Flatpak or Snap |
| `running_in_wsl() -> bool` | Windows Subsystem for Linux |

## `helper::file`

| Function | Purpose |
|---|---|
| `add_line_file(create_backup: bool, path, line) -> io::Result<()>` | Append a line once, with optional `.bak` |
| `get_config_value(filename, key) -> Option<String>` | Read `KEY=VALUE` / `KEY="VALUE"` files |
| `trim_and_clean(s: &str) -> &str` | Trim whitespace and enclosing quotes |
| `write_file(filename, content, base_path, path_list) -> io::Result<()>` | Write a file and create the directories |
| `write_to_temp`, `read_from_temp`, `remove_line_from_temp`, `clear_temp_file` | One temporary file per process |

## `helper::filesystem`

| Function | Purpose |
|---|---|
| `create_directory(path)` / `make_path(path)` / `make_path_dirname(filepath)` | Create directories (logging / returning / for a file path) |
| `directory_exists(path)` / `file_exists(path)` | Existence checks |
| `expand_path(path) -> PathBuf` | Replace a leading `~` by `$HOME` |
| `get_home_directory() -> Option<PathBuf>` | `$HOME`, then the system (`USERPROFILE` on Windows); `None` if there is no home directory |
| `delete_directory(path) -> io::Result<()>` | Remove an empty directory (`fs::remove_dir`) |
| `delete_files_in_dir(path)` | Delete the files of a directory (protected directories are refused) |
| `delete_files_and_parents(path, stop_dir)` / `delete_files_with_parent(path)` | Delete the files, then the empty directory and its empty parents |
| `search_file_directory(dir, text) -> io::Result<bool>` | Is there a file whose name contains the text? A missing directory is a `NotFound` error |
| `move_file_into(source, target_dir, new_name) -> io::Result<PathBuf>` | Move and rename a file, also to another file system |
| `require_dir_name(dir, name) -> io::Result<()>` | Refuse (`PermissionDenied`) unless the folder has this name: the check before a cleanup |
| `remove_dir_all_if_exists(dir)` / `clear_directory_except(dir, keep)` | Delete a folder / its content except some entries; a missing folder is fine |

## `helper::platform`

The layer between a program and the operating system. A program asks it where things are and what
the system can do; it does not contain `/usr/share`, `$HOME`, `pkexec` or `adb.exe`. Linux is
the platform that is run; the mappings for Windows and macOS are covered by tests that run
everywhere (`scripts/check-platform.sh` also compiles the layer for Windows).

It is built on two descriptions, each in **one small file per case**:

| Description | Files | Chosen by | What it knows |
|---|---|---|---|
| `OsProfile` | `platform/os/{linux,windows,macos,other}.rs` | `cfg(target_os)`: only the profile of the target is compiled | protected folders, how a command gets administrator rights, how a terminal opens, package managers, journal, program file names |
| `PackageFormat` | `platform/package_format/{flatpak,snap,appimage,native}.rs` | Cargo features `format-flatpak`, `format-snap`, `format-appimage`, `format-native`: only the formats that are enabled are compiled | how the format is recognized, where the program is mounted (`/app`, `$SNAP`, `$APPDIR`), where its programs (`bin_dirs`) and shared files (`share_dirs`) are, whether it is `sandboxed` or `relocated` |

Snap, Flatpak and AppImage have no code of their own in the library: they are three values of
`PackageFormat`, and the folders, the locale and icon lookup, the search for programs, `supports()`
and `in_sandbox()` all read the same fields. A program that is built for one format compiles
nothing of the others:

```sh
cargo build --release --no-default-features --features format-snap            # a Snap
cargo build --release --no-default-features --features format-native,log-viewer  # a deb or an RPM
```

Windows needs no feature to leave the Linux code out: no Linux profile is compiled for it.

During development nothing needs a flag: `cargo run` compiles the profile of your own system and **all** package formats (the default features), so `-debug -snp` and `-debug -flp` can pretend to be a Snap or a Flatpak. The format is found out from the environment, and `$SNAP` or `$APPDIR` count only if the program lives below them (a `cargo run` with the `cargo` of the rustup Snap inherits `$SNAP` and is still native). `ATL_PACKAGE_FORMAT=snap|flatpak|appimage|native` chooses the format by hand. Only a package build leaves the other formats out (`--no-default-features --features format-snap`); the saving is small (about 2.4 kB in the Fastboot-Assistant, whose size comes from GTK and the crates), it is about not carrying code that can never run.

**Adding an operating system:** copy the closest `os/*.rs`, fill in its `PROFILE`, add a variant to
`Os` (and its `cfg` to `Os::current`), add the `mod` line and the system to `os_profile`. **Adding a
package format:** copy the closest `package_format/*.rs`, fill in its `FORMAT`, add the Cargo feature
`format-<name>` (and to `default`), add the `mod` line and one line to `PACKAGE_FORMATS`. Nothing else
in the library changes.

| Item | Purpose |
|---|---|
| `Os`, `Arch`, `Target` | Which system and processor the program was built for; `Arch::name()` (`x86_64`, `aarch64`), `Arch::debian_name()` (`amd64`, `arm64`), `Os::exe_suffix()` |
| `OsProfile`, `os_profile()`, `os_profile_for(os)` | The description of the operating system (fields: `critical_paths`, `protected_dirs`, `standard_home_folders`, `elevate`, `terminal`, `package_managers`, `journal_follow`, `syslog`, `program_names`, …) |
| `PackageFormat`, `PACKAGE_FORMATS`, `current_package_format()`, `detect_package_format(var)`, `package_roots(var)` | The package format the program runs in, and the list of the formats that are compiled in |
| `AppDirs::new(app)` | `config_dir()`, `config_file()`, `data_dir()`, `cache_dir()`, `state_dir()`, `log_file()` of an application |
| `home_dir()`, `documents_dir()` | The home directory and the folder for documents |
| `DataDirs::current().data_dirs()` | Folders with read-only resources (translations, icons, guides) in search order, for every package format |
| `find_program(name)`, `program_path(name)`, `register_tool_dir(dir)`, `package_bin_dirs(&DataDirs)` | Where an external program is: bundled folders, the programs of the package format (`/app/bin`, `$SNAP/usr/bin`, `$APPDIR/usr/bin`), then `PATH`; `.exe` on Windows |
| `host_command(program)`, `host_command_line(&[String])` | `Command::new` for programs of the host; use these instead (no console window on Windows) |
| `elevation_command(command)` | The command with `pkexec` (Linux), `osascript` (macOS) or a UAC prompt (Windows) in front |
| `supports(Feature::…)`, `in_sandbox()` | `Elevation`, `HostTerminal`, `SystemLog`, `PackageInstall`, `PosixShell`: show a button only where the host offers it |
| `PackageManager::detect()`, `.install_command(packages)` | `apt-get`, `dnf`, `pacman`, `zypper`, `winget`, `brew` |
| `init_logging(app)` | syslog on Linux and macOS, `AppDirs::log_file` on Windows |

```rust
use atlbase::prelude::*;

// the program of the host, found in the folders of the application or in PATH
let output = host_command("adb").arg("devices").output();

// a button that installs programs only where it can work
if supports(Feature::PackageInstall) {
    let manager = PackageManager::detect().unwrap();
    let install = manager.install_command(&["adb"]);
    let _command = elevation_command(&install);
}
```

`run_command`, `capture_command_output`, `command_pkexec`, the command chains of the dialogs,
`get_config`, `init_language`, `GuideLoader`, `ensure_app_icon`, `delete_files_*` and
`SystemInfo::collect` use the layer, so a program that only calls them is portable already.

## `helper::sysinfo`

| Item | Purpose |
|---|---|
| `SystemInfo::collect()` | Distribution, version, kernel, processor, architecture, graphics card, desktop, language and session (reads a few files on Linux, no program is started: call it in the background; on Windows and macOS the name, architecture, language and desktop) |
| `os_release_value(text, key)`, `cpu_model(cpuinfo)`, `language_code(lang)` | The parsers behind it, usable on their own |
| `package_version(package) -> Option<String>` | The installed version of a Debian package (read from `/var/lib/dpkg/status`; no program is started) |

## `helper::archive` (feature `archive`)

| Item | Purpose |
|---|---|
| `detect_format(path) -> io::Result<Option<ArchiveFormat>>` | Recognize `tar`, `tar.xz`, `tar.gz`, `tar.bz2`, `xz`, `gz`, `bz2`, `zip`, `7z` by the first bytes, not by the name |
| `decompress(path, output_dir, format)` / `decompress_file(path, output_dir)` | Unpack; a damaged archive leaves `output_dir` as it was. Returns the files that were created |
| `ArchiveFormat::ALL`, `.label()` | For a menu when the format is unknown |


## `helper::download` (feature `download`)

| Item | Purpose |
|---|---|
| `download_file(url, destination, progress) -> io::Result<u64>` | Download a file over HTTP(S) without `curl`: written as `<destination>.part` and renamed when complete, redirects are followed, an `https` address stays `https`, `progress(done, total)` after every block. Needs a thread of its own (`run_background`, `show_work_dialog`) |

## `helper::language`

| Item | Purpose |
|---|---|
| `init_language(domain, default_dir, debug_lang)` | Initialize gettext (application and library domain) |
| `LIB_DOMAIN` | The text domain of the library (`"atlbase"`) |
| `translate(msgid) -> String` / `gettext!(msgid)` | Translate: application domain first, library domain second |
| `get_current_domain()`, `get_language_dir()`, `set_language_dir(dir)` | Read or change the settings |

## `helper::log`

| Item | Purpose |
|---|---|
| `init_syslog(app_name) -> Result<(), SetLoggerError>` | Send `log` messages to syslog |
| `show_log_viewer(app, domain)` | Window that follows the journal (feature `log-viewer`) |
| `create_custom_headerbar(app, domain) -> gtk4::Box` | Header bar menu: log viewer and exit (feature `log-viewer`) |

## `design::carousel`

| Item | Purpose |
|---|---|
| `Guide`, `Page`, `Block`, `Step`, `Callout`, `CalloutKind`, `ActionButton`, `Image`, `ImageSource`, `Labels` | The content of a guide (builders) |
| `GuideView` | The widget: `new`, `present_in_stack`, `present_dialog`, `connect_finished`, `connect_action`, `page`, `set_page` |
| `GuideLoader`, `GuideError` | Load guides from TOML (feature `carousel-toml`) |
| `page_widget`, `block_widget` | Render a page or a block without `GuideView` |
| `GUIDE_CSS` | The default style |

## `design::chooser`

`show_file_chooser(widget, callback)`, `show_files_chooser(widget, callback)` (several files), `show_folder_chooser(widget, callback)`, `show_chooser(widget, Pick, start_in, callback)` (the function behind all of them; `Pick::{File, Files, Folder}`)

## `design::dialogs`

| Function | Purpose |
|---|---|
| `show_alert_dialog(parent, title, body, button)` | Basic alert dialog |
| `show_info_dialog`, `show_info_button_dialog`, `show_dialog_title` | Information dialogs (blue "i") |
| `show_success_dialog`, `show_warning_dialog` | Success (green check mark) and warning (yellow "!") dialogs |
| `show_error_dialog`, `show_error_button_dialog`, `show_error_title_dialog`, `show_error_title_button_dialog` | Error dialogs (red "!") |
| `show_custom_content_dialog(parent, title, body, button, content)` | Alert dialog with your own widgets |
| `show_entry_dialog(…, on_submit) -> adw::Dialog` | Dialog with a text field |
| `show_image_dialog(…, path, width, height)` | Image preview |
| `show_about_dialog(parent, AboutInfo)` | The "About" dialog |
| `show_spinner_dialog(…, commands, IndicatorType, on_complete)` | Spinner or progress bar while commands run |
| `show_spinner_dialog_return_output(…)` | The same, with the result of every command (`CommandResult`) |
| `show_task_dialog(parent, title, message, IndicatorType, task)` | The same for a Rust function |
| `CommandTask::new(title, message, commands)` | A chain of commands with a progress dialog and a result dialog; `.progress_bar()`, `.success_message()`, `.show_output()`, `.explain_errors()`, `.run(parent)` |
| `show_confirm_dialog(parent, heading, body, confirm_label, on_confirm)` | "Cancel" and a red button before something cannot be undone |
| `show_work_dialog(parent, title, message, work)` | Spinner while a closure `-> Result<String, String>` works; then a success or error dialog |

## `design::form`

Dialogs with a form: a header bar with "Cancel" and an action button, rows in between.

| Item | Purpose |
|---|---|
| `FormDialog::new(title, ok_label)` | The dialog; `.group(title)`, `.on_ok(action)`, `.set_ok_enabled()`, `.set_ok_visible()`, `.widget()`, `.present(parent)` |
| `create_combo_row(title, items)`, `selected_index(row)`, `on_selection_change(row, update)` | A drop-down menu |
| `create_switch_row(title, subtitle)` | A switch |
| `PickRow::new(title, Pick, start_in, changed)` | A row that shows the chosen file(s) or folder and has a "Choose…" button; `.path()`, `.paths()`, `.set_paths()`, `.set_pick()` |
| `Pick::{File, Files, Folder}` | What a `PickRow` lets the user choose |
| `FormDialog::ok_button()` | The main button, for a closure that enables or hides it |

**Closures and memory.** A closure that is connected to a widget and keeps that widget alive
(`row.connect_changed(move |_| row.…)`, or an `Rc` closure that holds the row and is stored in a
handler of the row) frees neither the widget nor the dialog around it: every time a dialog is
opened it stays in memory (about 1 MB for a form). Capture the widgets that the closure is
connected to weakly with `glib::clone!(#[weak] row, move || …)`; `FormDialog` and `PickRow` do
that for their own buttons. The tests of the library (`dialogs are freed`) and of the
Fastboot-Assistant (`tests/ui_memory.rs`) open dialogs and pages, close them, and check with weak
references that nothing is left.

## `design::theme`

| Function | Purpose |
|---|---|
| `use_adw_provider(css) -> CssProvider`, `use_adw_provider_with_priority(css, priority) -> CssProvider` | CSS that follows light/dark |
| `set_renderer(Renderer::Software)` | Draw the windows on the processor instead of the graphics card: a small program saves 60 to 160 MB (the driver of the graphics card); call it first in `main`; a `GSK_RENDERER` of the user wins |
| `ensure_app_icon(icon_name, icon_file) -> bool` | Make the icon theme find the icon of the program: installed, in an AppDir (`$APPDIR`), in a Snap (`$SNAP`), next to the program, or from a file (source tree); also sets the icon of all windows |

## `design::utils`

| Function | Purpose |
|---|---|
| `create_entry`, `create_password_entry` | Label and entry in a row |
| `create_label_icon`, `create_label_icon_position` | Icon and label |
| `create_button`, `create_button_icon`, `create_button_icon_no_callback`, `create_button_two_icon`, `create_button_icon_position` | Pill buttons |
| `command_pkexec_spinner`, `commands_pkexec_spinner`, `command_pkexec_progressbar`, `commands_pkexec_progressbar` | `pkexec` with a progress dialog |
| `run_background(work, done)` | Run slow work in a thread and its result in the main loop; the window does not freeze |
| `create_page_container`, `create_rows_box`, `create_scrolling_page(rows, bottom)`, `create_clamped_scrolling_page(content, bottom, max_width)` | The containers of a page: centered, scrolling list with a button at the bottom, a narrow column |
| `create_heading`, `create_text_label`, `create_pill_button(icon, label, suggested, on_click)` | Heading, wrapping text, standard pill button with an icon |
| `MenuEntry::new(icon, label, action)`, `create_button_grid(entries, columns, MenuGridStyle)` | A grid of big buttons; long texts are cut to the column in the font that is used |
| `show_stack_page(stack, id, rebuild, build)` | Show a page of a `gtk4::Stack`, built when it is first needed (`rebuild`: on every visit) |
| `StackWindow::new(app, title, width, height)`, `.present(show_first)` | A window of fixed size with a header bar and a stack for the pages |
| `StatusRow::new(title)`, `.set_result(ok, text)`, `.is_ok()` | A row with a spinner that turns into a check mark or a warning |
