use std::ffi::OsStr;
use std::path::Path;

/// Checks if a file exists in a path
///
/// # Arguments
/// `path`     - The directory
/// `filename` - Filename to look up in `path`
///
/// # Returns
/// True if `filename` exists in `path`
pub fn exists_in_path(path : &Path, filename : &str) -> bool {
	path.clone().join(filename).exists()
}

/// Gets the filename of a path
///
/// # Arguments
/// `path` - The path
///
/// # Returns
/// The filename (last part) of the path, or an empty string if
/// the path could not be retrieved for any reason
pub fn file_name(path : &Path) -> &str {
	path.file_name().unwrap_or_else(|| OsStr::new("")).to_str().unwrap_or("")
}
