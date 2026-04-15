/**
* test_app.rs
*
* (C) Copyright 2026 AtlantisOS Project
* by @NachtsternBuild
*
* License: GNU GENERAL PUBLIC LICENSE Version 3
*
* Usage:
* cargo run --example test_app
*/

use adw::prelude::*;
use adw::{Application, ApplicationWindow, HeaderBar, ToolbarView};
use gtk4::{Box as GtkBox, Label, Orientation, Stack, StackTransitionType, Align, glib, Button, Grid};
use std::path::PathBuf;

// atl functions
use atlbase::helper::language::language;
use atlbase::design::utils::create_special_button;
use atlbase::design::utils::create_entry;
use atlbase::design::dialogs::dialog;
use atlbase::design::dialogs::about_dialog::AboutInfo;
use atlbase::design::chooser::file_chooser;
use atlbase::design::chooser::folder_chooser;
// use the extra gettext macro
use atlbase::gettext;
// use this for all preludes from the lib.rs
use atlbase::prelude::*;
use atlbase::ui_prelude::*;

// define the CSS string for the CSS provider
pub const ADW_CUSTOM_CSS: &str = r#"
window {
    background-color: @theme_bg_color;
    color: @theme_fg_color;
    padding: 12px;
    font-size: 14px;
    border: 2px solid @theme_bg_color;
    border-radius: 35px;
}
button {
    background-color: @accent_bg_color;
    color: @theme_fg_color;
    padding: 12px;
    border: 2px solid @theme_bg_color;
    border-radius: 35px;
    font-size: 14px;
}
button:hover {
     background-color: @accent_color;
     color: @accent_fg_color;
     transition: background-color 150ms ease;
}
label {
    color: @theme_fg_color;
    font-weight: bold;
    font-size: 16px;
}
headerbar {
    background-color: @theme_bg_color;
    font-weight: bold;
    color: @theme_fg_color;
    padding: 12px;
    border: 2px solid @theme_bg_color;
    border-radius: 35px;
    font-size: 14px;
}
frame {
     border-width: 5px;
     border-radius: 35px;
     border-color: @accent_bg_color;
     border-style: solid;
     padding: 10px;
}
"#;


// define the pages for the stack
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Page {
    Home,
    Settings,
    StandardDialog
    //UserManagement,
    //Statistics,
    //Logs,
}

// implement all the pages and there names
impl Page {
    fn to_id(&self) -> &str {
        match self {
            Page::Home => "home",
            Page::Settings => "settings",
            Page::StandardDialog => "standard_dialog",
            //Page::UserManagement => "users",
            //Page::Statistics => "stats",
            //Page::Logs => "logs",
        }
    }
}

// test page with all the test options
fn create_home_page(stack: &Stack) -> GtkBox {
	let container = GtkBox::new(Orientation::Vertical, 12); // spacing between the main groups
    container.set_valign(Align::Center);
    container.set_hexpand(true);
    container.set_vexpand(true);
    container.set_margin_top(24);
    container.set_margin_bottom(24);
    container.set_margin_start(24);
    container.set_margin_end(24);

    // create a grid
    let grid = Grid::new();
    grid.set_column_spacing(10); // spacing between columns
    grid.set_row_spacing(10);    // spacing between rows
    grid.set_halign(Align::Center);
    grid.set_hexpand(true);
    grid.set_vexpand(true);
	
	// about dialog
	let info = AboutInfo {
	    app_icon: "atlantis-os-logo".to_string(),
	    app_name: "Atlbase Test".to_string(),
	    developer_name: "NachtsternBuild".to_string(),
	    version: "2026.1.0".to_string(),
	    release_notes_version: "1.0.0".to_string(),
	    release_notes: "<p>Test Release</p>".to_string(),
	    comments: "Some UI".to_string(),
	    website: "https://example.org".to_string(),
	    issue_url: "https://example.org/issues".to_string(),
	    support_url: "https://example.org/support".to_string(),
	    copyright: "© 2026 AtlantisOS Project".to_string(),
	    developers: vec!["NachtsternBuild".to_string(), "OpenSource Contributor".to_string()],
	    artists: vec!["Gnome DesignTeam".to_string()],
	    documentation_url: "https://docs.atlantisos.org".to_string(),
	    font_usage: "Some cool font".to_string(),
	    special_thanks: vec!["GNOME Team".to_string(), "Rust Community".to_string()],
	    translator_credits: "Translated by AtlantisOS Project".to_string(),
	};
	
	let info_clone = info.clone();
	let btn_about = create_special_button::create_button_icon_position(
		"dialog-information-symbolic",
		"About Dialog",
		Align::Center,
		move |btn| {
			show_about_dialog(btn, info_clone.clone());
		}
	);
	
	// grid.attach(widget, spalte, zeile, breite, höhe)
	// grid.attach(widget, column, row, width, height)
    grid.attach(&btn_about, 0, 0, 1, 1);
	
    // test dialog button
    let btn_dialog = create_special_button::create_button_icon_position(
        "dialog-information-symbolic", 
        "Test Dialog", 
        Align::Center,
        move |btn| {
            dialog::show_alert_dialog(btn, &gettext!("Dialog Titel"), &gettext!("Inhalt des Dialogs"), &gettext!("OK"));
        }
    );
    grid.attach(&btn_dialog, 1, 0, 1, 1);
    
    // test filechooser
	// define callback
	fn something(pfad: PathBuf) {
		println!("File: {:?}", pfad);
	}
	
	let btn_test_file_chooser = create_special_button::create_button_icon(
		"media-zip-symbolic",
		"Test Filechooser",
		move |btn| {
			file_chooser::show_file_chooser(btn, something);
		}
	);
	grid.attach(&btn_test_file_chooser, 2, 0, 1, 1);
    
    // testing log
    let btn_test_log = create_special_button::create_button_icon_position(
        "dialog-information-symbolic", 
        "Test Log", 
        Align::Center,
        move |_| {
        	println!("Test Log");
            log::info!("UI successful started");
    		log::error!("Critical Error found!");
        }
    );
    grid.attach(&btn_test_log, 0, 1, 1, 1);
    
    // testing spinner dialog
    let btn_spinner_test = create_special_button::create_button_icon(
    	"update",
    	"Test Spinner",
    	move |btn| {
    		// get the main window with the button
        	if let Some(window) = btn.root().and_downcast_ref::<adw::ApplicationWindow>() {
    			show_spinner_dialog(
					window,
  					"System-Update",
  					"Please wait...",
  					vec![
  						"sleep 5".to_string(), 
  						"echo 'Ready'".to_string()
  					],
  					IndicatorType::Spinner 
  				);
    		}
    	}  
    );
    grid.attach(&btn_spinner_test, 1, 1, 1, 1);
    
    // test filechooser
	// define callback
	fn some_folder(path: PathBuf) {
		println!("Path: {:?}", path);
	}
	
	let btn_test_folder_chooser = create_special_button::create_button_icon(
		"media-zip-symbolic",
		"Test Folderchooser",
		move |btn| {
			folder_chooser::show_folder_chooser(btn, some_folder);
		}
	);
	grid.attach(&btn_test_folder_chooser, 2, 1, 1, 1);
    
    // testing progress dialog
    let btn_progress_test = create_special_button::create_button_icon(
    	"update",
    	"Test Progress",
    	move |btn| {
    		// get the main window with the button
        	if let Some(window) = btn.root().and_downcast_ref::<adw::ApplicationWindow>() {
    			show_spinner_dialog(
					window,
  					"System-Update",
  					"Please wait...",
  					vec![
  						"sleep 5".to_string(), 
  						"echo 'Ready'".to_string()
  					],
  					IndicatorType::ProgressBar
  				);
    		}
    	}
    );
    grid.attach(&btn_progress_test, 0, 2, 1, 1);
    
    // function to test a dialog with entry
    fn test_entry_call(test: &str) {
    	println!("{}", test);
    }
    
    let btn_test_entry_dialog = create_special_button::create_button_icon_position(
    	"error-correct-symbolic",
    	"Test Entry Dialog",
    	Align::Center,
    	move |btn| {
    		show_entry_dialog(
    			btn,
    			"Change Name",
    			"Add new Name",
    			"Save",
    			"Exit",
    			"Username: ",
    			"e.g. Donald Duck",
    			move |name| {
    				test_entry_call(&name);   			
    			}
    		 );
    	}
    );
    grid.attach(&btn_test_entry_dialog, 2, 2, 1, 1);
    		
	// function to test stack
	let stack_clone = stack.clone(); 
    let btn_test_stack = create_special_button::create_button_icon_position(
        "dialog-information-symbolic", 
        "Test Stack", 
        Align::Center,
        move |_| {
            switch_to_page(&stack_clone, Page::Settings);
        }
    );
	grid.attach(&btn_test_stack, 1, 2, 1, 1);
	
    // entry    
    let (username_row, username_entry) = create_entry::create_entry(&gettext!("Username:"), Some(&gettext!("Input Username")));
    let (password_row, password_entry) = create_entry::create_password_entry(&gettext!("Password:"), Some(&gettext!("Input Password")));
    
    // add the entries to the grid, but use to two columns
    grid.attach(&username_row, 0, 3, 2, 1);
    grid.attach(&password_row, 0, 4, 2, 1);

    // login Callback
    let btn_login = Button::with_label(&gettext!("Login"));
    btn_login.add_css_class("pill");
    btn_login.connect_clicked(move |_| {
        let user = username_entry.text();
        let pass = password_entry.text();
        println!("Login Try -> User: {}, Pass: {}", user, pass);
    });
    
	grid.attach(&btn_login, 0, 5, 2, 1);
    	
	container.append(&grid);
	container
}

// helper funtion for create the stack
fn create_settings_page(stack: &Stack) -> GtkBox {
	let container = GtkBox::new(Orientation::Vertical, 12);
	container.set_valign(Align::Center);
	container.set_hexpand(true);
	container.set_vexpand(true);
	
	let stack_clone1 = stack.clone();
	let btn_test_dialogs = create_special_button::create_button_icon_position(
		"folder-templates-symbolic",
		"Teste Standard Dialogs",
		Align::Center,
		move |_| {
			switch_to_page(&stack_clone1, Page::StandardDialog);
		}
	);
	container.append(&btn_test_dialogs);
	
	let stack_clone = stack.clone();
	let btn_back = create_special_button::create_button_icon_position(
		"dialog-information-symbolic",
		"Back",
		Align::Center,
		move |_| {
			switch_to_page(&stack_clone, Page::Home);
		}
	);
	container.append(&btn_back);
	container
}

// testing all the standard dialogs
fn create_test_standard_dialogs(stack: &Stack) -> GtkBox {
	let container = GtkBox::new(Orientation::Vertical, 12);
	container.set_valign(Align::Center);
	container.set_hexpand(true);
	container.set_vexpand(true);
		
	let stack_clone1 = stack.clone();	
	let btn1 = create_special_button::create_button_icon(
		"folder-saved-search-symbolic",
		"Dialog 1",
		move |_| {
			show_info_dialog(&stack_clone1, "test");
		}
	);
	container.append(&btn1);
	
	let stack_clone2 = stack.clone();
	let btn2 = create_special_button::create_button_icon(
		"folder-saved-search-symbolic",
		"Dialog 2",
		move |_| {
			show_info_button_dialog(&stack_clone2, "test", "TEST");
		}
	);
	container.append(&btn2);
	
	let stack_clone3 = stack.clone();
	let btn3 = create_special_button::create_button_icon(
		"folder-saved-search-symbolic",
		"Dialog 3",
		move |_| {
			show_dialog_title(&stack_clone3, "Test", "test");
		}
	);
	container.append(&btn3);
	
	let stack_clone4 = stack.clone();
	let btn4 = create_special_button::create_button_icon(
		"folder-saved-search-symbolic",
		"Dialog 4",
		move |_| {
			show_error_dialog(&stack_clone4, "test");
		}
	);
	container.append(&btn4);
	
	let stack_clone5 = stack.clone();
	let btn5 = create_special_button::create_button_icon(
		"folder-saved-search-symbolic",
		"Dialog 5",
		move |_| {
			show_error_button_dialog(&stack_clone5, "test", "TEST");
		}
	);
	container.append(&btn5);
	
	let stack_clone6 = stack.clone();
	let btn6 = create_special_button::create_button_icon(
		"folder-saved-search-symbolic",
		"Dialog 6",
		move |_| {
			show_error_title_dialog(&stack_clone6, "Test", "test");
		}
	);
	container.append(&btn6);
	
	let stack_clone7 = stack.clone();
	let btn7 = create_special_button::create_button_icon(
		"folder-saved-search-symbolic",
		"Dialog 7",
		move |_| {
			show_error_title_button_dialog(&stack_clone7, "Test", "test", "TEST");
		}
	);
	container.append(&btn7);
			
	let stack_clone = stack.clone();
	let btn_back = create_special_button::create_button_icon_position(
		"folder-templates-symbolic",
		"Back",
		Align::Center,
		move |_| {
			switch_to_page(&stack_clone, Page::Settings);
		}
	);
	container.append(&btn_back);
	
	container
}



/**
* @brief Function to switch between the pages
*/
fn switch_to_page(stack: &Stack, target: Page) {
    let id = target.to_id();

    // Check if page loaded
    if stack.child_by_name(id).is_none() {
        println!("Load page: {}...", id);
        
        // create the pages
        let widget = match target {
            Page::Home => create_home_page(stack),
            Page::Settings => create_settings_page(stack),
            Page::StandardDialog => create_test_standard_dialogs(stack),
            //Page::UserManagement => create_users_page(stack),
            //Page::Statistics => create_stats_page(stack),
            //Page::Logs => create_logs_page(stack),
        };
		
		// add the pages to the stack
        stack.add_named(&widget, Some(id));
    }

    // switch the page
    stack.set_visible_child_name(id);
}

/**
* Main function that create the basic gtk/adw UI
*/
fn build_ui(app: &adw::Application) {
	// init syslog
    init_syslog(language::LIB_DOMAIN).expect("Unable to initialize Syslog");
    // init language
    let test_dir = env!("COMPILED_LOCALE_DIR");
    language::init_language(language::LIB_DOMAIN, test_dir, false);
    
    // init style
    use_adw_provider(ADW_CUSTOM_CSS);
    
    // add a toolbar
    let toolbar_view = ToolbarView::new();
    toolbar_view.set_vexpand(true);
    toolbar_view.set_hexpand(true);
    let header_bar = HeaderBar::new();
	
	// set the title for the page
    header_bar.set_title_widget(Some(&Label::new(Some(&gettext!("Test UIBase")))));
    let custom_header_content = create_custom_headerbar(app, language::LIB_DOMAIN);
    
    // add the custom headerbar
    header_bar.pack_end(&custom_header_content);
    toolbar_view.add_top_bar(&header_bar);
    
    // create a stack
    let stack = Stack::builder()
        .transition_type(StackTransitionType::SlideLeftRight)
        .vexpand(true)
        .hexpand(true)
        .build();

	toolbar_view.set_content(Some(&stack));
	
    // initial load only the 'home' page
    switch_to_page(&stack, Page::Home);
	
	// create the window
    let window = ApplicationWindow::builder()
        .application(app)
        .content(&toolbar_view)
        .default_width(800)
        .default_height(600)
        .build();
	
	// present the window
    window.present();
}

// main function that create the gtk/adw application
fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id("org.test.atlantis.uibase")
        .build();

    app.connect_activate(build_ui);
    app.run()
}
