// carousel.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! Multi-page guides for GTK4 + Libadwaita.
//!
//! The module shows instructions, tours and documentation as a swipeable carousel with
//! back/next navigation. The content is *data*, not widget code, and can be built in two
//! ways:
//!
//! * **in code** with the builders [`Guide`], [`Page`], [`Callout`], [`Image`], …
//! * **from a TOML file** (feature `carousel-toml`, on by default) with [`GuideLoader`]
//!
//! The building blocks of a page are paragraphs, headings, bullet lists, numbered steps,
//! code blocks with a copy button, images (with caption and alternative text), notes
//! in four colors (info, tip, warning, danger) and buttons ([`ActionButton`]) that report a
//! click to the application, so a guide can also be used for a setup assistant.
//!
//! # Examples
//!
//! In code:
//!
//! ```no_run
//! use atlbase::ui_prelude::{Callout, Guide, GuideView, Page};
//! # gtk4::init().unwrap();
//! # let stack = gtk4::Stack::new();
//!
//! let guide = Guide::new("GSI").page(
//!     Page::new("Notes")
//!         .paragraph("GSI is a generic Android system image.")
//!         .callout(Callout::warning("Requires Project Treble."))
//!         .code("adb shell getprop ro.treble.enabled"),
//! );
//!
//! // show it in a GtkStack; "Exit" and "Done" switch back to the page "instruction_flash"
//! GuideView::present_in_stack(&stack, "instruction_gsi", &guide, "instruction_flash");
//! ```
//!
//! From a TOML file in a GResource of the application:
//!
//! ```no_run
//! use atlbase::ui_prelude::{GuideLoader, GuideView};
//! # gtk4::init().unwrap();
//! # let stack = gtk4::Stack::new();
//!
//! // texts are translated with gettext! by default
//! let loader = GuideLoader::new();
//! let guide = loader.load_resource("/org/example/App/guides/gsi.toml")?;
//! GuideView::present_in_stack(&stack, "instruction_gsi", &guide, "instruction_flash");
//! # Ok::<(), atlbase::ui_prelude::GuideError>(())
//! ```
//!
//! # Translations
//!
//! The texts of a guide are the `msgid`s. [`GuideLoader`] translates every visible text with
//! [`gettext!`](crate::gettext!); builders take the texts as you give them, so call
//! `gettext!` yourself and use [`Labels::localized`] for the navigation. `xgettext` does not
//! know TOML: collect the texts of the guide files with a small script or by hand into the
//! `.pot` file of your application.
//!
//! # Images and packaging
//!
//! Relative image paths of a TOML guide are looked up next to the guide, then in the
//! folders given with [`GuideLoader::image_data_subdir`] and [`GuideLoader::image_dir`]
//! (XDG data directories, `/app/share` in a Flatpak, `$SNAP/usr/share` in a Snap,
//! `$APPDIR/usr/share` in an AppImage). The most robust choice for every package format is a
//! GResource: put the images next to the TOML file in the resource and nothing has to be
//! installed beside the binary.
//!
//! # Style
//!
//! The default style is registered on first use, see [`GUIDE_CSS`]. Register a
//! [`use_adw_provider`](crate::design::theme::load_adw_provider::use_adw_provider)
//! stylesheet to override it.

mod model;
mod render;
mod style;
mod view;

#[cfg(feature = "carousel-toml")]
mod load;

pub use model::{
    ActionButton, Block, Callout, CalloutKind, Guide, Image, ImageSource, Labels, Page, Step,
};
pub use render::{block_widget, page_widget};
pub use style::GUIDE_CSS;
pub use view::GuideView;

#[cfg(feature = "carousel-toml")]
#[cfg_attr(docsrs, doc(cfg(feature = "carousel-toml")))]
pub use load::{GuideError, GuideLoader};
