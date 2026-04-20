//! Provides the standardized "About" dialog.
//!
//! This module uses `adw::AboutDialog` to display consistent metadata about the application,
//! licensing, and contributors.
/**
* about_dialog.rs
*
* (C) Copyright 2026 AtlantisOS Project
* by @NachtsternBuild
*
* License: GNU GENERAL PUBLIC LICENSE Version 3
*/

use adw::prelude::*;
use adw::{AboutDialog};
use gtk4::Widget;
use gtk4::License;

/// Contains all metadata to be displayed in the “About” dialog.
///
/// This structure serves as a container for passing the various pieces of information
/// to the [show_about_dialog] function in a clear and organized manner.
#[derive(Debug, Clone)]
pub struct AboutInfo {
    /// Name of the app icon (from the icon theme or path)
    pub app_icon: String,
    /// The display name of the application
    pub app_name: String,
    /// Primary developer or organization
    pub developer_name: String,
    /// Current program version
    pub version: String,
    /// Version of the current release notes
    pub release_notes_version: String,
    /// Brief summary of changes (Markup allowed)
    pub release_notes: String,
    /// A brief description of the application
    pub comments: String,
    /// Official website of the application
    pub website: String,
    /// URL to the issue tracker (e.g., GitHub/GitLab Issues)
    pub issue_url: String,
    /// Link to support resources
    pub support_url: String,
    /// Copyright string (e.g., “© 2026 AtlantisOS Project”)
    pub copyright: String,
    /// List of contributing developers
    pub developers: Vec<String>,
    /// Graphic artists and designers
    pub artists: Vec<String>,
    /// Link to online documentation
    pub documentation_url: String,
    /// Information about fonts used and their licenses
    pub font_usage: String,
    /// Special acknowledgments
    pub special_thanks: Vec<String>,
    /// Credits for translators
    pub translator_credits: String,
}

/// Opens a Libadwaita "About" dialog with the specified information.
///
/// The dialog is displayed modally relative to the `parent` widget (usually the main window).
/// It automatically includes sections for legal information, contributors, and acknowledgments.
///
/// # Arguments
///
/// * `parent` - The widget from which the dialog is "presented".
/// * `info` - An [AboutInfo] instance containing the data to be displayed.
///
/// # Usage:
///
/// ```rust
/// let info = AboutInfo {
///    app_icon: "atlantis-os-logo".to_string(),
///    app_name: "AtlantisOS Dashboard".to_string(),
///    developer_name: "NachtsternBuild".to_string(),
///    version: "2026.1.0".to_string(),
///    release_notes_version: "1.0.0".to_string(),
///    release_notes: "Stable Release".to_string(),
///    comments: "Some UI".to_string(),
///    website: "https://example.org".to_string(),
///    issue_url: "https://example.org/issues".to_string(),
///    support_url: "https://example.org/support".to_string(),
///    copyright: "© 2026 AtlantisOS Project".to_string(),
///    developers: vec!["NachtsternBuild".to_string(), "OpenSource Contributor".to_string()],
///    artists: vec!["DesignTeam Alpha".to_string()],
///    documentation_url: "https://docs.atlantisos.org".to_string(),
///    font_usage: "Some cool font".to_string(),
///    special_thanks: vec!["GNOME Team".to_string(), "Rust Community".to_string()],
///    translator_credits: "Translated by AtlantisOS Project".to_string(),
/// };
/// show_about_dialog(&main_window, info);
/// ```
pub fn show_about_dialog(parent: &impl IsA<Widget>, info: AboutInfo) {
    // create the dialog
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

    // add documentation
    about.add_link("Documentation", &info.documentation_url);

    // Fonts and Licenses
    about.add_legal_section(
        "Fonts",
        None,
        License::Custom,
        Some(&info.font_usage),
    );
    
    // add some thanks
    let thanks_refs: Vec<&str> = info.special_thanks.iter().map(|s| s.as_str()).collect();
    about.add_acknowledgement_section(Some("Special thanks to"), &thanks_refs);

    // show dialog
    about.present(Some(parent));
}
