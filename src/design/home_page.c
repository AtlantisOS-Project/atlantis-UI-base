/*
* free_wrapper.c
*
* (C) Copyright 2025 AtlantisOS Project
* by @NachtsternBuild
*
* License: GNU GENERAL PUBLIC LICENSE Version 3
*
* Usage:
* g_signal_connect(button_name, "clicked", G_CALLBACK(show_home_page), stack);
*
* Note: Requires a GTK stack
*/

#include <glib.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <gtk/gtk.h>
#include <gdk/gdk.h>
#include "helper.h"
#include "design.h"

// show home page
void show_home_page(GtkWidget *widget, gpointer stack) 
{
    gtk_stack_set_visible_child_name(GTK_STACK(stack), "home_page");
}
