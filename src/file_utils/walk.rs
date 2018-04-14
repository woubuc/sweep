use std::path::{Path, PathBuf};

/// Finds all directories and subdirectories for a given path, unless stopped
///
/// # Arguments
/// * `dir`    - The root directory
/// * `on_dir` - Function that is called for every directory found, should
///              return `true` if walking should continue into this directory
///              or `false` if not
pub fn walk_dirs(dir : &Path, on_dir : &Fn(&Path) -> bool) -> Vec<PathBuf> {
	let mut paths = Vec::new();

	// Read directory contents
	if let Ok(contents) = dir.read_dir() {
		for entry in contents.filter_map(|e| e.ok()) {
			// Skip files and hidden directories
			if entry.path().is_file() { continue; }
			if entry.file_name().to_string_lossy().starts_with(".") { continue; }

			// Store directory
			paths.push(entry.path());

			// Recurse if wanted
			if on_dir(&entry.path()) {
				paths.append(&mut walk_dirs(&entry.path(), on_dir));
			}
		}
	}

	return paths;
}

/// Finds all files in a directory and its subdirectories, unless stopped
///
/// # Arguments
/// * `dir`    - The root directory
/// * `on_dir` - Function that is called for every directory found, should
///              return `true` if walking should continue into this directory
///              or `false` if not
pub fn walk_files(dir : &Path, on_dir : &Fn(&Path) -> bool) -> Vec<PathBuf> {
	let mut files = Vec::new();

	// Read directory contents
	if let Ok(contents) = dir.read_dir() {
		for entry in contents.filter_map(|e| e.ok()) {
			// Store file
			if entry.path().is_file() {
				files.push(entry.path());
				continue;
			}

			// Recurse if wanted
			if on_dir(&entry.path()) {
				files.append(&mut walk_files(&entry.path(), on_dir));
			}
		}
	}

	return files;
}
