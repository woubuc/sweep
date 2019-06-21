use regex::Regex;
use std::path::Path;

/// Checks if a given path is ignored
///
/// # Arguments
/// * `ignore` - The ignore regex, if set
/// * `path`   - Path to check against the ignore regex
///
/// # Returns
/// * `true`  - If the path matches the regex
/// * `false` - If the regex and path don't match, if no ignore
///             regex was given, or if the path is empty
pub fn is_ignored(ignore : &Option<Regex>, path : &Path) -> bool {
	match ignore {
		None => false,
		Some(re) => {
			let path = path.to_str().unwrap_or("");

			if path.len() == 0 {
				false
			} else {
				re.is_match(path)
			}
		},
	}
}
