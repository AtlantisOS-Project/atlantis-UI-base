/*
 * dialogs_entry.c
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

typedef void (*EntryDialogCallback)(const gchar *text, gpointer user_data);

// callback handler for button response
static void on_dialog_response(AdwAlertDialog *dialog, const char *response, gpointer user_data)
{
    struct {
        GtkEntry *entry;
        EntryDialogCallback callback;
        gpointer user_data;
    } *ctx = user_data;

    if (g_strcmp0(response, "ok") == 0 && ctx->callback)
    {
        const gchar *text = gtk_editable_get_text(GTK_EDITABLE(ctx->entry));
        ctx->callback(text, ctx->user_data);
    }

    adw_dialog_force_close(ADW_DIALOG(dialog));
    g_free(ctx);
}

// show alert dialog with entry and callback
void show_entry_dialog(GtkWidget *parent, const char *title, const char *body, const char *ok_label, const char *cancel_label, const char *entry_label, const char *placeholder, EntryDialogCallback callback, gpointer user_data)
{
    AdwDialog *dialog = adw_alert_dialog_new(title, body);

    // buttons
    adw_alert_dialog_add_responses(ADW_ALERT_DIALOG(dialog),
                                   "cancel", cancel_label,
                                   "ok", ok_label,
                                   NULL);
    adw_alert_dialog_set_response_appearance(ADW_ALERT_DIALOG(dialog), "ok", ADW_RESPONSE_SUGGESTED);
    adw_alert_dialog_set_response_appearance(ADW_ALERT_DIALOG(dialog), "cancel", ADW_RESPONSE_DESTRUCTIVE);
    adw_alert_dialog_set_close_response(ADW_ALERT_DIALOG(dialog), "cancel");

    // entry
    GtkEntry *entry = NULL;
    GtkWidget *entry_box = create_entry(entry_label, placeholder, &entry);
    gtk_widget_set_margin_top(entry_box, 12);
    gtk_widget_set_margin_bottom(entry_box, 12);
    gtk_widget_set_margin_start(entry_box, 16);
    gtk_widget_set_margin_end(entry_box, 16);

    adw_alert_dialog_set_extra_child(ADW_ALERT_DIALOG(dialog), entry_box);

    // context für callback
    struct {
        GtkEntry *entry;
        EntryDialogCallback callback;
        gpointer user_data;
    } *ctx = g_new0(typeof(*ctx), 1);

    ctx->entry = entry;
    ctx->callback = callback;
    ctx->user_data = user_data;

    g_signal_connect(dialog, "response", G_CALLBACK(on_dialog_response), ctx);

    adw_dialog_present(dialog, parent);
}

