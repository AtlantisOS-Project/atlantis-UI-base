// about_dialog.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! The standard "About" dialog.
//!
//! The module uses [`adw::AboutDialog`] to show consistent information about an
//! application: version, license, contributors and acknowledgments.

use crate::gettext;
use adw::AboutDialog;
use adw::prelude::*;
use gtk4::{License, Widget};

/// All information shown in the "About" dialog.
///
/// The struct passes the many pieces of information to [`show_about_dialog`] in an organized
/// way. It implements [`Default`] (all fields empty), so you can set only what you need:
///
/// ```
/// use atlbase::ui_prelude::AboutInfo;
///
/// let info = AboutInfo {
///     app_name: "My App".to_string(),
///     version: "1.0.0".to_string(),
///     ..Default::default()
/// };
/// assert!(info.developers.is_empty());
/// ```
#[derive(Debug, Clone, Default)]
pub struct AboutInfo {
    /// Name of the application icon (from the icon theme).
    pub app_icon: String,
    /// The display name of the application.
    pub app_name: String,
    /// The main developer or organization.
    pub developer_name: String,
    /// The current version of the program.
    pub version: String,
    /// The version the release notes belong to.
    pub release_notes_version: String,
    /// A short summary of the changes (Pango markup allowed).
    pub release_notes: String,
    /// A short description of the application.
    pub comments: String,
    /// The website of the application.
    pub website: String,
    /// URL of the issue tracker (for example the GitHub issues).
    pub issue_url: String,
    /// Link to the support resources.
    pub support_url: String,
    /// The copyright line, for example `"© 2026 AtlantisOS Project"`.
    pub copyright: String,
    /// The developers.
    pub developers: Vec<String>,
    /// The graphic artists and designers.
    pub artists: Vec<String>,
    /// Link to the online documentation. Without a link the entry is left out.
    pub documentation_url: String,
    /// Fonts that are used and their licenses. Without text the section is left out.
    pub font_usage: String,
    /// Special acknowledgments. Without entries the section is left out.
    pub special_thanks: Vec<String>,
    /// The credits for the translators.
    pub translator_credits: String,
}

/// Shows the "About" dialog of the application.
///
/// The dialog is presented over `parent` (usually the main window). It always uses the
/// license GPL 3.0 and has sections for the legal information, the contributors and the
/// acknowledgments. The headings of the sections are translated with
/// [`gettext!`](crate::gettext!).
///
/// # Arguments
///
/// * `parent` - The widget the dialog is shown over.
/// * `info` - The information to show.
///
/// # Examples
///
/// ```no_run
/// use atlbase::ui_prelude::{AboutInfo, show_about_dialog};
/// # gtk4::init().unwrap();
/// # let main_window = gtk4::Window::new();
///
/// let info = AboutInfo {
///     app_icon: "atlantis-os-logo".to_string(),
///     app_name: "AtlantisOS Dashboard".to_string(),
///     developer_name: "NachtsternBuild".to_string(),
///     version: "2026.1.0".to_string(),
///     release_notes_version: "2026.1.0".to_string(),
///     release_notes: "Stable release".to_string(),
///     comments: "The dashboard of AtlantisOS".to_string(),
///     website: "https://example.org".to_string(),
///     issue_url: "https://example.org/issues".to_string(),
///     support_url: "https://example.org/support".to_string(),
///     copyright: "© 2026 AtlantisOS Project".to_string(),
///     developers: vec!["NachtsternBuild".to_string()],
///     artists: vec!["DesignTeam Alpha".to_string()],
///     documentation_url: "https://docs.atlantisos.org".to_string(),
///     font_usage: "Cantarell, SIL Open Font License".to_string(),
///     special_thanks: vec!["GNOME Team".to_string(), "Rust Community".to_string()],
///     translator_credits: "Translated by the AtlantisOS Project".to_string(),
/// };
/// show_about_dialog(&main_window, info);
/// ```
pub fn show_about_dialog(parent: &impl IsA<Widget>, info: AboutInfo) {
    let about = AboutDialog::builder()
        .application_icon(&info.app_icon)
        .application_name(&info.app_name)
        .developer_name(&info.developer_name)
        .version(&info.version)
        .release_notes_version(&info.release_notes_version)
        .release_notes(&info.release_notes)
        .comments(&info.comments)
        .website(&info.website)
        .issue_url(&info.issue_url)
        .support_url(&info.support_url)
        .copyright(&info.copyright)
        .license_type(License::Gpl30)
        .developers(info.developers)
        .artists(info.artists)
        .translator_credits(&info.translator_credits)
        .build();

    if !info.documentation_url.is_empty() {
        about.add_link(&gettext!("Documentation"), &info.documentation_url);
    }

    if !info.font_usage.is_empty() {
        about.add_legal_section(
            &gettext!("Fonts"),
            None,
            License::Custom,
            Some(&info.font_usage),
        );
    }

    if !info.special_thanks.is_empty() {
        let thanks: Vec<&str> = info.special_thanks.iter().map(String::as_str).collect();
        about.add_acknowledgement_section(Some(&gettext!("Special thanks to")), &thanks);
    }

    about.present(Some(parent));
}
