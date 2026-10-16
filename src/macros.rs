// macros.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! Macros of the library.
//!
//! Macros defined with `#[macro_export]` are always documented at the crate root; this
//! module only exists so that the macros have a place in the module overview.
//!
//! * [`gettext!`](crate::gettext!) — translates a string.

/// Translates a string with gettext.
///
/// The macro calls [`translate`](crate::helper::language::language::translate): the text is
/// looked up in the text domain set by
/// [`init_language`](crate::helper::language::language::init_language) first and then in the
/// domain of this library ([`LIB_DOMAIN`](crate::helper::language::language::LIB_DOMAIN)).
/// You do not have to pass a domain, and the crate that uses the macro does not need a
/// dependency on `gettext-rs`.
///
/// If no translation is found (or [`init_language`] was never called), the original string
/// is returned.
///
/// # Arguments
///
/// * `$msgid` - The text to translate; anything that implements `AsRef<str>`.
///
/// # Returns
///
/// The translated text as a `String`.
///
/// # Examples
///
/// ```
/// use atlbase::gettext;
///
/// // Without a translation catalog the text is returned unchanged.
/// let message = gettext!("Welcome to AtlantisOS");
/// assert_eq!(message, "Welcome to AtlantisOS");
/// ```
///
/// [`init_language`]: crate::helper::language::language::init_language
#[macro_export]
macro_rules! gettext {
    ($msgid:expr $(,)?) => {
        $crate::helper::language::language::translate($msgid)
    };
}
