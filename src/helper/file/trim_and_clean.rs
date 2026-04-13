/**
* trim_and_clean.rs
*
* (C) Copyright 2026 AtlantisOS Project
* by @NachtsternBuild
*
* License: GNU GENERAL PUBLIC LICENSE Version 3
*
* @brief Removes leading and trailing spaces/quotation marks from a string
* 
* Usage:
* fn main() {
*    let input = "  \"Hello AtlantisOS\"  ";
*    let cleaned = trim_and_clean(input);
*    println!("Input: |{}|", input);
*    println!("Output: |{}|", cleaned);
*}
*/

/**
 * @brief Removes leading and trailing spaces/quotation marks from a string
 */
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
