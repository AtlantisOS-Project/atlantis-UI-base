/**
* about_dialog.rs
*
* (C) Copyright 2026 AtlantisOS Project
* by @NachtsternBuild
*
* License: GNU GENERAL PUBLIC LICENSE Version 3
*
* @brief Show the adw about dialog
*
* Usage:
* let info = AboutInfo {
*    app_icon: "atlantis-os-logo".to_string(),
*    app_name: "AtlantisOS Dashboard".to_string(),
*    developer_name: "NachtsternBuild".to_string(),
*    version: "2026.1.0".to_string(),
*    release_notes_version: "1.0.0".to_string(),
*    release_notes: "Stable Release".to_string(),
*    comments: "Some UI".to_string(),
*    website: "https://example.org".to_string(),
*    issue_url: "https://example.org/issues".to_string(),
*    support_url: "https://example.org/support".to_string(),
*    copyright: "© 2026 AtlantisOS Project".to_string(),
*    developers: vec!["NachtsternBuild".to_string(), "OpenSource Contributor".to_string()],
*    artists: vec!["DesignTeam Alpha".to_string()],
*    documentation_url: "https://docs.atlantisos.org".to_string(),
*    font_usage: "Some cool font".to_string(),
*    special_thanks: vec!["GNOME Team".to_string(), "Rust Community".to_string()],
*    translator_credits: "Translated by AtlantisOS Project".to_string(),
* };
* show_about_dialog(&main_window, info);
*
* Note: Define the informations in the main program
*/

use adw::prelude::*;
use adw::{AboutDialog};
use gtk4::Widget;
use gtk4::License;

/**
* @brief Structure for all informations in the about dialog
*/
#[derive(Debug, Clone)]
pub struct AboutInfo {
    pub app_icon: String,
    pub app_name: String,
    pub developer_name: String,
    pub version: String,
    pub release_notes_version: String,
    pub release_notes: String,
    pub comments: String,
    pub website: String,
    pub issue_url: String,
    pub support_url: String,
    pub copyright: String,
    pub developers: Vec<String>,
    pub artists: Vec<String>,
    pub documentation_url: String,
    pub font_usage: String,
    pub special_thanks: Vec<String>,
    pub translator_credits: String,
}

/**
* @brief Show adw dialog about
*/  
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
