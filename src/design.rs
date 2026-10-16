// design.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! The visual half of AtlBase: everything with a user interface.
//!
//! The module bundles the components for the graphical user interface. They are built on
//! `gtk4` and `libadwaita` and give all AtlantisOS applications a modern and consistent
//! look. Most items are also available through the [`ui_prelude`](crate::ui_prelude).
//!
//! | Module | Purpose |
//! |---|---|
//! | [`carousel`] | Multi-page guides (instructions, tours) as a swipeable widget |
//! | [`chooser`] | File and folder dialogs |
//! | [`dialogs`] | Alert, entry, spinner, image and about dialogs |
//! | [`theme`] | CSS providers that follow the light/dark style |
//! | [`utils`] | Factories for entries, buttons and icon labels |

pub mod carousel;

pub mod chooser;

pub mod dialogs;

pub mod theme;

pub mod utils;

// Re-exports
pub use chooser::*;
pub use dialogs::*;
pub use theme::*;
pub use utils::*;
