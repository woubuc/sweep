use std::fs::remove_dir_all;
use std::io::stdin;
use std::path::PathBuf;

use dunce::canonicalize;
use regex::Regex;
use structopt::StructOpt;

pub use crate::output::output;
pub use crate::project::Project;

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
	pub force : bool,
}

impl Settings {
	pub fn get() -> Settings {
		// Explicit declaration because type hinting in IDEA doesn't know `from_args`
		let mut settings : Settings = Settings::from_args();

		if settings.paths.is_empty(){
			settings.paths.push(".".into());
		}

		// Resolve to absolute paths
		settings.paths = settings.paths.iter()
								 .map(|p| canonicalize(p).expect("Cannot resolve to absolute path")) // TODO improve error handling
								 .collect();

		for path in &settings.paths {
			output().settings_path(&path);
		}

		return settings;
	}
}

fn main() {
	output().main_title();

	let settings = Settings::get();

	// Discover cleanable projects
	let cleanables = find::discover(&settings);

	if cleanables.len() == 0 {
		output().main_no_cleanable_projects();
		return;
	}

	// Figure out which directories can be deleted
	let delete_dirs = analyse::analyse(cleanables, &settings);

	if delete_dirs.len() == 0 {
		output().main_no_deletable_directories();
		return;
	}

	output().main_directories_list(&delete_dirs);
	if !settings.force {
		output().main_delete();

		loop {
			output().main_delete_question();

			let mut input = String::new();
			stdin().read_line(&mut input).unwrap();
			let input = input.trim();

			if input == "n" { return; }
			if input == "y" { break; }
			output().main_delete_invalid_answer();
		}
	}

	for dir in delete_dirs {
		output().delete_path(&dir);
		remove_dir_all(dir).expect("Could not remove directory"); // TODO better error handling
	}

	output().delete_complete();
}
