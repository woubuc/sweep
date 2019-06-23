use core::borrow::Borrow;
use std::fs::remove_dir_all;
use std::io::stdin;
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
