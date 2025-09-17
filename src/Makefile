# Makefile
#
# (C) Copyright 2025 AtlantisOS Project
# by @NachtsternBuild
#
# License: GNU GENERAL PUBLIC LICENSE Version 3
#
# Usage:
# everything:
# make
# 
# building only object files
# make objects
#
# building libs
# make lib
#
# building test
# make test
#
# only languages:
# make -C po
# 
# init new languages:
# make -C po init-po
#
# update languages:
# make -C po update-po
#
# install everything
# sudo make install
#
# cleanup
# make clean
#
# create test app
# make test
#
# run test app
# make testing
# 
# create test config file
# make create-config


# compiler
#CC = gcc
CC = ccache gcc
# pass on to GCC where to search
INCLUDES = -I. -Iinclude
# compiler flags
CFLAGS = -Wall -fPIC $(INCLUDES) `pkg-config --cflags glib-2.0 gtk4 libadwaita-1 vte-2.91-gtk4`
# linker flags
LIBS   = `pkg-config --libs glib-2.0 gtk4 libadwaita-1 vte-2.91-gtk4`

# all .c files
HEADERS = $(shell find . -name '*.h')
SRC = $(shell find . -name '*.c' ! -name 'test.c')

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

.PHONY: all objects lib test po install clean testing create-config help

all: lib test po create-config

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
po:
	$(MAKE) -C po

# install libs, headers, languages
install: lib
	mkdir -p $(LIBDIR) $(INCDIR)
	cp $(STATIC_LIB) $(SHARED_LIB) $(LIBDIR)/
	cp $(HEADERS) $(INCDIR)/
	$(MAKE) -C po install

help:
	@echo " Building everything: "
	@echo " make "
	@echo " Building only object files: "
	@echo " make objects "
	@echo " Building libs: "
	@echo " make lib "
	@echo " Building test: "
	@echo " make test "
	@echo " Only languages: "
	@echo " make -C po "
	@echo " Init new languages: "
	@echo " make -C po init-po "
	@echo " Update languages: "
	@echo " make -C po update-po "
	@echo " Install everything: "
	@echo " sudo make install "
	@echo " Cleanup: "
	@echo " make clean "
	@echo " Create test app: "
	@echo " make test "
	@echo " Run test app: "
	@echo " make testing "
	@echo " Create test config file: "
	@echo " make create-config "
	@echo " Help: "
	@echo " make help "

# cleanup
clean:
	rm -f $(OBJ) $(STATIC_LIB) $(SHARED_LIB) $(TEST_APP)
	$(MAKE) -C po clean
	rm -rf datei.conf
