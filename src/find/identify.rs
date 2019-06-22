use std::path::Path;

use crate::file_utils::exists_in_path;
use crate::Project;

/// Checks if a given directory is cleanable and identifies the
/// dependency subdirectories
///
/// # Arguments
/// `path` - The path to check
///
/// # Returns
/// The identified project, or None if the given path is not a project
pub fn identify_cleanable_project(path : &Path) -> Option<Project> {

	// A project can only be a directory
	if !path.is_dir() {
		return None;
	}

	let mut project = Project::new(path.clone());
	let mut found = false;

	// Rust projects
	if exists_in_path(path, "Cargo.toml") {
		found = true;
		project.try_add_dependency_dir("target");
	}

	// Node.js projects
	if exists_in_path(path, "package.json") {
		found = true;
		project.try_add_dependency_dir("node_modules");
		project.try_add_dependency_dir(".cache");
	}

	// Java projects
	if exists_in_path(path, "pom.xml") {
		found = true;
		project.try_add_dependency_dir(".gradle");
		project.try_add_dependency_dir("build");
	}
	
	if found {
		return Some(project);
	} else {
		return None;
	}
}
