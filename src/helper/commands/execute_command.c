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
#define DEFAULT_TIMEOUT 0   // 0 = no timeout

/**
* @brief helper for whitelisting commands
*/
static bool is_allowed_command(const char *cmd)
{
    if (!cmd) 
    {
    	return false;
	}
	
    // whitelist
    const char *allowed[] = {
        "adb",
        "adb.exe",
        "fastboot",
        "fastboot.exe",
        "ls",
        "cat",
        "grep",
        "curl",
        "wget", 
        "unzip",
        "xz",
        "tar",
        "getprop",
        "pkexec",
        "bash",
        "sh",
        "heimdall",
        NULL
    };

    for (int i = 0; allowed[i]; i++)
    {
        if (strcmp(cmd, allowed[i]) == 0)
        {
            return true;
        }
    }

    return false;
}

/**
* @brief helper that execute a command safe
*/
static bool is_safe_executable(const char *cmd)
{
    if (!cmd) 
    {
    	return false;
	}
	
    // absolute path 
    if (cmd[0] == '/')
    {
        return access(cmd, X_OK) == 0;
	}
    return is_allowed_command(cmd);
}

/**
* @brief helper that wait (with optional timeout)
*/
static bool wait_for_child(pid_t pid, int *exit_status, int timeout_sec)
{
    int status;
    int waited = 0;

    while (1)
    {
        pid_t result = waitpid(pid, &status, (timeout_sec > 0) ? WNOHANG : 0);

        if (result == pid)
        {
            if (WIFEXITED(status))
            {
                if (exit_status) 
                {
                    *exit_status = WEXITSTATUS(status);
                }
                return WEXITSTATUS(status) == 0;
            }
            return false;
        }

        if (result == -1)
        {
            return false;
        }

        // blocking mode
        if (timeout_sec == 0)
        {
            continue;
        }

        // timeout handling
        if (waited >= timeout_sec)
        {
            kill(pid, SIGKILL);
            waitpid(pid, NULL, 0);
            return false;
        }

        sleep(1);
        waited++;
    }
}

/**
* @brief function that run a shell command in a safe 
*/
bool run_command_safe(char *const argv[], int timeout_sec)
{
    if (!argv || !argv[0])
    {
        return false;
	}
	
    pid_t pid = fork();

    if (pid < 0)
    {
        return false;
	}
	
    if (pid == 0)
    {
        // secure PATH
    	setenv("PATH", "/usr/bin:/bin:/usr/sbin:/sbin", 1);

    	// validate command
    	if (!is_safe_executable(argv[0]))
    	{
        	fprintf(stderr, "Blocked unsafe command: %s\n", argv[0]);
        	_exit(127);
    	}

        // child
        // reset signals
        signal(SIGINT, SIG_DFL);
        signal(SIGTERM, SIG_DFL);

        execvp(argv[0], argv);

        // only reached on error
        perror("execvp failed");
        _exit(127);
    }

    // parent
    return wait_for_child(pid, NULL, timeout_sec);
}


/**
* @brief Run a command in terminal via system()
*/
void run_command(char *const argv[])
{
    run_command_safe(argv, DEFAULT_TIMEOUT);
}

/**
* @brief Run a command and return true or false
*/
bool run_command_bool(char *const argv[])
{
    return run_command_safe(argv, DEFAULT_TIMEOUT);
}

/**
* @brief Function to execute a command and capture its output
*/
char *execute_command(char *const argv[])
{
    if (!argv || !argv[0])
    {
        return NULL;
	}
	
    int pipefd[2];
    if (pipe(pipefd) == -1)
    {
        return NULL;
	}
	
    pid_t pid = fork();

    if (pid < 0)
    {
        close(pipefd[0]);
        close(pipefd[1]);
        return NULL;
    }

    if (pid == 0)
    {
        // secure PATH
    	setenv("PATH", "/usr/bin:/bin:/usr/sbin:/sbin", 1);

    	// validate command
    	if (!is_safe_executable(argv[0]))
    	{
        	fprintf(stderr, "Blocked unsafe command: %s\n", argv[0]);
        	_exit(127);
    	}

        
        // child
        close(pipefd[0]);

        dup2(pipefd[1], STDOUT_FILENO);
        dup2(pipefd[1], STDERR_FILENO);
        close(pipefd[1]);

        execvp(argv[0], argv);
        _exit(127);
    }

    // parent
    close(pipefd[1]);

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
            char *tmp = realloc(result, capacity);
            if (!tmp)
            {
                free(result);
                close(pipefd[0]);
                return NULL;
            }
            result = tmp;
        }

        memcpy(result + size, buffer, bytes);
        size += bytes;
    }

    result[size] = '\0';

    close(pipefd[0]);

    wait_for_child(pid, NULL, DEFAULT_TIMEOUT);

    return result;
}
