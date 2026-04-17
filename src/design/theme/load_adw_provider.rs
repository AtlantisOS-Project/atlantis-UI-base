//! Function that apply extern CSS style 
/**
 * load_adw_provider.rs
 *
 * (C) Copyright 2026 AtlantisOS Project
 * by @NachtsternBuild
 *
 * License: GNU GENERAL PUBLIC LICENSE Version 3
 */
use adw::StyleManager;
use gtk4::{CssProvider, style_context_add_provider_for_display, STYLE_PROVIDER_PRIORITY_USER};
use std::cell::RefCell;

thread_local! {
    static PROVIDER_ADW: RefCell<Option<CssProvider>> = RefCell::new(None);
}

/**
 * @brief Load the CSS data from a string
 */
fn load_adw_provider_from_string(css_data: &str) { 
    PROVIDER_ADW.with(|storage| {
        if let Some(provider) = storage.borrow().as_ref() {
            // using loading from string
            provider.load_from_string(css_data);
        }
    });
}

/// Init the CSS ADW provider
/// ### Usage:
///
/// ```rust
/// pub const ADW_CUSTOM_CSS: &str = r#"
/// window {
///    background-color: @theme_bg_color;
///    color: @theme_fg_color;
///    padding: 12px;
///    font-size: 14px;
///    border: 2px solid @theme_bg_color;
///    border-radius: 35px;
/// }"#;
/// use_adw_provider(ADW_CUSTOM_CSS); 
/// ```
pub fn use_adw_provider(css_content: &str) {
    let new_provider = CssProvider::new();

    if let Some(display) = gtk4::gdk::Display::default() {
        style_context_add_provider_for_display(
            &display,
            &new_provider,
            STYLE_PROVIDER_PRIORITY_USER,
        );
    }

    PROVIDER_ADW.with(|storage| {
        *storage.borrow_mut() = Some(new_provider);
    });

    // init laod the string
    load_adw_provider_from_string(css_content);

    // handeling theme
    let style_manager = StyleManager::default();
    
    // clone the string for closure
    let content_clone = css_content.to_string();
    style_manager.connect_dark_notify(move |_| {
        load_adw_provider_from_string(&content_clone);
    });
}
