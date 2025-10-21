/*
* helper.h
*
* (C) Copyright 2025 AtlantisOS Project
* by @NachtsternBuild
*
* License: GNU GENERAL PUBLIC LICENSE Version 3
*
* Usage:
* #include "helper.h"
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
#include <syslog.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <errno.h>
#include <limits.h>
#include <glib/gi18n.h>
#include <locale.h>
#include <signal.h>
#include <vte/vte.h>

#ifndef HELPER_H
#define HELPER_H

#ifdef __cplusplus
extern "C" {
#endif

/*
* Defined in the main Code
*
* Usage:
* const char *LOCALE_DOMAIN = "atlupdater";
* const char *DEFAULT_FILTER = "/path/to/the/filterfile.conf";
*/
// define the local domain 
extern const char *LOCALE_DOMAIN;
// define the path to the filter file
extern const char *DEFAULT_FILTER;

/* 
* Makro for autofree the memory 
*
* Usage:
* auto_free char *output = execute_command("ls /");
* char *output = execute_command("ls -l");
* g_print("%s", output);
*
* Note: Works only with GCC 
*/
// declare the wrapper for autofree the memory
void free_wrapper(void *p);
// the makro that use the wrapper
#define auto_free __attribute__((cleanup(free_wrapper)))

/*
* Filesystem helper
*/
// helper function that create a directory
void make_dir(const char *path);

// creates a complete path by creating missing parent directories
int make_path(const char *path);
// creates parent directory of a file 
int make_path_dirname(const char *filepath);
// check if directory exsits
int directory_exists(const char *path);
// get home dir
const char *get_home_directory();
// add home in a path 
gchar *get_home(const gchar *path);
// delete a file in  a directory
void delete_files_in_dir(const char *path);
/*
*
* Delete a full path with parents
*
* Usage:
* delete_files_with_parent("/other/path"); 
*
* Notes:
* Warning: This code deletes an entire path until it finds a stop point.
* Warning: If no stop point is defined, files and directories that should not be deleted will be deleted.
* Warning: Never use this code to delete foreign or non-program directories.
*/
void delete_files_with_parent(const char *path);

/*
* Helper for running commands
*/
// run a command
void run_command(const char *command);
// run command and read output
char *execute_command(const char *command);
// run command with pkexec
void command_pkexec(const gchar *command);

/*
* Get values from a config file
*
* Usage:
* char *update_type = get_conf_value(conf_file, "UPDATE_TYPE");
*/
// function that get values from config files
char *get_config_value(const char *filename, const char *key);

/*
* Load file filter from a config file
*
* Usage:
* GListStore *filter_list = load_file_filters("/path/to/filechooser-filters.conf");
* // now work with the filter
* if (filter_list) 
* {
* 	  gtk_file_dialog_set_filters(dialog, G_LIST_MODEL(filter_list));
* }
*/
// function that load the filter from a config file
GListStore* load_file_filters(const char *config_path);


/*
* Logging:
*
* Usage:
* 0 = own logging function to /var/log/LOCALE_DOMAIN or 
* 1 = syslog
* 
* This at the start:
* set_logging_mode(1);
*
* In the program:
* LOG_INFO("Programm gestartet");
* LOG_INFO("Hallo %s!", "Welt");
* LOG_ERROR("Dies ist ein Fehler mit Code %d", 42);
* LOG_WARN("Dies ist eine Warnung");
* LOG_DEBUG("Debug-Informationen: x=%d, y=%d", 10, 20);
*
* At the end of the program:
* close_logging();
*/

/* Define the syslog usage
*
* Usage:
* int use_syslog = 0; (defined at write_log)
*/
extern int use_syslog;

/*
* Main logging functions
*/
// set a new log mode
void set_logging_mode(int syslog_mode);
// get the path of the log file
const char *get_log_path(void);
// function that close the log
void close_logging(void);
// function that create the log message
void log_message(const char *level, int syslog_level, const char *fmt, va_list args);
// wrapper: takes variadic arguments and forwards to log_message
void log_message_wrap(const char *level, int syslog_level, const char *fmt, ...);

/* 
* Makros for the logging
*/
// own logging vars
#define LOG_LEVEL_INFO   1
#define LOG_LEVEL_WARN   2
#define LOG_LEVEL_ERROR  3
#define LOG_LEVEL_DEBUG  4

// logging Macros
#define LOGI(fmt, ...)  log_message_wrap("INFO",  LOG_LEVEL_INFO,  fmt, ##__VA_ARGS__)
#define LOGW(fmt, ...)  log_message_wrap("WARN",  LOG_LEVEL_WARN,  fmt, ##__VA_ARGS__)
#define LOGE(fmt, ...)  log_message_wrap("ERROR", LOG_LEVEL_ERROR, fmt, ##__VA_ARGS__)
#define LOGD(fmt, ...)  log_message_wrap("DEBUG", LOG_LEVEL_DEBUG, fmt, ##__VA_ARGS__)


/*
* Log viewer
* 
* Usage:
* Add the headerbar to your UI:
* GtkWidget *headerbar = create_custom_headerbar(stack);

* Add the box to the stack or the window
* gtk_box_append(GTK_BOX(content_box), headerbar);
*/
// function that open the log 
void open_log_source();
// function to write string to the textview 
void write_to_textview(GtkWidget *text_view, const char *str);
// function that reads the new logs
gboolean update_text_view_from_log(gpointer user_data);
// cleanup when log viewer is closed 
void log_viewer_destroyed(GtkWidget *widget, gpointer user_data);

// header that create the popover menu
GtkWidget* create_custom_headerbar(gpointer stack);

/*
* Open terminal/URL
* Note: 
* default OS is linux
*/
// open url in the standard webbrowser
void open_url(const char *url);
//  run a command in a new terminal
void open_terminal_by_desktop(const char *function_command);

#ifdef __cplusplus
}
#endif

#endif // HELPER_H
