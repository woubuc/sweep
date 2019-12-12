use std::fs::remove_dir_all;
use std::io::{ stdin, stdout, Write };

use yansi::{ Paint, Color };

use crate::analyse_projects::analyse_projects;
use crate::discover_projects::discover_projects;
use crate::settings::{ Settings, SettingsError };
use crate::project::Project;

mod cleanuprc;
mod output;
mod project;
mod settings;

mod analyse_projects;
mod discover_projects;
mod utils;

fn main() {
	if cfg!(windows) && !Paint::enable_windows_ascii() {
		Paint::disable();
	}

	println!("{} v{}", Paint::new("Project Cleanup").bold(), Paint::new(env!("CARGO_PKG_VERSION")).dimmed());

	let settings = match Settings::get() {
		Ok(settings) => settings,
		Err(err) => {
			match err {
				SettingsError::InvalidPath(path) => output::error(format!("Invalid path: {}", path.to_str().unwrap_or(""))),
			};

			return;
		}
	};

	for path in &settings.paths {
		output::println("Path", Color::Blue, path.to_str().unwrap_or(""));
	}

	// Discover cleanable projects
	let cleanables = match discover_projects(&settings) {
		Some(cleanables) => cleanables,
		None => {
			output::println_plain(Some(Color::Yellow), "No cleanable projects found");
			output::println_plain(None, "  Check your paths and try again.");
			output::println_plain(None, "  See `--help` for more options");
			return;
		},
	};

	output::println_info(format!("{} cleanable projects found", cleanables.len()));


	// Figure out which directories can be deleted
	let delete_dirs = analyse_projects(cleanables, &settings);

	if delete_dirs.len() == 0 {
		output::println_plain(Some(Color::Yellow), "No cleanable projects found");
		output::println_plain(None, "  This is likely because your projects were recently modified");
		output::println_plain(None, "  Run the application with `--all` to disregard file age");
		output::println_plain(None, "  See `--help` for more options");
		return;
	}


	let message = if delete_dirs.len() == 1 {
		format!("Found 1 directory that can be deleted:")
	} else {
		format!("Found {} directories that can be deleted:", delete_dirs.len())
	};

	output::println("Result", Color::Green, &message);
	for dir in &delete_dirs {
		output::println_info(dir.to_str().unwrap_or(""));
	}


	if !settings.force {
		println!(
			"{}{} {}",
			" ".repeat(output::LABEL_WIDTH - 8),
			Paint::white(" DANGER ").bold().bg(Color::Red),
			Paint::red("Above directories will be permanently deleted").bold()
		);

		loop {
			print!(
				"{} {} (y/n): ",
				" ".repeat(output::LABEL_WIDTH),
				Paint::new("Continue?").bold()
			);
			stdout().flush().unwrap();

			let mut input = String::new();
			stdin().read_line(&mut input).expect("Could not read CLI input");
			let input = input.trim();

			if input == "n" {
				return;
			}

			if input == "y" {
				break;
			}

			output::println_info("Please answer either 'y' or 'n'");
		}
	}

	for dir in delete_dirs {
		output::print("Deleting", Color::Cyan, dir.to_str().unwrap_or(""));
		match remove_dir_all(&dir) {
			Err(error) => {
				println!();
				output::error(format!("Could not delete directory {}", &dir.to_str().unwrap_or("")));
				output::println_info(error.to_string());
				return;
			},
			_ => (),
		}
	}

	output::println("Deleted", Color::Green, "All directories deleted");
}
