/*
* create_special_icon.c
*
* (C) Copyright 2025 AtlantisOS Project
* by @NachtsternBuild
*
* License: GNU GENERAL PUBLIC LICENSE Version 3
*
* 2. show the adw about dialog
*
* Usage:
* show_about_dialog(GtkWidget *parent);
*
* Note: Define the informations in the main program
*/

#include "helper.h"
#include "design.h"
#include "dialogs.h"
#include "language.h"

/*
* define some infos for the about page
*
* Note: Define the informations in the main program
*
* Usage:
* const char *CHARNAME = "CONTENT";
*
* const char *CHARNAME[] = {
* 	CONTENT1,
*	CONTENT2,
*	CONTENT3,
*	NULL}; // NULL always at the end
*/
// define the infos for the about page
extern const char *app_icon;
extern const char *app_name;
extern const char *developer_name;
extern const char *version;
extern const char *release_notes_version;
extern const char *release_notes;
extern const char *comments;
extern const char *website;
extern const char *issue_url;
extern const char *support_url;
extern const char *copyright;
extern const char *developers[];
extern const char *artists[];
extern const char *documentation_url;
extern const char *font_usage;
extern const char *special_thanks[];

// show adw dialog about   
void show_about_dialog(GtkWidget *parent)
{
    // create the adw about dialog and add a lot of informations
    AdwDialog *about;       
    about = g_object_new(ADW_TYPE_ABOUT_DIALOG,
                 		"application-icon", app_icon,
                 		"application-name", app_name,
                 		"developer-name", developer_name,
                 		"version", version,
                 		"release-notes-version", release_notes_version,
                 		"release-notes", release_notes,
                 		"comments", comments,
                 		"website", website,
                 		"issue-url", issue_url,
                 		"support-url", support_url,
                 		"copyright", copyright,
                 		"license-type", GTK_LICENSE_GPL_3_0,
                 		"developers", developers,
                 		"artists", artists,
                 		"translator-credits", _("translator-credits"),
                 		NULL);

    // add section for the documentation
	adw_about_dialog_add_link(ADW_ABOUT_DIALOG(about), _("Documentation"), documentation_url);
	
	// add section for legal informations
	adw_about_dialog_add_legal_section(ADW_ABOUT_DIALOG(about),
                                      _("Fonts"),
                                      NULL,
                                      GTK_LICENSE_CUSTOM,
                                      font_usage);
	// add some thanks
  	adw_about_dialog_add_acknowledgement_section(ADW_ABOUT_DIALOG(about), _("Special thanks to"), special_thanks);
   
   // show the dialog
    adw_dialog_present(about, GTK_WIDGET(parent));
}

