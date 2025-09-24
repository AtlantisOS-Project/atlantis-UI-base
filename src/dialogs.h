/*
* dialogs.h
*
* (C) Copyright 2025 AtlantisOS Project
* by @NachtsternBuild
*
* License: GNU GENERAL PUBLIC LICENSE Version 3
*
* Usage:
* #include "dialogs.h"
*/
#ifndef DIALOGS_H
#define DIALOGS_H

#ifdef __cplusplus
extern "C" {
#endif

#pragma once

#include <stdio.h>
#include <stdlib.h>
#include <glib.h>
#include <stdarg.h>
#include <time.h>
#include <unistd.h>
#include <string.h>
#include <gtk/gtk.h>
#include <adwaita.h>
#include "language.h"
#include "helper.h"


/*
* Show adw alert dialog
*
* Usage:
* show_alert_dialog(widget, "Dialog Title", "Some text in the dialog", "OK");
*/
// function that show a adw alert dialog
void show_alert_dialog(GtkWidget *parent, const char *title, const char *body, const char *button_label);

/*
* Show the adw about dialog
*
* Usage:
* show_about_dialog(GtkWidget *parent);
*/
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

// function that show the adw about dialog
void show_about_dialog(GtkWidget *parent);

#ifdef __cplusplus
}
#endif

#endif // DIALOGS_H
