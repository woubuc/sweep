use languages::Language;
use std::path::Path;

/// Inspects a directory and determines whether it is a code project
///
/// # Arguments
/// * `entry` - The directory
///
/// # Returns
/// If the directory is identified as a code project, this function will return
/// the corresponding language, or None if the directory is not a code project
pub fn identify(path : &Path) -> Option<Language> {
	if exists(path, "package.json") { return Some(Language::NODE); }
	if exists(path, "Cargo.toml") { return Some(Language::RUST); }
	return None;
}

/// Checks if a file exists in a directory
fn exists(path : &Path, file : &str) -> bool {
	return path.join(file).exists();
}
