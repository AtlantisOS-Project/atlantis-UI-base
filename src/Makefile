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


# compiler
#CC = gcc
CC = ccache gcc
CFLAGS = -Wall -fPIC `pkg-config --cflags gtk4 libadwaita-1`
LIBS = `pkg-config --cflags gtk4 libadwaita-1`

# all .c files
SRC = $(shell find . -name '*.c' ! -name 'test.c')
HDR = $(shell find . -name '*.h')

# objectfiles
OBJ = $(SRC:.c=.o)

# targets
STATIC_LIB = libatluibase.a
SHARED_LIB = libatluibase.so
TEST_BUILD = test_app

# installtion paths
PREFIX = /usr/local
INCDIR = $(PREFIX)/include/atluibase
LIBDIR = $(PREFIX)/lib

.PHONY: all lib test po install clean

all: lib test po

# build libary
lib: $(STATIC_LIB) $(SHARED_LIB)

# static library
$(STATIC_LIB): $(OBJ)
	ar rcs $@ $(OBJ)

# build dynamic library
$(SHARED_LIB): $(OBJ)
	$(CC) -shared -o $@ $(OBJ) $(LIBS)

# create objectfiles
%.o: %.c $(HDR)
	$(CC) -c $< -o $@ $(CFLAGS)

# build test program
$(TEST_APP): test/test.c $(SRC)
	$(CC) test/test.c $(SRC) -o $(TEST_BUILD) $(CFLAGS) $(LIBS)

# run test program
test: $(TEST_BUILD)
	./$(TEST_BUILD)

# running language logic in /po
po:
	$(MAKE) -C po

# install libs, headers, languages
install: lib
	mkdir -p $(LIBDIR) $(INCDIR)
	cp $(STATIC_LIB) $(SHARED_LIB) $(LIBDIR)/
	cp $(HDR) $(INCDIR)/
	$(MAKE) -C po install

# cleanup
clean:
	rm -f $(OBJ) $(STATIC_LIB) $(SHARED_LIB) $(TEST_APP)
	$(MAKE) -C po clean
