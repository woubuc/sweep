use std::path::PathBuf;
use regex::Regex;
use std::io::{ stdin, stdout, Write };

use colored::*;
use structopt::StructOpt;

mod languages;
mod file_utils;
mod find_paths;
mod get_stats;
mod filter_paths;
mod remove_paths;
mod spinner;

use crate::get_stats::format_size;

fn main() {

	/// Clean up build artifacts and dependency directories in Rust, Java and NodeJS projects to free up disk space.  
	///
	/// Questions, bugs & other issues: github.com/woubuc/project-cleanup/issues
	#[derive(Debug,StructOpt)]
	pub struct Settings {
		/// Paths where potential cleanup candidates are located
		#[structopt(name = "PATH...", help = "One or more directories where the project-cleanup should start searching for projects. Defaults to current working directory")]
		pub paths : Vec<PathBuf>,
		/// Remove even if project was touched recently (within the last month).
		#[structopt(short = "a", long = "all", help = "Cleanup even recently used projects")]
		pub all : bool,
		/// Exclude projects in directories matched by this regex
		#[structopt(short = "i", long = "ignore", help = "Regex to specify project directories to be ignored")]
		pub ignore : Option<Regex>,
		/// Skip confirmation before removal
		#[structopt(short = "f", long = "force", help = "Skip confirmation prompt")]
		pub force : bool
	}

	// If on Windows, we need to enable the virtual terminal
	// to allow for proper colour support. Other platforms should
	// support ansi colouring without a problem.
	#[cfg(windows)]
	colored::control::set_virtual_terminal(true).expect("Could not initialise virtual terminal");

	// Parse CLI settings
	let mut settings = Settings::from_args();
	// Check if we need to include the working directory because no path was provided
	if settings.paths.is_empty(){
	settings.paths.push(".".into())
	}

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
