/*
 * open_url.c
 *
 * (C) 2025 AtlantisOS Project
 * by @NachtsternBuild
 *
 * License: GNU GENERAL PUBLIC LICENSE Version 3
 *
 * Dialog with spinner/progress bar for AtlantisOS
 *
 * Usage:
 * open_url("https://www.google.com");
 * 
 * Note:
 * default OS is linux
 */

#include "helper.h"

// for windows
#ifdef _WIN32
    #include <windows.h>
#endif

// max length of url
#define MAX_CMD_LEN 2048

// open a url in the webbrowser
void open_url(const char *url) 
{
    #ifdef _WIN32
        /* 
        * Windows: 
        * Uses ShellExecute to perform the default action for the URL
        * NOTE: 
        * ShellExecute is a WinAPI function that may require a linker setting to 'Shell32.lib' if the code is not compiled as part of a typical Windows application. 
        */
        char cmd[MAX_CMD_LEN];
        snprintf(cmd, MAX_CMD_LEN, "start \"\" \"%s\"", url);
        system(cmd);

    #elif defined(__APPLE__) && defined(__MACH__)
        /*
        * macOS: 
        * Using open cmd
        */
        char cmd[MAX_CMD_LEN];
        snprintf(cmd, MAX_CMD_LEN, "open \"%s\"", url);
        system(cmd);     

    #else
        /*
        * Linux: 
        * Using xdg-open, as fallback gnome-open or kde-open
        */
        char cmd[MAX_CMD_LEN];
        
        // try xdg-open
        snprintf(cmd, MAX_CMD_LEN, "xdg-open \"%s\"", url);
        if (system(cmd) == 0) 
        {
            return; // success
        }
        
        // fallback
        snprintf(cmd, MAX_CMD_LEN, "gnome-open \"%s\"", url);
        if (system(cmd) == 0) 
        {
            return; 
        }

        snprintf(cmd, MAX_CMD_LEN, "kde-open \"%s\"", url);
        if (system(cmd) == 0) 
        {
            return;
        }

        // error
        LOGE("Error: Could not open default browser on Linux.");
        fprintf(stderr, "Error: Could not open default browser on Linux.\n");
    #endif
}
