# Module overview

An overview of all modules and their purpose. The details are in the API documentation
(`cargo doc --open`, or docs.rs).

```text
atlbase
├── prelude          Logic helper exports  (use atlbase::prelude::*)
├── ui_prelude       UI exports            (use atlbase::ui_prelude::*)
├── macros           gettext!
├── helper           everything without a user interface
│   ├── archive          archive_format (feature archive)
│   ├── cli              common_options (-debug, --version, --help)
│   ├── commands         command_exists, command_line, command_pkexec, execute_command,
│   │                    open_terminal_desktop, open_url
│   ├── configuration    application_environment, toml_config
│   ├── file             add_line_file, get_config_value, temp_app_file,
│   │                    trim_and_clean, write_file
│   ├── filesystem       delete_directory, delete_files, directory_exists, expand_path,
│   │                    guarded_delete, home_dir, make_path, move_file,
│   │                    search_file_directory, search_string_name
│   ├── language         language (gettext, locale directory lookup)
│   ├── log              write_log (syslog), write_log_text (journal viewer, feature log-viewer)
│   ├── platform         target, dirs, program, elevate, feature, package, logging
│   │                    (the layer between a program and the operating system)
│   └── sysinfo          system_info (distribution, kernel, processor, packages)
├── design           everything with a user interface
│   ├── carousel         multi-page guides: model, render, view, load (TOML), style
│   ├── chooser          file_chooser, folder_chooser
│   ├── dialogs          about_dialog, command_task, dialog, dialog_confirm, dialog_custom,
│   │                    dialog_entry, dialog_function, dialog_spinner_return, dialog_work,
│   │                    dialogs_spinner, image_dialog, standard_dialogs
│   ├── form             FormDialog, PickRow, combo and switch rows
│   ├── theme            load_adw_provider
│   └── utils            background, command_pkexec_ui, create_entry, create_label_icon,
│                        create_special_button, menu_grid, page_layout, stack_pages,
│                        stack_window, status_row
└── system           (feature experimental-api) unfinished network and configuration API
```

| Module | Purpose |
|---|---|
| `helper::archive::archive_format` | Recognize and unpack tar, xz, gz, bz2, zip and 7z (feature `archive`) |
| `helper::cli::common_options` | `-debug`, `--version` and `--help` for every AtlantisOS program |
| `helper::commands::command_exists` | Is a program installed (searches `PATH`, checks the execute bit) |
| `helper::commands::command_line` | Build and quote command lines, ask a command with a time limit |
| `helper::commands::command_pkexec` | Run a command as root with `pkexec` |
| `helper::commands::execute_command` | Run a command / capture its output |
| `helper::commands::open_terminal_desktop` | Open the terminal of the desktop and run a command |
| `helper::commands::open_url` | Open a URL or file with the default application |
| `helper::configuration::application_environment` | Detect Flatpak, Snap, AppImage, PPA, local deb |
| `helper::configuration::toml_config` | Store and read settings in a TOML file |
| `helper::file::add_line_file` | Append a line once, with optional backup |
| `helper::file::get_config_value` | Read `KEY=VALUE` files |
| `helper::file::temp_app_file` | One temporary file per process |
| `helper::file::trim_and_clean` | Trim whitespace and quotes |
| `helper::file::write_file` | Write a file and create its directories |
| `helper::filesystem::*` | Create, check, search, expand, move and delete paths (`guarded_delete`: only a folder with the right name) |
| `helper::language::language` | gettext initialization, `LIB_DOMAIN`, `translate` |
| `helper::log::write_log` | `log` crate backend for syslog |
| `helper::log::write_log_text` | Journal viewer window and header bar menu |
| `helper::platform::target` | `Os`, `Arch`, `Target`: the system and processor the program was built for |
| `helper::platform::dirs` | `AppDirs` (settings, data, cache, logs), `home_dir`, `DataDirs` (resources), protected paths |
| `helper::platform::program` | Find and start programs of the host: `find_program`, `host_command`, `register_tool_dir` |
| `helper::platform::elevate` | Run a command as administrator: `pkexec`, `osascript` or UAC |
| `helper::platform::feature` | `supports(Feature::…)`: what the host offers |
| `helper::platform::package` | `PackageManager`: `apt-get`, `dnf`, `pacman`, `zypper`, `winget`, `brew` |
| `helper::platform::logging` | `init_logging`: syslog or a log file; the command that follows the journal |
| `helper::sysinfo::system_info` | Distribution, kernel, processor, graphics card, package versions |
| `design::carousel` | Swipeable multi-page guides (builder or TOML) |
| `design::chooser::*` | File and folder dialogs |
| `design::dialogs::*` | Alert, entry, custom, image, about, confirm, spinner, work and task dialogs; `CommandTask` |
| `design::form` | Dialogs with a form: drop-down menus, switches, file and folder rows |
| `design::theme::load_adw_provider` | CSS providers that follow light/dark |
| `design::utils::*` | Widget factories, page layout, button grid, stack pages and window, background work |

The project also has the directories `examples/` (programs), `tests/` (integration tests),
`po/` (translations of the library) and `docs/`.
