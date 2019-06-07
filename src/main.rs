use std::io::{ stdin, stdout, Write };
use std::process;

use colored::*;

mod settings;
mod languages;
mod file_utils;
mod find_paths;
mod get_stats;
mod filter_paths;
mod remove_paths;
mod spinner;

use crate::settings::Settings;
use crate::get_stats::format_size;

fn main() {
	println!("{}", format!("Project Cleanup v{}", env!("CARGO_PKG_VERSION")).as_str().bold());

	// Parse CLI settings
	let settings = match Settings::from_args(std::env::args()) {
		settings::ParseResult::Ok(settings) => settings,
		settings::ParseResult::Done => return,
		settings::ParseResult::Errored => process::exit(1),
	};

	// Find the project paths
	let paths = find_paths::find(settings.paths, settings.ignore);

	// Get stats for the discovered projects
	let stats = get_stats::get(paths);

	// Find the paths that should be removed
	let (remove, remove_size) = filter_paths::filter(stats, settings.all);

	// Verify paths to remove
	println!("Ready to remove {} of unnecessary files", format_size(remove_size).cyan().bold());
	println!("{}", "ALL CONTENTS OF THESE DIRECTORIES WILL BE DELETED".white().on_red().bold());
	for path in &remove { println!("    {}", path.display()); }

	if !settings.force {
		loop {
			print!("Do you want to continue? (y/n) ");
			let _ = stdout().flush();

			let mut input = String::new();
			stdin().read_line(&mut input).unwrap();
			let input = input.trim();

			if input == "n" { return; }
			if input == "y" { break; }
			println!("  {}", "Please enter either 'y' or 'n'".yellow());
		}
	}

	// Delete directories
	remove_paths::remove(remove);
}
