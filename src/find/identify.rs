use std::path::Path;

use crate::file_utils::exists_in_path;

/// If any of these files exist in a directory, it is a project directory
const DETECT_FILES : [&'static str; 3] = [
//	".cleanup",     // Custom project-cleanup list
//	".gitignore",   // Generic Git repository - TODO add support for ignore files
	"Cargo.toml",   // Rust
	"package.json", // Node.js
	"pom.xml",      // Java
];

/// Checks if a given directory is cleanable
pub fn is_cleanable(path : &Path) -> bool {

	// Make sure this is a directory, normally this is checked by the caller
	// but you can never be too sure
	if !path.is_dir() {
		return false;
	}

	for filename in &DETECT_FILES {
		if exists_in_path(path, filename) {
			return true;
		}
	}

	return false;
}
