// preludes.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! The preludes must keep exporting what the documentation promises.
//!
//! The test only has to compile: every name that disappears from a prelude breaks the build.

#![allow(unused_imports)]

use atlbase::prelude::*;
use atlbase::ui_prelude::*;

// Every function of the preludes by name: a missing export is a compile error.
use atlbase::prelude::{
    add_line_file, capture_command_output, check_application_environment, clear_temp_file,
    command_pkexec, create_directory, delete_directory, delete_files_and_parents,
    delete_files_in_dir, delete_files_with_parent, directory_exists, expand_path, file_exists,
    get_config, get_config_value, get_execution_environment, get_home_directory, get_language_dir,
    init_language, init_syslog, make_path, make_path_dirname, open_terminal_by_desktop, open_url,
    read_from_temp, remove_line_from_temp, run_command, search_file_directory, search_string_file,
    set_config, set_language_dir, write_file, write_to_temp,
};
use atlbase::ui_prelude::{
    CommandResult, GuideView, ImageSource, Step, block_widget, command_pkexec_progressbar,
    command_pkexec_spinner, commands_pkexec_progressbar, commands_pkexec_spinner, create_button,
    create_button_icon, create_button_icon_no_callback, create_button_icon_position,
    create_button_two_icon, create_entry, create_label_icon, create_label_icon_position,
    create_password_entry, page_widget, show_about_dialog, show_alert_dialog,
    show_custom_content_dialog, show_dialog_title, show_entry_dialog, show_error_button_dialog,
    show_error_dialog, show_error_title_button_dialog, show_error_title_dialog, show_file_chooser,
    show_folder_chooser, show_image_dialog, show_info_button_dialog, show_info_dialog,
    show_spinner_dialog, show_spinner_dialog_return_output, show_task_dialog, use_adw_provider,
    use_adw_provider_with_priority,
};

#[test]
fn the_preludes_export_the_documented_names() {
    // helper functions (prelude)
    let _ = (
        command_exists_native as fn(&str) -> bool,
        trim_and_clean as fn(&str) -> &str,
        get_current_domain as fn() -> String,
    );
    let _ = (LIB_DOMAIN, AppEnvironment::Unknown);

    // data types of the ui_prelude
    let _ = (
        IndicatorType::Spinner,
        CalloutKind::Info,
        AboutInfo::default(),
        Labels::default(),
    );
}

#[cfg(feature = "log-viewer")]
#[test]
fn the_log_viewer_is_part_of_the_ui_prelude() {
    let _ = (create_custom_headerbar, show_log_viewer);
}

#[cfg(feature = "carousel-toml")]
#[test]
fn the_guide_loader_is_part_of_the_ui_prelude() {
    let _ = GuideLoader::new();
}

#[test]
fn the_guide_builders_work_without_a_display() {
    let guide = Guide::new("Tour").labels(Labels::localized()).page(
        Page::new("Welcome")
            .icon("help-browser-symbolic")
            .heading("Hello")
            .paragraph("Text")
            .markup("<b>bold</b>")
            .bullets(["a", "b"])
            .steps(["one", "two"])
            .steps_from(3, [Step::new("three").detail("more")])
            .code("ls")
            .callout(Callout::danger("Careful").title("Warning"))
            .image(
                Image::from_file("a.png")
                    .alt("A")
                    .caption("B")
                    .max_height(100),
            ),
    );

    assert_eq!(guide.pages[0].blocks.len(), 9);
    assert!(matches!(guide.pages[0].blocks[3], Block::Bullets(ref items) if items.len() == 2));
    assert!(matches!(
        guide.pages[0].blocks[8],
        Block::Image(Image {
            max_height: 100,
            ..
        })
    ));
}
