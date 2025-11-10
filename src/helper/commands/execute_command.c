/**
* execute_command.c
*
* (C) Copyright 2025 AtlantisOS Project
* by @NachtsternBuild
*
* License: GNU GENERAL PUBLIC LICENSE Version 3
*
* Usage:
* gchar *full_command = g_strdup_printf("pkexec %s", command);
* gchar *execution_result = execute_command(full_command);
* g_free(full_command);
*
* run_command("sleep 5");
*
* if (!run_command(cmd)) {}
*/

#include "helper.h"

// run a command in terminal via system()
void run_command(const char *command) 
{
    if (command != NULL) 
    {
        // use system()-command
        system(command);
    }
}

// run a command and return true or false
bool run_command_bool(const char *cmd) 
{
    int status = system(cmd);
    return WIFEXITED(status) && WEXITSTATUS(status) == 0;
}


// function to execute a command and capture its output
char *execute_command(const char *command) 
{
    FILE *fp;
    char buffer[2048];
    char *result = calloc(1, 8192);  // memory for output

    if (!result) 
        return NULL;

    fp = popen(command, "r");
    if (!fp) 
    {
        free(result);
        return NULL;
    }

    while (fgets(buffer, sizeof(buffer), fp) != NULL) 
    {
        strncat(result, buffer, 8192 - strlen(result) - 1);
    }

    pclose(fp);

    return result; 
}
