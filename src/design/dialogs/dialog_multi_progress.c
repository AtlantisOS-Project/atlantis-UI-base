/*
 * dialog_multi_progress.c
 *
 * (C) 2025 AtlantisOS Project
 * by @NachtsternBuild
 *
 * License: GNU GENERAL PUBLIC LICENSE Version 3
 *
 * ADW dialog with progress that show the steps
 *
 * Usage:
 * show_progress_dialog_multi(widget, "System-Update", "This may take some time...", commands);
 */

#include "dialogs.h"
#include "helper.h"
#include "design.h"

typedef struct {
    GtkWidget *dialog;
    GSList *commands; // List of CommandEntry*
    guint pulse_id;
    GtkWidget *description_label; // label for description
    GtkWidget *progress_bar; 
    GtkWidget *title_label; 
} RunData;

// run the update to the main thread
static gboolean update_dialog_description(gpointer data)
{
    RunData *rdata = (RunData *)data;
    if (rdata->commands) 
    {
        CommandEntry *entry = (CommandEntry *)rdata->commands->data;
        // update the label in the dialog
        gtk_label_set_text(GTK_LABEL(rdata->description_label), entry->description);
    }
     
    else 
    {
        // 
        gtk_label_set_text(GTK_LABEL(rdata->description_label), _("Completed."));
    }

    // G_SOURCE_REMOVE is returned for the label because it is a one-time action.
    return G_SOURCE_REMOVE;
}


// create the thread for running the command
static gpointer run_command_thread(gpointer data)
{
    RunData *rdata = (RunData *)data;
    GSList *current = rdata->commands;

    while (current != NULL) 
    {
        CommandEntry *entry = (CommandEntry *)current->data;

        // update main thread
        rdata->commands = current; // set command
        g_idle_add((GSourceFunc)update_dialog_description, rdata);

        // time for update 
        g_usleep(10000); // wait 10ms

        // run the command
        LOGI(entry->cmd);
        system(entry->cmd);

        current = g_slist_next(current);
    }

    // stop timer
    if (rdata->pulse_id > 0)
    {
        g_source_remove(rdata->pulse_id);
    }

    // close dialog in the main thread
    g_idle_add((GSourceFunc)adw_dialog_force_close, rdata->dialog);

    // free the memory
    GSList *list_ptr = rdata->commands;
    while (list_ptr != NULL) 
    {
        CommandEntry *entry = (CommandEntry *)list_ptr->data;
        g_free(entry->cmd);
        g_free(entry->description);
        g_free(entry);
        list_ptr = g_slist_next(list_ptr);
    }
    g_slist_free(rdata->commands);
    g_free(rdata);

    return NULL;
}


/*
 * show the dialog with progressbar for multiple commands
 */
void show_progress_dialog_multi(GtkWidget *parent, const char *title, const char *body, GSList *commands)
{
    if (commands == NULL) 
    {
        LOGE("No commands passed.");
        return;
    }

    AdwDialog *dialog = adw_alert_dialog_new(title, body);
    GtkWidget *content = gtk_box_new(GTK_ORIENTATION_VERTICAL, 12);
    gtk_widget_set_margin_top(content, 16);
    gtk_widget_set_margin_bottom(content, 16);
    gtk_widget_set_margin_start(content, 16);
    gtk_widget_set_margin_end(content, 16);
    gtk_widget_set_halign(content, GTK_ALIGN_CENTER);
    gtk_widget_set_valign(content, GTK_ALIGN_CENTER);

    // Progress Bar
    GtkWidget *progress = gtk_progress_bar_new();
    gtk_widget_set_hexpand(progress, TRUE);
    gtk_box_append(GTK_BOX(content), progress);

    // label for description
    GtkWidget *description_label = gtk_label_new(NULL);
    gtk_widget_set_hexpand(description_label, TRUE);
    gtk_label_set_wrap(GTK_LABEL(description_label), TRUE);
    gtk_label_set_justify(GTK_LABEL(description_label), GTK_JUSTIFY_CENTER);
    gtk_box_append(GTK_BOX(content), description_label);

    adw_alert_dialog_set_extra_child(ADW_ALERT_DIALOG(dialog), content);
    adw_dialog_present(dialog, parent);

    // thread data
    RunData *rdata = g_new0(RunData, 1);
    rdata->dialog = GTK_WIDGET(dialog);
    rdata->commands = commands; // get list
    rdata->description_label = description_label; // save label
    rdata->progress_bar = progress; // save progress bar

    // start timer for progress bar pulse
    rdata->pulse_id = g_timeout_add(100, (GSourceFunc)pulse_progress, progress);

    // run thread
    g_thread_new("run_command_thread", run_command_thread, rdata);
}
