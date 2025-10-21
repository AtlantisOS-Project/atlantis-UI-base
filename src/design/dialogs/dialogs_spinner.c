/*
 * dialogs_spinner.c
 *
 * (C) 2025 AtlantisOS Project
 * by @NachtsternBuild
 *
 * License: GNU GENERAL PUBLIC LICENSE Version 3
 *
 * Dialog with spinner/progress bar for AtlantisOS
 *
 * Usage:
 * show_spinner_dialog(GTK_WIDGET(widget), "Update running", "Please wait...", "sleep 5");
 * show_progress_dialog(GTK_WIDGET(widget), "Installation", "Install package...", "sleep 5");
 */

#include "dialogs.h"
#include "helper.h"
#include "design.h"

// helper structure
typedef struct {
    GtkWidget *dialog;
    gchar *cmd;
    guint pulse_id;
} RunData;

// create the thread for running the command
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

// show the dialog with spinner
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

    GtkWidget *spinner = gtk_spinner_new();
    gtk_widget_set_size_request(spinner, 150, 150);
    gtk_spinner_start(GTK_SPINNER(spinner));
    gtk_box_append(GTK_BOX(content), spinner);

    adw_alert_dialog_set_extra_child(ADW_ALERT_DIALOG(dialog), content);
    adw_dialog_present(dialog, parent);

    RunData *rdata = g_new0(RunData, 1);
    rdata->dialog = GTK_WIDGET(dialog);
    rdata->cmd = g_strdup(cmd);
    rdata->pulse_id = 0;

    g_thread_new("run_command_thread", run_command_thread, rdata);
}

// porgress bar pulse
static gboolean pulse_progress(GtkProgressBar *pbar)
{
    if (!GTK_IS_PROGRESS_BAR(pbar)) 
    {
        return G_SOURCE_REMOVE; // stop, if destroyed
    }
    gtk_progress_bar_pulse(pbar);
    return G_SOURCE_CONTINUE;
}

// show the dialog with progressbar 
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


