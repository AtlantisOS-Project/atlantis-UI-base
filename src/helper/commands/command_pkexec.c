/**
* command_pkexec.c
*
* (C) Copyright 2026 AtlantisOS Project
* by @NachtsternBuild
*
* License: GNU GENERAL PUBLIC LICENSE Version 3
*
* Usage:
* command_pkexec("cd /usr/bin");
* commands_pkexec("cd /usr/bin && ls");
* command_pkexec_spinner(widget, "cd /usr/bin", "Info", "Text");
* commands_pkexec_spinner(widget, "cd /usr/bin && ls", "Info", "Text");
* command_pkexec_progressbar(widget, "cd /usr/bin", "Info", "Text");
* commands_pkexec_progressbar(widget, "cd /usr/bin && ls", "Info", "Text");
*/

#include "helper.h"
#include "design.h"

/** 
* @brief Command to run something with pkexec
*/
void command_pkexec(char *const argv[]) 
{
	int argc = 0;
    while (argv[argc]) argc++;

    char **new_argv = malloc((argc + 2) * sizeof(char*));
    if (!new_argv) return;

    new_argv[0] = "pkexec";

    for (int i = 0; i < argc; i++)
    {
        new_argv[i + 1] = argv[i];
    }

    new_argv[argc + 1] = NULL;
	
	gchar *execution_result = execute_command(new_argv);
	
	g_free(new_argv);
	g_free(execution_result);
}

/**
* @brief Wrapper for running a pkexec command with spinner
*/
void command_pkexec_spinner(GtkWidget *widget, const gchar *command, const char *title, const char *text)
{
	gchar *full_command = g_strdup_printf("pkexec %s", command);
	
	show_spinner_dialog(GTK_WIDGET(widget), title, text, full_command);
	g_free(full_command);
}

/**
* @brief Wrapper for running a multiple pkexec commands with spinner
*/
void commands_pkexec_spinner(GtkWidget *widget, const gchar *command, const char *title, const char *text)
{
	gchar *full_command = g_strdup_printf("pkexec sh -c \"%s\"", command);
	
	show_spinner_dialog(GTK_WIDGET(widget), title, text, full_command);
	g_free(full_command);
}


/**
* @brief Wrapper for running a pkexec command with spinner
*/
void command_pkexec_progressbar(GtkWidget *widget, const gchar *command, const char *title, const char *text)
{
	gchar *full_command = g_strdup_printf("pkexec %s", command);
	
	show_progress_dialog(GTK_WIDGET(widget), title, text, full_command);
	g_free(full_command);
}

/**
* @brief Wrapper for running a mutliple pkexec commands with spinner
*/
void commands_pkexec_progressbar(GtkWidget *widget, const gchar *command, const char *title, const char *text)
{
	gchar *full_command = g_strdup_printf("pkexec sh -c \"%s\"", command);
	
	show_progress_dialog(GTK_WIDGET(widget), title, text, full_command);
	g_free(full_command);
}
