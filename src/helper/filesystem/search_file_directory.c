/**
* search_file_directory.c
*
* (C) Copyright 2025 AtlantisOS Project
* by @NachtsternBuild
*
* License: GNU GENERAL PUBLIC LICENSE Version 3
*
*
* @brief Search if a predefined string exsists in the file of predefined directory
*
* Usage:
*  int result = search_file_directory(SOURCES_DIR, SEARCH_STRING); 
*  if (result == -1) {
*  	  return 2; // error
*  } else if (result == 1) {
*     return 0; // no file
*  } else {
*     return 1; // file
*  }
*
* Notes: 
* @param directory: Search at this path.
* @param search_string: Search for this string.
* @return 0 file not exsists, 1 file exsists, -1 on error, .
*/

#include "helper.h"

int search_file_directory(const char *directory, const char *search_string) 
{
    DIR *dir;
    struct dirent *entry;
    int found = 0;

    dir = opendir(directory);
    if (!dir) 
    {
        perror("Kann Verzeichnis nicht öffnen");
        return -1; 
    }

    while ((entry = readdir(dir)) != NULL) 
    {
        // ignore . and ..
        if (strcmp(entry->d_name, ".") == 0 || strcmp(entry->d_name, "..") == 0)
        {
            continue;
		}
		
		// found file/files with string
        if (strstr(entry->d_name, search_string) != NULL) 
        {
            found = 1;
            break; 
        }
    }

    closedir(dir);
    return found;
}
