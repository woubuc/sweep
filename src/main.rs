use std::process;

use dunce::canonicalize;

pub use crate::output::output;
pub use crate::project::Project;
use crate::settings::Settings;

mod analyse;
mod find;
mod util;

mod output;
mod settings;
mod project;

//mod get_stats;
//mod languages;
//mod file_utils;
//mod find_paths;
//mod get_stats;
//mod filter_paths;
//mod remove_paths;
//mod spinner;

fn main() {

	output().main_title();

	// Parse CLI settings
	let mut settings = match Settings::from_args(std::env::args()) {
		settings::ParseResult::Ok(settings) => settings,
		settings::ParseResult::Done => return,
		settings::ParseResult::Errored => process::exit(1),
	};

	// Resolve to absolute paths - TODO move this into settings
	settings.paths = settings.paths.iter()
		.map(|p| canonicalize(p).expect("Cannot resolve to absolute path"))
		.collect();

	// Display resolved paths to user
	for path in &settings.paths {
		output().main_input_path(path);
	}

	// Discover cleanable directories
	let cleanables = find::discover(&settings);

	if cleanables.len() == 0 {
		output().main_no_cleanables_found();
		return;
	}

	analyse::analyse(cleanables, &settings);

	// Get stats for the discovered projects
//	let stats = get_stats::get(cleanables);

//	// Find the paths that should be removed
//	let (remove, remove_size) = filter_paths::filter(stats, settings.all);
//
//	// Verify paths to remove
//	println!("Ready to remove {} of unnecessary files", format_size(remove_size).cyan().bold());
//	println!("{}", "ALL CONTENTS OF THESE DIRECTORIES WILL BE DELETED".white().on_red().bold());
//	for path in &remove { println!("    {}", path.display()); }
//
//	if !settings.force {
//		loop {
//			print!("Do you want to continue? (y/n) ");
//			let _ = stdout().flush();
//
//			let mut input = String::new();
//			stdin().read_line(&mut input).unwrap();
//			let input = input.trim();
//
//			if input == "n" { return; }
//			if input == "y" { break; }
//			println!("  {}", "Please enter either 'y' or 'n'".yellow());
//		}
//	}
//
//	// Delete directories
//	remove_paths::remove(remove);
}
