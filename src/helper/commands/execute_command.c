/**
* execute_command.c
*
* (C) Copyright 2026 AtlantisOS Project
* by @NachtsternBuild
*
* License: GNU GENERAL PUBLIC LICENSE Version 3
*
* Usage:
* gchar *full_command = g_strdup_printf("pkexec %s", command);
* gchar *execution_result = execute_command(full_command);
* g_free(full_command);
*
* run_command("sleep 5");
*
* if (!run_command(cmd)) {}
*/

#include "helper.h"

/**
* @brief Helperfunction to check if the command is safe
*/

bool is_safe_command(const char *cmd) 
{
    if (!cmd) 
    {
    	return false;
    }

    // some filters
    // TODO: 
    // later more
    if (strstr(cmd, "&") || strstr(cmd, ";") || strstr(cmd, "|") ||
        strstr(cmd, "`") || strstr(cmd, "$(")) 
    {
        return false;
    }

    return true;
}

/**
* @brief Run a command in terminal via system()
*/
void run_command(const char *command) 
{
    if (!command || !is_safe_command(command))
    {
        return;
	}
	
    pid_t pid = fork();

    if (pid == 0) 
    {
        // child
        execl("/bin/sh", "sh", "-c", command, (char *)NULL);
        _exit(127); // if exec returns with error
    }
     
    else if (pid > 0) 
    {
        // parent
        waitpid(pid, NULL, 0);
    }
}


/**
* @brief Run a command and return true or false
*/
bool run_command_bool(const char *cmd) 
{
    if (!cmd || !is_safe_command(cmd))
    {
        return false;
	}
	
	// child
    pid_t pid = fork();

    if (pid == 0) 
    {
        execl("/bin/sh", "sh", "-c", cmd, (char *)NULL);
        _exit(127); // if exec returns with error
    } 
    
    else if (pid > 0) 
    {
        int status;
        // parent
        waitpid(pid, &status, 0);

        return WIFEXITED(status) && WEXITSTATUS(status) == 0;
    }

    return false;
}


/**
* @brief Function to execute a command and capture its output
*/
char *execute_command(const char *command) 
{
    if (!command || !is_safe_command(command))
    {
        return NULL;
	}
	
    int pipefd[2];
    if (pipe(pipefd) == -1)
    {
        return NULL;
	}
	
    pid_t pid = fork();

    if (pid == 0) 
    {
        // child
        close(pipefd[0]); // read end

        dup2(pipefd[1], STDOUT_FILENO);
        dup2(pipefd[1], STDERR_FILENO);

        close(pipefd[1]);

        execl("/bin/sh", "sh", "-c", command, (char *)NULL);
        _exit(127);
    }

    close(pipefd[1]); // write end

    size_t capacity = 4096;
    size_t size = 0;
    char *result = malloc(capacity);

    if (!result) 
    {
        close(pipefd[0]);
        return NULL;
    }

    char buffer[512];
    ssize_t bytes;

    while ((bytes = read(pipefd[0], buffer, sizeof(buffer))) > 0) 
    {
        if (size + bytes + 1 > capacity) 
        {
            capacity *= 2;
            result = realloc(result, capacity);
            if (!result) 
            {
                close(pipefd[0]);
                return NULL;
            }
        }

        memcpy(result + size, buffer, bytes);
        size += bytes;
    }

    result[size] = '\0';

    close(pipefd[0]);
    waitpid(pid, NULL, 0);

    return result;
}
