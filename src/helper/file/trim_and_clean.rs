//! Utility for cleaning up strings.
//!
//! This module provides functions to remove unnecessary spaces and 
//! enclosing quotation marks from strings, which is particularly important when parsing 
//! user input or configuration values.
/**
* trim_and_clean.rs
*
* (C) Copyright 2026 AtlantisOS Project
* by @NachtsternBuild
*
* License: GNU GENERAL PUBLIC LICENSE Version 3
*/

/// Removes leading and trailing whitespace as well as enclosing quotation marks.
///
/// The function operates efficiently on string slices (`&str`), so no 
/// new string allocations in memory are required.
///
/// # How it works
/// 1. First, all whitespace at the beginning and end is removed (`trim`).
/// 2. If the remaining text begins **and** ends with `"`, these 
///    two characters are removed.
///
/// # Arguments
/// * `s` - The string to be cleaned.
///
/// # Return Value
/// A cleaned string slice.
///
/// # Usage:
///
/// ```rust
/// fn main() {
///    let input = "  \"Hello AtlantisOS\"  ";
///    let cleaned = trim_and_clean(input);
///    println!("Input: |{}|", input);
///    println!("Output: |{}|", cleaned);
/// }
/// ```
pub fn trim_and_clean(s: &str) -> &str {
    // remove empty space
    let mut trimmed = s.trim();

    // check for '"'
    if trimmed.len() >= 2 && trimmed.starts_with('"') && trimmed.ends_with('"') {
        // remove 
        trimmed = &trimmed[1..trimmed.len() - 1];
    }

    trimmed
}
