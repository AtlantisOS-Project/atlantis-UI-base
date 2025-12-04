/**
* command_pkexec.c
*
* (C) Copyright 2025 AtlantisOS Project
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
void command_pkexec(const gchar *command) 
{
	gchar *full_command = g_strdup_printf("pkexec %s", command);
	
	gchar *execution_result = execute_command(full_command);
	
	g_free(full_command);
	g_free(execution_result);
}

/**
* @brief Command to run multiple commands with pkexec
*/
void commands_pkexec(const gchar *command)
{
	gchar *full_command = g_strdup_printf("pkexec sh -c \"%s\"", command);
	
	gchar *execution_result = execute_command(full_command);
	
	g_free(full_command);
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
