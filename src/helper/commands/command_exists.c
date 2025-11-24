/**
* command_exists.c
*
* (C) Copyright 2025 AtlantisOS Project
* by @NachtsternBuild
*
* License: GNU GENERAL PUBLIC LICENSE Version 3
*
* Usage:
* if (command_exists("cd") != 0) {}
*/

#include "helper.h"

// check if command exsists
int command_exists(const char *command) 
{
    char check_cmd[256];
	// try with command -v 
    snprintf(check_cmd, sizeof(check_cmd), "command -v %s > /dev/null 2>&1", command);
    return system(check_cmd) == 0; 
}
