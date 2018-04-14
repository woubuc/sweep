extern crate humansize;

mod settings;
mod languages;
mod file_utils;
mod find_paths;
mod get_stats;
mod filter_paths;
mod remove_paths;

use settings::Settings;
use get_stats::format_size;

use std::io::{stdin, stdout, Write};

fn main() {
	println!("Project Cleanup v{}", env!("CARGO_PKG_VERSION"));

	// Parse CLI settings
	let settings = Settings::from_args(std::env::args());

	// Find the project paths
	let paths = find_paths::find(settings.paths);

	// Get stats for the discovered projects
	let stats = get_stats::get(paths);

	// Find the paths that should be removed
	let (remove, remove_size) = filter_paths::filter(stats, settings.all);

	// Verify paths to remove
	println!("Ready to remove {} of unnecessary files", format_size(remove_size));
	println!("Directories that will be removed:");
	for path in &remove { println!("  - {}", path.display()); }

	if !settings.force {
		println!("ALL CONTENTS OF THESE DIRECTORIES WILL BE DELETED.");
		print!("Do you want to continue? (y/n) ");
		let _ = stdout().flush();

		let mut input = String::new();
		stdin().read_line(&mut input).unwrap();
		if !input.starts_with("y") { return; }
	}

	// Delete directories
	remove_paths::remove(remove);
	println!("Done");
}
