//! Global macros for the AtlantisOS project.
//!
//! This module contains useful macros that simplify complex function calls 
//! and improve code readability throughout the project.
/// Macro for easily translating strings.
///
/// This macro wraps the call to `dgettext`. It automatically uses the 
/// text domain currently specified in the language settings. This eliminates the need 
/// to manually pass the domain for each individual string.
///
/// # How it works
/// The macro calls `get_current_domain()` and passes this, along with the 
/// message, to the `gettextrs` library.
///
/// # Arguments
/// * `$msgid` - The string to be translated (string literal).
///
/// # Return Value
/// The translated string as a `String`. If no translation is found, 
/// the original string is returned.
///
/// # Usage:
///
/// ```rust
/// use crate::gettext;
///
/// fn main() {
///     let message = gettext!("Welcome to AtlantisOS");
///     println!("{}", message);
/// }
/// ```
#[macro_export]
macro_rules! gettext {
    ($msgid:expr) => {
        gettextrs::dgettext(
            &$crate::helper::language::language::get_current_domain(), 
            $msgid
        )
    };
}
