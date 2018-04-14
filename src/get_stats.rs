use languages::*;
use file_utils::{fname, walk_files};

use humansize::{FileSize, file_size_opts as options};

use std::thread;
use std::process;
use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::collections::HashMap;

/// The file stats corresponding to a single project directory
#[derive(Debug)]
pub struct Stats {
	pub size_deps : u64,
	pub size_src : u64,
	pub modified : u64
}

/// The result set after analysing all projects
pub struct StatsResult {
	stats : HashMap<PathBuf, Stats>,
	langs : HashMap<Language, u32>,
	total_size_src : u64,
	total_size_deps_candelete : u64,
	total_size_deps_modified : u64
}

/// Collects stats and metadata for each discovered project
///
/// # Arguments
/// * `project_paths` - Paths to analyse
pub fn get(project_paths : Vec<PathBuf>) -> HashMap<PathBuf, Stats> {
	let (tx, rc) = channel();

	// Spawn thread to process the data so it doesn't block the main thread
	thread::spawn(move || {
		let mut stats = HashMap::new();

		let mut langs = HashMap::new();
		langs.insert(NODE, 0);
		langs.insert(RUST, 0);
		langs.insert(JAVA, 0);

		let mut total_size_src = 0;
		let mut total_size_deps_candelete = 0;
		let mut total_size_deps_modified = 0;

		for path in project_paths {
			// Send progress bar tick
			let _ = tx.send(None);

			// Identify the project language
			let lang = identify(&path).unwrap();

			// Keep track of the number of projects in each language
			if let Some(lang_counter) = langs.get_mut(&lang) {
				*lang_counter += 1;
			}

			// Get size of the source files
			let size_src = walk_files(&path, &|p| !lang.get_paths().contains(&fname(&&p)))
				.into_iter()
				.filter_map(|p| p.metadata().ok())
				.map(|d| d.len())
				.fold(0, |acc, i| acc + i);

			total_size_src += size_src;

			// Get size of the dependencies, build files, etc.
			let mut size_deps = 0;
			for p in lang.get_paths() {
				size_deps += walk_files(&path.join(p), &|_| true)
					.into_iter()
					.filter_map(|p| p.metadata().ok())
					.map(|d| d.len())
					.fold(0, |acc, i| acc + i);
			}

			// Get how long ago this project was modified
			let modified = walk_files(&path, &|p| !lang.get_paths().contains(&fname(&&p)))
				.into_iter()
				.filter_map(|p| p.metadata().ok())
				.filter_map(|d| d.modified().ok())
				.filter_map(|m| m.elapsed().ok())
				.map(|e| e.as_secs())
				.min().unwrap();

			// Count the total size (depending on modified date)
			if modified > 2592000 {
				total_size_deps_candelete += size_deps;
			} else {
				total_size_deps_modified += size_deps;
			}

			stats.insert(path.clone(), Stats {size_deps, size_src, modified});
		}

		// Send the results to the main thread
		let _ = tx.send(Some(StatsResult {stats, langs, total_size_src, total_size_deps_candelete, total_size_deps_modified}));
	});

	println!("Analysing projects...");
	let mut i = 0;

	loop {
		// Wait for a message from the thread
		let data = rc.recv();

		// Handle errors
		if let Err(err) = data {
			println!("Error in thread: {}", err);
			process::exit(0);
		}

		// If the stats are loaded, return them
		let data = data.unwrap();
		if let Some(res) = data {
			// Log the stats
			let results : StatsResult = res;
			println!("  Analysed {} projects", results.stats.len());

			for (lang, count) in results.langs {
				if count == 1 {
					println!("  - {} {} project", count, lang.name());
				} else if count > 1 {
					println!("  - {} {} projects", count, lang.name());
				}
			}

			println!("  {} of source code and project files", format_size(results.total_size_src));

			if results.total_size_deps_candelete > 0 || results.total_size_deps_modified > 0 {
				if results.total_size_deps_candelete == 0 {
					println!("  No dependencies & builds over 1 month old");
				} else {
					println!("  {} of dependencies & builds over 1 month old", format_size(results.total_size_deps_candelete));
				}

				if results.total_size_deps_modified == 0 {
					println!("  No recently used dependencies & builds");
				} else {
					println!("  {} of recently used dependencies & builds", format_size(results.total_size_deps_modified));
				}
			}

			return results.stats;
		}

		// If we're still going, display the progress
		i += 1;
		if i % 10 == 0 {
			println!("  Analysed {} projects", i);
		}
	}
}

/// Formats a size in bytes as a human-readable string
pub fn format_size(size : u64) -> String {
	return size.file_size(options::CONVENTIONAL).unwrap();
}
