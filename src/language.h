/*
* language.h
*
* (C) Copyright 2025 AtlantisOS Project
* by @NachtsternBuild
*
* License: GNU GENERAL PUBLIC LICENSE Version 3
*
* Usage:
* #include "language.h"
*/
 
#ifndef LANGUAGE_H
#define LANGUAGE_H

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <gtk/gtk.h>
#include <unistd.h>  
#include <sys/stat.h> 
#include <sys/types.h> 
#include <libintl.h>
#include <locale.h>
#include "helper.h"

/* define gettext macros
* Usage:
* _("Some text")
*/
#define _(STRING) gettext(STRING)


// function that get the path of the language file
char *get_lang_path();
// function that get the current language
gchar *get_current_language();

// new function that init the language
// this usage the system language or fallback to english
void init_language(void);

/*
* function that set the language manuell
*
* Usage: 
* set_language("de");
*/
void set_language(const char *lang);


/*
* additional functions for the language managment
*/
// try to bind local dir and .mo files
void bind_language(const char *lang);

#endif // LANGUAGE_H

