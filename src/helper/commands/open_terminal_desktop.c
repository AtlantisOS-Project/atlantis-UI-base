/*
 * open_terminal_desktop.c
 *
 * (C) 2025 AtlantisOS Project
 * by @NachtsternBuild
 *
 * License: GNU GENERAL PUBLIC LICENSE Version 3
 *
 * Run a command in a new terminal
 *
 * Usage:
 * open_terminal_by_desktop("echo \"Hello World \" && sleep 5");
 * 
 * Note:
 * default OS is linux
 */
 
#include "helper.h"

#ifdef _WIN32
    #include <windows.h>
#endif

// max command length
#define MAX_CMD_LEN 2048

// Create a new terminal and run a command
void open_terminal_by_desktop(const char *function_command) 
{
    char cmd[MAX_CMD_LEN];
    int result = 0;
	
	char *desktop = getenv("XDG_CURRENT_DESKTOP");
    char command[2048];

    /*
    * Running in WSL:
    * Using cmd.exe that start wsl.exe
    */
    if (directory_exists("/mnt/c/Users")) 
    {
        // use the cmd.exe to start a new cmd with wsl.exe
		snprintf(command, sizeof(command), "cmd.exe /C start cmd.exe /K \"wsl.exe %s\"", function_command);
        run_command(command);
        return;
    }
	
    #ifdef _WIN32
        /*
        * Windows: 
        * Using 'start' to start the cmd.exe
        */
        snprintf(cmd, MAX_CMD_LEN, "start cmd.exe /K \"%s\"", function_command);
        result = system(cmd);

    #elif defined(__APPLE__) && defined(__MACH__)
        /* 
        * macOS: 
        * Using 'osascript'         
        */
        snprintf(cmd, MAX_CMD_LEN, "osascript -e 'tell application \"Terminal\" to do script \"%s\"'", function_command);
        result = system(cmd);

    #else
        /* 
        * Linux: 
        * get the desktop enivorment and open based on this a new terminal
        */    
        // GNOME
        if (strstr(desktop, "GNOME") != NULL) 
        {
            snprintf(cmd, sizeof(cmd), "gnome-terminal -- bash -c '%s; exec bash'", function_command);
        }
        // KDE Plasma
        else if (strstr(desktop, "KDE") != NULL) 
        {
            snprintf(cmd, sizeof(cmd), "konsole -e bash -c '%s; exec bash'", function_command);
        }
        // Xfce oder Cinnamon
        else if (strstr(desktop, "X-Cinnamon") != NULL) 
        {
            snprintf(cmd, sizeof(cmd), "gnome-terminal -- bash -c '%s; exec bash'", function_command);
        }
        // LXDE/LXQt
        else if (strstr(desktop, "LXDE") != NULL || strstr(desktop, "LXQt") != NULL) 
        {
            snprintf(cmd, sizeof(cmd), "lxterminal -e bash -c '%s; exec bash'", function_command);
        }
        // MATE
        else if (strstr(desktop, "MATE") != NULL) 
        {
            snprintf(cmd, sizeof(cmd), "mate-terminal -- bash -c '%s; exec bash'", function_command);
        } 
        
        else 
        {
            LOGE("Error: Unsupported desktop environment: %s\n", desktop);
            fprintf(stderr, "Error: Unsupported desktop environment: %s\n", desktop);
            return;
        }
        
        result = system(cmd);
    #endif

    if (result != 0) 
    {
        LOGW("Warning: system() call to open terminal may have failed. Error code: %d\n", result);
        fprintf(stderr, "Warnung: system() Aufruf zur Terminal-Öffnung könnte fehlgeschlagen sein. Fehlercode: %d\n", result);
    }
}
