/*
* create_entry.c
*
* (C) Copyright 2025 AtlantisOS Project
* by @NachtsternBuild
*
* License: GNU GENERAL PUBLIC LICENSE Version 3
*
* Usage:
* 1. Normal entry: 
* GtkEntry *username_entry;
* GtkWidget *username_row = create_entry("Username:", "Gib deinen Benutzernamen ein", &username_entry);
* gtk_box_append(GTK_BOX(vbox), username_row);
*
* 2. Password entry:
* GtkEntry *password_entry;
* GtkWidget *password_row = create_password_entry("Passwort:", "Gib dein Passwort ein", &password_entry);
* gtk_box_append(GTK_BOX(vbox), password_row);
*/

#include "helper.h"
#include "design.h"

//  create normal entry with label
GtkWidget* create_entry(const char *label_text, const char *placeholder, GtkEntry **entry_out)
{
    // new box
    GtkWidget *box = gtk_box_new(GTK_ORIENTATION_HORIZONTAL, 6);

    // create label and add it to the box
    GtkWidget *label = gtk_label_new(label_text);
    gtk_widget_set_halign(label, GTK_ALIGN_START);
    gtk_widget_set_valign(label, GTK_ALIGN_CENTER);
    gtk_widget_set_margin_start(label, 8);
    gtk_widget_set_margin_end(label, 8);

    // create a new entry
    GtkWidget *entry = gtk_entry_new();
    // add some placeholder text
    if (placeholder)
    {
        gtk_entry_set_placeholder_text(GTK_ENTRY(entry), placeholder);
	}
	
	gtk_widget_set_hexpand(entry, TRUE);
	
    // add everything to the box
    gtk_box_append(GTK_BOX(box), label);
    gtk_box_append(GTK_BOX(box), entry);
	
	// the entry output
    if (entry_out)
    {
        *entry_out = GTK_ENTRY(entry);
	}
	
	// return the box
    return box;
}

// create password entry with label
GtkWidget* create_password_entry(const char *label_text, const char *placeholder, GtkEntry **entry_out)
{
    // new box
    GtkWidget *box = gtk_box_new(GTK_ORIENTATION_HORIZONTAL, 6);
	
	// add the label to the start
    GtkWidget *label = gtk_label_new(label_text);
    gtk_widget_set_halign(label, GTK_ALIGN_START);
    gtk_widget_set_valign(label, GTK_ALIGN_CENTER);
	
	// create new entry
    GtkWidget *entry = gtk_entry_new();
    // hide the passwort
    // TODO: option to show the password
    gtk_entry_set_visibility(GTK_ENTRY(entry), FALSE); 
    gtk_entry_set_invisible_char(GTK_ENTRY(entry), '*');
    
	// add some placeholder text
    if (placeholder)
    {
        gtk_entry_set_placeholder_text(GTK_ENTRY(entry), placeholder);
	}
	
	// add everything to the box
    gtk_box_append(GTK_BOX(box), label);
    gtk_box_append(GTK_BOX(box), entry);
	
	// the entry output
    if (entry_out)
    {
        *entry_out = GTK_ENTRY(entry);
	}
	
	// return the box
    return box;
}
