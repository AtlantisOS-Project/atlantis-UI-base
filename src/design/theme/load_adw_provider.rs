//! Dynamic CSS management for Libadwaita applications.
//!
//! This module allows you to load custom CSS at runtime and
//! ensures that styles are correctly reapplied even when switching between light and 
//! dark mode.
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
	/// Holds the instance of the global CSS provider for the current thread.
    static PROVIDER_ADW: RefCell<Option<CssProvider>> = RefCell::new(None);
}

/// Loads CSS data from a string into the active provider.
///
/// This function is used internally to initially load the CSS content 
/// or to update it when the theme changes.
fn load_adw_provider_from_string(css_data: &str) { 
    PROVIDER_ADW.with(|storage| {
        if let Some(provider) = storage.borrow().as_ref() {
            // using loading from string
            provider.load_from_string(css_data);
        }
    });
}

/// Initializes and activates a global CSS provider for Libadwaita/GTK4.
///
/// This function registers a `CssProvider` for the default display and 
/// ensures that custom styles take precedence over the 
/// default system styles (`STYLE_PROVIDER_PRIORITY_USER`).
///
/// # Special Features
/// - **Reactivity:** Registers a listener on `adw::StyleManager` to reload the CSS when 
///   the `dark-notify` property changes. This is important 
///   because some CSS variables only respond correctly to theme changes after a reload.
/// - **Thread Safety:** Uses `thread_local!` to safely manage the provider within the context 
///   of the GTK main loop.
///
/// # Arguments
/// * `css_content` - A string slice containing the CSS rules.
///
/// # Usage:
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
