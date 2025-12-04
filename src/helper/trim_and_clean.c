/**
* trim_and_clean.c
*
* (C) Copyright 2025 AtlantisOS Project
* by @NachtsternBuild
*
* License: GNU GENERAL PUBLIC LICENSE Version 3
*
* @brief Removes leading and trailing spaces/quotation marks from a string
* 
* Usage:
* trim_and_clean(value_buffer);
*
*/

#include "helper.h"

/**
 * @brief Removes leading and trailing spaces/quotation marks from a string
 * @param str The string to be cleaned up.
 * @return The cleaned string.
 */
char *trim_and_clean(char *str) 
{
    if (!str || *str == '\0') 
    {
        return str;
    }
    
    // remove newline and empty space
    char *end = str + strlen(str) - 1;
    while (end > str && isspace((unsigned char)*end)) 
    {
        *end-- = '\0';
    }

    // remove empty space at start
    char *start = str;
    while (*start && isspace((unsigned char)*start)) 
    {
        start++;
    }
    
    // remove '"' at start and at end
    if (start[0] == '"' && start[strlen(start) - 1] == '"') 
    {
        start++;
        start[strlen(start) - 1] = '\0';
    }

    // copy at the start of the function
    if (start != str) 
    {
        memmove(str, start, strlen(start) + 1);
    }

    return str;
}
