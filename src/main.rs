use core::borrow::Borrow;
use std::fs::remove_dir_all;
use std::io::stdin;
use std::process;

use dunce::canonicalize;
use structopt::StructOpt;

pub use crate::output::output;
pub use crate::project::Project;
use std::path::PathBuf;
use regex::Regex;

mod analyse;
mod find;
mod util;

mod output;
mod project;

/// Clean up build artifacts and dependency directories in Rust, Java and NodeJS projects to free up disk space.
///
/// Questions, bugs & other issues: github.com/woubuc/project-cleanup/issues
#[derive(Debug,StructOpt)]
pub struct Settings {
	/// One or more directories where the project-cleanup should start searching for projects.
	/// Defaults to the current working directory if no paths are given.
	#[structopt(name = "PATH...")]
	pub paths : Vec<PathBuf>,

	/// Cleanup even projects that were modified within the last 30 days.
	#[structopt(short = "a", long = "all")]
	pub all : bool,

	/// Exclude projects in directories matched by this regex pattern
	#[structopt(short = "i", long = "ignore")]
	pub ignore : Option<Regex>,

	/// Skip confirmation prompt before removing directories
	#[structopt(short = "f", long = "force")]
	pub force : bool
}

fn main() {

	output().main_title();

	// Parse CLI settings
	let mut settings = Settings::from_args();

	// Check if we need to include the working directory because no path was provided
	if settings.paths.is_empty(){
		settings.paths.push(".".into());
	}

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

	let delete_dirs = analyse::analyse(cleanables, &settings);

	if delete_dirs.len() == 0 {
		return;
	}

	output().main_delete_dirs_identified(&delete_dirs);
	if !settings.force {
		output().main_question();

		loop {
			output().main_question_continue();

			let mut input = String::new();
			stdin().read_line(&mut input).unwrap();
			let input = input.trim();

			if input == "n" { return; }
			if input == "y" { break; }
			output().main_question_illegal_answer();
		}
	}

	// TODO multithread this
	for dir in delete_dirs {
		output().delete_path(&dir);
		remove_dir_all(dir);
	}

	output().delete_complete();
}
