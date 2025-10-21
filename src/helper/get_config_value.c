/*
* get_config_value.c
*
* (C) Copyright 2025 AtlantisOS Project
* by @NachtsternBuild
*
* License: GNU GENERAL PUBLIC LICENSE Version 3
*
* callbacks from the main UI are received and passed on to the background scripts 
* 
* Usage:
* char *update_type = get_conf_value(conf_file, "UPDATE_TYPE");
*
* This get the all values from a config file
*/

#include "helper.h"

// function that get values from config files
char *get_config_value(const char *filename, const char *key) 
{
    // open the file
    FILE *fp = fopen(filename, "r");
    if (!fp) 
    {
        LOGE("Could not open %s.\n", filename);
        fprintf(stderr, "Could not open %s.\n", filename);
        return NULL;
    }

    static char line[512];
    size_t keylen = strlen(key);
    while (fgets(line, sizeof(line), fp)) 
    {
        if (strncmp(line, key, keylen) == 0 && line[keylen] == '=') 
        {
            // line has the correct format
            char *val = strchr(line, '=');
            if (val) 
            {
                val++; // jump to '='
                // remove quotation marks
                if (*val == '"') 
                {
                	val++;
                }
                char *end = strchr(val, '"');
                if (end) 
                {
                	*end = '\0';
                }
                // close the file
                fclose(fp);
                return strdup(val);
            }
        }
    }
    fclose(fp);
    return NULL;
}
