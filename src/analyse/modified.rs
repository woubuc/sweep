use std::cmp::max;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

use crossbeam::queue::SegQueue;

use crate::output;
use crate::Project;
use crate::util::file_utils::file_name;
use crate::util::process_queue;

const ALWAYS_IGNORE_DIRS : [&'static str; 3] = [
	".idea",
	".vscode",
	".git",
];

pub fn filter_by_modified(projects : SegQueue<Project>) -> SegQueue<Project> {

	let old_projects = SegQueue::new();
	let recent_projects = AtomicUsize::new(0);

	process_queue(max(2, num_cpus::get()),
		&projects,
		|project| {
			let paths = SegQueue::new();
			let modified = SegQueue::new();

			find_modified_date(&project, project.root(), &paths, &modified);

			process_queue(max(8, num_cpus::get() * 2),
				&paths,
				|path| {
					output().analyse_filter_by_modified_path(&path);
					find_modified_date(&project, &path, &paths, &modified);
				},
				|_| ()
			);

			let time_since_last_modified = {
				let mut vec = Vec::with_capacity(modified.len());
				while let Ok(m) = modified.pop() {
					if let Some(m) = m {
						vec.push(m);
					}
				}

				vec.into_iter().min().unwrap_or(0)
			};

			// Count the total size (depending on modified date)
			if time_since_last_modified > 2_592_000 {
				old_projects.push(project);
			} else {
				recent_projects.fetch_add(1, Ordering::SeqCst);
			}
		},
		|tries| output().analyse_filter_by_modified_retry(tries)
	);

	output().analyse_filter_by_modified_done(old_projects.len(), recent_projects.into_inner());

	return old_projects;
}

fn find_modified_date(project : &Project, path : &Path, paths : &SegQueue<PathBuf>, modified : &SegQueue<Option<u64>>) {

	let (dirs, files) = {
		let mut dirs = Vec::new();
		let mut files = Vec::new();

		let read_dir = path.read_dir().expect("Could not read directory"); // TODO add better error handling
		for entry in read_dir.filter_map(|e| e.ok()) {
			if entry.file_type().unwrap().is_dir() {
				dirs.push(entry.path());
			} else {
				files.push(entry);
			}
		}
		(dirs, files)
	};

	modified.push(files.iter()
		.filter_map(|p| p.metadata().ok())
		.filter_map(|d| d.modified().ok())
		.filter_map(|m| m.elapsed().ok())
		.map(|e| e.as_secs())
		.min());

	for dir in dirs {
		if ALWAYS_IGNORE_DIRS.contains(&file_name(&dir)) {
			continue;
		}

		if project.is_dependency_dir(&dir) {
			continue;
		}

		paths.push(dir);
	}
}
