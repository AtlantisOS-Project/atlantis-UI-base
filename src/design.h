/*
* design.h
*
* (C) Copyright 2025 AtlantisOS Project
* by @NachtsternBuild
*
* License: GNU GENERAL PUBLIC LICENSE Version 3
*
* Usage:
* #include "design.h"
*/

#ifndef DESIGN_H
#define DESIGN_H

#include <stdio.h>
#include <stdlib.h>
#include <glib.h>
#include <stdarg.h>
#include <time.h>
#include <unistd.h>
#include <string.h>
#include <gtk/gtk.h>
#include <gdk/gdk.h>
#include <adwaita.h>
#include "language.h"
#include "helper.h"
#include "dialogs.h"

#ifdef __cplusplus
extern "C" {
#endif

/*
* Define special window width and height
*/
#define WINDOW_WIDTH 800
#define WINDOW_HEIGHT 600

/*
* Style Setting with libadwaita and CSS
*
*/
/*
* helper functions for libadwaita style and custom CSS
*/
// the adw_provider
const char *get_custom_adw_css(void);
// function that create a css provider
GtkCssProvider* create_css_provider();
// function that unload a css provider
void unload_css_provider(GtkCssProvider *provider);
// function that load the main css from get_custom_adw_css()
void load_adw_provider(void);
// function that reload the theme, if theme changed
void adw_theme_changed(AdwStyleManager *style_manager, GParamSpec *pspec, gpointer user_data);

/*
* use the libadwaita and custom CSS theme
*
* Usage:
* use-adw_provider();
*/
void use_adw_provider(void);

/*
* Switch page in the stack
*
* Usage:
* g_signal_connect(button_name, "clicked", G_CALLBACK(switch_page), "name_of_next_stack_part");
* g_signal_connect(button_name, "clicked", G_CALLBACK(show_home_page), stack);
*
* Note: Requires a GTK stack
*/
// switch to the home page
void show_home_page(GtkWidget *widget, gpointer stack);
// switch to pages
void switch_page(GtkWidget *widget, gpointer user_data);

/*
* Creating special labels
*
* Usage:
* create_label_icon("utilities-terminal", "some text");
* create_label_icon_position("utilities-terminal", "some text", GTK_ALIGN_CENTER);
*/
// label with icon
GtkWidget *create_label_icon(const char *icon_name, const char *label_text);
// label with icon and postion
GtkWidget *create_label_icon_position(const char *icon_name, const char *label_text, GtkAlign alignment);

/*
* Creating special buttons
*
* Usage:
* create_button("some text", callback, data);
* create_button_icon("utilities-terminal", "some text", callback, data);
* create_button_icon_no_callback("utilities-terminal", "some text");
* create_button_two_icon("utilities-terminal", "utilities-terminal", "some text", callback, data);
* create_button_icon_position("utilities-terminal", "some text", callback, data, GTK_ALIGN_CENTER);
*/
// normal button
GtkWidget *create_button(const char *label, GCallback callback, gpointer data) ;
// button with icon
GtkWidget *create_button_icon(const char *icon_name, const char *label_text, GCallback callback, gpointer data);
// button with icon, but without a callback
GtkWidget *create_button_icon_no_callback(const char *icon_name, const char *label_text);
// button with two icons
GtkWidget *create_button_two_icon(const char *main_icon, const char *second_icon, const char *label_text, GCallback callback, gpointer data);
// button with icon and special position
GtkWidget *create_button_icon_position(const char *icon_name, const char *label_text, GCallback callback, gpointer data, GtkAlign alignment);

/*
* Create a file chooser
*
* Usage:
* GtkWidget *btn1 = create_button("label", G_CALLBACK(show_file_chooser), (gpointer)callback_function);
* GtkWidget *btn2 = create_button("Choose folder", G_CALLBACK(show_folder_chooser), (gpointer)callback_function);
*/
// type definition for a function that processes a file
typedef void (*FileProcessorFunc)(const gchar *filename);
// type definition for a function that processes a path
typedef void (*FolderProcessorFunc)(const char *path);

// function that create the file chooser
void show_file_chooser(GtkWidget *widget, gpointer data);
// function that create the folder chooser 
void show_folder_chooser(GtkWidget *widget, gpointer data);

/*
* Creating entries
*
* Usage:
* 1. Normal entry: 
* GtkEntry *username_entry;
* GtkWidget *username_row = create_entry("Username:", "Input username: ", &username_entry);
* gtk_box_append(GTK_BOX(vbox), username_row);
*
* 2. Password entry:
* GtkEntry *password_entry;
* GtkWidget *password_row = create_password_entry("Password:", "Enter password", &password_entry);
* gtk_box_append(GTK_BOX(vbox), password_row);
*/
// create normal entry
GtkWidget* create_entry(const char *label_text, const char *placeholder, GtkEntry **entry_out);
// create password entry
GtkWidget* create_password_entry(const char *label_text, const char *placeholder, GtkEntry **entry_out);

/*
* pkexec command with spinner/progressbar
*/
// spinner varinate
void command_pkexec_spinner(GtkWidget *widget, const gchar *command, const char *title, const char *text);
// progressbar variante
void command_pkexec_progressbar(GtkWidget *widget, const gchar *command, const char *title, const char *text);

#ifdef __cplusplus
}
#endif

#endif // DESIGN_H
