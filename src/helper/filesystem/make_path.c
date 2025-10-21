/*
* make_path.c
*
* (C) Copyright 2025 AtlantisOS Project
* by @NachtsternBuild
*
* License: GNU GENERAL PUBLIC LICENSE Version 3
*
*
* Creates a complete path by creating missing parent directories
* Usage:
* make_path(dir_path);
*
* Notes: 
* @param path: The path to be created.
* @return 0 on success, -1 on error.
*/

#include "helper.h"

// creates the full path 
int make_path(const char *path)
{
    char tmp[PATH_MAX];
    char *p;
    size_t len;

    if (!path || *path == '\0') 
    {
        errno = EINVAL;
        return -1;
    }

    // create a safe copy of the path
    strncpy(tmp, path, sizeof(tmp));
    tmp[sizeof(tmp)-1] = '\0';

    len = strlen(tmp);
    // remove trailing slash, if exsists
    if (len > 1 && tmp[len-1] == '/')
        tmp[len-1] = '\0';

    // iterate through path and create intermediate dirs 
    for (p = tmp + 1; *p; p++) 
    {
        if (*p == '/') {
            *p = '\0';
            if (mkdir(tmp, 0755) != 0) 
            {
                if (errno != EEXIST) 
                {
                    perror("mkdir");
                    return -1;
                }
            }
            *p = '/';
        }
    }

    // create final component
    if (mkdir(tmp, 0755) != 0) 
    {
        if (errno != EEXIST) 
        {
            perror("mkdir");
            return -1;
        }
    }

    return 0;
}

// creates parent directory of a file 
int make_path_dirname(const char *filepath)
{
    char buff[PATH_MAX];
    char *last;

    if (!filepath || *filepath == '\0') 
    {
        errno = EINVAL;
        return -1;
    }

    // copy
    strncpy(buff, filepath, sizeof(buff));
    buff[sizeof(buff)-1] = '\0';

    last = strrchr(buff, '/');
    if (!last) 
    {
        // no file part
        return 0;
    }

    if (last == buff) 
    {
        // path such as “/file” → Directory is '/' (exists)
        return 0;
    }

    // schedule: buff now contains the directory
    *last = '\0';
    return make_path(buff);
}



