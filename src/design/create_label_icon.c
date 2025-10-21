/*
* create_label_icon.c
*
* (C) Copyright 2025 AtlantisOS Project
* by @NachtsternBuild
*
* License: GNU GENERAL PUBLIC LICENSE Version 3
*
* Usage:
* create_label_icon("utilities-terminal", "some text");
* create_label_icon_position("utilities-terminal", "some text", GTK_ALIGN_CENTER);
*/

#include "helper.h"
#include "design.h"

// create a label with icon 
GtkWidget *create_label_icon(const char *icon_name, const char *label_text)
{
	GtkWidget *box = gtk_box_new(GTK_ORIENTATION_HORIZONTAL, 5);

    GtkWidget *image = gtk_image_new_from_icon_name(icon_name);
    GtkWidget *label = gtk_label_new(label_text);

    gtk_box_append(GTK_BOX(box), image);
    gtk_box_append(GTK_BOX(box), label);

    return box;
}

// create a label with icon and special position
GtkWidget *create_label_icon_position(const char *icon_name, const char *label_text, GtkAlign alignment)
{
	GtkWidget *box = gtk_box_new(GTK_ORIENTATION_HORIZONTAL, 5);

    GtkWidget *image = gtk_image_new_from_icon_name(icon_name);
    GtkWidget *label = gtk_label_new(label_text);

    gtk_box_append(GTK_BOX(box), image);
    gtk_box_append(GTK_BOX(box), label);
    
    gtk_widget_set_halign(box, alignment);

    return box;
} 
