/*
 * directory_exists.c
 *
 * (C) 2025 AtlantisOS Project
 * by @NachtsternBuild
 *
 * License: GNU GENERAL PUBLIC LICENSE Version 3
 *
 * Check if a directory exsists
 *
 * Usage:
 * if (directory_exists("/mnt/c/Users")) {}
 */
 
#include "helper.h"

int directory_exists(const char *path) 
{
    char command[2048];
    snprintf(command, sizeof(command), "test -d %s", path);
    return system(command) == 0;
}

// check if file exsists
int file_exists(const char *path) 
{
    char command[2048];
    snprintf(command, sizeof(command), "test -e %s", path);
    return system(command) == 0;
}
