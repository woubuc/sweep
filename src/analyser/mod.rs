mod project;

pub use analyser::project::Project;

use crawler;
use languages;
use file_utils;
use languages::Language;

use std;
use std::cmp;
use std::path::Path;

/// Analyses a project and returns a new Project struct containing the total
/// size and last modified time
pub fn analyse(p : crawler::Project) -> Project {
	let modified = last_modified(&p.language, &p.path);
	let size_src = total_size(&p.language, &p.path);
	let size_deps = languages::get_paths(&p.language).iter()
			.map(|path| total_size(&p.language, &p.path.join(path)))
			.fold(0, |acc, size| acc + size);

	return Project {
		name: p.name,
		path: p.path,
		language: p.language,
		modified,
		size_src,
		size_deps
	};
}

/// Calculates the total file size of a given directory
fn total_size(language : &Language, path : &Path) -> u64 {
	let mut size = 0;

	let paths = languages::get_paths(&language);

	if let Ok(contents) = path.read_dir() {
		for entry in contents.filter_map(|e| e.ok()) {
			if file_utils::is_file(&entry) {
				size += entry.metadata().unwrap().len();
			} else if !paths.contains(&file_utils::file_name(&entry).as_str()) {
				size += total_size(language, &entry.path());
			}
		}
	}

	return size;
}

/// Calculates the last modified date of a given directory
fn last_modified(language : &Language, path : &Path) -> u64 {
	let mut modified = std::u64::MAX;

	let paths = languages::get_paths(language);

	if let Ok(contents) = path.read_dir() {
		for entry in contents.filter_map(|e| e.ok()) {
			if file_utils::is_file(&entry) {
				modified = cmp::min(modified, file_utils::last_modified(&entry));
			} else if !paths.contains(&file_utils::file_name(&entry).as_str()) {
				modified = cmp::min(modified, last_modified(language, &entry.path()));
			}
		}
	}

	return modified;
}
