use std::process;
use std::path::PathBuf;
use std::collections::HashMap;

use colored::*;

use crate::get_stats::Stats;
use crate::languages::identify;

/// Filters the given projects to determine which directories should be removed
///
/// # Arguments
/// * `projects` - Hash map of the projects and their collected stats
/// * `all`      - True to ignore the modified date
///
/// # Returns
/// The directories to remove, and the total size that will be fred if all
/// paths are removed
pub fn filter(projects : HashMap<PathBuf, Stats>, all : bool) -> (Vec<PathBuf>, u64) {
	let mut remove = Vec::new();
	let mut remove_size = 0;

	for (path, stats) in projects {
		// If a project is older than a month its dependencies should be removed
		if stats.modified > 2592000 || all {
			if let Some(lang) = identify(&path) {
				remove_size += stats.size_deps;

				// Each language has different paths that should be removed
				for lang_path in lang.get_paths() {
					let p = path.join(lang_path);
					if p.exists() { remove.push(p); }
				}
			}
		}
	}

	// If there are no projects that should actually be removed, just stop here
	if remove.len() == 0 {
		println!("{}", "No projects have directories that can be removed".yellow());
		println!("  This is likely because your projects were recently modified");
		println!("  Run the application with `{}` to disregard file age", "--all".bold());
		println!("  Try `{}` for more options", "--help".bold());
		process::exit(0);
	}

	return (remove, remove_size);
}
