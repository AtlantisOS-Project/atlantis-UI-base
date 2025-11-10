/**
* search_file_directory.c
*
* (C) Copyright 2025 AtlantisOS Project
* by @NachtsternBuild
*
* License: GNU GENERAL PUBLIC LICENSE Version 3
*
*
* Expand '~' to $HOME
*
* Usage:
*  
* char *local_conf_path = expand_path(LOCAL_CONF);
*/

#include "helper.h"

// expand '~' to $HOME
char *expand_path(const char *path) 
{
    if (path[0] != '~') 
    {
        return strdup(path);
    }
    const char *home = getenv("HOME");
    size_t len = strlen(home) + strlen(path);
    char *res = malloc(len);
    sprintf(res, "%s%s", home, path + 1);
    return res;
}
