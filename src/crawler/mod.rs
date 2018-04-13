mod project;
mod identify;

pub use crawler::project::Project;

use crawler::identify::identify;
use file_utils::*;

use std::io::{stdout, Write};
use std::path::PathBuf;

/// Recursively searches the given directories for Node projects
///
/// # Arguments
/// * `paths` - One or more directories to search
///
/// # Returns
/// When all subdirectories have been searched, will return a Vec containing
/// all discovered projects
pub fn crawl(paths : Vec<PathBuf>) -> Vec<Project> {
	let mut projects = Vec::new();

	println!("Searching projects...");

	let path_len = paths.iter().map(|d| d.to_str().unwrap().len()).max().unwrap();

	// Search the directories
	for path in paths {
		print!("   - {}{}   ", path.display(), " ".repeat(path_len - path.to_str().unwrap().len()));
		let _ = stdout().flush();
		projects.append(&mut read_dir(path));
		println!("OK");
	}

	// Sort the projects by name
	projects.sort_unstable_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

	return projects;
}

/// Reads the given directory to discover if it is a code project
///
/// If the directory is not a code project, this function will recursively
/// search all subdirectories for projects. When the function finds a project
/// directory, it will add the path to the projects vector.
///
/// # Arguments
/// * `dir` - The directory to search
fn read_dir (dir : PathBuf) -> Vec<Project> {
	let mut projects = Vec::new();

	if let Ok(contents) = dir.read_dir() {
		for entry in contents.filter_map(|e| e.ok()) {
			// We only want directories so skip all files
			if is_file(&entry) { continue; }

			// Skip hidden files and directories
			if is_dotfile(&entry) { continue; }

			// Determine if it's a project directory
			if let Some(language) = identify(&entry.path()) {
				let name = entry.file_name().to_str().unwrap().to_owned();
				projects.push(Project {name, path: entry.path().to_path_buf(), language});
				continue;
			}

			// If it's not a project directory, the search continues
			projects.append(&mut read_dir(entry.path()));
		}
	}

	return projects;
}
