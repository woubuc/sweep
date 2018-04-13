//! Utility functions for working with DirEntries

use std;
use std::fs::DirEntry;

/// Checks if a DirEntry is a file or a symlink
pub fn is_file(entry : &DirEntry) -> bool {
	if let Ok(file_type) = entry.file_type() {
		return file_type.is_file() || file_type.is_symlink();
	}
	return false;
}

/// Checks if a DirEntry is a dotfile or -directory
pub fn is_dotfile(entry : &DirEntry) -> bool {
	return file_name(entry).starts_with(".");
}

// Gets the filename of a DirEntry
pub fn file_name(entry : &DirEntry) -> String {
	if let Some(name) = entry.file_name().to_str() {
		return name.to_owned();
	}
	return "".to_owned();
}

// Gets the number of seconds since a DirEntry was last modified
pub fn last_modified(entry : &DirEntry) -> u64 {
	if let Ok(metadata) = entry.metadata() {
		if let Ok(modified) = metadata.modified() {
			if let Ok(elapsed) = modified.elapsed() {
				return elapsed.as_secs();
			}
		}
	}

	return std::u64::MAX;
}
