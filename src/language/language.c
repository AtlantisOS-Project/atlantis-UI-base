/**
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

/**
* @brief Init local binding
*/
void init_language(void) 
{   
    if (debug_lang) 
    {
        LOGW("Using fallback for language.");
        
        // try to get to set the env
        g_setenv("LC_ALL", "en_US.UTF-8", TRUE);
        g_setenv("LANG", "en_US.UTF-8", TRUE);
        
        // set the local to english
        setlocale(LC_ALL, "en_US.UTF-8"); 
        
        // set default path
        set_language_dir(LOCALEDIR_PATH); 
        
        // bind the textdomain
    	bindtextdomain(LOCALE_DOMAIN, current_localedir);
    	bind_textdomain_codeset(LOCALE_DOMAIN, "UTF-8");
    	textdomain(LOCALE_DOMAIN);
    	return;
    }
    
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
    
    // installed as debian package
    else if (g_file_test("/usr/share/locale", G_FILE_TEST_IS_DIR))
    {
    	set_language_dir("/usr/share/locale");
    }
    
    // installed as snap package 
    else if (getenv("SNAP") != NULL || getenv("SNAP_NAME") != NULL)
    {
    	char snap_language[128];
    	const char *snapenv = g_getenv("SNAP");
    	if (snapenv == NULL) 
    	{
    		snapenv = g_getenv("SNAP_NAME");
    	} 
    	snprintf(snap_language, sizeof(snap_language), "%s/usr/share/locale", snapenv);
    	set_language_dir(snap_language);
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
