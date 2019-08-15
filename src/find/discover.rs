use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

use crossbeam::queue::SegQueue;

use crate::output;
use crate::Project;
use crate::Settings;
use crate::util::process_queue;

use super::identify::identify_cleanable_project;
use super::ignore::is_ignored;
use std::cmp::max;

/// Recursively walks the configured paths and discovers all cleanable directories
///
/// In case there are several levels of subdirectories to walk, this function
/// will spawn several worker threads to improve processing throughput
///
/// # Arguments
/// `settings` - The application settings object
///
/// # Returns
/// A queue containing all discovered projects
pub fn discover(settings : &Settings) -> SegQueue<Project> {

	// Will contain a queue of paths that still need to be processed
	let path_queue = SegQueue::new();

	// Will contain all discovered cleanable directories
	let discovered = SegQueue::new();

	// Atomic counter for the total number of paths processed
	// Just for displaying this information in the output, it's
	// not used for anything else
	let total_paths = AtomicUsize::new(settings.paths.len());


	// Before starting, check if any of the configured paths are cleanable and
	// discover the first level of subdirectories, to ensure the paths queue
	// already contains several directories. If the paths queue has enough
	// paths queued for all threads to start working immediately, the work
	// will finish faster and there will be less risk of threads timing out
	// before all paths have been processed.
	//
	// If there was only one level to crawl, the queue will be empty after this
	// and the thread creation can be skipped entirely, improving performance
	// even more.
	for path in &settings.paths {
		if let Some(project) = identify_cleanable_project(&path) {
			discovered.push(project);
		} else {
			discover_in_directory(&path, &settings, &path_queue, &discovered);
		}
	}

	if path_queue.len() == 0 {
		output().discover_searching_done(total_paths.into_inner(), discovered.len());
		return discovered;
	}

	// I've set the number of threads to spawn to twice the number of CPU
	// cores available, but this is not based on any real insights, rather
	// this is an assumption of what might be a good balance between read
	// speed, disk usage and CPU usage. Real-world tests and experience may
	// give different results and the number of threads may need to be
	// adjusted later on.
	process_queue(
		max(8, num_cpus::get() * 2),
		&path_queue,
		|path| {
			output().discover_searching_path(&path);
			total_paths.fetch_add(1, Ordering::SeqCst);

			discover_in_directory(&path, &settings, &path_queue, &discovered);
		},
		|tries| output().discover_searching_retry(tries)
	);

	output().discover_searching_done(total_paths.into_inner(), discovered.len());
	return discovered;
}


/// Discovers the subdirectories of a given path
///
/// # Arguments
/// `path`       - Path to search
/// `settings`   - The application settings object
/// `path_queue` - Subdirectories that need to be discovered will be added to this queue
/// `discovered` - Identified cleanable projects will be added to this queue
fn discover_in_directory(path : &Path, settings : &Settings, path_queue : &SegQueue<PathBuf>, discovered : &SegQueue<Project>) {

	// We can only read in directories
	if !path.is_dir() {
		return;
	}

	let read_dir = match path.read_dir() {
		Err(e) => {
			output().discover_searching_error(&e.to_string(), &path);
			return;
		},

		Ok(entries) => entries
			.filter_map(|entry| entry.ok())
			.map(|entry| entry.path())
			.filter(|path| path.is_dir())
			.filter(|path| !is_ignored(&settings.ignore, path))
	};

	// Go over all subdirectories in the given directory and check if they're cleanable
	for path in read_dir {
		if let Some(project) = identify_cleanable_project(&path) {
			discovered.push(project);
		} else {
			path_queue.push(path);
		}
	}
}
