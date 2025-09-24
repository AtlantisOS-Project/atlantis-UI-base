/*
* dialog.c
*
* (C) Copyright 2025 AtlantisOS Project
* by @NachtsternBuild
*
* License: GNU GENERAL PUBLIC LICENSE Version 3
*
* 1. show adw alert dialog
*
* Usage:
* show_alert_dialog(widget, "Dialog Title", "Some text in the dialog", "OK");
*
* 2. Probaly some other dialogs
* TODO: create multiple dialogs
*/

#include <glib.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <gtk/gtk.h>
#include "helper.h"
#include "design.h"
#include "dialogs.h"
#include "language.h"

// show a new adw alert dialog
void show_alert_dialog(GtkWidget *parent, const char *title, const char *body, const char *button_label)
{
	// create new adw dialog
	AdwDialog *dialog;
  	dialog = adw_alert_dialog_new(title, body);
	
	// add response to the dialog
  	adw_alert_dialog_add_responses(ADW_ALERT_DIALOG (dialog),
                                  "ok",  button_label,
                                  NULL);
	// set the adw response to the button
  	adw_alert_dialog_set_response_appearance(ADW_ALERT_DIALOG(dialog), "ok", ADW_RESPONSE_SUGGESTED);
	
	// define the event of the dialog
  	adw_alert_dialog_set_close_response(ADW_ALERT_DIALOG(dialog), "ok");
	
	// show the dialog
  	adw_dialog_present(dialog, GTK_WIDGET(parent));
}
