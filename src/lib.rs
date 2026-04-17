//! Functions of the main library that incorporate all functions
/**
* lib.rs
*
* (C) Copyright 2026 AtlantisOS Project
* by @NachtsternBuild
*
* License: GNU GENERAL PUBLIC LICENSE Version 3
*/
/// AtlBase
/// ### Usage:
///
/// ```rust
/// use atlbase::*;
/// ```
pub mod helper;
pub mod design;
pub mod macros;

pub mod prelude {
	pub use crate::{
    	helper::{
    		commands::{
    			command_pkexec::command_pkexec,
    			command_exists::command_exists_native,
    			execute_command::{
    				run_command,
    				capture_command_output,
    			},
    			open_terminal_desktop::open_terminal_by_desktop,
    			open_url::open_url,
    		},
    		configuration::{
    			toml_config::{
    				get_config,
    				set_config,
    			},
    			application_environment::{
    				AppEnvironment,
    				get_execution_environment,
    				check_application_environment,
    			},
    		},
    		file::{
    			get_config_value::get_config_value,
    			trim_and_clean::trim_and_clean,
    			add_line_file::add_line_file,
    			write_file::write_file,
    		},
    		filesystem::{
    			delete_directory::delete_directory,
    			delete_files::{
    				delete_files_in_dir,
    				delete_files_and_parents,
    				delete_files_with_parent,
    			},
    			directory_exists::{
    				directory_exists,
    				file_exists,
    			},
    			expand_path::expand_path,
    			home_dir::get_home_directory,
    			make_path::{
    				create_directory,
    				make_path,
    				make_path_dirname,
    			},
    			search_file_directory::search_file_directory,
    			search_string_name::search_string_file,
    		},
    		language::language::{
    			set_language_dir,
    			get_language_dir,
    			init_language,
    		},
    		log::write_log::init_syslog,  		
    	},
    };
}

pub mod ui_prelude {
    pub use crate::{
    	helper::log::write_log_text::{
    		show_log_viewer,
    		create_custom_headerbar,
    	},
    	design::{
    		chooser::{
    			file_chooser::show_file_chooser,
    			folder_chooser::show_folder_chooser,
    		},
    		dialogs::{
    			about_dialog::{
    				show_about_dialog,
    				AboutInfo,
    			},
    			dialog::show_alert_dialog,
    			dialog_entry::show_entry_dialog, 
    			dialogs_spinner::{
    				show_spinner_dialog,
    				IndicatorType,
    			},
    			standard_dialogs::{
    				show_info_dialog,
    				show_info_button_dialog,
    				show_dialog_title,
    				show_error_dialog,
    				show_error_button_dialog,
    				show_error_title_dialog,
    				show_error_title_button_dialog,
    			},
    		},
    		theme::load_adw_provider::use_adw_provider,
    		utils::{
    			create_entry::{
    				create_entry,
    				create_password_entry,
    			},    		
    			create_label_icon::{
    				create_label_icon,
    				create_label_icon_position,
    			},
    			create_special_button::{
    				create_button,
    				create_button_icon,
    				create_button_icon_no_callback,
    				create_button_two_icon,
    				create_button_icon_position,
    			},
    			command_pkexec_ui::{
    				command_pkexec_spinner,
    				command_pkexec_progressbar,
    			},
    		},
    	},
    };
}
