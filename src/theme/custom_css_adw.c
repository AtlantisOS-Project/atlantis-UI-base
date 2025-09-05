/*
* custom_css_adw.c
*
* (C) Copyright 2025 AtlantisOS Project
* by @NachtsternBuild
*
* License: GNU GENERAL PUBLIC LICENSE Version 3
*
* Usage:
* const char *adw_css = get_custom_adw_css();
* gtk_css_provider_load_from_string(provider_adw, adw_css);
*
* Note: Requires GTK4 (> 4.10), libadwaita (> 1.4)
*/

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <gtk/gtk.h>
#include <unistd.h>
#include <adwaita.h>  
#include <sys/stat.h> 
#include <sys/types.h> 
#include "helper.h"
#include "design.h"

const char *get_custom_adw_css(void) 
{
    return 
 		"window {\n"
    	"    background-color: @theme_bg_color;\n" // bg color - 1
    	"    border: 2px solid @accent_bg_color;\n" // accent color - 2
    	"    border-radius: 35px;\n"
    	"    padding: 12px 24px;\n"
    	"    color: @theme_fg_color;\n" // fg color - 3
    	"    font-size: 16px;\n"
    	"}\n"
    	"button {\n"
    	"    border: 2px solid @accent_bg_color;\n" // accent color - 4
    	"    border-radius: 35px;\n"
    	"    padding: 24px 48px;\n"
    	"    background-color: @theme_bg_color;\n" // bg color - 5
    	"    color: @theme_fg_color;\n" // text / fg color - 6 
    	"    font-size: 16px;\n"
    	"}\n"
    	"button:hover {\n"
  		"	background-color: @accent_color;\n"
  		"	color: @accent_fg_color;\n"
  		"	transition: background-color 150ms ease;\n"
		"}\n"
    	"label {\n"
    	"    color: @theme_fg_color;\n" // text / fg color - 8
    	"    font-size: 16px;\n"
    	"    font-weight: bold;\n"
    	"}\n"
    	"headerbar {\n"
    	"    background-color: @theme_bg_color;\n" // bg color - 15
    	"    font-weight: bold;\n"
    	"    border: 2px solid @accent_bg_color;\n" // accent color - 16
    	"    border-radius: 35px;\n"
    	"    padding: 12px 24px;\n"
    	"    color: @theme_fg_color;\n" // text / fg color - 17
    	"    font-size: 16px;\n"
    	"}\n"
    	"frame {\n"
    	"	border-width: 5px;\n"
    	"	border-radius: 35px;\n"
		"   border-color: @accent_bg_color;\n" // accent color - 18
    	"	border-style: solid;\n"
    	"	padding: 10px;\n"
		"}\n";
}			
	
