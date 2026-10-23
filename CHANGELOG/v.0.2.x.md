## [0.2.0] — unreleased

### Upgrade guide

What you have to look at when you update. Everything not listed here keeps working as before.

| Area                            | Before                                                                                                      | Now                                                               | What to do                                                                          |
| ------------------------------- | ----------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------- | ----------------------------------------------------------------------------------- |
| `AppEnvironment`                | 5 variants                                                                                                  | new variant `AppImage`                                            | Add a match arm, or use `_ =>`                                                      |
| `check_application_environment` | ended the process (`exit(1)`) when the APT directory could not be read; a missing directory gave `LocalDeb` | returns `AppEnvironment::Unknown` in both cases and never exits   | Handle `Unknown` (for example rpm or Arch systems)                                  |
| `use_adw_provider`              | returned `()`                                                                                               | returns the `CssProvider`                                         | Nothing; ignore the value as before                                                 |
| `use_adw_provider` (2nd call)   | the first stylesheet replaced the second one on a theme change                                              | every call is independent                                         | Nothing; remove workarounds                                                         |
| `create_custom_headerbar`       | `domain: &'static str`; "Exit" ended the process with `exit(0)`                                             | `domain: &str`; "Exit" calls `Application::quit`                  | Nothing; shutdown handlers run now                                                  |
| `init_language`                 | looked into `./po` first; panicked when gettext rejected a path                                             | no `./po`; prints a warning instead of panicking                  | Pass your locale directory as `default_dir` or set `ATL_LOCALEDIR`                  |
| `gettext!`                      | needed `gettext-rs` as a direct dependency of your crate                                                    | works without it                                                  | You may remove `gettext-rs` if you only used it for the macro                       |
| `delete_files_and_parents`      | deleted the files of **every** parent directory on the way up                                               | deletes files only in the given directory                         | Nothing; this was a data-loss bug                                                   |
| `delete_files_in_dir`           | no protection at all                                                                                        | refuses system paths, the home directory and its standard folders | Nothing                                                                             |
| Cargo features                  | `vte4` always compiled                                                                                      | feature `log-viewer` (default on)                                 | Use `default-features = false` to drop VTE                                          |
| Cargo features (package formats) | no such features | `format-flatpak`, `format-snap`, `format-appimage`, `format-native`; the default has all four | With `default-features = false` enable at least one (`compile_error!` otherwise); a program that is built for one package enables only that one |
| `open_url`                      | started `xdg-open` (the `open` crate)                                                                       | GIO opens the link (the portal in a Flatpak and a Snap)           | Nothing; `xdg-open` is no longer needed, and the `open` crate is gone |
| Dependencies                    | `tokio`, `networkmanager` and `thiserror` always compiled                                                   | only with `experimental-api`                                      | Nothing; `libdbus` is no longer needed to build                                     |
| `SystemInfo`                    | had no architecture                                                                                         | new field `architecture`: a struct literal needs `..Default::default()` |
| `syslog`, `log`                 | `syslog` always compiled                                                                                    | `syslog` only on Unix; `log` with `std`; Windows logs to a file   |
| `get_home_directory`            | only `$HOME`                                                                                                | `$HOME`, then the system (`USERPROFILE` on Windows)               |
| `delete_directory`              | returned `()` and ended the whole process (`exit(1)`) when the directory could not be removed                | returns `io::Result<()>`                                          | Handle the result: `?`, or `.unwrap()` if the program should stop (a library must not end it) |
| `get_home_directory`            | returned `PathBuf` and ended the whole process when there was no home directory                              | returns `Option<PathBuf>` (the same as `home_dir()`)              | `get_home_directory().expect("a home directory")`, or handle `None` |
| `add_line_file`                 | `create_backup: &bool`                                                                                       | `create_backup: bool`                                             | Pass `true` / `false` instead of `&true` / `&false` (the compiler shows every place) |
| `search_string_file`            | a function of its own: a missing directory gave `Ok(false)`                                                  | removed: it is `search_file_directory`                            | Call `search_file_directory`; map the error kind `NotFound` to `false` if a missing directory means "no file" |
| Rust version                    | not declared                                                                                                | `rust-version = "1.93"`                                           | Use Rust 1.93 or newer (gtk4 0.11 needs 1.92, sevenz-rust2 of the feature `archive` 1.93) |
| Version                         | 0.1.9                                                                                                       | 0.2.0                                                             | Change `atlbase = "0.1"` to `"0.2"` (Cargo does not update across `0.x` on its own) |

### Added

#### New module `design::carousel`

Multi-page guides as a swipeable widget; the content is data (builder or TOML). It comes from the
standalone `adw-guide` crate and is integrated into `design`.

| Item | What it does |
|---|---|
| `Guide::new(title)`, `.page(page)`, `.labels(labels)` | A guide: title, pages, navigation labels |
| `Page::new(title)`, `.subtitle()`, `.icon()` | A page |
| `Page::heading()`, `.paragraph()`, `.markup()`, `.bullets()`, `.steps()`, `.steps_from()`, `.code()`, `.image()`, `.callout()`, `.button()`, `.block()` | The blocks of a page |
| `ActionButton::new(action, label)`, `.icon()`, `Block::Button` | A button on a page; a click is reported to the application (TOML: `button = "Text"`, `action = "id"`, `icon = "…"`) |
| `Step::new(title)`, `.detail()` | A numbered step with a second line |
| `Callout::info()`, `::tip()`, `::warning()`, `::danger()`, `.title()` | Highlighted notes |
| `Image::from_file()`, `::from_resource()`, `::from_texture()`, `.caption()`, `.alt()`, `.max_height()` | Screenshots with caption and alternative text |
| `Labels::default()`, `Labels::localized()` | Navigation texts (English / translated) |
| `GuideView::new()`, `::present_in_stack()`, `::present_dialog()`, `.connect_finished()`, `.connect_action()`, `.page()`, `.n_pages()`, `.set_page()` | The widget |
| `GuideLoader` (feature `carousel-toml`) | `new`, `translate`, `base_dir`, `base_resource`, `image_data_subdir`, `image_dir`, `load_str`, `load_file`, `load_resource` |
| `GuideError` (feature `carousel-toml`) | `Parse`, `Io`, `Resource`, `Invalid` with the position of the problem |
| `page_widget()`, `block_widget()` | Render a page or a block without `GuideView` |
| `GUIDE_CSS` | The default style (overridable with `use_adw_provider`) |

Differences to the standalone `adw-guide` crate: the default style uses the `theme` module of this
library (no second CSS manager); `present_dialog` is always available (no `dialog` feature, libadwaita
1.8 is required anyway); the GObject type is called `AtlGuideView`; `GuideLoader` translates with
`gettext!` when no `translate` function is set; relative image paths also search `$APPDIR/usr/share`
(AppImage).

#### New building blocks (moved over from the Fastboot-Assistant)

These pieces were written for the Fastboot-Assistant, a program with many pages, and were
repeated on page after page there. They are generic, so they live here now and every program
that uses AtlBase gets them. `cargo run --example pages` shows the UI blocks together.

| Item | Module | What it does |
|---|---|---|
| `command_line(program, args)` | `helper::commands` | Builds a `Vec<String>` command line for the progress dialogs from `&str` and `String` arguments |
| `shell_line(command)` | `helper::commands` | Quotes a command line for a shell (for `open_terminal_by_desktop`) |
| `command_output_timeout(command, limit)` | `helper::commands` | Output of a command (stdout, then stderr); `None` if it cannot be started or is too slow, and then it is killed |
| `move_file_into(source, target_dir, new_name)` | `helper::filesystem` | Moves and renames a file, also to another file system (copy and delete) |
| `require_dir_name(dir, name)`, `remove_dir_all_if_exists(dir)`, `clear_directory_except(dir, keep)` | `helper::filesystem` | Cleaning up a working folder safely: delete only if the folder has the expected name; a missing folder is fine |
| `read_common_options(args)`, `CliOptions`, `CliRequest`, `apply_debug_options(options, flatpak_id)`, `version_text(name, version, env)`, `COMMON_OPTIONS_HELP`, `debug_mode()`, `set_debug_mode()`, `fallback_language()` | `helper::cli` | `-debug` (`-snp`, `-flp`, `-lang`), `--version` and `--help` for every AtlantisOS program |
| `current_environment(search_string, source_dir)` | `helper::configuration` | `check_application_environment`, determined once and remembered |
| `AppEnvironment::label()`, `AppEnvironment::is_sandboxed()`, `running_in_wsl()` | `helper::configuration` | A readable package name; Flatpak or Snap; Windows Subsystem for Linux |
| `SystemInfo::collect()`, `package_version(package)`, `os_release_value`, `cpu_model`, `language_code` | `helper::sysinfo` | Distribution, kernel, processor, graphics card (from `/sys`, no `lspci`), desktop, session and installed Debian package versions (from `/var/lib/dpkg/status`, no `dpkg-query`); no program is started. `os_release_value` and `cpu_model` exist on Linux only |
| `ArchiveFormat`, `detect_format(path)`, `decompress(path, dir, format)`, `decompress_file(path, dir)` | `helper::archive` (feature `archive`) | Recognizes tar, tar.xz, tar.gz, tar.bz2, xz, gz, bz2, zip and 7z by the first bytes and unpacks them; a damaged archive leaves the folder unchanged |
| `download_file(url, destination, progress)` | `helper::download` (feature `download`) | Downloads a file over HTTP(S) without `curl` or `wget`: written under a temporary name and renamed when complete (a failed download leaves nothing under the real name), redirects are followed, an `https` address stays `https` (a redirect to plain `http` is refused), 30 s timeouts, `progress(done, total)` after every block |
| `run_background(work, done)` | `design::utils` | Slow work in a thread, its result in the main loop |
| `create_page_container()`, `create_rows_box()`, `create_scrolling_page(rows, bottom)`, `create_clamped_scrolling_page(content, bottom, max_width)`, `create_heading(text)`, `create_text_label(text)`, `create_pill_button(icon, label, suggested, on_click)` | `design::utils` | The parts a page is made of |
| `MenuEntry`, `MenuGridStyle`, `create_button_grid(entries, columns, style)` | `design::utils` | A grid of big buttons; a long text is cut to the column in the font that is used, so the window never gets wider |
| `show_stack_page(stack, id, rebuild, build)` | `design::utils` | Pages of a `gtk4::Stack` that are built when they are first shown (or on every visit) |
| `StackWindow::new(app, title, width, height)`, `.present(show_first)` | `design::utils` | A window of fixed size with a header bar and a stack; no slide animation to the first page |
| `StatusRow::new(title)`, `.set_result(ok, text)`, `.is_ok()` | `design::utils` | A row with a spinner that turns into a check mark or a warning |
| `show_confirm_dialog(parent, heading, body, confirm_label, on_confirm)` | `design::dialogs` | "Cancel" (the default) and a red button before something cannot be undone |
| `show_work_dialog(parent, title, message, work)` | `design::dialogs` | A spinner while a closure runs in the background, then a success or error dialog |
| `CommandTask`, `TaskOutcome`, `ErrorHint`, `task_error_text(error, hint)` | `design::dialogs` | A chain of commands with a progress dialog, a result dialog and optional explanations for known errors |
| `run_command_chain(commands)`, `CommandResult` | `helper::commands` | The chain of commands that every progress dialog runs, usable without a window |
| `show_chooser(widget, pick, start_in, on_chosen)`, `show_files_chooser(widget, on_chosen)`, `Pick` | `design::chooser` | One function behind all selection dialogs; several files at once; a start folder |
| `FormDialog`, `PickRow`, `create_combo_row`, `selected_index`, `on_selection_change`, `create_switch_row` | `design::form` | Dialogs with a form: drop-down menus, switches, and rows to choose files or a folder |

#### New module `helper::platform`: the layer between a program and the operating system

Everything that differs between Linux, Windows and macOS (and between x86_64 and arm64) is asked
here, so a program does not contain `/usr/share`, `$HOME`, `pkexec` or `adb.exe`. The library uses
the layer itself: the commands, the settings, the translations, the guides, the icons, the
protection of `delete_files_*`, the system log and `SystemInfo` all go through it.

The layer has two descriptions, each one **small file per case**:

* an **`OsProfile`** per operating system (`platform/os/linux.rs`, `windows.rs`, `macos.rs`,
  `other.rs`): the folders that are never deleted, how a command gets administrator rights, how a
  terminal is opened, the package managers, where the log goes, the names of a program file;
* a **`PackageFormat`** per package format (`platform/package_format/flatpak.rs`, `snap.rs`,
  `appimage.rs`, `native.rs`): how the format is recognized, where the program is mounted, where
  its programs and shared files are, whether it keeps the program away from the host.

Snap, Flatpak and AppImage no longer have their own way of handling things in the library: they are
three values of the same description, and everything that used to test `$SNAP` or `/.flatpak-info`
reads the fields of the format instead. **Adding a system or a format is one file, one line in a
list and a Cargo feature** (the steps are in the documentation of `platform::os` and
`platform::package_format`).

**Only the code of the target is compiled.** The profile of the operating system the program is
built for is the only one that is compiled (`cfg(target_os)`): a Windows build contains no line of
Linux code, and `vte4`, `syslog` and the Linux parsers are Linux-only dependencies. Of the package
formats only those whose Cargo feature is on are compiled: a Snap that is built with
`--no-default-features --features format-snap` has no code for Flatpak, AppImage, Windows or macOS
and does not ask at run time what it is (checked with `strings` on the release binaries). The unit
tests compile every profile and format, so the mapping of all systems is tested everywhere; the
Windows build is checked with `cargo check --target x86_64-pc-windows-gnu`
(`scripts/check-platform.sh`, with the features of a format).

**Which format a program is in** is found out from the environment, and a Snap or an AppImage counts only if the program itself lives below `$SNAP` or `$APPDIR`: a program that was started by a program of a Snap (`cargo run` with the `cargo` of the rustup Snap, which exports `$SNAP`) is not in that Snap. `ATL_PACKAGE_FORMAT=snap|flatpak|appimage|native` chooses the format by hand and wins over everything (`-debug -snp` and `-debug -flp` set it, so they work wherever the program is). A plain `cargo run` has all formats compiled in (the default features), so the flags work during development; a package is built with one format only.

| Item | What it does |
|---|---|
| `Os`, `Arch`, `Target` (`Os::current()`, `.exe_suffix()`, `.has_posix_shell()`, `Arch::name()`, `.debian_name()`) | Which operating system and which processor the program was built for, with the names other systems use |
| `AppDirs::new(app)` (`.config_dir()`, `.config_file()`, `.data_dir()`, `.cache_dir()`, `.state_dir()`, `.log_file()`) | The folders of an application for settings, data, cache and logs (the mapping of XDG on Linux, `%APPDATA%` on Windows, `~/Library` on macOS) |
| `home_dir()`, `documents_dir()` | The home directory (`$HOME`, then the system) and the folder for documents |
| `DataDirs::current()`, `.data_dirs()`, `.roots()` | The folders with read-only resources (translations, icons, guides), in search order, for every package format and relocated installs; the roots come from the formats, one implementation instead of three |
| `PackageFormat`, `PACKAGE_FORMATS`, `detect_package_format(var)`, `detect_package_format_at(var, exe)`, `FORMAT_OVERRIDE` (`ATL_PACKAGE_FORMAT`), `current_package_format()`, `package_roots(var)`, `Lookup` | The description of a package format (`id`, `label`, `sandboxed`, `relocated`, `detect`, `root`, `bin_dirs`, `share_dirs`); only the formats of the enabled `format-*` features exist |
| `OsProfile`, `os_profile()`, `os_profile_for(os)`, `Has` | The description of an operating system: protected folders (`critical_paths`, `protected_dirs`, `standard_home_folders`), elevation, terminal, package managers, journal, program names; `os_profile_for` of another system is available in tests only, `is_mount_point(path)` stays a function |
| `find_program(name)`, `program_path(name)`, `register_tool_dir(dir)`, `package_bin_dirs(places)`, `is_executable(path)`, `program_file_names(os, name, pathext)` | Where a program is: bundled tool folders first, then the folders of the package format (`/app/bin`, `$SNAP/usr/bin`, `$APPDIR/usr/bin`), then `PATH`; `.exe` and `PATHEXT` on Windows |
| `host_command(program)`, `host_command_line(&[String])` | `Command::new` for programs of the host: resolved like `find_program`, no console window flashing on Windows |
| `elevation_command(command)`, `elevation_command_for(os, command)` | `pkexec` on Linux, `osascript` on macOS, `powershell … -Verb RunAs` on Windows |
| `Feature` (`Elevation`, `HostTerminal`, `SystemLog`, `PackageInstall`, `PosixShell`), `supports(feature)`, `in_sandbox()` | What the host offers to this program; hide a button instead of failing |
| `PackageManager` (`detect()`, `install_command(packages)`, `needs_elevation()`), `APT_SOURCES_DIR` | `apt-get`, `dnf`, `pacman`, `zypper`, `winget`, `brew` |
| `init_logging(app)`, `init_file_logging(path)`, `journal_follow_command(app)` | syslog on Linux and macOS, a log file on Windows; the command the log viewer follows |
| `SystemInfo::architecture` | `x86_64` or `aarch64`; `SystemInfo::collect()` also works on Windows and macOS (name, architecture, language, desktop) |

#### Less memory

| Item | What it does |
|---|---|
| `set_renderer(Renderer::Software \| Gpu \| Vulkan \| Auto)` | Chooses how GTK draws the windows (`GSK_RENDERER`; the choice of the user wins, and Broadway keeps its own). A small window takes about 60 MB drawn on the processor, 115 MB with the default and 160 to 220 MB with `vulkan` or `ngl`: the driver of the graphics card is what costs. A program that wants to save the memory calls it; by default GTK decides |
| tests `dialogs are freed` (`tests/design_gtk.rs`) | Every dialog of the library is opened and closed, and weak references show that nothing is left |

#### New functions and types in existing modules

| Item | Module | What it does |
|---|---|---|
| `LIB_DOMAIN` | `helper::language` | The text domain of the library (`"atlbase"`); its catalogs are compiled in |
| `translate(msgid)` | `helper::language` | What `gettext!` calls: application domain first, library domain second |
| `AppEnvironment::AppImage` | `helper::configuration` | Detected through `$APPIMAGE`; `get_execution_environment()` returns `"appimage"` |
| `show_success_dialog(parent, body)` | `design::dialogs` | Green circle with a check mark; title "Success" (translated) |
| `show_warning_dialog(parent, body)` | `design::dialogs` | Yellow circle with an "!"; title "Warning" (translated) |
| `ensure_app_icon(icon_name, icon_file)` | `design::theme` | Makes the icon theme find the icon of the program (installed, AppImage, Snap, next to the program, or from a file) and sets the icon of all windows; `show_about_dialog` calls it |
| `AboutInfo::icon_file` | `design::dialogs` | A file with the icon (SVG or PNG) for when the icon is not installed, e.g. while developing |
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
| `archive` | off | `helper::archive` (adds `tar`, `xz2`, `flate2`, `bzip2`, `zip`, `sevenz-rust2`; the C code of bzip2 is compiled in, because the name of the system library differs between Debian and Fedora) |
| `download` | off | `helper::download`: `download_file(url, destination, progress)` over HTTP(S) without `curl` (adds `ureq` with rustls and the bundled root certificates; no system library) |
| `format-flatpak`, `format-snap`, `format-appimage`, `format-native` | all on | The package formats that are compiled in (see `helper::platform`): enable only the one you build for to leave the code of the others out; at least one is needed |

#### New files and directories

| Path | Purpose |
|---|---|
| `examples/helper_tools.rs` | Helper functions in a command line program (no window) |
| `examples/dialogs.rs` | Every dialog in one application (language, syslog, CSS) |
| `examples/guide.rs`, `examples/assets/guide.toml` | Guides from TOML and from code |
| `examples/pages.rs` | A program with several pages: the building blocks above together |
| `tests/` | Integration tests (see the README) |
| `po/atlbase.pot`, `po/de.po` | Translations of the library (`po/` had no translation before) |
| `po/update-pot.sh` | Collects the texts into `po/atlbase.pot` and merges them into the catalogs |
| `docs/PACKAGING.md` | deb, rpm, Flatpak, Snap and AppImage; packages for another architecture; builds that work on any system (containers, fixed tool versions, one file per format, Launchpad) |
| `CHANGELOG/`, `docs/index.html` | This changelog as text (one file per version series) and as a web page |

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

The next four **do not compile unchanged** (they were the suggestions of the review of 0.1.9 for the
next breaking release; 0.2 is that release):

| Function | Before | Now |
|---|---|---|
| `delete_directory(path)` | returned nothing; printed the error and ended the process with status `1` | `-> io::Result<()>`: the error of `fs::remove_dir` (`DirectoryNotEmpty`, `NotFound`, `PermissionDenied`) goes to the caller |
| `get_home_directory()` | `-> PathBuf`; printed `[ERROR]` and ended the process with status `1` if there was no home directory | `-> Option<PathBuf>` |
| `add_line_file(create_backup, path, line)` | `create_backup: &bool` | `create_backup: bool` |
| `search_file_directory(dir, text)`, `search_string_file(dir, text)` | two functions that did nearly the same: the second returned `Ok(false)` for a missing directory | one function, `search_file_directory`: a missing directory is an `io::Error` of kind `NotFound` |

#### Behavior changed

##### `helper::commands`

| Function | Before | Now |
|---|---|---|
| `command_exists_native` | an absolute path counted if it existed; a name counted if a *file* of that name was in `PATH` | the file must be executable (execute bit); directories and plain files give `false` |
| `run_command`, `capture_command_output` | panicked on an empty slice | return an error of kind `InvalidInput` |
| `open_terminal_by_desktop` | used only the terminal of the desktop and did nothing (silently) if it was not installed; MATE used `--`, Xfce used `-e` | tries the terminal of the desktop first, then every known terminal, and uses the first one that is installed; MATE and Xfce use `-x`; errors are printed; macOS quoting is escaped |
| `open_url` | started `xdg-open` and printed a garbled error text ("nicht öffnen") | opens the link through GIO (the *OpenURI* portal in a Flatpak and a Snap; no tool of the system); prints `Could not open '<url>': <reason>` |

##### `helper::configuration`

| Function | Before | Now |
|---|---|---|
| `get_execution_environment` | `"flatpak"`, `"snap"`, `"native"` | also `"appimage"`; `"flatpak"` also when `$FLATPAK_ID` is set (as `-debug -flp` does) |
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
| temporary file | a file with a name that stayed in `$TMPDIR` after the program had ended, with everything that was written to it | a file without a name: the system deletes it when the program ends or is killed |

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
| `show_file_chooser`, `show_folder_chooser` | printed an error when the user cancelled | cancelling is silent; other errors are logged |
| `show_info_dialog`, `show_info_button_dialog`, `show_dialog_title` | an alert dialog with a heading, a text and a highlighted button; looked the same as an error dialog | a blue circle with an "i", the heading, the text, a neutral button |
| `show_error_dialog`, `show_error_button_dialog`, `show_error_title_dialog`, `show_error_title_button_dialog` | the same plain alert dialog, only the heading said "Error" | a red circle with an "!", a red heading and a solid red button; the text can be selected and copied, a long text scrolls |
| `show_about_dialog` | headings "Documentation", "Fonts", "Special thanks to" were English only; empty sections were shown | headings are translated; sections without content are left out |
| `show_entry_dialog` | the buttons held strong references to the dialog (memory leak) | weak references |
| `show_spinner_dialog`, `show_spinner_dialog_return_output`, `show_task_dialog` | three copies of the same code with three slightly different looks; the progress bar timer ran forever; `show_task_dialog` put the title into Pango markup (a `&` or `<` broke it) | one implementation: same look (`title-3` heading, spinner 96 px, bar 250 px wide), the timer stops with the dialog, titles are plain text |
| the progress dialogs (`show_spinner_dialog`, `show_spinner_dialog_return_output`, `show_task_dialog`) | polled a channel every 25 ms, had two copies of the loop that runs the commands, and stayed open if the work panicked | one worker (`run_command_chain`) and one background helper; no polling; the dialog closes when the work panicked, too |
| `show_alert_dialog`, `show_custom_content_dialog`, `show_image_dialog`, `create_button_icon*`, `create_label_icon*`, the buttons of the log viewer menu | each built the same alert dialog or the same icon-and-label box itself | share the code; the box of the log viewer entries has 5 instead of 6 pixels between icon and text |
| `search_file_directory`, `create_directory`, `expand_path`, `get_home_directory`, `delete_files_*`, `write_file`, `os_release_value`, `package_version` | each had its own copy of reading `$HOME`, listing a directory, creating a directory or parsing `KEY=VALUE` | use one implementation; the results are the same |
| `commands_pkexec_*` | an empty list showed a dialog that closed at once | does nothing (as `command_pkexec_*` did already) |
| `use_adw_provider` | one global provider; a second call made the first stylesheet overwrite the second on every theme change | one provider per call, reloaded independently |

### Removed

**One public function was removed** (`search_string_file`, merged into `search_file_directory`); no type or module. What went away:

| Removed | Why | Replacement |
|---|---|---|
| `./po` lookup in `init_language` | it holds `.po` sources, not `.mo` files, so it hid the real directory whenever the working directory had a `po/` folder; relative to the working directory it was also unsafe | `default_dir` (use `env!("COMPILED_LOCALE_DIR")` while developing) or `ATL_LOCALEDIR` |
| `search_string_file` | it did what `search_file_directory` does, except that a missing directory gave `Ok(false)` | `search_file_directory`; map the error kind `NotFound` to `false` |
| `exit(1)` in `check_application_environment` | a library must not end the application | returns `AppEnvironment::Unknown` |
| `exit(1)` in `delete_directory` and `get_home_directory` | a library must not end the application: the caller decides what a failed delete or a missing home directory means | `io::Result<()>` and `Option<PathBuf>` |
| `exit(0)` behind the header bar "Exit" button (private `kill_program`) | skipped shutdown handlers and unsaved state | `Application::quit()` |
| `expect(…)` in `init_language` | a broken locale directory crashed the application at start-up | warning on `stderr` |
| need for `gettext-rs` in your own crate | the macro named `gettextrs::dgettext` | the macro uses `$crate::…::translate` |
| dependencies `shlex`, `walkdir`, `libc` | never used | none |
| dependency `lazy_static` | std `LazyLock` does the same | none |
| `tokio`, `networkmanager`, `thiserror` as mandatory dependencies | only the unfinished `system` module uses them; `networkmanager` pulled in `libdbus` | feature `experimental-api` |
| `#[doc(cfg(…))]` attributes | unstable on stable Rust | `#[cfg_attr(docsrs, doc(cfg(…)))]` |

### Known issues and suggestions

No function of the library ends the process any more, and the four suggestions of the review of 0.1.9
(`delete_directory`, `get_home_directory`, `add_line_file`, the two search functions) are done in this
release (see "Call changed"). What is left:

| Function | Issue | Suggestion |
|---|---|---|
| module `system` (`experimental-api`) | does not compile (`apply_wifi.rs`) | out of scope of this release |

### Fixed

* `FormDialog` and `PickRow` were never freed: the handlers of their buttons held the dialog (the
  row) strongly, so each opened form stayed in memory (about 1 MB). They hold it weakly now, and
  `FormDialog::ok_button()` gives the main button to closures that must not keep the form alive.
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
* Doc examples that did not compile or called functions that do not exist; all examples are
  compiled (and, where possible, run) by `cargo test --doc` now.

* The progress bar of the `dialogs` example was never visible: its commands ended within a few
  milliseconds, so only the final error appeared. The example now uses commands that take a few
  seconds.

### Security

A security review and a code audit (`cargo audit` for the dependencies, a read through the code that
touches files, archives, temporary files and commands) went into this release. What they found and
what was done:

* `show_log_viewer`: no shell command line is built from `domain` any more.
* The deletion helpers protect more directories and check them before deleting anything.
* No function of the library ends the process (`delete_directory` and `get_home_directory` were the
  last ones): a library that can stop its caller is a denial of service waiting for an unusual
  input, and skips the cleanup of the program.
* The `./po` lookup relative to the working directory is gone.
* `open_terminal_by_desktop`: documented that the command is run by a shell; the AppleScript string
  is escaped on macOS.
* Unpacking an archive (`decompress`, `decompress_file`) is safe against crafted archives now:
  * A 7z entry with `..` or an absolute path is refused and nothing is written outside the output
    folder (RUSTSEC-2026-0245 of `sevenz-rust`). The unmaintained `sevenz-rust` is replaced by
    `sevenz-rust2`.
  * `zip` is updated from 1.x to 9 (RUSTSEC-2025-0168, RUSTSEC-2026-0367, RUSTSEC-2026-0368,
    RUSTSEC-2026-0369): setuid and setgid bits of a zip are no longer set on the unpacked files,
    and symbolic links of an archive cannot lead out of the output folder.
  * The list of unpacked files does not follow a symbolic link of the archive any more, so a link
    to `/` can no longer make the library walk through the whole file system.
* The temporary file (`write_to_temp`) no longer stays in `$TMPDIR` after the program has ended.
* `gettext-rs` is updated to 0.8 (RUSTSEC-2026-0244): `setlocale` is `unsafe` there, because it reads
  the environment without a lock. `init_language` documents that it must run before other threads
  start.

### Build and packaging

* `include` in `Cargo.toml` lacked `build.rs` and `po/`, so the published crate had no
  translations. It now lists them, plus `examples/`, `tests/` and the documentation.
* `build.rs` no longer fails when `msgfmt` is missing (it warns), so documentation builds work
  without `gettext`. The catalogs are compiled into the crate.
* New `[package.metadata.docs.rs]`, `rust-version`, `keywords` and `categories`; the description now
  says Libadwaita 1.8 (what the code requires).
* Examples that need a feature declare it (`required-features`), and `tests/design_gtk.rs` runs
  without the test harness because GTK needs the main thread.
