# Makefile language
#
# (C) Copyright 2025 AtlantisOS Project
# by @NachtsternBuild
#
# License: GNU GENERAL PUBLIC LICENSE Version 3
#
# Usage:
# make
# make mo-all
# make init-po
# make update-po
# sudo make install
# make clean

# change the package name for your porgram
PACKAGE   = test
PODIR     = .
LOCALEDIR = $(PODIR)/locale
POT_FILE  = $(PODIR)/$(PACKAGE).pot

# example languages
LANGS = de en es pt ru fr
PO_FILES = $(addprefix $(PODIR)/,$(addsuffix .po,$(LANGS)))
MO_FILES = $(addprefix $(LOCALEDIR)/,$(addsuffix /LC_MESSAGES/$(PACKAGE).mo,$(LANGS)))

# use sources from the main project 
SRCS = $(shell find .. -name '*.c')
HEADERS = $(shell find .. -name '*.h')

.PHONY: all init-po update-po mo-all install clean

all: mo-all

# create .pot file
$(POT_FILE): $(SRCS) $(HEADERS)
	@mkdir -p $(PODIR)
	xgettext --keyword=_ --language=C --from-code=UTF-8 \
		--add-comments --sort-output \
		--output=$@ $(SRCS)

# init .po files
init-po: $(PO_FILES)

define INIT_PO
$(PODIR)/$1.po: $(POT_FILE)
	@if [ ! -f $$@ ]; then \
		echo "Init locale $1"; \
		msginit --no-translator --input=$(POT_FILE) --locale=$1 --output-file=$$@; \
	else \
		echo "$$@ already exists, skipped."; \
	fi
endef
$(foreach lang,$(LANGS),$(eval $(call INIT_PO,$(lang))))

# update .po files
update-po: $(POT_FILE)
	@for lang in $(LANGS); do \
		if [ -f $(PODIR)/$$lang.po ]; then \
			msgmerge --update --backup=none $(PODIR)/$$lang.po $(POT_FILE); \
			echo "Updated: $(PODIR)/$$lang.po"; \
		else \
			echo "$(PODIR)/$$lang.po missing, run make init-po first."; \
		fi; \
	done

# compile .po files to .mo files
$(LOCALEDIR)/%/LC_MESSAGES/$(PACKAGE).mo: $(PODIR)/%.po
	@mkdir -p $(dir $@)
	msgfmt $< -o $@

mo-all: $(MO_FILES)

# install mo files
install: mo-all
	@for lang in $(LANGS); do \
		dest=/usr/local/share/locale/$$lang/LC_MESSAGES/$(PACKAGE).mo; \
		echo "Installing $$lang -> $$dest"; \
		mkdir -p $$(dirname $$dest); \
		cp $(LOCALEDIR)/$$lang/LC_MESSAGES/$(PACKAGE).mo $$dest; \
	done

# cleanup
clean:
	rm -rf $(LOCALEDIR) $(POT_FILE)

