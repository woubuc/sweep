use std::path::{PathBuf, Path};

use crossbeam::queue::SegQueue;

use crate::settings::Settings;
use crate::output::output;
use crate::file_utils::exists_in_path;

/// Finds all directories that can be cleaned, based on
pub fn find_dirs(cleanables : SegQueue<PathBuf>, settings : &Settings) -> SegQueue<PathBuf> {
	let mut dirs = SegQueue::new();

	while let Ok(path) = cleanables.pop() {

		/// Adds a subdirectory to the found dirs, if said subdirectory exists
		let try_add_dir = |dir : &str| {
			let mut p = path.clone();
			p.push(dir);

			if p.exists() {
				dirs.push(p);
			}
		};

		output().analyse_processing_path(&path);

		if exists_in_path(&path, "Cargo.toml") {
			try_add_dir("target");
		}

		if exists_in_path(&path, "package.json") {
			try_add_dir("node_modules");
			try_add_dir(".cache");
		}

		if exists_in_path(&path, "pom.xml") {
			try_add_dir(".gradle");
			try_add_dir("build");
		}
	}

	return dirs;
}
