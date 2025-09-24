# Makefile
#
# (C) Copyright 2025 AtlantisOS Project
# by @NachtsternBuild
#
# License: GNU GENERAL PUBLIC LICENSE Version 3
#
# Main makefile for the UI base
# Note: modify this to use this as your main makefile

# include the language makefile
include language.mk

.DEFAULT_GOAL := all

# compiler
#CC = gcc
CC = ccache gcc
# pass on to GCC where to search
INCLUDES = -I. -Iinclude
# compiler config
PKG_CFLAGS = `pkg-config --cflags glib-2.0 gtk4 libadwaita-1 vte-2.91-gtk4`
# linker config
PKG_LIBS = `pkg-config --libs glib-2.0 gtk4 libadwaita-1 vte-2.91-gtk4`

# compiler flags
CFLAGS = -Wall -fPIC $(PKG_CFLAGS) $(INCLUDES)
# linker flags
LIBS = $(PKG_LIBS)

# all .c files
HEADERS = $(shell find . -name '*.h')
SRC = $(shell find . -name '*.c' ! -name 'test.c')
OBJ_CLEAN = $(shell find . -name '*.o')

# objectfiles
OBJ = $(SRC:.c=.o)

# targets
STATIC_LIB = libatluibase.a
SHARED_LIB = libatluibase.so
TEST_APP = test_app

# installtion paths
PREFIX = /usr/local
INCDIR = $(PREFIX)/include/atluibase
LIBDIR = $(PREFIX)/lib

.PHONY: all objects lib po test install clean testing create-config help

all: lib po test create-config

# Building only object files
objects: $(OBJ)

# build libary
lib: $(STATIC_LIB) $(SHARED_LIB)

# build test program
test: $(TEST_APP) create-config

# static library
$(STATIC_LIB): $(OBJ) 
	ar rcs $@ $(OBJ)

# build dynamic library
$(SHARED_LIB): $(OBJ)
	$(CC) -shared -o $@ $(OBJ) $(LIBS)

# create objectfiles
%.o: %.c $(HEADERS)
	$(CC) $(CFLAGS) -c $< -o $@ 

# build test program
$(TEST_APP): test/test.c $(SRC) $(HEADERS)
	$(CC) test/test.c $(SRC) -o $(TEST_APP) $(CFLAGS) $(LIBS)

# run test program
testing: $(TEST_APP)
	./$(TEST_APP)

create-config:
	@echo 'UPDATE_TYPE="Update""' > datei.conf

# running language logic in /po
po: lang
	
# install libs, headers, languages
install: lib
	mkdir -p $(LIBDIR) $(INCDIR)
	cp $(STATIC_LIB) $(SHARED_LIB) $(LIBDIR)/
	cp $(HEADERS) $(INCDIR)/
	$(MAKE) -C po install

# run with defined localdir
dev:
	$(MAKE) all LOCALEDIR=./po

# run with po and dev mode
dev-mode-po:
	$(MAKE) all LOCALEDIR=./po DEV_MODE=1

# run only in dev mode
dev-mode:
	$(MAKE) all DEV_MODE=1

# cleanup
clean:
	rm -f $(OBJ) $(STATIC_LIB) $(SHARED_LIB) $(TEST_APP) $(OBJ_CLEAN)
	rm -rf datei.conf
	rm -f $(POTFILE)
	@for lang in $(LINGUAS); do \
		rm -f $(LOCALEDIR)/$$lang/LC_MESSAGES/$(PACKAGE).mo; \
		rm -rf $(LOCALEDIR)/$$lang; \
	done

# cleaning language files	
clean-lang:
	rm -f $(POTFILE)
	rm -rf *.po
	
help:
	@echo "=========================================="
	@echo "        AtlantisOS Makefile Help        "
	@echo "=========================================="
	@echo "General Commands:"
	@echo "  make all           -  Builds everything (default)"
	@echo "  make clean         -  Removes all built files (.o, libs, config, etc.)"
	@echo "  sudo make install  -  Installs the libraries and headers to /usr/local"
	@echo ""
	@echo "Build Options:"
	@echo "  make objects       -  Builds only the object files (*.o)"
	@echo "  make lib           -  Builds both the static (.a) and shared (.so) libraries"
	@echo "  make test          -  Builds the test application"
	@echo ""
	@echo "Language (i18n) Options:"
	@echo "  make lang          -  Builds all language files (.mo)"
	@echo "  make update-po     -  Updates existing language files based on source code"
	@echo "  make pot           -  Initializes new language files for translation"
	@echo ""
	@echo "Testing & Configuration:"
	@echo "  make testing       -  Runs the built test application"
	@echo "  make create-config -  Creates a configuration file named 'datei.conf'"
	@echo ""
	@echo "Development Modes:"
	@echo "  make dev           -  Runs a full build with the local directory for languages"
	@echo "  make dev-mode      -  Runs a full build with development mode flags"
	@echo "  make dev-mode-po   -  Runs a full build with both dev mode and local language directory"
	@echo ""
	@echo "For more detailed language options, see 'make -C po help'."
	
