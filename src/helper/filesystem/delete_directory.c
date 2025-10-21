/*
 * delete_directory.c
 *
 * (C) 2025 AtlantisOS Project
 * by @NachtsternBuild
 *
 * License: GNU GENERAL PUBLIC LICENSE Version 3
 *
 * Check if a directory exsists
 *
 * Usage:
 * delete_directory("/this/path");
 */
 
#include "helper.h"

// delete directroy
void delete_directory(const char *path)
{
    if (rmdir(path) == 0)
    {
        LOGI("Directory deleted: %s", path);
    }
    
    else
    {
        LOGE("Error deleting directory %s: %s (Note: Only empty directories can be deleted.)", path, strerror(errno));
		exit(1);
    }
}
