/**
* search_file_directory.c
*
* (C) Copyright 2026 AtlantisOS Project
* by @NachtsternBuild
*
* License: GNU GENERAL PUBLIC LICENSE Version 3
*
* @brief Expand '~' to $HOME
*
* Usage:
*  
* char *local_conf_path = expand_path(LOCAL_CONF);
*/

#include "helper.h"

/**
* @brief Expand '~' to $HOME
*/
char *expand_path(const char *path) 
{
    if (path == NULL) return NULL;

    if (path[0] != '~') 
    {
        return strdup(path);
    }

    const char *home = getenv("HOME");
    if (home == NULL) 
    {
        // fallback if $HOME not set
        return strdup(path);
    }

    // get the full the length
    size_t len = strlen(home) + strlen(path + 1) + 1;
    
    char *res = malloc(len);
    if (res == NULL) 
    {
    	return NULL; 
    }

    snprintf(res, len, "%s%s", home, path + 1);

    return res;
}
