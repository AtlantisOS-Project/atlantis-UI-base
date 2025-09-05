/*
* bind_language.c
*
* (C) Copyright 2025 AtlantisOS Project
* by @NachtsternBuild
*
* License: GNU GENERAL PUBLIC LICENSE Version 3
*
* Usage:
* bind_language("de")
*/

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <gtk/gtk.h>
#include <unistd.h>  
#include <sys/stat.h> 
#include <sys/types.h>
#include "helper.h"
#include "language.h" 

// extern config
extern const char *LOCALE_DOMAIN;

// set two paths for the .po files
static const char *locale_paths[] = {
    "./locale",   				  // build / run in source tree
    "/usr/share/locale"           // system-wide packages  
    "/usr/local/share/locale",    // self-installed
};

// try to bind local dir and .mo files
void bind_language(const char *lang)
{
    int found = 0;
    for (int i = 0; i < (int)(sizeof(locale_paths)/sizeof(locale_paths[0])); i++) 
    {
        char testpath[512];
        snprintf(testpath, sizeof(testpath), "%s/%.*s/LC_MESSAGES/%s.mo", locale_paths[i], 2, lang, LOCALE_DOMAIN);

        FILE *file = fopen(testpath, "r");
        if (file) 
        {
            fclose(file);
            bindtextdomain(LOCALE_DOMAIN, locale_paths[i]);
            found = 1;
            LOGI("Using translations from: %s", locale_paths[i]);
            break;
        }
    }

    if (!found) 
    {
        LOGI("No .mo found, fallback to /usr/share/locale");
        bindtextdomain(LOCALE_DOMAIN, "/usr/share/locale");
    }

    textdomain(LOCALE_DOMAIN);
    LOGI("Set textdomain: %s", LOCALE_DOMAIN);
}
