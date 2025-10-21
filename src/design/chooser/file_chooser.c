/*
* file_chooser.c
*
* (C) Copyright 2025 AtlantisOS Project
* by @NachtsternBuild
*
* License: GNU GENERAL PUBLIC LICENSE Version 3
*
* Usage:
* GtkWidget *btn1 = create_button("label", G_CALLBACK(show_file_chooser), (gpointer)callback_function);
*/

#include "helper.h"
#include "language.h"
#include "design.h"

extern const char *DEFAULT_FILTER;

// function for work with the file
static void file_dialog_response_callback(GtkFileDialog *dialog, GAsyncResult *res, gpointer user_data) 
{
    g_autoptr(GFile) file = gtk_file_dialog_open_finish(dialog, res, NULL);
    if (file) 
    {
        g_autofree char *filename = g_file_get_path(file);
        if (filename) 
        {
            // the user work with the function
            FileProcessorFunc process_func = (FileProcessorFunc)user_data;
            process_func(filename);
        }
    }
}

// function for the filechooser dialog
void show_file_chooser(GtkWidget *widget, gpointer data) 
{
    // get the parent window
    GtkWindow *parent_window = GTK_WINDOW(gtk_widget_get_root(widget));
    if (!GTK_IS_WINDOW(parent_window)) 
    {
        parent_window = NULL;
    }

    // create filechooser
    GtkFileDialog *dialog = gtk_file_dialog_new();

	// load the external filter list
    GListStore *filter_list = load_file_filters(DEFAULT_FILTER);
    if (filter_list) 
    {
        gtk_file_dialog_set_filters(dialog, G_LIST_MODEL(filter_list));
    }

    // show dialog and connect callback
    gtk_file_dialog_open(dialog, parent_window, NULL, (GAsyncReadyCallback)file_dialog_response_callback, data);
}

