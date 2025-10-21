/*
* dialogs.c
*
* (C) Copyright 2025 AtlantisOS Project
* by @NachtsternBuild
*
* License: GNU GENERAL PUBLIC LICENSE Version 3
*
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

#include "helper.h"
#include "design.h"
#include "dialogs.h"
#include "language.h"

// show information dialog
void show_info_dialog(GtkWidget *parent, const char *body)
{
	const char *title = _("Information");
	const char *button_label = _("OK");
	// run dialog
	show_alert_dialog(parent, title, body, button_label);
}

// show information dialog with button title
void show_info_button_dialog(GtkWidget *parent, const char *body, const char *button_label)
{
	const char *title = _("Information");
	// run dialog
	show_alert_dialog(parent, title, body, button_label);
}

// show dialog with title
void show_dialog_title(GtkWidget *parent, const char *title, const char *body)
{
	const char *button_label = _("OK");
	// run dialog
	show_alert_dialog(parent, title, body, button_label);
}

// show error dialog
void show_error_dialog(GtkWidget *parent, const char *body)
{
	const char *title = _("Error");
	const char *button_label = _("OK");
	// run dialog
	show_alert_dialog(parent, title, body, button_label);
}

// show error dialog with special button
void show_error_button_dialog(GtkWidget *parent, const char *body, const char *button_label)
{
	const char *title = _("Error");
	// run dialog
	show_alert_dialog(parent, title, body, button_label);
}

// show error message dialog with special title
void show_error_title_dialog(GtkWidget *parent, const char *title, const char *body)
{
	const char *button_label = _("OK");
	const char *formatted_title = g_strdup_printf("Error: %s", title);
	// run  dialog
	show_alert_dialog(parent, formatted_title, body, button_label);
	g_free((char *)formatted_title);
}

// show error message dialog with special title and special button
void show_error_title_button_dialog(GtkWidget *parent, const char *title, const char *body, const char *button_label)
{
	const char *formatted_title = g_strdup_printf("Error: %s", title);
	// run  dialog
	show_alert_dialog(parent, formatted_title, body, button_label);
	g_free((char *)formatted_title);
}
