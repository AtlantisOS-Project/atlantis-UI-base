//! macro for gettext
/**
* macros.rs
*
* (C) Copyright 2026 AtlantisOS Project
* by @NachtsternBuild
*
* License: GNU GENERAL PUBLIC LICENSE Version 3
*/
/// Define gettext Macro
/// ### Usage:
///
/// ```rust
/// gettext!("some text");
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
