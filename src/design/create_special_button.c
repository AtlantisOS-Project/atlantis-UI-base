/*
* create_special_icon.c
*
* (C) Copyright 2025 AtlantisOS Project
* by @NachtsternBuild
*
* License: GNU GENERAL PUBLIC LICENSE Version 3
*
* Usage:
* create_button("some text", callback, data);
* create_button_icon("utilities-terminal", "some text", callback, data);
* create_button_icon_no_callback("utilities-terminal", "some text");
* create_button_two_icon("utilities-terminal", "utilities-terminal", "some text", callback, data);
* create_button_icon_position("utilities-terminal", "some text", callback, data, GTK_ALIGN_CENTER);
*/

#include "helper.h"
#include "design.h"

// create a button for navigation
GtkWidget *create_button(const char *label, GCallback callback, gpointer data) 
{
    GtkWidget *button = gtk_button_new_with_label(label);
    g_signal_connect(button, "clicked", callback, data);
    return button;
}

// create a button with icon
GtkWidget *create_button_icon(const char *icon_name, const char *label_text, GCallback callback, gpointer data) 
{
    GtkWidget *button = gtk_button_new();
    GtkWidget *box = gtk_box_new(GTK_ORIENTATION_HORIZONTAL, 5);

    GtkWidget *image = gtk_image_new_from_icon_name(icon_name);
    GtkWidget *label = gtk_label_new(label_text);

    gtk_box_append(GTK_BOX(box), image);
    gtk_box_append(GTK_BOX(box), label);
    gtk_button_set_child(GTK_BUTTON(button), box);
    g_signal_connect(button, "clicked", callback, data);

    return button;
}

// create a button with icon, but without callback
GtkWidget *create_button_icon_no_callback(const char *icon_name, const char *label_text) 
{
    GtkWidget *button = gtk_button_new();
    GtkWidget *box = gtk_box_new(GTK_ORIENTATION_HORIZONTAL, 5);

    GtkWidget *image = gtk_image_new_from_icon_name(icon_name);
    GtkWidget *label = gtk_label_new(label_text);

    gtk_box_append(GTK_BOX(box), image);
    gtk_box_append(GTK_BOX(box), label);
    gtk_button_set_child(GTK_BUTTON(button), box);
    
    return button;
}

// create a button with two icons
GtkWidget *create_button_two_icon(const char *first_icon, const char *second_icon, const char *label_text, GCallback callback, gpointer data) 
{
    GtkWidget *button = gtk_button_new();
    GtkWidget *box = gtk_box_new(GTK_ORIENTATION_HORIZONTAL, 5);

    if (first_icon != NULL)
    {
        GtkWidget *first_img = gtk_image_new_from_icon_name(first_icon);
        gtk_box_append(GTK_BOX(box), first_img);
    }

    GtkWidget *second_img = gtk_image_new_from_icon_name(second_icon);
    GtkWidget *label = gtk_label_new(label_text);

    gtk_box_append(GTK_BOX(box), second_img);
    gtk_box_append(GTK_BOX(box), label);
    gtk_button_set_child(GTK_BUTTON(button), box);
    g_signal_connect(button, "clicked", callback, data);

    return button;
}

// create a button with an icon and set special position
GtkWidget *create_button_icon_position(const char *icon_name, const char *label_text, GCallback callback, gpointer data, GtkAlign alignment)
{
    GtkWidget *button = gtk_button_new();
    GtkWidget *box = gtk_box_new(GTK_ORIENTATION_HORIZONTAL, 5);

    GtkWidget *image = gtk_image_new_from_icon_name(icon_name);
    GtkWidget *label = gtk_label_new(label_text);

    gtk_box_append(GTK_BOX(box), image);
    gtk_box_append(GTK_BOX(box), label);

    gtk_widget_set_halign(box, alignment);

    gtk_button_set_child(GTK_BUTTON(button), box);
    g_signal_connect(button, "clicked", callback, data);

    return button;
}
