/*
* write_log.c
*
* (C) Copyright 2025 AtlantisOS Project
* by @NachtsternBuild
*
* License: GNU GENERAL PUBLIC LICENSE Version 3
*
Usage:
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

#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include <syslog.h>
#include <glib.h>
#include <sys/stat.h>
#include <unistd.h>
#include <errno.h>
#include <string.h>
#include <stdarg.h>

// global int 
int use_syslog = 0;

// define logfile
static FILE *logfile = NULL;
// define the log path
char log_path[1024] = {0};

// extern configs
extern const char *LOCALE_DOMAIN;

// helper function that create a directory
void make_dir(const char *path) 
{
    if (mkdir(path, 0755) == -1 && errno != EEXIST) 
    {
        perror("mkdir");
        exit(EXIT_FAILURE);
    }
}

int map_level_to_syslog(int level) 
{
    switch(level) 
    {
        case 1: return LOG_INFO;
        case 2: return LOG_WARNING;
        case 3: return LOG_ERR;
        case 4: return LOG_DEBUG;
        default: return LOG_INFO;
    }
}


// function that set a new logging mode 
void set_logging_mode(int syslog_mode) 
{
    use_syslog = syslog_mode;
	
    // use syslog
    if (use_syslog) 
    {
        openlog(LOCALE_DOMAIN, LOG_PID | LOG_CONS, LOG_USER);
        log_path[0] = '\0';  
    } 
    // own logging method
    else 
    {
        char log_dir[512];
        char log_file[1024];
        time_t now = time(NULL);
        struct tm *t = localtime(&now);
		
        // use /var/log
        snprintf(log_dir, sizeof(log_dir), "/var/log/%s", LOCALE_DOMAIN);
        // create the log dir 
        if (mkdir(log_dir, 0755) == -1 && errno != EEXIST) 
        {
            // use ~/.local as fallback
            snprintf(log_dir, sizeof(log_dir), "%s/.local/log/%s", getenv("HOME"), LOCALE_DOMAIN);
        }
        // create the dir
        make_dir(log_dir);
		
        // create the name of the log file
        snprintf(log_file, sizeof(log_file),
                 "%s/%04d-%02d-%02d_%02d-%02d-%02d.log",
                 log_dir,
                 t->tm_year + 1900,
                 t->tm_mon + 1,
                 t->tm_mday,
                 t->tm_hour,
                 t->tm_min,
                 t->tm_sec);
		
        // create the log file
        logfile = fopen(log_file, "a");
        if (!logfile) 
        {
            perror("fopen logfile");
            exit(EXIT_FAILURE);
        }
		
        // write every line to the log file
        setvbuf(logfile, NULL, _IOLBF, 0);

        // save the path of the log file
        strncpy(log_path, log_file, sizeof(log_path)-1);

        g_print("[INFO] Manual logging started: %s\n", log_path);
    }
}

// function that get the path of the log file
const char *get_log_path(void)
{
    return (use_syslog ? NULL : log_path);
}

// function that close the log
void close_logging(void) 
{
    // using syslog
    if (use_syslog) 
    {
        closelog();
    } 
    // manual logging
    else if (logfile) 
    {
        fclose(logfile);
        logfile = NULL;
    }
}

// function that create create the message for the logging
void log_message(const char *level, int syslog_level, const char *fmt, va_list args) 
{
    time_t now = time(NULL);
    struct tm *tm_info = localtime(&now);
    char time_buf[20];
    strftime(time_buf, sizeof(time_buf), "%Y-%m-%d %H:%M:%S", tm_info);
	
	// using syslog
    if (use_syslog) 
    {
    	char buffer[1024];
    	vsnprintf(buffer, sizeof(buffer), fmt, args);
    	syslog(map_level_to_syslog(syslog_level), "[%s] %s", level, buffer);
	}
    
    // manual logging
    else 
    {
        if (logfile) 
        {
            fprintf(logfile, "[%s] [%s]: ", time_buf, level);
            vfprintf(logfile, fmt, args);
            fprintf(logfile, "\n");
            fflush(logfile);
        }
        
        // ouput to the terminal
        g_print("[%s] [%s]: ", time_buf, level);
        vprintf(fmt, args);
        g_print("\n");
    }
}

// wrapper: takes variadic arguments and forwards to log_message
void log_message_wrap(const char *level, int syslog_level, const char *fmt, ...) 
{
    va_list args;
    va_start(args, fmt);
    log_message(level, syslog_level, fmt, args);
    va_end(args);
}
