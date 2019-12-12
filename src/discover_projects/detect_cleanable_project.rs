use std::path::Path;

use crate::Project;
use crate::utils::file_utils::exists_in_path;

/// Checks if a given directory is cleanable and identifies the
/// dependency subdirectories
///
/// # Arguments
/// `path` - The path to check
///
/// # Returns
/// The identified project, or None if the given path is not a project
pub fn detect_cleanable_project(path : &Path) -> Option<Project> {

	// A project can only be a directory
	if !path.is_dir() {
		return None;
	}

	// Create an empty project so we can add cleanable directories to it
	let mut project = Project::new(path.clone());

	// This flag will keep track of whether we've found a project
	let mut is_project = false;


	// Rust projects
	if exists_in_path(path, "Cargo.toml") {
		is_project = true;
		project.add_cleanable_dir_if_exists("target");
	}


	// Node.js projects
	if exists_in_path(path, "package.json") {
		is_project = true;
		project.add_cleanable_dir_if_exists("node_modules");
		project.add_cleanable_dir_if_exists(".cache");
	}


	// Java projects
	if exists_in_path(path, "pom.xml") {
		is_project = true;
		project.add_cleanable_dir_if_exists(".gradle");
		project.add_cleanable_dir_if_exists("build");
	}



	if is_project {
		return Some(project);
	} else {
		return None;
	}
}



#[cfg(test)]
mod test {
	use super::detect_cleanable_project;
	use crate::utils::test_utils;

	#[test]
	fn empty_dir() {
		test_utils::with_temp_dir(|dir| {
			assert!(detect_cleanable_project(&dir).is_none(), "Project detected in empty directory");
		});
	}

	#[test]
	fn no_project() {
		test_utils::with_temp_dir(|dir| {
			test_utils::create_dir(dir, "not_a_project");
			test_utils::create_dir(dir, "another_test_directory");
			test_utils::create_file(dir, "no_project_here.txt");

			assert!(detect_cleanable_project(&dir).is_none(), "Project detected in unrelated directory");
		});
	}


	#[test]
	fn rust_project_empty() {
		test_utils::with_temp_dir(|dir| {
			test_utils::create_file(dir, "Cargo.toml");
			test_utils::create_dir(dir, "src");

			assert!(detect_cleanable_project(&dir).is_some(), "No project detected");
		});
	}

	#[test]
	fn rust_project() {
		test_utils::with_temp_dir(|dir| {
			test_utils::create_file(dir, "Cargo.toml");
			test_utils::create_dir(dir, "src");
			test_utils::create_dir(dir, "target");

			match detect_cleanable_project(&dir) {
				None => panic!("No project detected"),
				Some(project) => {
					assert!(project.is_cleanable_dir(&dir.join("target")));

					assert_eq!(project.into_cleanable_dirs().len(), 1);
				}
			}
		});
	}


	#[test]
	fn nodejs_project_empty() {
		test_utils::with_temp_dir(|dir| {
			test_utils::create_file(dir, "package.json");
			test_utils::create_dir(dir, "src");

			assert!(detect_cleanable_project(&dir).is_some(), "No project detected");
		});
	}

	#[test]
	fn nodejs_project() {
		test_utils::with_temp_dir(|dir| {
			test_utils::create_file(dir, "package.json");
			test_utils::create_dir(dir, "src");
			test_utils::create_dir(dir, "node_modules");
			test_utils::create_dir(dir, ".cache");
			test_utils::create_dir(dir, ".idea");

			match detect_cleanable_project(&dir) {
				None => panic!("No project detected"),
				Some(project) => {
					assert!(project.is_cleanable_dir(&dir.join("node_modules")));
					assert!(project.is_cleanable_dir(&dir.join(".cache")));

					assert_eq!(project.into_cleanable_dirs().len(), 2);
				}
			}
		});
	}


	#[test]
	fn java_project_empty() {
		test_utils::with_temp_dir(|dir| {
			test_utils::create_file(dir, "pom.xml");
			test_utils::create_dir(dir, "src");

			assert!(detect_cleanable_project(&dir).is_some(), "No project detected");
		});
	}

	#[test]
	fn java_project() {
		test_utils::with_temp_dir(|dir| {
			test_utils::create_file(dir, "pom.xml");
			test_utils::create_dir(dir, "src");
			test_utils::create_dir(dir, "build");

			match detect_cleanable_project(&dir) {
				None => panic!("No project detected"),
				Some(project) => {
					assert!(project.is_cleanable_dir(&dir.join("build")));

					assert_eq!(project.into_cleanable_dirs().len(), 1);
				}
			}
		});
	}

}
