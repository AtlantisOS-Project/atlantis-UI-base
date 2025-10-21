/*
* write_log_text.c
*
* (C) Copyright 2025 AtlantisOS Project
* by @NachtsternBuild
*
* License: GNU GENERAL PUBLIC LICENSE Version 3
*
*/

#include "helper.h"
#include "language.h"
#include "design.h"

#define MAX_LOG_LINES 200

// use extern configs
extern int use_syslog;       
extern const char *LOCALE_DOMAIN;

// create new process
static void spawn_cb(VteTerminal *terminal, GPid pid, GError *error, gpointer user_data)
{
    if (error) 
    {
        LOGE("Error starting the process: %s\n", error->message);
        g_printerr("Error starting the process: %s\n", error->message);
        g_error_free(error);
    }
     
    else 
    {
        LOGI("Process started, PID=%d\n", pid);
        g_print("Process started, PID=%d\n", pid);
    }
}

// create a window, that shows the log
void log_viewer(void)
{
    GtkWidget *window_log;
    GtkWidget *terminal;

    // create window for the log viewer
    window_log = gtk_window_new();
    gtk_window_set_title(GTK_WINDOW(window_log), _("Log Viewer"));
    gtk_window_set_default_size(GTK_WINDOW(window_log), 800, 600);

    // create new terminal widget 
    terminal = vte_terminal_new();
    gtk_window_set_child(GTK_WINDOW(window_log), terminal);

    // use journalctl for the output
    // NOTE: 
    // use for logging journald
    // full log of journald
    //char *argv[] = { "journalctl", "-f", NULL };
    // specific for the domain
	char cmd[512];
	snprintf(cmd, sizeof(cmd), "journalctl -f -t %s --output=short", LOCALE_DOMAIN);

	char *argv[] = { "/bin/sh", "-c", cmd, NULL };
	
	// create a child process for the terminal widget
    vte_terminal_spawn_async(
        VTE_TERMINAL(terminal),
        VTE_PTY_DEFAULT,        // PTY
        NULL,                   // working dir
        argv,                   // argv
        NULL,                   // envv
        G_SPAWN_DEFAULT,        // flags
        NULL,                   // child_setup
        NULL,                   // child_setup_data
        NULL,                   // child_setup_data_destroy
        -1,                     // timeout
        NULL,                   // cancellable
        spawn_cb,               // callback
        NULL                    // user_data
    );
	
	// show the window
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
GtkWidget *create_custom_headerbar(gpointer stack) 
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
