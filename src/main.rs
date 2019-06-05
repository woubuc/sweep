use structopt::StructOpt;
use std::io::{ stdin, stdout, Write };
use colored::*;

use std::path::{ PathBuf };

mod languages;
mod file_utils;
mod find_paths;
mod get_stats;
mod filter_paths;
mod remove_paths;
mod spinner;

use crate::get_stats::format_size;

/// Contains the user-configurable settings for the application
#[derive(Debug,StructOpt)]
pub struct Settings {
	/// Paths where potential cleanup candidates are located
	#[structopt(name = "PATH...", help = "One or more directories where the utility should start searching for projects. If omitted, will use the current working directory")]
	pub paths : Vec<PathBuf>,

	/// If true, remove even in recently used projects
	#[structopt(short = "a", long = "all", help = "Remove directories even in recently used projects")]
	pub all : bool,

	/// If true, skip confirmation before removal
	#[structopt(short = "f", long = "force", help = "Remove directories even in recently used projects")]
	pub force : bool
}

fn main() {
	println!("{}", format!("Project Cleanup v{}", env!("CARGO_PKG_VERSION")).as_str().bold());

	// Parse CLI settings
	let mut settings = Settings::from_args();
	// Check if we need to include the working directory because no path was provided
	if settings.paths.is_empty(){
		settings.paths.push(".".into())
	}


	// Find the project paths
	let paths = find_paths::find(settings.paths);

	// Get stats for the discovered projects
	let stats = get_stats::get(paths);

	// Find the paths that should be removed
	let (remove, remove_size) = filter_paths::filter(stats, settings.all);

	// Verify paths to remove
	println!("Ready to remove {} of unnecessary files", format_size(remove_size).cyan().bold());
	println!("{}", "ALL CONTENTS OF THESE DIRECTORIES WILL BE DELETED".white().on_red().bold());
	for path in &remove { println!("    {}", path.display()); }

	if !settings.force {
		print!("Do you want to continue? (y/n) ");
		let _ = stdout().flush();

		let mut input = String::new();
		stdin().read_line(&mut input).unwrap();
		if !input.starts_with("y") { return; }
	}

	// Delete directories
	remove_paths::remove(remove);
}
