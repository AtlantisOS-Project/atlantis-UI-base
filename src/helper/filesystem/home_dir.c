/*
* home_dir.c
*
* (C) Copyright 2025 AtlantisOS Project
* by @NachtsternBuild
*
* License: GNU GENERAL PUBLIC LICENSE Version 3
*
* Usage:
* get_home("~/long/path");
* const char *new = get_home_directory();
*/

#include "helper.h"

gchar *get_home(const gchar *path) 
{
    const gchar *home = getenv("HOME");
    if (home == NULL) 
    {
        LOGE("HOME environment variable not found.");
        return NULL;
    }
    // for '~/'
    if (g_str_has_prefix(path, "~/")) 
    {
        gchar *short_path = g_strconcat(home, path + 1, NULL);
        return short_path;
    }
    return g_strdup(path);
}

// get the user's home directory
const char *get_home_directory() 
{
    const char* home_dir = getenv("HOME");
    if (!home_dir) 
    {
        LOGE("Could not determine the home directory.");
        exit(1);
    }
    return home_dir;
}
