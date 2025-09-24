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
#include <glib.h>
#include <glib/gi18n.h>
#include <libintl.h>
#include <locale.h>
#include "helper.h"

#ifdef __cplusplus
extern "C" {
#endif

/*
* Define the path to the langauge files
*
* Usage:
* const char *LOCALEDIR_PATH = "/usr/share/locale";
*/
extern const char *LOCALEDIR_PATH;

// function that get the path of the language file
void set_language_dir(const char *dir);
// get the language dir
const char *get_language_dir(void);

// new function that init the language
// this usage the system language or fallback to english
void init_language(void);

// try to bind local dir and .mo files
// dev mode = direct binding of languages 
void bind_language(const char *lang);


#ifdef __cplusplus
}
#endif

#endif // LANGUAGE_H

