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
* **Errors:** functions return `io::Result` or `Result`; "fire and forget" helpers (for
  example `create_directory`) print their result to the console instead. Two helpers end the
  process on purpose and say so in their documentation: `delete_directory` and
  `get_home_directory`.
* **Tests:** unit tests sit next to the code, integration tests in `tests/`, programs that show
  how to use the library in `examples/`.

## `helper::commands`

| Function | Purpose |
|---|---|
| `command_exists_native(command: &str) -> bool` | Is the program installed (`PATH` or absolute path, execute bit)? |
| `command_pkexec(args: &[&str])` | Run a command as root with `pkexec`, print the result |
| `run_command(args: &[&str]) -> io::Result<ExitStatus>` | Run a command, output goes to the console |
| `capture_command_output(args: &[&str]) -> io::Result<String>` | Run a command, return its `stdout` |
| `open_terminal_by_desktop(function_command: &str)` | Open the terminal of the desktop and run a command |
| `open_url(url: &str)` | Open a URL or file with the default application |

## `helper::configuration`

| Item | Purpose |
|---|---|
| `get_config(app_name, key) -> Option<String>` | Read a setting from `<config dir>/<app>/default-config.toml` |
| `set_config(app_name, key, value) -> Result<(), ConfyError>` | Store a setting |
| `enum AppEnvironment { Flatpak, Snap, AppImage, Ppa, LocalDeb, Unknown }` | Where the application was installed from |
| `get_execution_environment() -> String` | `"flatpak"`, `"snap"`, `"appimage"` or `"native"` |
| `check_application_environment(search_string, source_dir) -> AppEnvironment` | Detect the installation source (APT sources) |

## `helper::file`

| Function | Purpose |
|---|---|
| `add_line_file(create_backup: &bool, path, line) -> io::Result<()>` | Append a line once, with optional `.bak` |
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
| `get_home_directory() -> PathBuf` | `$HOME`; ends the process if it is not set |
| `delete_directory(path)` | Remove an empty directory; ends the process on failure |
| `delete_files_in_dir(path)` | Delete the files of a directory (protected directories are refused) |
| `delete_files_and_parents(path, stop_dir)` / `delete_files_with_parent(path)` | Delete the files, then the empty directory and its empty parents |
| `search_file_directory(dir, text)` / `search_string_file(dir, text)` | Is there a file whose name contains the text? |

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

`show_file_chooser(widget, callback)`, `show_folder_chooser(widget, callback)`

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

## `design::theme`

`use_adw_provider(css) -> CssProvider`, `use_adw_provider_with_priority(css, priority) -> CssProvider`

## `design::utils`

| Function | Purpose |
|---|---|
| `create_entry`, `create_password_entry` | Label and entry in a row |
| `create_label_icon`, `create_label_icon_position` | Icon and label |
| `create_button`, `create_button_icon`, `create_button_icon_no_callback`, `create_button_two_icon`, `create_button_icon_position` | Pill buttons |
| `command_pkexec_spinner`, `commands_pkexec_spinner`, `command_pkexec_progressbar`, `commands_pkexec_progressbar` | `pkexec` with a progress dialog |
