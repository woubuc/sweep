use languages::identify;
use file_utils::walk_dirs;

use std::thread;
use std::io::stdout;
use std::io::Write;
use std::process;
use std::path::PathBuf;
use std::sync::mpsc::channel;

/// Finds all code projects in the given directory or directories
///
/// # Arguments
/// * `root_paths` - The path(s) to search
pub fn find(root_paths : Vec<PathBuf>) -> Vec<PathBuf> {
	// Create message channel for thread communication
	let (tx, rc) = channel();

	// Spawn controller thread
	thread::spawn(move || {
		let mut threads = Vec::new();

		// Create thread for each path
		for path in root_paths {
			// Each thread gets its own channel transmitter
			let tx_t = tx.clone();

			threads.push(thread::spawn(move || {
				let mut found_paths = Vec::new();

				// Walk all directories until finding a project directory
				found_paths.append(&mut walk_dirs(&path, &|p| {
					let _ = tx_t.send(None);
					return !identify(p).is_some();
				}));

				// Return all found paths to the controller thread
				return found_paths;
			}));
		}

		// Wait for all threads to finish and combine the results
		let mut found_paths = Vec::new();
		for thread in threads {
			let data = &mut thread.join().unwrap();
			found_paths.append(data);
		}

		// Send the results to the main thread
		let _ = tx.send(Some(found_paths.into_iter().filter(|p| identify(p).is_some()).collect()));
	});

	println!("Searching for code projects");
	print!("  Searching 0 directories...");
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
		if let Some(res) = data {
			// Log the search stats
			let results : Vec<PathBuf> = res;
			println!("\r  Searched {} directories     ", searched);
			println!("  Found {} projects", results.len());

			return results;
		}

		// If we're still going, display the progress
		searched += 1;
		print!("\r  Searched {} directories", searched);
		stdout().flush();
	}
}
