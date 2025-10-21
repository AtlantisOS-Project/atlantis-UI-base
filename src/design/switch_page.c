/*
* switch_page.c
*
* (C) Copyright 2025 AtlantisOS Project
* by @NachtsternBuild
*
* License: GNU GENERAL PUBLIC LICENSE Version 3
*
* Usage:
* g_signal_connect(button_name, "clicked", G_CALLBACK(switch_page), "name_of_next_stack_part");
*
* Note: Requires a GTK stack
*/
 
#include "helper.h"
#include "design.h"

// function to switch pages in the stack 
void switch_page(GtkWidget *widget, gpointer user_data) 
{
    GtkStack *stack = GTK_STACK(g_object_get_data(G_OBJECT(widget), "stack"));
    const gchar *change_page = user_data; // next page from parameter
    gtk_stack_set_visible_child_name(stack, change_page);
}

