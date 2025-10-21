/*
 * language.c
 *
 * (C) Copyright 2025 AtlantisOS Project
 * by @NachtsternBuild
 *
 * License: GNU GENERAL PUBLIC LICENSE Version 3
 *
 * Provides gettext binding and translation setup
 */

#include "language.h"
#include "helper.h"

// extern config
extern const char *LOCALE_DOMAIN;
extern const char *LOCALEDIR_PATH;

static char *current_localedir = NULL;

// set the language dir
void set_language_dir(const char *dir) 
{
    if (dir && g_file_test(dir, G_FILE_TEST_IS_DIR)) 
    {
        g_free(current_localedir);
        current_localedir = g_strdup(dir);
    }
}

// get the language dir
const char *get_language_dir(void) 
{
    return current_localedir ? current_localedir : LOCALEDIR_PATH;
}

// init local binding
void init_language(void) 
{
    setlocale(LC_ALL, "");

    // check for env = ATL_LOCALDIR
    const char *envdir = g_getenv("ATL_LOCALEDIR");
    // set dir by env
    if (envdir && g_file_test(envdir, G_FILE_TEST_IS_DIR)) 
    {
        set_language_dir(envdir);
    }
    
    // local /po dir
    else if (g_file_test("./po", G_FILE_TEST_IS_DIR)) 
    {
        set_language_dir("./po");
    }
    
    // default dir
    else 
    {
        set_language_dir(LOCALEDIR_PATH);
    }
	
	// bind the textdomain
    bindtextdomain(LOCALE_DOMAIN, current_localedir);
    bind_textdomain_codeset(LOCALE_DOMAIN, "UTF-8");
    textdomain(LOCALE_DOMAIN);

    LOGI("Using locale dir: %s (package: %s)", current_localedir, LOCALE_DOMAIN);
}
