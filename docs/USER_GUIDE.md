# User Manual

This manual explains how to integrate Atl UIBase into your GTK4/Libadwaita project.

## Installation

```bash
git clone https://github.com/AtlantisOS-Project/atlantis-UI-base.git
cd atlantis-UI-base
# compile UIBase
make
# run test app
./test_app
```

**Requirements:**

- `> GTK 4.12`
- `> Libadwaita 1.6`
- `> libvte-gtk4`
- `make`
- `GCC, ccache`
- `other basic tools`

- Install build and runtime tools with:
```bash
sudo apt install make gtk4 libadwaita-1-0 libsecret-1-0 vte-2.91-gtk4 gcc libgtk-4-dev desktop-file-utils make dpkg-dev debhelper ccache libsecret-1-dev build-essential libadwaita-1-dev rsync git curl vte2.91-gtk4-dev curl pkexec xdg-desktop-portal, xdg-desktop-portal-gtk, xdg-utils
``` 

## Project Integration

### With Makefile
- copy the following makefile as example
```makefile
# Dependencies.mk
# configs
BUILD_DIR := build
TMP_DIR := $(BUILD_DIR)/.external
# use SSH by default, switch to HTTPS with "make GIT_HTTPS=1"
GIT_HTTPS ?= 0
ifeq ($(GIT_HTTPS),1)
  UIBASE_URL := https://github.com/AtlantisOS-Project/atlantis-UI-base.git
  DEBBUILD_URL := https://github.com/AtlantisOS-Project/base-debian-build.git
else
  UIBASE_URL := git@github.com:AtlantisOS-Project/atlantis-UI-base.git
  DEBBUILD_URL := git@github.com:AtlantisOS-Project/base-debian-build.git
endif
# atl ui base configs
UIBASE_NAME := atlantis-UI-base
UIBASE_SUBFOLDER := src
# deb build base configs
DEBBUILD_NAME := base-debian-build
DEBBUILD_SUBFOLDER := src
DEBBUILD_CONFFOLDER := $(DEBBUILD_SUBFOLDER)/config
DEBBUILD_TARGET := deb
all: prepare clean-tmp
prepare:
	@echo "[INFO] Preparing external dependencies..."
	rm -rf $(TMP_DIR)
	mkdir -p $(TMP_DIR)
	@echo "[INFO] Cloning Atlantis UI Base..."
	git clone --depth=1 $(UIBASE_URL) $(TMP_DIR)/$(UIBASE_NAME)
	@echo "[INFO] Cloning Base Debian Build..."
	git clone --depth=1 $(DEBBUILD_URL) $(TMP_DIR)/$(DEBBUILD_NAME)
	# create target dirs
	mkdir -p $(BUILD_DIR)/uibase
	mkdir -p $(DEBBUILD_TARGET)
		
	# copy code and other files
	rsync -av --update --exclude '.git/' --exclude='*.md' --exclude='test/' --exclude='po/' --exclude='custom_css_adw.c' $(TMP_DIR)/$(UIBASE_NAME)/$(UIBASE_SUBFOLDER)/* $(BUILD_DIR)/
	rsync -av --update --exclude '.git/' --exclude='*.md' --exclude='*/config' --exclude='deb/config' $(TMP_DIR)/$(DEBBUILD_NAME)/$(DEBBUILD_SUBFOLDER)/* $(DEBBUILD_TARGET)
clean-tmp:
	rm -rf $(TMP_DIR)
.PHONY: all prepare clean-tmp
```

- **Notes:**
    * Exclude `test.c` and other code parts, like `custom_css_adw.c` → provide this in your code
    * include `language.mk` to your main makefile
    * create in your working directory folder `po/` and add the `LINGUAS` file
    * include this makefile as `Dependencies.mk`:
```makefile 
	include Dependencies.mk
	include language.mk
```


## Basic usage

```c
#include "atlbase.h"
```
- now use the functions/modules in your code

## Modules

- `design/` – UI components (buttons, labels, switches)
- `dialogs/` – Dialog system (spinners, progress bars, inputs)
- `helper/` – Helper functions
- `language/` – Localization system

## Module Usage
- [Developer Docs](https://github.com/AtlantisOS-Project/atlantis-UI-base/blob/main/docs/DEVELOPER_DOCS.md)
