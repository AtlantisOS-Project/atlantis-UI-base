// lib.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! # AtlBase
//!
//! `atlbase` is a collection of building blocks for GTK4 + Libadwaita applications on Linux (with
//! a [platform layer](helper::platform) that prepares the way to other systems):
//! dialogs, widget factories, theming, multi-page guides, translations (gettext), logging
//! and a set of small file system and command helpers.
//!
//! The crate has two halves:
//!
//! * [`helper`] — everything without a user interface: commands, configuration,
//!   files, file system, language and logging, and the
//!   [`platform`](helper::platform) layer that maps folders, programs, administrator rights and
//!   logging to the operating system. Import it with [`prelude`].
//! * [`design`] — everything with a user interface: dialogs, choosers, theme, widget
//!   factories and the [`carousel`](design::carousel) guide widget. Import it with
//!   [`ui_prelude`].
//!
//! # Getting started
//!
//! ```no_run
//! use adw::prelude::*;
//! use atlbase::{gettext, prelude::*, ui_prelude::*};
//!
//! fn main() -> glib::ExitCode {
//!     let app = adw::Application::builder()
//!         .application_id("org.example.Demo")
//!         .build();
//!
//!     app.connect_activate(|app| {
//!         // Binds the translations of the application *and* of this library.
//!         init_language("demo", "/usr/share/locale", false);
//!
//!         let window = adw::ApplicationWindow::builder()
//!             .application(app)
//!             .title(gettext!("Demo"))
//!             .build();
//!         let button = create_button(&gettext!("Say hello"), |button| {
//!             show_info_dialog(button, "Hello from AtlBase");
//!         });
//!         window.set_content(Some(&button));
//!         window.present();
//!     });
//!     app.run()
//! }
//! ```
//!
//! More complete programs live in the `examples/` directory of the repository
//! (`cargo run --example dialogs`, `--example guide`, `--example helper_tools`,
//! `--example localized_app`).
//!
//! # Feature flags
//!
//! | Feature | Default | Description |
//! |---|---|---|
//! | `carousel-toml` | yes | Loads multi-page guides from TOML files ([`GuideLoader`](design::carousel::GuideLoader)). |
//! | `archive` | no | Recognizes and unpacks archives ([`helper::archive`]): tar, tar.xz, tar.gz, tar.bz2, xz, gz, bz2, zip and 7z. |
//! | `log-viewer` | yes | Journal viewer window and header bar menu; needs VTE (`libvte-2.91-gtk4`). |
//! | `experimental-api` | no | Unfinished `system` module. Does not compile yet. |
//!
//! # Localization
//!
//! Call [`init_language`](helper::language::language::init_language) once at start-up. The
//! texts of this library are translated with the [`gettext!`] macro: first in the text domain
//! of your application (so you can override any string), then in the domain of the library
//! ([`LIB_DOMAIN`](helper::language::language::LIB_DOMAIN)). The library's own catalogs are
//! compiled into the binary, so the dialogs are translated without installing any extra
//! file. See [`helper::language`] for the lookup order of the locale directory.
//!
//! # Packaging
//!
//! `atlbase` is used as a normal crate dependency and works the same in every package
//! format. It adapts to the sandbox it runs in:
//!
//! | Format | Detected via | What to know |
//! |---|---|---|
//! | Flatpak | `/.flatpak-info` | Config and cache go to `~/.var/app/<id>/…` (XDG variables). `pkexec`, host terminals and `journalctl` are not available in the sandbox. VTE is not part of the GNOME runtime: add it as a module or build with `default-features = false`. |
//! | Snap | `$SNAP` | Translations are searched in `$SNAP/usr/share/locale`. Strict confinement cannot use `pkexec`. |
//! | AppImage | `$APPIMAGE` | Translations are searched in `$APPDIR/usr/share/locale`. |
//! | deb / rpm / `cargo install` | — | Translations are searched in `/usr/share/locale`. |
//!
//! At build time the library needs the GTK4, Libadwaita (and VTE) development packages;
//! `msgfmt` from `gettext` is optional. Use [`get_execution_environment`] or
//! [`check_application_environment`] to react to the environment at run time.
//!
//! [`get_execution_environment`]: helper::configuration::application_environment::get_execution_environment
//! [`check_application_environment`]: helper::configuration::application_environment::check_application_environment

#![warn(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod design;
pub mod helper;
pub mod macros;

/// Experimental core functions intended to handle system configuration tasks
/// (for the live system and installer), but which are not yet fully implemented.
#[cfg(feature = "experimental-api")]
#[cfg_attr(docsrs, doc(cfg(feature = "experimental-api")))]
pub mod system;

/// Quick access to all functions that need no user interface.
///
/// # Examples
///
/// ```
/// use atlbase::prelude::*;
///
/// assert!(directory_exists("/"));
/// ```
pub mod prelude {
    pub use crate::helper::{
        cli::common_options::{
            COMMON_OPTIONS_HELP, CliOptions, CliRequest, apply_debug_options, debug_mode,
            fallback_language, read_common_options, set_debug_mode, version_text,
        },
        commands::{
            command_chain::{CommandResult, run_command_chain},
            command_exists::command_exists_native,
            command_line::{command_line, command_output_timeout, shell_line},
            command_pkexec::command_pkexec,
            execute_command::{capture_command_output, run_command},
            open_terminal_desktop::open_terminal_by_desktop,
            open_url::open_url,
        },
        configuration::{
            application_environment::{
                AppEnvironment, check_application_environment, current_environment,
                get_execution_environment, running_in_wsl,
            },
            toml_config::{get_config, set_config},
        },
        file::{
            add_line_file::add_line_file,
            get_config_value::get_config_value,
            temp_app_file::{
                clear_temp_file, read_from_temp, remove_line_from_temp, write_to_temp,
            },
            trim_and_clean::trim_and_clean,
            write_file::write_file,
        },
        filesystem::{
            delete_directory::delete_directory,
            delete_files::{
                delete_files_and_parents, delete_files_in_dir, delete_files_with_parent,
            },
            directory_exists::{directory_exists, file_exists},
            expand_path::expand_path,
            guarded_delete::{clear_directory_except, remove_dir_all_if_exists, require_dir_name},
            home_dir::get_home_directory,
            make_path::{create_directory, make_path, make_path_dirname},
            move_file::move_file_into,
            search_file_directory::search_file_directory,
            search_string_name::search_string_file,
        },
        language::language::{
            LIB_DOMAIN, get_current_domain, get_language_dir, init_language, set_language_dir,
        },
        log::write_log::init_syslog,
        platform::{
            dirs::{
                AppDirs, DataDirs, critical_paths, documents_dir, home_dir, is_mount_point,
                protected_dirs, standard_home_folders,
            },
            elevate::{elevation_command, elevation_command_for},
            feature::{Feature, in_sandbox, supports},
            logging::{
                init_file_logging, init_logging, journal_follow_command,
                journal_follow_command_for,
            },
            package::{APT_SOURCES_DIR, PackageManager},
            program::{
                find_program, host_command, host_command_line, is_executable, package_bin_dirs,
                program_file_names, program_path, register_tool_dir,
            },
            target::{Arch, Os, Target},
        },
        sysinfo::system_info::{
            SystemInfo, cpu_model, gpu_model, language_code, os_release_value, package_version,
        },
    };

    #[cfg(feature = "archive")]
    #[cfg_attr(docsrs, doc(cfg(feature = "archive")))]
    pub use crate::helper::archive::archive_format::{
        ArchiveFormat, decompress, decompress_file, detect_format,
    };
}

/// Quick access to all graphical components for Libadwaita applications.
///
/// # Examples
///
/// ```no_run
/// use atlbase::ui_prelude::*;
/// # gtk4::init().unwrap();
/// # let window = gtk4::Window::new();
///
/// show_error_dialog(&window, "Something went wrong");
/// ```
pub mod ui_prelude {
    pub use crate::design::{
        carousel::{
            ActionButton, Block, Callout, CalloutKind, Guide, GuideView, Image, ImageSource,
            Labels, Page, Step, block_widget, page_widget,
        },
        chooser::{
            choose::{Pick, show_chooser},
            file_chooser::{show_file_chooser, show_files_chooser},
            folder_chooser::show_folder_chooser,
        },
        dialogs::{
            about_dialog::{AboutInfo, show_about_dialog},
            command_task::{CommandTask, ErrorHint, TaskOutcome, task_error_text},
            dialog::show_alert_dialog,
            dialog_confirm::show_confirm_dialog,
            dialog_custom::show_custom_content_dialog,
            dialog_entry::show_entry_dialog,
            dialog_function::show_task_dialog,
            dialog_spinner_return::{CommandResult, show_spinner_dialog_return_output},
            dialog_work::show_work_dialog,
            dialogs_spinner::{IndicatorType, show_spinner_dialog},
            image_dialog::show_image_dialog,
            standard_dialogs::{
                show_dialog_title, show_error_button_dialog, show_error_dialog,
                show_error_title_button_dialog, show_error_title_dialog, show_info_button_dialog,
                show_info_dialog, show_success_dialog, show_warning_dialog,
            },
        },
        form::{
            FormDialog, PickRow, create_combo_row, create_switch_row, on_selection_change,
            selected_index,
        },
        theme::{
            app_icon::ensure_app_icon,
            load_adw_provider::{use_adw_provider, use_adw_provider_with_priority},
            renderer::{Renderer, set_renderer},
        },
        utils::{
            background::run_background,
            command_pkexec_ui::{
                command_pkexec_progressbar, command_pkexec_spinner, commands_pkexec_progressbar,
                commands_pkexec_spinner,
            },
            create_entry::{create_entry, create_password_entry},
            create_label_icon::{create_label_icon, create_label_icon_position},
            create_special_button::{
                create_button, create_button_icon, create_button_icon_no_callback,
                create_button_icon_position, create_button_two_icon,
            },
            menu_grid::{MenuAction, MenuEntry, MenuGridStyle, create_button_grid},
            page_layout::{
                create_clamped_scrolling_page, create_heading, create_page_container,
                create_pill_button, create_rows_box, create_scrolling_page, create_text_label,
            },
            stack_pages::show_stack_page,
            stack_window::StackWindow,
            status_row::StatusRow,
        },
    };

    #[cfg(feature = "carousel-toml")]
    #[cfg_attr(docsrs, doc(cfg(feature = "carousel-toml")))]
    pub use crate::design::carousel::{GuideError, GuideLoader};

    #[cfg(feature = "log-viewer")]
    #[cfg_attr(docsrs, doc(cfg(feature = "log-viewer")))]
    pub use crate::helper::log::write_log_text::{create_custom_headerbar, show_log_viewer};
}
