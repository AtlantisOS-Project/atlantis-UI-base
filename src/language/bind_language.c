/*
* bind_language.c
*
* (C) Copyright 2025 AtlantisOS Project
* by @NachtsternBuild
*
* License: GNU GENERAL PUBLIC LICENSE Version 3
*
* Usage:
* bind_language("de")
*/

#include "helper.h"
#include "language.h" 

// extern config
extern const char *LOCALE_DOMAIN;

// try to bind local dir and .mo files
// dev mode = direct binding of languages 
void bind_language(const char *lang)
{
#ifdef DEV_MODE
    // set two paths for the .po files
    const char *locale_paths[] = {
        "./locale",                // build / run in source tree
        "/usr/local/share/locale", // self-installed
        "/usr/share/locale"        // system-wide packages
    };

    int found = 0;
    for (int i = 0; i < (int)(sizeof(locale_paths)/sizeof(locale_paths[0])); i++) 
    {
        char testpath[512];
        snprintf(testpath, sizeof(testpath), "%s/%.*s/LC_MESSAGES/%s.mo",
                 locale_paths[i], 2, lang, LOCALE_DOMAIN);
		
		// open testpath
        FILE *file = fopen(testpath, "r");
        if (file) 
        {
            fclose(file);
            // bind the files from the path
            bindtextdomain(LOCALE_DOMAIN, locale_paths[i]);
            bind_textdomain_codeset(LOCALE_DOMAIN, "UTF-8");
            textdomain(LOCALE_DOMAIN);
            LOGI("Using translations from: %s (lang=%s)", locale_paths[i], lang);
            found = 1;
            break;
        }
    }
	
	// no file in the testpaths = use fallback to main logic
    if (!found) 
    {
        LOGW("No .mo found for %s, fallback to /usr/share/locale", lang);
        bindtextdomain(LOCALE_DOMAIN, "/usr/share/locale");
        bind_textdomain_codeset(LOCALE_DOMAIN, "UTF-8");
        textdomain(LOCALE_DOMAIN);
    }
#else
    LOGI("bind_language(%s) ignored: build without DEV_MODE", lang);
#endif
}
