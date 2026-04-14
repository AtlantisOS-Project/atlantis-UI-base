/**
* macros.rs
*
* (C) Copyright 2026 AtlantisOS Project
* by @NachtsternBuild
*
* License: GNU GENERAL PUBLIC LICENSE Version 3
*
* Usage:
* gettext!("some text");
*/

// macro for gettext
#[macro_export]
macro_rules! gettext {
    ($msgid:expr) => {
        gettextrs::dgettext($crate::helper::language::language::LIB_DOMAIN, $msgid)
    };
}
