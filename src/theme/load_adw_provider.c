/*
* load_adw_provider.c
*
* (C) Copyright 2025 AtlantisOS Project
* by @NachtsternBuild
*
* License: GNU GENERAL PUBLIC LICENSE Version 3
*
* Usage:
* use_adw_provider();
*
* Note: Requires GTK4 (> 4.10), libadwaita (> 1.4)
*/

#include "helper.h"
#include "design.h"

GtkCssProvider *provider_adw = NULL;
AdwStyleManager *style_manager;

// function that create a css provider
GtkCssProvider* create_css_provider() 
{
    GtkCssProvider *new_provider = gtk_css_provider_new();
    if (!new_provider) 
    {
        LOGE("Could not create the CSS provider.");
        exit(1);
    }
    return new_provider;
}

// function to unload the css provider
void unload_css_provider(GtkCssProvider *provider) 
{
    if (provider) 
    {
        g_object_unref(provider);
    }
}

// function to load the css provider with adw themes
void load_adw_provider(void) 
{
    const char *adw_css = get_custom_adw_css();
    gtk_css_provider_load_from_string(provider_adw, adw_css);
}


// function that reload the adw theme
void adw_theme_changed(AdwStyleManager *style_manager, GParamSpec *pspec, gpointer user_data) 
{
    load_adw_provider();  // reload the adw provider
}

// function that load the adw provider and check for changed theme
void use_adw_provider(void) 
{
    // check for the adw provider
    if (provider_adw) 
    {
        g_object_unref(provider_adw);
    }
    // create a new adw provider
    provider_adw = create_css_provider();

	// load the adw provider
    load_adw_provider();
	
	// add the provider to the app
    gtk_style_context_add_provider_for_display(
        gdk_display_get_default(),
        GTK_STYLE_PROVIDER(provider_adw),
        GTK_STYLE_PROVIDER_PRIORITY_USER);
	
    // check for theme change
    AdwStyleManager *style_manager = adw_style_manager_get_default();
    g_signal_connect(style_manager, "notify::dark", G_CALLBACK(adw_theme_changed), NULL);
}
