// style.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! The default style of the guide widgets.
//!
//! The style is registered once through
//! [`use_adw_provider_with_priority`](crate::design::theme::load_adw_provider::use_adw_provider_with_priority)
//! with the priority `STYLE_PROVIDER_PRIORITY_APPLICATION`. Providers created with
//! [`use_adw_provider`](crate::design::theme::load_adw_provider::use_adw_provider) have the
//! higher priority `STYLE_PROVIDER_PRIORITY_USER`, so they always win: that is the place for
//! your own adjustments.

use crate::design::theme::load_adw_provider as theme;
use std::cell::Cell;

/// The default stylesheet of the guide widgets.
///
/// It uses only Libadwaita color variables, so it follows the light/dark style and the
/// accent color automatically. All rules are meant to be overridden with
/// [`use_adw_provider`](crate::design::theme::load_adw_provider::use_adw_provider):
///
/// | CSS class | Element |
/// |---|---|
/// | `.adw-guide` | root of a [`GuideView`](super::GuideView) |
/// | `.guide-step-number` | number bubble of a step |
/// | `.guide-callout` | note box (plus `-info`, `-tip`, `-warning` or `-danger`) |
/// | `.guide-code` | code block |
/// | `.guide-image-missing` | placeholder for a missing image |
///
/// # Examples
///
/// ```no_run
/// use atlbase::ui_prelude::use_adw_provider;
/// # gtk4::init().unwrap();
///
/// use_adw_provider(
///     ".adw-guide .guide-step-number { background-color: @purple_3; }
///      .adw-guide .guide-callout-tip { background-color: alpha(@purple_3, 0.2); }",
/// );
/// ```
pub const GUIDE_CSS: &str = r#"
.guide-step-number {
    background-color: @accent_bg_color;
    color: @accent_fg_color;
    border-radius: 999px;
    min-width: 28px;
    min-height: 28px;
    font-weight: 700;
}
.guide-callout {
    border-radius: 12px;
    padding: 12px 14px;
}
.guide-callout-info    { background-color: alpha(@accent_bg_color,  0.14); }
.guide-callout-tip     { background-color: alpha(@success_bg_color, 0.14); }
.guide-callout-warning { background-color: alpha(@warning_bg_color, 0.18); }
.guide-callout-danger  { background-color: alpha(@error_bg_color,   0.14); }
.guide-code {
    background-color: alpha(currentColor, 0.07);
    border-radius: 8px;
    padding: 6px 6px 6px 12px;
}
.guide-image-missing {
    border-radius: 12px;
    padding: 18px;
}
"#;

thread_local! {
    static DEFAULT_STYLE: Cell<bool> = const { Cell::new(false) };
}

/// Registers [`GUIDE_CSS`] exactly once per thread. The guide widgets call this themselves.
pub(super) fn ensure_default_style() {
    theme::ensure_default_style(&DEFAULT_STYLE, GUIDE_CSS);
}
