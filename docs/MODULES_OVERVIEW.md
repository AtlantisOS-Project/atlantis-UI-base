# Module overview

An overview of all modules and their purpose. The details are in the API documentation
(`cargo doc --open`, or docs.rs).

```text
atlbase
├── prelude          Logic helper exports  (use atlbase::prelude::*)
├── ui_prelude       UI exports            (use atlbase::ui_prelude::*)
├── macros           gettext!
├── helper           everything without a user interface
│   ├── commands         command_exists, command_pkexec, execute_command,
│   │                    open_terminal_desktop, open_url
│   ├── configuration    application_environment, toml_config
│   ├── file             add_line_file, get_config_value, temp_app_file,
│   │                    trim_and_clean, write_file
│   ├── filesystem       delete_directory, delete_files, directory_exists, expand_path,
│   │                    home_dir, make_path, search_file_directory, search_string_name
│   ├── language         language (gettext, locale directory lookup)
│   └── log              write_log (syslog), write_log_text (journal viewer, feature log-viewer)
├── design           everything with a user interface
│   ├── carousel         multi-page guides: model, render, view, load (TOML), style
│   ├── chooser          file_chooser, folder_chooser
│   ├── dialogs          about_dialog, dialog, dialog_custom, dialog_entry, dialog_function,
│   │                    dialog_spinner_return, dialogs_spinner, image_dialog, standard_dialogs
│   ├── theme            load_adw_provider
│   └── utils            command_pkexec_ui, create_entry, create_label_icon,
│                        create_special_button
└── system           (feature experimental-api) unfinished network and configuration API
```

| Module | Purpose |
|---|---|
| `helper::commands::command_exists` | Is a program installed (searches `PATH`, checks the execute bit) |
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
| `helper::filesystem::*` | Create, check, search, expand and delete paths |
| `helper::language::language` | gettext initialization, `LIB_DOMAIN`, `translate` |
| `helper::log::write_log` | `log` crate backend for syslog |
| `helper::log::write_log_text` | Journal viewer window and header bar menu |
| `design::carousel` | Swipeable multi-page guides (builder or TOML) |
| `design::chooser::*` | File and folder dialogs |
| `design::dialogs::*` | Alert, entry, custom, image, about, spinner and task dialogs |
| `design::theme::load_adw_provider` | CSS providers that follow light/dark |
| `design::utils::*` | Widget factories |

The project also has the directories `examples/` (programs), `tests/` (integration tests),
`po/` (translations of the library) and `docs/`.
