/*
* write_log_text.c
*
* (C) Copyright 2025 AtlantisOS Project
* by @NachtsternBuild
*
* License: GNU GENERAL PUBLIC LICENSE Version 3
*
*/

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <fcntl.h>
#include <glib.h>
#include <gtk/gtk.h>
#include <signal.h>
#include <unistd.h>
#include "helper.h"
#include "language.h"
#include "design.h"

GtkWidget *text_view = NULL;
FILE *log_file_fc;
int log_fd;
gsize last_offset = 0;
guint log_timeout_id = 0;

FILE *log_stream = NULL;

// use extern configs
extern int use_syslog;       
extern const char *LOCALE_DOMAIN;

// function that open the log 
void open_log_source()
{
    // V1: use journalctl (syslog only)
    if (use_syslog) 
    {
        char cmd[512];
        snprintf(cmd, sizeof(cmd), "journalctl -f -t %s --output=short", LOCALE_DOMAIN);
        log_stream = popen(cmd, "r");
    }
    // use own log file
    else 
    {
		// get the log file
        const char *log_file = get_log_path();  
        if (!log_file) 
        {
        	return;
        }
        
        // open the log file
        log_stream = fopen(log_file, "r");
        if (log_stream) 
        {
            fseek(log_stream, 0, SEEK_END);
            last_offset = ftell(log_stream);
        }
    }
}

// function to write string to the textview 
void write_to_textview(GtkWidget *text_view, const char *str)
{
    GtkTextBuffer *buffer = gtk_text_view_get_buffer(GTK_TEXT_VIEW(text_view));
    if (!buffer) return;

    GtkTextIter iter;
    gtk_text_buffer_get_end_iter(buffer, &iter);
    gtk_text_buffer_insert(buffer, &iter, str, -1);
    gtk_text_view_scroll_to_iter(GTK_TEXT_VIEW(text_view), &iter, 0, TRUE, 0.0, 0.0);
}


// function that reads the new logs
gboolean update_text_view_from_log(gpointer user_data)
{
    if (!log_stream) 
    {
    	return G_SOURCE_CONTINUE;
    }

    gchar buffer[512];
    if (use_syslog) 
    {
        // journalctl continuously outputs new lines
        if (fgets(buffer, sizeof(buffer), log_stream)) 
        {
            // write this to the textview
            write_to_textview(text_view, buffer);
        }
    } 
    
    else 
    {
        // own file 
        fseek(log_stream, last_offset, SEEK_SET);
        while (fgets(buffer, sizeof(buffer), log_stream)) 
        {
            // write this to the textview
            write_to_textview(text_view, buffer);
        }
        last_offset = ftell(log_stream);
    }
    return G_SOURCE_CONTINUE;
}

// cleanup when log viewer is closed 
void log_viewer_destroyed(GtkWidget *widget, gpointer user_data)
{
    if (log_timeout_id > 0)
    {
        g_source_remove(log_timeout_id);
        log_timeout_id = 0;
    }
}

// function that create the log viewer
void log_viewer(GtkButton *button, gpointer user_data)
{
	// create the log window
    GtkWidget *window_log = gtk_window_new();
    gtk_window_set_title(GTK_WINDOW(window_log), _("Log Viewer"));
    gtk_window_set_default_size(GTK_WINDOW(window_log), WINDOW_WIDTH, WINDOW_HEIGHT);
    g_signal_connect(window_log, "destroy", G_CALLBACK(log_viewer_destroyed), NULL);
	
	// add the main box
    GtkWidget *main_box = gtk_box_new(GTK_ORIENTATION_VERTICAL, 6);
    gtk_window_set_child(GTK_WINDOW(window_log), main_box);
	
	// create the text view
    text_view = gtk_text_view_new();
    gtk_text_view_set_editable(GTK_TEXT_VIEW(text_view), FALSE);
    gtk_text_view_set_cursor_visible(GTK_TEXT_VIEW(text_view), FALSE);
	
	// create the scrolled window
    GtkWidget *scrolled_window = gtk_scrolled_window_new();
    gtk_scrolled_window_set_policy(GTK_SCROLLED_WINDOW(scrolled_window),
                                   GTK_POLICY_AUTOMATIC, GTK_POLICY_AUTOMATIC);
    gtk_scrolled_window_set_child(GTK_SCROLLED_WINDOW(scrolled_window), text_view);
    gtk_widget_set_hexpand(scrolled_window, TRUE); 
    gtk_widget_set_vexpand(scrolled_window, TRUE); 
	
	// add the scrolled window to the box
    gtk_box_append(GTK_BOX(main_box), scrolled_window);
	
	// open the log file
    open_log_source();
    	
	// update the log every 250 ms
    log_timeout_id = g_timeout_add(250, update_text_view_from_log, NULL);

    gtk_window_present(GTK_WINDOW(window_log));
}

// function that kills the program itself
void kill_program(GtkButton *button, gpointer user_data)
{
	// get the pid of the program
	pid_t pid = getpid(); 
	// kill itself
    kill(pid, SIGKILL);
}

// header that create the popover menu
GtkWidget* create_custom_headerbar(gpointer stack) 
{    
    /*
    Icons for the menu:
    - item 1 - utilities-terminal / utilities-terminal-symbolic
    - item 2 - process-stop / application-exit
    */
    
    // create the headerbar as GTK box and create a menu
    GtkWidget *headerbar = gtk_box_new(GTK_ORIENTATION_HORIZONTAL, 0);
    GtkWidget *menu_button = gtk_menu_button_new();
    GtkWidget *popover = gtk_popover_new();
    GtkWidget *menu = gtk_list_box_new();
    
    /* item 1 */
    // function: show log window
   	GtkWidget *item1 = gtk_list_box_row_new();
	GtkWidget *icon1 = gtk_image_new_from_icon_name("utilities-system-monitor-symbolic"); // utilities-terminal - utilities-terminal-symbolic - utilities-system-monitor-symbolic
	GtkWidget *label1 = gtk_label_new(_("Show Log"));

	// box for the icon and the label
	GtkWidget *hbox1 = gtk_box_new(GTK_ORIENTATION_HORIZONTAL, 6); // 6 px distance
	gtk_box_append(GTK_BOX(hbox1), icon1);
	gtk_box_append(GTK_BOX(hbox1), label1);

	// create the button 
	GtkWidget *button1 = gtk_button_new();
	gtk_button_set_child(GTK_BUTTON(button1), hbox1);
	gtk_widget_set_tooltip_text(button1, "Show Log ");
	gtk_widget_set_halign(button1, GTK_ALIGN_START);
	// connect to the log viewer
	g_signal_connect(button1, "clicked", G_CALLBACK(log_viewer), NULL);

	// add button to the item1
	gtk_list_box_row_set_child(GTK_LIST_BOX_ROW(item1), button1);
	// add the item1 to the menu
	gtk_list_box_append(GTK_LIST_BOX(menu), item1);  
        	
	/* item 2 */
	// function: kill the fastboot-asssistant
    GtkWidget *item2 = gtk_list_box_row_new();
    GtkWidget *icon2 = gtk_image_new_from_icon_name("process-stop"); // process-stop / application-exit
    GtkWidget *label2 = gtk_label_new(_("Exit"));
    
    // box for the icon and the label
	GtkWidget *hbox2 = gtk_box_new(GTK_ORIENTATION_HORIZONTAL, 6);
	gtk_box_append(GTK_BOX(hbox2), icon2);
	gtk_box_append(GTK_BOX(hbox2), label2);
    
    // create the button 
	GtkWidget *button2 = gtk_button_new();
	gtk_button_set_child(GTK_BUTTON(button2), hbox2);
	gtk_widget_set_tooltip_text(button2, "Kill App");
	gtk_widget_set_halign(button2, GTK_ALIGN_START);

	g_signal_connect(button2, "clicked", G_CALLBACK(kill_program), NULL); 
	
	// add button to the item
	gtk_list_box_row_set_child(GTK_LIST_BOX_ROW(item2), button2);
	// add the item2 to menu
	gtk_list_box_append(GTK_LIST_BOX(menu), item2);
	
	// add the menu to the popover
    gtk_popover_set_child(GTK_POPOVER(popover), menu);
    gtk_menu_button_set_popover(GTK_MENU_BUTTON(menu_button), popover);
    
    // add the menu_button to the headerbar
    gtk_box_append(GTK_BOX(headerbar), menu_button);

    return headerbar;
}
