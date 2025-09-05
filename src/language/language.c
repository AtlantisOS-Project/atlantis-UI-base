/*
* language.c
*
* (C) Copyright 2025 AtlantisOS Project
* by @NachtsternBuild
*
* License: GNU GENERAL PUBLIC LICENSE Version 3
*
* Usage:
* init_language();
* set_language("de");
*/

/* headers */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <locale.h>
#include <sys/stat.h>
#include <unistd.h>
#include "language.h"

// extern config
extern const char *LOCALE_DOMAIN;

// function that read the language from files
// function that get the system language directly
void init_language(void)
{
    // get the locale from the system
    setlocale(LC_ALL, "");

    LOGI("Set language by the system language.");
    char langbuf[16] = {0};

    // read environment variable LANG
    const char *envlang = getenv("LANG");
    if (!envlang || strlen(envlang) < 2)
    {
        strcpy(langbuf, "en"); // fallback to English
    }
    else
    {
        strncpy(langbuf, envlang, 2); // only first 2 chars like "de", "en"
        langbuf[2] = '\0';
    }

    // set global usage of the language
    LOGI("Set new language: %s", langbuf);
    setenv("LANGUAGE", langbuf, 1);
    bind_language(langbuf);
}

// function that set the language manuell
void set_language(const char *lang)
{
    if (!lang || strlen(lang) < 2)
    {
    	LOGI("Using fallback language");
    	lang = "en"; // fallback to english
    }
	
	LOGI("Set the new language: %s", lang);
    setenv("LANGUAGE", lang, 1);
    setlocale(LC_ALL, "");
    bind_language(lang);
}



