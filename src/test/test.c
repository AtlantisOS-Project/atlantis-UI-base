/*
* test.c
*
* (C) Copyright 2025 AtlantisOS Project
* by @NachtsternBuild
*
* License: GNU GENERAL PUBLIC LICENSE Version 3
*
* Usage:
* ./test
* 
* Note: This is a test code that is used to test all functions from the atlantis-UI-base.
*/

#include <stdio.h>
#include <stdlib.h>
#include <glib.h>
#include <stdarg.h>
#include <time.h>
#include <unistd.h>
#include <string.h>
#include <gtk/gtk.h>
#include <adwaita.h>
#include "language.h"
#include "helper.h"
#include "design.h"

// define the local domain
const char *LOCALE_DOMAIN = "test";

// dummy callback
static void test_dummy_callback(GtkWidget *widget, gpointer stack)
{
	LOGI("Button clicked");
}

// test the log
static void test_log_callback(GtkWidget *widget, gpointer stack)
{
	LOGI("Hello %s!", "World");
	LOGI("Total shit");
	LOGE("This are the are with code %d", 42);
	LOGI("Some bullshit");
	LOGW("This are warning. Nobody cares!");
	LOGD("Debug-Information: ");
	LOGD("Bring some fruits: ");
	LOGD("bananas=%dkg, apples=%dkg, raspberries=%dkg", 10, 20, 50);
	LOGI("No more nonsense");
}

// test switching in the stack
static void test_stack(GtkWidget *widget, gpointer stack) 
{
	// create box for get_devices
    GtkWidget *test_stack = gtk_box_new(GTK_ORIENTATION_VERTICAL, 5);
    gtk_widget_set_halign(test_stack, GTK_ALIGN_CENTER);
    gtk_widget_set_valign(test_stack, GTK_ALIGN_CENTER);
	
	// create some button 
    GtkWidget *btn1 = create_button(_("Normal Button"), G_CALLBACK(test_dummy_callback), stack);
    GtkWidget *btn2 = create_button_icon("preferences-other-symbolic", _("1. Special Button"), G_CALLBACK(test_dummy_callback), stack);
    GtkWidget *btn3 = create_button_icon_no_callback("preferences-other-symbolic", _("2. Special Button"));
    GtkWidget *btn4 = create_button_two_icon("preferences-other-symbolic", "preferences-other-symbolic", _("3. Special Button"), G_CALLBACK(test_dummy_callback), stack);
    GtkWidget *btn5 = create_button_icon_position("preferences-other-symbolic", _("4. Special Button"), G_CALLBACK(test_dummy_callback), stack, GTK_ALIGN_END);
    GtkWidget *btn_back = create_button_icon_position("pan-start-symbolic", _("Back Button"), G_CALLBACK(show_home_page), stack, GTK_ALIGN_CENTER);
	
    // add the button to the box
    gtk_box_append(GTK_BOX(test_stack), btn1); 
    gtk_box_append(GTK_BOX(test_stack), btn2); 
    gtk_box_append(GTK_BOX(test_stack), btn3); 
    gtk_box_append(GTK_BOX(test_stack), btn4); 
    gtk_box_append(GTK_BOX(test_stack), btn5); 
    gtk_box_append(GTK_BOX(test_stack), btn_back); 

	// is needed to prevent it from being stacked again when called again
    if (!gtk_stack_get_child_by_name(GTK_STACK(stack), "test_stack")) 
    {
        gtk_stack_add_named(GTK_STACK(stack), test_stack, "test_stack");
    }
    
    // show the page
	gtk_stack_set_visible_child_name(GTK_STACK(stack), "test_stack");
}

// switching the logging mode
static void set_syslog(GtkWidget *widget, gpointer stack)
{
	set_logging_mode(1);
}
static void set_manual(GtkWidget *widget, gpointer stack)
{
	set_logging_mode(0);
}
// switch the log type
static void switch_log_type_callback(GtkWidget *widget, gpointer stack)
{
	// create box for get_devices
    GtkWidget *switch_log_type = gtk_box_new(GTK_ORIENTATION_VERTICAL, 5);
    gtk_widget_set_halign(switch_log_type, GTK_ALIGN_CENTER);
    gtk_widget_set_valign(switch_log_type, GTK_ALIGN_CENTER);
	
	// create some labels
	GtkWidget *label1 = create_label_icon("preferences-other-symbolic", "some nonsense");
 	GtkWidget *label2 = create_label_icon_position("preferences-other-symbolic", "some bullshit", GTK_ALIGN_END);
	
	// create some button 
    GtkWidget *btn1 = create_button_icon("preferences-other-symbolic", _("Manual Logging"), G_CALLBACK(test_dummy_callback), stack);
    GtkWidget *btn2 = create_button_icon("preferences-other-symbolic", _("Syslog"), G_CALLBACK(test_dummy_callback), stack);
    GtkWidget *btn_back = create_button_icon("pan-start-symbolic", _("Back Button"), G_CALLBACK(show_home_page), stack);
    	
    // add the button  and label to the box
    gtk_box_append(GTK_BOX(switch_log_type), label1); 
    gtk_box_append(GTK_BOX(switch_log_type), label2); 
    gtk_box_append(GTK_BOX(switch_log_type), btn1); 
    gtk_box_append(GTK_BOX(switch_log_type), btn2); 
    gtk_box_append(GTK_BOX(switch_log_type), btn_back); 

	// is needed to prevent it from being stacked again when called again
    if (!gtk_stack_get_child_by_name(GTK_STACK(stack), "switch_log_type")) 
    {
        gtk_stack_add_named(GTK_STACK(stack), switch_log_type, "switch_log_type");
    }
    
    // show the page
	gtk_stack_set_visible_child_name(GTK_STACK(stack), "switch_log_type");
}

// main window
static void activate_test(GtkApplication* app, gpointer user_data) 
{
    // start logging
    // 0 → manual logging
    // 1 → syslog
    set_logging_mode(0);
    
    // use the advanced custom css provider
    //use_adw_provider();
    
    //adw_init();
    
    // create the main window
    GtkWidget *window = gtk_application_window_new (app); 
    //GtkWindow *window = GTK_WINDOW(gtk_window_new()); // second variante → use adw_init();  
    gtk_window_set_title(GTK_WINDOW(window), _("Test UI Base"));
    gtk_window_set_default_size(GTK_WINDOW(window), 400, 400);
    

    // create a box container for the main content
    GtkWidget *content_box = gtk_box_new(GTK_ORIENTATION_VERTICAL, 0);
    gtk_window_set_child(GTK_WINDOW(window), content_box);
    gtk_widget_set_halign(content_box, GTK_ALIGN_CENTER);
    gtk_widget_set_valign(content_box, GTK_ALIGN_CENTER);
    gtk_widget_set_hexpand(content_box, TRUE);
    gtk_widget_set_vexpand(content_box, TRUE);

    // create the stack for navigation
    GtkWidget *stack = gtk_stack_new();
    gtk_stack_set_transition_type(GTK_STACK(stack), GTK_STACK_TRANSITION_TYPE_SLIDE_LEFT_RIGHT);
    gtk_stack_set_transition_duration(GTK_STACK(stack), 300);
    gtk_widget_set_hexpand(stack, TRUE);
    gtk_widget_set_vexpand(stack, TRUE);

    // add the headerbar
    GtkWidget *headerbar = create_custom_headerbar(stack);
    gtk_box_append(GTK_BOX(content_box), headerbar);

    // add the stack to the box
    gtk_box_append(GTK_BOX(content_box), stack);

    // create the home page grid
    GtkWidget *home_page = gtk_grid_new();
    gtk_grid_set_row_homogeneous(GTK_GRID(home_page), TRUE);
    gtk_grid_set_column_homogeneous(GTK_GRID(home_page), TRUE);
    gtk_widget_set_halign(home_page, GTK_ALIGN_CENTER);
    gtk_widget_set_valign(home_page, GTK_ALIGN_CENTER);
    gtk_widget_set_hexpand(home_page, TRUE);
    gtk_widget_set_vexpand(home_page, TRUE);

    // create the buttons with translated labels
    GtkWidget *btn1 = create_button_icon_position("pan-start-symbolic", _("Test Stack"), G_CALLBACK(test_stack), stack, GTK_ALIGN_CENTER);
    GtkWidget *btn2 = create_button_icon_position("pan-start-symbolic", _("Test Log"), G_CALLBACK(test_log_callback), stack, GTK_ALIGN_CENTER);
    GtkWidget *btn3 = create_button_icon_position("pan-start-symbolic", _("Switch Log Type"), G_CALLBACK(switch_log_type_callback), stack, GTK_ALIGN_CENTER);
   
    // add grid to stack
    gtk_stack_add_named(GTK_STACK(stack), home_page, "home_page");
    gtk_stack_set_visible_child_name(GTK_STACK(stack), "home_page");
	
	// present the window
    gtk_window_present(GTK_WINDOW(window));
    
    // stop logging
    close_logging();
}

// main function
int main(int argc, char *argv[]) 
{
	g_autoptr(AdwApplication) app = NULL;

    app = adw_application_new("org.test.atlantis.uibase", G_APPLICATION_DEFAULT_FLAGS);
    g_signal_connect(app, "activate", G_CALLBACK (activate_test), NULL);

    return g_application_run(G_APPLICATION (app), argc, argv);
}

// alternative main
/*
int main(int argc, char *argv[]) 
{
    GtkApplication *app;
    int status;

    app = gtk_application_new("org.test.atlantis.uibase", G_APPLICATION_DEFAULT_FLAGS);
    g_signal_connect(app, "activate", G_CALLBACK(activate_test), NULL);
    status = g_application_run(G_APPLICATION(app), argc, argv);
    g_object_unref(app);

    return status;
}
*/


