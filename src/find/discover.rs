use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use std::time::Duration;

use crossbeam::queue::SegQueue;
use crossbeam::scope;

use crate::output;
use crate::Project;
use crate::settings::Settings;

use super::identify::identify_cleanable_project;
use super::ignore::is_ignored;

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
			discover_in_directory(path, &settings, &path_queue, &discovered);
		}
	}

	if path_queue.len() == 0 {
		output().discover_searching_done(total_paths.into_inner(), discovered.len());
		return discovered;
	}

	// Since walking the directories is highly IO-dependent and not CPU-heavy,
	// the total runtime can be heavily optimised by using several concurrent
	// threads to process the queue of directories.
	//
	// I've set it to twice the number of CPU cores, because adding more shows
	// diminishing returns on my quad-core (8 threads) machine with SSD and
	// doesn't seem to provide any noticeable speed-up based on some (limited
	// and very non-deterministic) tests. Real-world testing on a variety of
	// devices may give different results however, and this number may need to
	// be adjusted later on.
	let num_threads = num_cpus::get() * 2;

	// Scoped threads provided by crossbeam
	scope(|s| {
		for _ in 0..num_threads {
			s.spawn(|_| {

				// Since the queue may be empty at one point but new paths
				// could get added by another thread right after, every thread
				// should try the queue several times before terminating.
				const TIMEOUT_MS_BETWEEN_TRIES : u64 = 50;
				const MAX_TRIES : usize = 5;

				let mut tries : usize = 0;

				while tries < MAX_TRIES {

					// Try to get the next path from the queue and process it
					if let Ok(path) = path_queue.pop() {
						tries = 0;

						output().discover_searching_path(&path);
						discover_in_directory(&path, &settings, &path_queue, &discovered);

						total_paths.fetch_add(1, Ordering::SeqCst);
						continue;
					}


					thread::sleep(Duration::from_millis(TIMEOUT_MS_BETWEEN_TRIES));
					output().discover_searching_sleep(tries);

					tries += 1;
				}
			});
		}
	}).expect("A threading error occured");


	// Crossbeam's `scope` function will wait to join all created threads
	// so at this point all processing is done and we have our queue of
	// discovered cleanable directories
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
