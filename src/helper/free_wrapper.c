/*
* free_wrapper.c
*
* (C) Copyright 2025 AtlantisOS Project
* by @NachtsternBuild
*
* License: GNU GENERAL PUBLIC LICENSE Version 3
*
* Usage:
* used to auto free something
*/
 
#include "helper.h"

// function to auto free
void free_wrapper(void *p) 
{ 
    free(*(void **)p); 
}
