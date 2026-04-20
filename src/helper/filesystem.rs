//! Comprehensive file system management.
//!
//! This module provides tools for manipulating the directory structure. 
//! It includes functions for path expansion, for recursively creating or 
//! deleting directories, and for searching for specific files 
//! within the system.
 
/// Deleting entire directory trees.
pub mod delete_directory;

/// Selectively deleting files and their parent directories.
pub mod delete_files;

/// Validating the existence of files and directories.
pub mod directory_exists;

/// Expanding tildes (`~`) and environment variables into absolute paths.
pub mod expand_path;

/// Determining the user's home directory ($HOME).
pub mod home_dir;

/// Recursive creation of paths and directories.
pub mod make_path;

/// Search for substrings in filenames within a directory.
pub mod search_file_directory;

/// Specialized search for strings in filenames (e.g., for PPA checks).
pub mod search_string_name;

// Re-Export
pub use delete_directory::delete_directory;
pub use delete_files::*;
pub use directory_exists::{directory_exists, file_exists};
pub use expand_path::expand_path;
pub use home_dir::get_home_directory;
pub use make_path::*;
pub use search_file_directory::search_file_directory;
pub use search_string_name::search_string_file;
