/**
* dialogs_spinner.c
*
* (C) 2025 AtlantisOS Project
* by @NachtsternBuild
*
* License: GNU GENERAL PUBLIC LICENSE Version 3
*
* @brief Dialog with spinner/progress bar for AtlantisOS
*
* Usage:
* show_spinner_dialog(GTK_WIDGET(widget), "Update running", "Please wait...", "sleep 5");
* show_spinner_dialog_return(GTK_WIDGET(widget), "Running update", "Please wait...", "ls /boot && sleep 5", on_command_done, NULL);
* show_progress_dialog(GTK_WIDGET(widget), "Installation", "Install package...", "sleep 5");
* show_progress_dialog_return(GTK_WIDGET(widget), "Running update", "Please wait...", "ls /boot && sleep 5", on_command_done, NULL);
*/

#include "dialogs.h"
#include "helper.h"
#include "design.h"

// callback type for returning output
typedef void (*CommandFinishedCallback)(const gchar *output, gpointer user_data);

// helper structure
typedef struct {
    GtkWidget *dialog;
    gchar *cmd;
    guint pulse_id;
} RunData;

// helper structure
typedef struct {
    GtkWidget *dialog;
    gchar *cmd;
    guint pulse_id;
    CommandFinishedCallback callback;
    gpointer user_data;
} RunDataReturn;

/**
* @brief Command and capture output
*/
static gchar *run_command_capture_output(const gchar *cmd)
{
    GError *error = NULL;
    gchar *stdout_buf = NULL;
    gchar *stderr_buf = NULL;
    gint exit_status = 0;

    // create shell comand structure
    gchar *shell_cmd = g_strdup_printf("/bin/sh -c \"%s\"", cmd);

    gboolean success = g_spawn_command_line_sync(shell_cmd, &stdout_buf, &stderr_buf, &exit_status, &error);
    g_free(shell_cmd);

    if (!success)
    {
        gchar *msg = g_strdup_printf("Command failed: %s\nError: %s", cmd, error ? error->message : "unknown");
        g_clear_error(&error);
        return msg;
    }

    // combine stdout and stderr
    gchar *result = g_strdup_printf("%s%s%s", stdout_buf ? stdout_buf : "", stderr_buf && *stderr_buf ? "\n--- stderr ---\n" : "", stderr_buf ? stderr_buf : "");

    g_free(stdout_buf);
    g_free(stderr_buf);

    return result;
}

/**
* @brief Create the thread for running the command
*/
static gpointer run_command_thread(gpointer data)
{
    RunData *rdata = (RunData *)data;
    system(rdata->cmd);

    // stop timer, if exsits
    if (rdata->pulse_id > 0) 
    {
        g_source_remove(rdata->pulse_id);
	}
	
    // close the dialog in the main thread
    g_idle_add((GSourceFunc)adw_dialog_force_close, rdata->dialog);

    g_free(rdata->cmd);
    g_free(rdata);
    return NULL;
}

/**
* @brief Thread for running the command
*/
static gpointer run_command_thread_return(gpointer data)
{
    RunDataReturn *rundata = (RunDataReturn *)data;

    gchar *output = run_command_capture_output(rundata->cmd);

    // stop timer, if exists
    if (rundata->pulse_id > 0)
    {
        g_source_remove(rundata->pulse_id);
    }

    // close the dialog in the main thread
    g_idle_add((GSourceFunc)adw_dialog_force_close, rundata->dialog);

    // call callback in main thread
    if (rundata->callback)
    {
        gchar *output_copy = g_strdup(output);
        g_idle_add_full(G_PRIORITY_DEFAULT, (GSourceFunc)(void *)rundata->callback, output_copy, rundata->user_data);
    }

    g_free(output);
    g_free(rundata->cmd);
    g_free(rundata);
    return NULL;
}

/**
* @brief Progress bar pulse
*/
gboolean pulse_progress(GtkProgressBar *pbar)
{
    if (!GTK_IS_PROGRESS_BAR(pbar)) 
    {
        return G_SOURCE_REMOVE; // stop, if destroyed
    }
    gtk_progress_bar_pulse(pbar);
    return G_SOURCE_CONTINUE;
}

/**
* @brief Show the dialog with spinner
*/
void show_spinner_dialog(GtkWidget *parent, const char *title, const char *body, const char *cmd)
{
    LOGI(cmd);
    
    AdwDialog *dialog = adw_alert_dialog_new(title, body);
    GtkWidget *content = gtk_box_new(GTK_ORIENTATION_VERTICAL, 12);
    gtk_widget_set_margin_top(content, 16);
    gtk_widget_set_margin_bottom(content, 16);
    gtk_widget_set_margin_start(content, 16);
    gtk_widget_set_margin_end(content, 16);
    gtk_widget_set_halign(content, GTK_ALIGN_CENTER);
    gtk_widget_set_valign(content, GTK_ALIGN_CENTER);
	
	guint major = adw_get_major_version();
    guint minor = adw_get_minor_version();
	
	#if ADW_CHECK_VERSION(1,6,0)

    	GtkWidget *spinner;
    	
    	if (major > 1 || (major == 1 && minor >= 6))
    	{
        	// use libadwaita >= 1.6
        	spinner = adw_spinner_new();
        	gtk_widget_set_size_request(spinner, 150, 150);
    	}
    
    	else
    	{
        	// compiled with libadwaita >= 1.6, run with <= 1.5
        	spinner = gtk_spinner_new();
        	gtk_widget_set_size_request(spinner, 150, 150);
        	gtk_spinner_start(GTK_SPINNER(spinner));
    	}
    	gtk_box_append(GTK_BOX(content), spinner);
    
    // compiled with <= 1.5
    #else 
    	GtkWidget *spinner = gtk_spinner_new();
    	gtk_widget_set_size_request(spinner, 150, 150);
    	gtk_spinner_start(GTK_SPINNER(spinner));
    	gtk_box_append(GTK_BOX(content), spinner);
	
	#endif
		
    adw_alert_dialog_set_extra_child(ADW_ALERT_DIALOG(dialog), content);
    adw_dialog_present(dialog, parent);

    RunData *rdata = g_new0(RunData, 1);
    rdata->dialog = GTK_WIDGET(dialog);
    rdata->cmd = g_strdup(cmd);
    rdata->pulse_id = 0;

    g_thread_new("run_command_thread", run_command_thread, rdata);
}

/** 
* @brief Show the dialog with progressbar 
*/
void show_progress_dialog(GtkWidget *parent, const char *title, const char *body, const char *cmd)
{
    LOGI(cmd);
    
    AdwDialog *dialog = adw_alert_dialog_new(title, body);
    GtkWidget *content = gtk_box_new(GTK_ORIENTATION_VERTICAL, 12);
    gtk_widget_set_margin_top(content, 16);
    gtk_widget_set_margin_bottom(content, 16);
    gtk_widget_set_margin_start(content, 16);
    gtk_widget_set_margin_end(content, 16);
    gtk_widget_set_halign(content, GTK_ALIGN_CENTER);
    gtk_widget_set_valign(content, GTK_ALIGN_CENTER);

    GtkWidget *progress = gtk_progress_bar_new();
    gtk_widget_set_hexpand(progress, TRUE);
    gtk_box_append(GTK_BOX(content), progress);

    adw_alert_dialog_set_extra_child(ADW_ALERT_DIALOG(dialog), content);
    adw_dialog_present(dialog, parent);

    RunData *rdata = g_new0(RunData, 1);
    rdata->dialog = GTK_WIDGET(dialog);
    rdata->cmd = g_strdup(cmd);

    // save timer, remove it later
    rdata->pulse_id = g_timeout_add(100, (GSourceFunc)pulse_progress, progress);

    g_thread_new("run_command_thread", run_command_thread, rdata);
}

/**
* @brief Show the dialog with spinner and return the output
*/
void show_spinner_dialog_return(GtkWidget *parent, const char *title, const char *body, const char *cmd, CommandFinishedCallback callback, gpointer user_data)
{
    LOGI(cmd);

    AdwDialog *dialog = adw_alert_dialog_new(title, body);
    GtkWidget *content = gtk_box_new(GTK_ORIENTATION_VERTICAL, 12);
    gtk_widget_set_margin_top(content, 16);
    gtk_widget_set_margin_bottom(content, 16);
    gtk_widget_set_margin_start(content, 16);
    gtk_widget_set_margin_end(content, 16);
    gtk_widget_set_halign(content, GTK_ALIGN_CENTER);
    gtk_widget_set_valign(content, GTK_ALIGN_CENTER);
	guint major = adw_get_major_version();
    guint minor = adw_get_minor_version();
	
	#if ADW_CHECK_VERSION(1,6,0)

    	GtkWidget *spinner;
    	
    	if (major > 1 || (major == 1 && minor >= 6))
    	{
        	// use libadwaita >= 1.6
        	spinner = adw_spinner_new();
        	gtk_widget_set_size_request(spinner, 200, 200);
    	}
    
    	else
    	{
        	// compiled with libadwaita >= 1.6, run with <= 1.5
        	spinner = gtk_spinner_new();
        	gtk_widget_set_size_request(spinner, 150, 150);
        	gtk_spinner_start(GTK_SPINNER(spinner));
    	}
    	gtk_box_append(GTK_BOX(content), spinner);
    
    // compiled with <= 1.5
    #else 
    	GtkWidget *spinner = gtk_spinner_new();
    	gtk_widget_set_size_request(spinner, 150, 150);
    	gtk_spinner_start(GTK_SPINNER(spinner));
    	gtk_box_append(GTK_BOX(content), spinner);
	
	#endif


    adw_alert_dialog_set_extra_child(ADW_ALERT_DIALOG(dialog), content);
    adw_dialog_present(dialog, parent);

    RunDataReturn *rundata = g_new0(RunDataReturn, 1);
    rundata->dialog = GTK_WIDGET(dialog);
    rundata->cmd = g_strdup(cmd);
    rundata->callback = callback;
    rundata->user_data = user_data;
    rundata->pulse_id = 0;

    g_thread_new("run_command_thread", run_command_thread_return, rundata);
}

/**
* @brief Show the dialog with progressbar and return the output
*/
void show_progress_dialog_return(GtkWidget *parent, const char *title, const char *body, const char *cmd, CommandFinishedCallback callback, gpointer user_data)
{
    LOGI(cmd);

    AdwDialog *dialog = adw_alert_dialog_new(title, body);
    GtkWidget *content = gtk_box_new(GTK_ORIENTATION_VERTICAL, 12);
    gtk_widget_set_margin_top(content, 16);
    gtk_widget_set_margin_bottom(content, 16);
    gtk_widget_set_margin_start(content, 16);
    gtk_widget_set_margin_end(content, 16);
    gtk_widget_set_halign(content, GTK_ALIGN_CENTER);
    gtk_widget_set_valign(content, GTK_ALIGN_CENTER);

    GtkWidget *progress = gtk_progress_bar_new();
    gtk_widget_set_hexpand(progress, TRUE);
    gtk_box_append(GTK_BOX(content), progress);

    adw_alert_dialog_set_extra_child(ADW_ALERT_DIALOG(dialog), content);
    adw_dialog_present(dialog, parent);

    RunDataReturn *rundata = g_new0(RunDataReturn, 1);
    rundata->dialog = GTK_WIDGET(dialog);
    rundata->cmd = g_strdup(cmd);
    rundata->callback = callback;
    rundata->user_data = user_data;

    rundata->pulse_id = g_timeout_add(100, (GSourceFunc)pulse_progress, progress);

    g_thread_new("run_command_thread", run_command_thread_return, rundata);
}
