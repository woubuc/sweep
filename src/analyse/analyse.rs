use std::path::PathBuf;

use crossbeam::queue::SegQueue;

use crate::output::output;
use crate::Project;
use crate::Settings;

use super::modified::filter_by_modified;

pub fn analyse(projects : SegQueue<Project>, settings : &Settings) -> Vec<PathBuf> {

	let filtered = if settings.all {
		output().analyse_filter_by_modified_skip();
		projects
	} else {
		filter_by_modified(projects)
	};

	if filtered.len() == 0 {
		return Vec::new();
	}

	let mut dirs = Vec::new();
	while let Ok(project) = filtered.pop() {
		dirs.append(&mut project.into_dependency_dirs());
	}

	dirs.sort();
	return dirs;
}
