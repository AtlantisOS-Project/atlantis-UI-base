/*
* command_pkexec.c
*
* (C) Copyright 2025 AtlantisOS Project
* by @NachtsternBuild
*
* License: GNU GENERAL PUBLIC LICENSE Version 3
*
* Usage:
* command_pkexec("cd /usr/bin");
*/

#include "helper.h"

// command to run something with pkexec
void command_pkexec(const gchar *command) 
{
	gchar *full_command = g_strdup_printf("pkexec %s", command);
	
	gchar *execution_result = execute_command(full_command);
	
	g_free(full_command);
	g_free(execution_result);
}
