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
#include <pthread.h>
#include <gtk/gtk.h>
#include <adwaita.h>
#include "language.h"
#include "helper.h"

// callback type for returning output
typedef void (*CommandFinishedCallback)(const gchar *output, gpointer user_data);

/*
* Show adw alert dialog
*
* Usage:
* show_alert_dialog(widget, "Dialog Title", "Some text in the dialog", "OK");
*/
// function that show a adw alert dialog
void show_alert_dialog(GtkWidget *parent, const char *title, const char *body, const char *button_label);

/*
* Multiple dialogs for special operations
*
* Usage:
* show_alert_dialog(widget, "Dialog Title", "Some text in the dialog", "OK");
* show_info_dialog(widget, "Some text in the dialog");
* show_info_button_dialog(widget, "Some text in the dialog", "OK");
* show_dialog_title(widget, "Dialog Title", "Some text in the dialog");
* show_error_dialog(widget, "Some text in the dialog");
* show_error_button_dialog(widget, "Some text in the dialog", "OK");
* show_error_title_dialog(widget, "Dialog Title", "Some text in the dialog");
* show_error_title_button_dialog(widget, "Dialog Title", "Some text in the dialog", "OK");
*/
// show information dialog
void show_info_dialog(GtkWidget *parent, const char *body);
// show information dialog with button title
void show_info_button_dialog(GtkWidget *parent, const char *body, const char *button_label);
// show dialog with title
void show_dialog_title(GtkWidget *parent, const char *title, const char *body);
// show error dialog
void show_error_dialog(GtkWidget *parent, const char *body);
// show error dialog with special button
void show_error_button_dialog(GtkWidget *parent, const char *body, const char *button_label);
// show error message dialog with special title
void show_error_title_dialog(GtkWidget *parent, const char *title, const char *body);
// show error message dialog with special title and special button
void show_error_title_button_dialog(GtkWidget *parent, const char *title, const char *body, const char *button_label);
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

/*
* Dialogs with spinner / progress bar
*
* Usage:
* show_spinner_dialog(GTK_WIDGET(widget), "Update running", "Please wait...", "sleep 5");
* show_progress_dialog(GTK_WIDGET(widget), "Installation", "Install package...", "sleep 5");
*/
// show dialog with spinner
void show_spinner_dialog(GtkWidget *parent, const char *title, const char *body, const char *cmd);
// show the dialog with spinner and return the output
void show_spinner_dialog_return(GtkWidget *parent, const char *title, const char *body, const char *cmd, CommandFinishedCallback callback, gpointer user_data);
// show dialog with progressbar
void show_progress_dialog(GtkWidget *parent, const char *title, const char *body, const char *cmd);
// show the dialog with progressbar and return the output
void show_progress_dialog_return(GtkWidget *parent, const char *title, const char *body, const char *cmd, CommandFinishedCallback callback, gpointer user_data);

#ifdef __cplusplus
}
#endif

#endif // DIALOGS_H
