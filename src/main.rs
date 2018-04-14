extern crate humansize;
extern crate term_size;

mod out;
mod settings;
mod languages;
mod file_utils;
mod find_paths;
mod get_stats;

use languages::identify;
use get_stats::format_size;

//use analyser::analyse;
use settings::Settings;
use std::fs::remove_dir_all;
use std::io::{stdin, stdout, Write};

fn main() {
	println!("Project Cleanup v{}", env!("CARGO_PKG_VERSION"));

	// Parse CLI settings
	let settings = Settings::from_args(std::env::args());

	// Only hide the cursor after settings are loaded because if the help is
	// displayed it will exit immediately
	out::hide_cursor();

	// Find the project paths
	let paths = find_paths::find(settings.paths);

	// Get stats for the discovered projects
	let stats = get_stats::get(paths);

	let mut remove = Vec::new();
	let mut remove_size = 0;
	for (path, stats) in stats {
		if stats.modified > 2592000 || settings.all {
			if let Some(lang) = identify(&path) {
				remove_size += stats.size_deps;

				for lang_path in lang.get_paths() {
					let p = path.join(lang_path);
					if p.exists() { remove.push(p); }
				}
			}
		}
	}

	if remove.len() == 0 {
		println!("No projects have directories that can be removed");
		println!("  This is likely because your projects were recently modified");
		println!("  Run the application with `--all` to disregard file age");
		println!("  See --help for more options");
		out::show_cursor();
		return;
	}

	println!("Ready to remove {} of unnecessary files", format_size(remove_size));
	println!("Directories that will be removed:");
	for path in &remove {
		println!("  - {}", path.display());
	}
	println!("ALL CONTENTS OF THESE DIRECTORIES WILL BE DELETED.");
	print!("Do you want to continue? (y/n) ");

	out::show_cursor();
	let _ = stdout().flush();

	let mut input = String::new();
	stdin().read_line(&mut input).unwrap();
	if !input.starts_with("y") { return; }

	out::hide_cursor();
	println!("Deleting directories...");

	for path in remove {
		println!("  {}", path.display());
		if let Err(err) = remove_dir_all(path) {
			println!("ERR");
			println!("{}", err);
		}
	}

	println!("Done");
}
