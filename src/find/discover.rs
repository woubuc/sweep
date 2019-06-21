use std::path::{PathBuf, Path};
use std::sync::atomic::{ AtomicUsize, Ordering };
use std::thread;
use std::time::Duration;
use std::error::Error;

use crossbeam::queue::SegQueue;
use crossbeam::scope;

use crate::output::output;
use crate::settings::Settings;

use super::identify::is_cleanable;
use super::ignore::is_ignored;

/// How long a thread should sleep before retrying if the queue returned empty
const RETRY_SLEEP_DURATION : Duration = Duration::from_millis(50);

/// Number of times the queue may return empty before a thread is closed
const MAX_RETRIES : usize = 10;


/// Recursively walks the configured paths and discovers all cleanable directories
pub fn discover(settings : Settings) -> SegQueue<PathBuf> {
	let input_paths = settings.paths.len();

	// Queue of paths to process
	let paths = SegQueue::new();
	let total_paths = AtomicUsize::new(0);

	// Will contain all discovered cleanable directories
	let discovered = SegQueue::new();


	for path in &settings.paths {

		// If a configured path is cleanable, we don't need to discover it anymore
		if is_cleanable(&path) {
			discovered.push(path.to_path_buf());
		} else {
			discover_in_directory(path, &settings, &paths, &discovered);
		}
	}

	// If all configured paths were cleanable, cut things short
	// before creating any discovery threads
	if paths.len() == 0 {
		output().discover_searching_done(input_paths, discovered.len());
		return discovered;
	}

	// Thank god for crossbeam
	scope(|s| {
		for _ in 0..num_cpus::get() {
			s.spawn(|_| {
				// To keep track of how many times the queue came up empty
				let mut tries : usize = 0;

				loop {
					if let Ok(path) = paths.pop() {
						tries = 0;

						// Discover the path we just got from the queue
						output().discover_searching_path(&path);
						discover_in_directory(&path, &settings, &paths, &discovered);

						// Done with this path, continue on to the next
						total_paths.fetch_add(1, Ordering::SeqCst);
						continue;
					}

					// If no paths were in the queue, sleep the thread for a few
					// ms to give other threads time to push new paths to the queue
					thread::sleep(RETRY_SLEEP_DURATION);
					output().discover_searching_sleep(tries);

					// If there are still no paths in the queue after 10 tries,
					// return out of the closure to stop the thread
					tries += 1;
					if tries > MAX_RETRIES { return }
				}
			});
		}

	}).expect("A threading error occured");


	// The `scope` function will wait to join all created threads, so
	// once we're here all that's left to do is return the discovered
	// cleanable directories
	output().discover_searching_done(total_paths.into_inner(), discovered.len());
	return discovered;
}


/// Discovers the subdirectories of a given path
///
/// If a path is discovered as a cleanable directory, it's added to
/// the `discovered` queue. All other directories are added to the
/// `paths` queue to be discovered later.
fn discover_in_directory(path : &Path, settings : &Settings, paths : &SegQueue<PathBuf>, discovered : &SegQueue<PathBuf>) {

	// We can only read in directories
	if !path.is_dir() {
		return;
	}

	let read_dir = path.read_dir();
	if let Err(e) = read_dir {
		output().discover_searching_error(e.description(), &path);
		return;
	}

	// Go over all subdirectories in the given directory and check if they're cleanable
	for path in read_dir.unwrap()
		.filter_map(|e| e.ok())
		.map(|e| e.path())
		.filter(|p| !is_ignored(&settings.ignore, p))
		.filter(|p| p.is_dir()) {

		if is_cleanable(&path) {
			discovered.push(path.to_path_buf());
		} else {
			paths.push(path.to_path_buf());
		}
	}
}
