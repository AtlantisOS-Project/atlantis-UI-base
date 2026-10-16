// trim_and_clean.rs
//
// (C) Copyright 2026 AtlantisOS Project
// by @NachtsternBuild
//
// License: GNU GENERAL PUBLIC LICENSE Version 3

//! Cleans up strings.
//!
//! The module removes unnecessary spaces and enclosing quotation marks, which is useful
//! when parsing user input or configuration values.

/// Removes leading and trailing whitespace as well as enclosing double quotes.
///
/// 1. All whitespace at the beginning and end is removed.
/// 2. If the remaining text begins **and** ends with `"`, these two characters are removed.
///
/// The function works on string slices, so no memory is allocated.
///
/// # Examples
///
/// ```
/// use atlbase::prelude::trim_and_clean;
///
/// assert_eq!(trim_and_clean("  \"Hello AtlantisOS\"  "), "Hello AtlantisOS");
/// assert_eq!(trim_and_clean("  plain  "), "plain");
/// assert_eq!(trim_and_clean("\""), "\"");
/// ```
pub fn trim_and_clean(s: &str) -> &str {
    let trimmed = s.trim();

    if trimmed.len() >= 2 && trimmed.starts_with('"') && trimmed.ends_with('"') {
        &trimmed[1..trimmed.len() - 1]
    } else {
        trimmed
    }
}
