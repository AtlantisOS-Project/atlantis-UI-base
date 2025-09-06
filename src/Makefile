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

# compiler
#CC = gcc
CC = ccache gcc
# pass on to GCC where to search
INCLUDES = -I. -Iinclude
# compiler flags
CFLAGS = -Wall -fPIC $(INCLUDES) `pkg-config --cflags gtk4 libadwaita-1`
# linker flags
LIBS   = `pkg-config --libs gtk4 libadwaita-1`

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

.PHONY: all objects lib test po install clean testing

all: lib test po

# Building only object files
objects: $(OBJ)

# build libary
lib: $(STATIC_LIB) $(SHARED_LIB)

# build test program
test: $(TEST_APP)

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
	$(CC) test/test.c $(SRC) -o $(TEST_BUILD) $(CFLAGS) $(LIBS)

# run test program
testing: $(TEST_BUILD)
	./$(TEST_BUILD)

# running language logic in /po
po:
	$(MAKE) -C po

# install libs, headers, languages
install: lib
	mkdir -p $(LIBDIR) $(INCDIR)
	cp $(STATIC_LIB) $(SHARED_LIB) $(LIBDIR)/
	cp $(HEADERS) $(INCDIR)/
	$(MAKE) -C po install

# cleanup
clean:
	rm -f $(OBJ) $(STATIC_LIB) $(SHARED_LIB) $(TEST_APP)
	$(MAKE) -C po clean
