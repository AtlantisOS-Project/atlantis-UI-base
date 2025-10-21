/*
* file_filter.c
*
* (C) Copyright 2025 AtlantisOS Project
* by @NachtsternBuild
*
* License: GNU GENERAL PUBLIC LICENSE Version 3
*
* Usage:
* GListStore *filter_list = load_file_filters("/path/to/filechooser-filters.conf");
* // now work with the filter
* if (filter_list) 
* {
* 	  gtk_file_dialog_set_filters(dialog, G_LIST_MODEL(filter_list));
* }
*/

#include "helper.h"

// function that load the filter from a config file
GListStore* load_file_filters(const char *config_path)
{
    if (!config_path || strlen(config_path) == 0) 
    {
        // no file = return empty list
        return NULL;
    }
	
	// open the config file
    FILE *fp = fopen(config_path, "r");
    if (!fp) 
    {
        LOGW("No filter config found: %s (using no filters)", config_path);
        // use no filters
        return NULL;
    }
	
	// create new filter list
    GListStore *filter_list = g_list_store_new(GTK_TYPE_FILE_FILTER);
    char line[256];
	
	// loop for all filters from the config file
    while (fgets(line, sizeof(line), fp)) {
        // skipt comments and blank space
        if (line[0] == '#' || line[0] == '\n') 
        {
        	continue;
		}
		
        // remove linebreaks
        line[strcspn(line, "\r\n")] = 0;

        // formate: name:pattern[,pattern,...]
        char *sep = strchr(line, ':');
        if (!sep) 
        {
        	continue;
        }
        *sep = '\0';
        const char *name = line;
        const char *patterns = sep + 1;
		
		// create a new filter
        GtkFileFilter *filter = gtk_file_filter_new();
        gtk_file_filter_set_name(filter, name);

        // use multiple pattern
        char *token, *saveptr;
        char *patterns_copy = g_strdup(patterns);
        for (token = strtok_r(patterns_copy, ",", &saveptr);
             token != NULL;
             token = strtok_r(NULL, ",", &saveptr)) {
            // add the new pattern
            gtk_file_filter_add_pattern(filter, token);
        }
        // free the pattern space
        g_free(patterns_copy);
		
		// append the filter to the filter list
        g_list_store_append(filter_list, filter);
    }
	
	// close the file
    fclose(fp);

    // if no filters have been created 
    if (g_list_model_get_n_items(G_LIST_MODEL(filter_list)) == 0) 
    {
        // use no filters
        g_object_unref(filter_list);
        return NULL;
    }
	
	// return the filters
    return filter_list;
}
