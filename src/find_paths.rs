use std::{ thread, process };
use std::path::{PathBuf, Path};
use std::sync::mpsc::channel;

use colored::*;
use regex::Regex;

use crate::languages::identify;
use crate::file_utils::{ walk_dirs, WalkDirsAction };
use crate::spinner::Spinner;

/// Finds all code projects in the given directory or directories
///
/// # Arguments
/// * `root_paths` - The path(s) to search
pub fn find(root_paths : Vec<PathBuf>, ignore : Option<Regex>) -> Vec<PathBuf> {
	// Create message channel for thread communication
	let (tx, rc) = channel();

	// Spawn controller thread
	thread::spawn(move || {
		let mut threads = Vec::new();

		// Create thread for each path
		for path in root_paths {
			// Each thread gets its own channel transmitter
			let tx_t = tx.clone();
			let ignore_t = ignore.clone();

			threads.push(thread::spawn(move || {
				// Walk all directories until finding a project directory
				return walk_dirs(&path, &|p| {
					let _ = tx_t.send(None);

					if is_ignored(&ignore_t, p) {
						return WalkDirsAction::Ignore;
					} else if identify(p).is_some() {
						return WalkDirsAction::Add;
					} else {
						return WalkDirsAction::AddAndRecurse;
					}
				});
			}));
		}

		// Wait for all threads to finish and combine the results
		let mut found_paths = Vec::new();
		let mut ignored_paths = Vec::new();

		for thread in threads {
			let (found, ignored) = &mut thread.join().unwrap();
			found_paths.append(found);
			ignored_paths.append(ignored);
		}

		// Send the results to the main thread
		let _ = tx.send(Some((found_paths.into_iter().filter(|p| identify(p).is_some()).collect::<Vec<PathBuf>>(), ignored_paths)));
	});

	println!("Searching for code projects");
	let mut spinner = Spinner::new("Searching directories...");
	let mut searched = 0;

	loop {
		// Wait for a message from the thread
		let data = rc.recv();

		// Handle errors
		if let Err(err) = data {
			println!("Error in thread: {}", err);
			process::exit(0);
		}

		// If the paths are loaded, return them
		let data = data.unwrap();
		if let Some((found, ignored)) = data {
			// Log the search stats
			spinner.finish(format!("Searched {} directories", searched).as_str());

			println!("  {} Found {} {}", "i".blue(), found.len(), plural(found.len(), "project", "projects"));

			if ignored.len() > 0 {
				println!("  {} Ignored {} {}", "i".blue(), ignored.len(), plural(ignored.len(), "directory", "directories"));
			}

			return found;
		}

		// If we're still going, display the progress
		searched += 1;
		spinner.update(format!("Searching {} directories", searched).as_str());
	}
}

/// Checks if a given path is ignored
///
/// # Arguments
/// * `ignore` - The ignore regex, if set
/// * `path`   - Path to check against the ignore regex
///
/// # Returns
/// * `true`  - If the path matches the regex
/// * `false` - If the regex and path don't match, if no ignore
///             regex was given, or if the path is empty
fn is_ignored(ignore : &Option<Regex>, path : &Path) -> bool {
	match ignore {
		None => false,
		Some(re) => {
			match path.to_str() {
				None => false,
				Some(path) => {
					if path.len() == 0 {
						false
					} else {
						re.is_match(path)
					}
				},
			}
		},
	}
}

/// Returns one of two values, depending on whether the count is 1 or not
///
/// # Arguments
/// * `count`    - The item count
/// * `singular` - Value when count == 1
/// * `plural`   - Value when count != 1
#[inline]
fn plural<T>(count : usize, singular : T, plural : T) -> T {
	match count {
		1 => singular,
		_ => plural,
	}
}
