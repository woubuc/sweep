use std::path::Path;

/// Checks if a file exists in a path
pub fn exists_in_path(path : &Path, filename : &str) -> bool {
	path.clone().join(filename).exists()
}
