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
use gtk4::{Box as GtkBox, Label, Orientation, Stack, StackTransitionType, Align, glib, Button};
use atlbase::helper::language::language;
use atlbase::design::utils::create_special_button;
use atlbase::design::utils::create_entry;
use atlbase::design::dialogs::dialog;
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
            //Page::UserManagement => "users",
            //Page::Statistics => "stats",
            //Page::Logs => "logs",
        }
    }
}

// test page with all the test options
fn create_home_page(stack: &Stack) -> GtkBox {
	let container = GtkBox::new(Orientation::Vertical, 0);
    container.set_valign(Align::Center);
    container.set_hexpand(true);
	container.set_vexpand(true);

    // Test Dialog Button
    let btn_dialog = create_special_button::create_button_icon_position(
        "dialog-information-symbolic", 
        "Test Dialog", 
        Align::Center,
        move |btn| {
            dialog::show_alert_dialog(btn, &gettext!("Dialog Titel"), &gettext!("Inhalt des Dialogs"), &gettext!("OK"));
        }
    );
    container.append(&btn_dialog);
    
    
    let btn_test_log = create_special_button::create_button_icon_position(
        "dialog-information-symbolic", 
        "Test Log", 
        Align::Center,
        move |_| {
            log::info!("UI successful started");
    		log::error!("Critical Error found!");
        }
    );
    container.append(&btn_test_log);

    // Entry    
    let (username_row, username_entry) = create_entry::create_entry(&gettext!("Username:"), Some(&gettext!("Input Username")));
    let (password_row, password_entry) = create_entry::create_password_entry(&gettext!("Password:"), Some(&gettext!("Input Password")));
    
    container.append(&username_row);
    container.append(&password_row);

    // Login Callback
    let btn_login = Button::with_label(&gettext!("Login"));
    btn_login.connect_clicked(move |_| {
        let user = username_entry.text();
        let pass = password_entry.text();
        println!("Login Versuch -> User: {}, Pass: {}", user, pass);
    });
    
    container.append(&btn_login);
    
    let stack_clone = stack.clone(); 
    let btn_test_stack = create_special_button::create_button_icon_position(
        "dialog-information-symbolic", 
        "Test Stack", 
        Align::Center,
        move |_| {
            switch_to_page(&stack_clone, Page::Settings);
        }
    );
	container.append(&btn_test_stack);
	
	container
}

// helper funtion for create the stack
fn create_settings_page(stack: &Stack) -> GtkBox {
	let container = GtkBox::new(Orientation::Vertical, 0);
	container.set_valign(Align::Center);
	container.set_hexpand(true);
	container.set_vexpand(true);
	
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
    init_syslog("test_app").expect("Unable to initialize Syslog");
    // init language
    language::init_language("test_app", "./po", false);
    // init style
    use_adw_provider(ADW_CUSTOM_CSS);
    
    // add a toolbar
    let toolbar_view = ToolbarView::new();
    toolbar_view.set_vexpand(true);
    toolbar_view.set_hexpand(true);
    let header_bar = HeaderBar::new();
	
	// set the title for the page
    header_bar.set_title_widget(Some(&Label::new(Some(&gettext!("Test UIBase")))));
    let custom_header_content = create_custom_headerbar(app, "test_app");
    
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
