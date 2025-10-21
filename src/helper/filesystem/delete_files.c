/*
* delete_files.c
*
* (C) Copyright 2025 AtlantisOS Project
* by @NachtsternBuild
*
* License: GNU GENERAL PUBLIC LICENSE Version 3
*
* Delete a file in a direector
* Delete a full path with parents
*
* Usage:
* delete_files_in_dir("/this/path");
*
* delete_files_with_parent("/other/path"); 
*
* Notes:
* Warning: This code deletes an entire path until it finds a stop point.
* Warning: If no stop point is defined, files and directories that should not be deleted will be deleted.
* Warning: Never use this code to delete foreign or non-program directories.
*/

#include "helper.h"


// max path lenght
#define MAX_PATH_CHECK 1024

// function to delete file in dir
void delete_files_in_dir(const char *path) 
{
    DIR *dir;
    struct dirent *entry;
    char fullpath[PATH_MAX];

    // open dir
    dir = opendir(path);
    if (dir == NULL) 
    {
        LOGE("Error opening the directory.");
        exit(EXIT_FAILURE);
    }

    // read dir
    while ((entry = readdir(dir)) != NULL) 
    {
        // skip '.' and '..'
        if (strcmp(entry->d_name, ".") == 0 || strcmp(entry->d_name, "..") == 0) 
        {
            continue;
        }

        // get full path
        snprintf(fullpath, sizeof(fullpath), "%s/%s", path, entry->d_name);

        // delet files
        if (remove(fullpath) != 0) 
        {
            LOGE("Error deleting the file.");
        } 
        
        else 
        {
            LOGI("File deleted: %s", fullpath);
        }
    }

    // close dir
    closedir(dir);
}

/**
 * Checks whether the path is a mount point (i.e., whether its inode differs from the parent inode).
 * @param path The path to be checked.
 * @param parent_path The parent path.
 * @return true if it is a mount point, false otherwise.
 */
static bool is_mount_point(const char *path, const char *parent_path) 
{
    struct stat path_stat, parent_stat;

    // check if parent exsits
    if (parent_path == NULL || *parent_path == '\0') 
    {
        return false;
    }

    if (stat(path, &path_stat) != 0 || stat(parent_path, &parent_stat) != 0) 
    {
        // error: allow rmdir
        return false; 
    }

    // a mount point is reached when the device ID (st_dev) is different.
    // or when st_dev is the same, but the root directories are different.
    if (path_stat.st_dev != parent_stat.st_dev) 
    {
        LOGW("Blocked: Mount Point detected (different device ID): %s", path);
        return true;
    }

    return false;
}

/**
 * Checks whether the path is a critical system directory.
 * This is only a basic list!
 *
 * @param path The path to be checked.
 * @return true if the path is classified as critical, otherwise false.
 */
static bool is_critical_system_path(const char *path) 
{
    // root path and system paths
    const char *critical_prefixes[] = {
        "/",          // Root
        "/bin",       // Binaries
        "/sbin",      // System Binaries
        "/usr",       // Unix System Resources
        "/etc",       // Configuration Files
        "/var",       // Variable Data
        "/proc",      // Process Information
        "/sys",       // Kernel and hardware information
        "/dev",       // Devices
        "/lib",       // Libraries
        "/boot",      // Boot files
        "/root",      // Root's home
        NULL 
    };

    for (int i = 0; critical_prefixes[i] != NULL; i++) 
    {
        // check if path is critical
        if (strncmp(path, critical_prefixes[i], strlen(critical_prefixes[i])) == 0) 
        {
            // path lenght OK or end with '/'
            size_t prefix_len = strlen(critical_prefixes[i]);
            if (strlen(path) == prefix_len || path[prefix_len] == '/') 
            {
                 LOGE("Blocked: Critical System Path Prefix '%s' detected for %s", critical_prefixes[i], path);
                 return true;
            }
        }
    }
    return false;
}

/**
 * Checks whether the path belongs to one of the standard, important
 * home subfolders. This is the part with language dependency.
 *
 * @param path The path to be checked.
 * @param home_dir The user's home directory.
 * @return true if the path is a standard home directory, otherwise false.
 */
static bool is_standard_home_directory(const char *path, const char *home_dir) 
{
    if (home_dir == NULL || *home_dir == '\0') 
    {
        return false;
    }

    // check the standard home path
    if (strcmp(path, home_dir) == 0) 
    {
        LOGE("Blocked: Attempt to delete the User Home Directory: %s", path);
        return true;
    }

    // list with standard home paths
    const char *standard_folders[] =
     {
        "Desktop",
        "Documents",
        "Downloads",
        "Music",
        "Pictures",
        "Videos",
        "Public",
        "Templates",
        ".config", 
        ".local",  
        NULL
    };

    char check_path[MAX_PATH_CHECK];

    for (int i = 0; standard_folders[i] != NULL; i++) 
    {
        // create full path
        snprintf(check_path, MAX_PATH_CHECK, "%s/%s", home_dir, standard_folders[i]);

        // check if full path is path
        if (strcmp(path, check_path) == 0) 
        {
            LOGE("Blocked: Standard Home Folder (English name) %s", path);
            return true;
        }
    }
    
    return false;
}

/**
 * Deletes all files in the path. If the directory is empty afterwards,
 * it is deleted and the process is applied recursively to the parent path
 * until a non-empty directory or the home directory is reached.
 *
 * @param path The path in which the files are to be deleted.
 * @param stop_dir The path at which the recursive deletion of the parent should stop (optional, e.g., home directory).
 */
static void delete_files_and_parents(const char *path, const char *stop_dir)
{    
    DIR *dir;
    struct dirent *entry;
    char fullpath[PATH_MAX];
    char parent_path[PATH_MAX];
    size_t path_len;
    
    // avoid recursion into the home directory to prevent deleting too much.
    if (stop_dir != NULL && strcmp(path, stop_dir) == 0) 
    {
        return; 
    }

    dir = opendir(path);
    if (dir == NULL)
    {
        LOGE("Error opening the directory: %s", path);
        return;
    }

    // remove files in the 'path'
    while ((entry = readdir(dir)) != NULL)
    {
        if (strcmp(entry->d_name, ".") == 0 || strcmp(entry->d_name, "..") == 0)
        {
            continue;
        }

        snprintf(fullpath, sizeof(fullpath), "%s/%s", path, entry->d_name);

        // Check if it is a directory (this function should only delete files)
        struct stat st;
        if (stat(fullpath, &st) == 0 && S_ISDIR(st.st_mode)) 
        {
             LOGI("Skipping directory: %s", fullpath);
             continue; // skip directory
        }
        
        // remove files
        if (remove(fullpath) != 0)
        {
            LOGE("Error deleting file %s: %s", fullpath, strerror(errno));
        }
        else
        {
            LOGI("File deleted: %s", fullpath);
        }
    }
    
    // close file stream
    closedir(dir);

    // --- Phase 2: Aktuelles Verzeichnis und Eltern rekursiv löschen ---
    
    // check if directory is empty
    dir = opendir(path);
    if (dir == NULL) 
    {
    	return; // directory aren't empty
    }
    
    int is_empty = 1;
    while ((entry = readdir(dir)) != NULL)
    {
        if (strcmp(entry->d_name, ".") != 0 && strcmp(entry->d_name, "..") != 0)
        {
            is_empty = 0;
            break;
        }
    }
    
    closedir(dir);
    
    if (is_empty)
    {
        // check if directory is a mount point
        if (is_mount_point(path, parent_path)) 
        {
        	LOGW("Stopping recursion: Path is a Mount Point: %s", path);
        	return;
    	}

    	// check if path is standard or critical system directory
    	if (is_critical_system_path(path) || is_standard_home_directory(path, stop_dir))
    	{
        	LOGW("Stopping recursion: Path is protected (System/Standard Home Folder): %s", path);
        	return;
    	}
    	
		// remove the direcotry
        if (rmdir(path) == 0)
        {
           	LOGI("Directory deleted: %s (Empty)", path);

           	// get the parent path
           	strncpy(parent_path, path, PATH_MAX);
           	path_len = strlen(parent_path);
            
           	// find the last '/'
           	while (path_len > 0 && parent_path[path_len - 1] != '/') 
           	{
           	    path_len--;
           	}
            
           	if (path_len > 1) // make sure we are not in the root directory
           	{ 
           	    parent_path[path_len - 1] = '\0'; // shorten the path
            	    
           	    // recursive call of the path
           	    delete_files_and_parents(parent_path, stop_dir);
           	}
        }
        
        else
        {
            LOGE("Error deleting empty directory %s: %s", path, strerror(errno));
        }
    }
    
    else
    {
        LOGI("Stopping recursion: Directory %s is not empty.", path);
    }
}

/**
 * Wrapper function that performs security checks before
 * recursive deletion is started.
 *
 * @param path The path where deletion should begin.
 */
void delete_files_with_parent(const char *path) 
{
    LOGW("Warning: This code deletes an entire path until it finds a stop point.");
    LOGW("Warning: If no stop point is defined, files and directories that should not be deleted will be deleted.");
    LOGW("Warning: Never use this code to delete foreign or non-program directories.");
    
    char clean_path[PATH_MAX];
    const char *home_dir = get_home_directory(); 

    // resolve path
    if (realpath(path, clean_path) == NULL) 
    {
        strncpy(clean_path, path, PATH_MAX); // use original path as fallback
    }
		
    // check if the start diretory isn't critical 
    if (is_critical_system_path(clean_path)) 
    {
        LOGE("Aborted: Starting path is a critical system path: %s", clean_path);
        return;
    }

    // starting delte the directory with parent
    delete_files_and_parents(clean_path, home_dir);
}

