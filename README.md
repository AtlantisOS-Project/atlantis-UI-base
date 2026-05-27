# AtlBase

[![CodeQL](https://github.com/AtlantisOS-Project/atlantis-UI-base/actions/workflows/codeql.yml/badge.svg)](https://github.com/AtlantisOS-Project/atlantis-UI-base/actions/workflows/codeql.yml)

Modern GTK4 + Libadwaita foundation library for Rust applications.

AtlBase is a modular Rust framework designed to simplify the development of modern Linux desktop applications with GTK4 and Libadwaita. It provides reusable UI components, helper utilities, filesystem tools, configuration management, localization support, logging helpers, and system integration APIs.

# Features

- GTK4 (`>4.20`) + Libadwaita 1.8 integration
- Ready-to-use dialogs and UI widgets
- File and folder chooser utilities
- Journald logging helpers
- Command execution utilities
- TOML configuration management
- Filesystem helper functions
- Internationalization (gettext)
- Root command execution helpers (`pkexec`)
- Reusable UI widget generators
- Modular architecture

# Architecture Overview

AtlBase is divided into multiple high-level modules.

```text
atlbase
├── helper/          -> Helper for basic operations
├── design/          -> GTK4 + Libadwaita UI framework
├── macros.rs        -> Reusable macros
├── prelude          -> Logic helper exports
└── ui_prelude       -> UI helper exports
```

## Module Responsibilities

| Module | Description |
|---|---|
| `helper` | Helper functions and utilities |
| `design` | UI dialogs, widgets, themes and utility components |
| `macros` | Shared macros used across applications |
| `prelude` | Central import point for backend helpers |
| `ui_prelude` | Central import point for UI helpers |

# Installation

## Cargo.toml

Add AtlBase to your project:

```toml
[dependencies]
atlbase = "0.1"
```


# System Requirements

AtlBase depends on GTK4 and Libadwaita development libraries.

## Ubuntu / Debian

```bash
sudo apt install \
    libgtk-4-dev \
    libadwaita-1-dev \
    libvte-2.91-gtk4-dev \
    gettext
```

## Fedora

```bash
sudo dnf install \
    gtk4-devel \
    libadwaita-devel \
    vte291-gtk4-devel \
    gettext
```

## Arch Linux

```bash
sudo pacman -S \
    gtk4 \
    libadwaita \
    vte4 \
    gettext
```

# Using the Prelude Modules

AtlBase provides two convenience preludes.

## Backend Prelude

```rust
use atlbase::prelude::*;
```

This imports:

- Command helpers
- Configuration utilities
- Filesystem helpers
- File utilities
- Localization helpers
- Logging utilities

## UI Prelude

```rust
use atlbase::ui_prelude::*;
```

This imports:

- Dialog systems
- File choosers
- Theme helpers
- Widget creators
- Spinner dialogs
- Logging viewers


# Core Modules

# Helper Module

The `helper` module contains backend and system integration functionality.

## Commands

Utilities for executing and managing system commands.

### Available Features

- Run shell commands
- Capture command output
- Execute commands via `pkexec`
- Open terminal applications
- Open URLs in browser
- Detect command availability

## Configuration

TOML-based configuration helpers.

### Features

- Store structured settings
- Load configuration automatically
- Detect runtime environment
- Application environment helpers


## File Utilities

Helpers for reading, writing and modifying files.

### Features

- Write files
- Append lines
- Temporary application storage
- Clean and trim strings
- Read configuration values

## Filesystem Utilities

Filesystem abstraction helpers.

### Features

- Create directories
- Search files
- Expand paths (`~` support)
- Delete files and directories
- Home directory detection
- File existence checks

## Localization

gettext-based internationalization support.

### Features

- Translation initialization
- Language directory handling
- Runtime locale switching

## Logging

`journald` integration.

### Features

- Syslog initialization
- Journald log viewer
- Custom headerbars for logs
- Runtime debugging

# Design Module

The `design` module contains reusable GTK4 and Libadwaita UI components.

## Dialogs

Ready-to-use dialogs for modern applications.


### Available Dialogs

| Dialog | Description |
|---|---|
| Alert Dialog | Standard alert message |
| About Dialog | ADW application about window |
| Entry Dialog | Text input dialog |
| Spinner Dialog | Background task progress dialog |
| Error Dialog | Error presentation |
| Image Dialog | Image preview dialog |
| Custom Dialog | Fully custom content container |

## Widget Utilities

AtlBase contains reusable widget generators.

### Available Widgets

- Buttons with icons
- Icon-positioned buttons
- Entry widgets
- Password entry widgets
- Label + icon combinations

# Dependency Overview

| Dependency | Purpose |
|---|---|
| `gtk4` | GTK4 UI toolkit |
| `libadwaita` | GNOME/Adwaita components |
| `vte4` | Embedded terminal widget |
| `tokio` | Async runtime |
| `serde` | Serialization |
| `confy` | Configuration management |
| `gettext-rs` | Internationalization |
| `networkmanager` | Network integration |
| `syslog` | System logging |
| `walkdir` | Recursive filesystem traversal |

# Release Optimization

AtlBase is configured for highly optimized release builds.

```toml
[profile.release]
opt-level = "z"
lto = true
codegen-units = 1
panic = "abort"
strip = true
```

This configuration minimizes binary size while preserving performance.

# Building

## Debug Build

```bash
cargo build
```

## Release Build

```bash
cargo build --release
```


# Documentation

Generate local API documentation:

```bash
cargo doc --open
```

# Testing

Run the example application:

```bash
cargo run --example test_app
```

# License

GPL-3.0 License.

See the LICENSE file for more information.


# Repository

GitHub Repository:

`https://github.com/AtlantisOS-Project/atlantis-UI-base`

# Author

The AtlantisOS Project

Primary Developer: @NachtsternBuild