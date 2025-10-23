/*
 * folder_chooser.c
 *
 * (C) Copyright 2025 AtlantisOS Project
 * by @NachtsternBuild
 *
 * License: GNU GENERAL PUBLIC LICENSE Version 3
 *
 * Usage:
 * GtkWidget *btn = create_button("Choose folder", G_CALLBACK(show_folder_chooser), (gpointer)callback_function);
 *
 * callback_function(const char *folder_path);
 */

#include "helper.h"
#include "language.h"
#include "design.h"

// type definition for a function that processes a path
typedef void (*FolderProcessorFunc)(const char *path);

// function for work with the path
static void folder_dialog_response_callback(GtkFileDialog *dialog, GAsyncResult *res, gpointer user_data)
{
    g_autoptr(GFile) folder = gtk_file_dialog_select_folder_finish(dialog, res, NULL);
    if (folder)
    {
        g_autofree char *folder_path = g_file_get_path(folder);
        if (folder_path)
        {
            // the user work with the function
            FolderProcessorFunc process_func = (FolderProcessorFunc)user_data;
            if (process_func)
            {
                process_func(folder_path);
            }
        }
    }
}

// function for the folderchooser dialog
void show_folder_chooser(GtkWidget *widget, gpointer data)
{
    // get the parent window
    GtkWindow *parent_window = GTK_WINDOW(gtk_widget_get_root(widget));
    if (!GTK_IS_WINDOW(parent_window))
    {
        parent_window = NULL;
	}
	
	// create the folder chooser
    GtkFileDialog *dialog = gtk_file_dialog_new();

    //gtk_file_dialog_set_initial_folder(dialog, g_file_new_for_path(g_get_home_dir()));
	
	// show dialog and connect callback
    gtk_file_dialog_select_folder(
        dialog,
        parent_window,
        NULL,
        (GAsyncReadyCallback)folder_dialog_response_callback,
        data
    );
}

