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

#include "helper.h"
#include "design.h"

const char *get_custom_adw_css(void) 
{
    return 
 		"window {\n"
    	"    background-color: @theme_bg_color;\n" // bg color - 1
    	"    color: @theme_fg_color;\n" // fg color - 2
    	"    padding: 12px;\n"
    	"    font-size: 14px;\n"
    	"    border: 2px solid @theme_bg_color;\n" // bg color - 3
    	"    border-radius: 35px;\n"
    	"}\n"
    	"button {\n"
    	"    background-color: @accent_bg_color;\n" // bg color - 4
    	"    color: @theme_fg_color;\n" // text / fg color - 5
    	"    padding: 12px;\n"
    	"    border: 2px solid @theme_bg_color;\n" // bg color - 6
    	"    border-radius: 35px;\n"
    	"    font-size: 14px;"
    	"}\n"
    	"button:hover {\n"
  		"	 background-color: @accent_color;\n" // accent color - 7
  		"	 color: @accent_fg_color;\n" // fg color - 8
  		"	 transition: background-color 150ms ease;\n"
		"}\n"
    	"label {\n"
    	"    color: @theme_fg_color;\n" // text / fg color - 9
    	"    font-weight: bold;\n"
    	"    font-size: 16px;"
    	"}\n"
    	"headerbar {\n"
    	"    background-color: @theme_bg_color;\n" // bg color - 10
    	"    font-weight: bold;\n"
    	"    color: @theme_fg_color;\n" // text / fg color - 11
    	"    padding: 12px;\n"
    	"    border: 2px solid @theme_bg_color;\n" // bg color - 12
    	"    border-radius: 35px;\n"
    	"    font-size: 14px;"
    	"}\n"
    	"frame {\n"
    	"	 border-width: 5px;\n"
    	"	 border-radius: 35px;\n"
		"    border-color: @accent_bg_color;\n" // accent color - 13
    	"	 border-style: solid;\n"
    	"	 padding: 10px;\n"
		"}\n";
}			
	
