// load_adw_provider.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! Dynamic CSS management for Libadwaita applications.
//!
//! The module loads CSS at run time and makes sure that the styles are applied again when
//! the system switches between the light and the dark style.

use adw::StyleManager;
use gtk4::prelude::*;
use gtk4::{
    CssProvider, STYLE_PROVIDER_PRIORITY_APPLICATION, STYLE_PROVIDER_PRIORITY_USER, gdk,
    style_context_add_provider_for_display,
};
use std::cell::Cell;
use std::thread::LocalKey;

/// Registers CSS for the whole application and keeps it up to date.
///
/// A new [`CssProvider`] is created for the default display with the priority
/// `STYLE_PROVIDER_PRIORITY_USER`, so these rules win over the system theme and over the
/// default styles of this library. Every call creates an **independent** provider: an
/// application and the libraries it uses can all register CSS without overwriting each
/// other.
///
/// # Special features
///
/// * **Reactivity:** a handler on [`adw::StyleManager`] loads the CSS again when the dark
///   style changes. Some color variables follow a switch of the style only after a reload.
/// * **Handle:** the returned provider can be ignored (the style then stays active for the
///   whole run time). Keep it to change the CSS later with
///   [`CssProvider::load_from_string`] — remember to reload on a style change yourself in
///   that case — or to remove it again with
///   [`gtk4::style_context_remove_provider_for_display`].
///
/// Call the function once per stylesheet, after GTK was initialized (for example in the
/// `activate` handler).
///
/// # Arguments
///
/// * `css_content` - The CSS rules.
///
/// # Examples
///
/// ```no_run
/// use atlbase::ui_prelude::use_adw_provider;
/// # gtk4::init().unwrap();
///
/// const ADW_CUSTOM_CSS: &str = r#"
/// window {
///     background-color: @theme_bg_color;
///     color: @theme_fg_color;
///     font-size: 14px;
/// }"#;
///
/// use_adw_provider(ADW_CUSTOM_CSS);
/// ```
pub fn use_adw_provider(css_content: &str) -> CssProvider {
    use_adw_provider_with_priority(css_content, STYLE_PROVIDER_PRIORITY_USER)
}

/// Like [`use_adw_provider`], but with an explicit priority.
///
/// Use a priority below `STYLE_PROVIDER_PRIORITY_USER` (for example
/// `STYLE_PROVIDER_PRIORITY_APPLICATION`) for default styles that the application should be
/// able to override.
///
/// # Arguments
///
/// * `css_content` - The CSS rules.
/// * `priority` - The style provider priority, for example
///   [`gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION`].
///
/// # Examples
///
/// ```no_run
/// use atlbase::ui_prelude::use_adw_provider_with_priority;
/// # gtk4::init().unwrap();
///
/// use_adw_provider_with_priority(
///     "button { border-radius: 12px; }",
///     gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
/// );
/// ```
pub fn use_adw_provider_with_priority(css_content: &str, priority: u32) -> CssProvider {
    let provider = CssProvider::new();
    provider.load_from_string(css_content);

    if let Some(display) = gdk::Display::default() {
        style_context_add_provider_for_display(&display, &provider, priority);
    }

    // A weak reference: once the provider is removed, the handler has nothing left to do.
    let weak = provider.downgrade();
    let css = css_content.to_string();
    StyleManager::default().connect_dark_notify(move |_| {
        if let Some(provider) = weak.upgrade() {
            provider.load_from_string(&css);
        }
    });

    provider
}

/// Registers the default style of a part of the library, once per thread.
///
/// `done` is a `thread_local!` flag of the caller. The style gets the priority
/// `STYLE_PROVIDER_PRIORITY_APPLICATION`, lower than [`use_adw_provider`], so applications can
/// always override it. Without a display (for example in unit tests) nothing is registered and
/// the next call tries again.
pub(crate) fn ensure_default_style(done: &'static LocalKey<Cell<bool>>, css: &str) {
    done.with(|done| {
        if done.get() || gdk::Display::default().is_none() {
            return;
        }
        use_adw_provider_with_priority(css, STYLE_PROVIDER_PRIORITY_APPLICATION);
        done.set(true);
    });
}
