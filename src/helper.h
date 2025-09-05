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

#ifndef HELPER_H
#define HELPER_H

/*
* Defined in the main Code
*
* Usage:
* const char *LOCALE_DOMAIN = "atlupdater";
*/
// define the local domain 
extern const char *LOCALE_DOMAIN;

/* 
* makro for autofree the memory 
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



// create a directory
void make_dir(const char *path);

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

/* define the syslog usage
*
* Usage:
* int use_syslog = 0; (defined at write_log)
*/
extern int use_syslog;

/*
* main functions
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
* makros for the logging
*/
/*
// infos
#define LOG_INFO(msg, ...)  do { va_list ap; va_start(ap, msg); log_message("INFO",  LOG_INFO,  msg, ap); va_end(ap); } while(0)
// errors
#define LOG_ERROR(msg, ...) do { va_list ap; va_start(ap, msg); log_message("ERROR", LOG_ERROR,   msg, ap); va_end(ap); } while(0)
// warnings
#define LOG_WARN(msg, ...)  do { va_list ap; va_start(ap, msg); log_message("WARN",  LOG_WARNING, msg, ap); va_end(ap); } while(0)
// debug infos
#define LOG_DEBUG(msg, ...) do { va_list ap; va_start(ap, msg); log_message("DEBUG", LOG_DEBUG, msg, ap); va_end(ap); } while(0)
*/
// eigene Level-Konstanten
#define LOG_LEVEL_INFO   1
#define LOG_LEVEL_WARN   2
#define LOG_LEVEL_ERROR  3
#define LOG_LEVEL_DEBUG  4

// Logging Macros
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

#endif // HELPER_H
