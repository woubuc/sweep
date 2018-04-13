extern crate humansize;

mod out;
mod crawler;
mod analyser;
mod settings;
mod languages;
mod file_utils;

use analyser::analyse;
use settings::Settings;

use std::fs::remove_dir_all;
use std::io::{stdin, stdout, Write};
use humansize::{FileSize, file_size_opts as options};

fn main() {
	out::intro();

	// Parse CLI settings
	let settings = Settings::from_args(std::env::args());

	// Find all code projects
	let projects = crawler::crawl(settings.paths);

	if projects.len() == 0 {
		println!("No projects found");
		return;
	}

	// Load project data & calculate directory sizes
	if projects.len() == 1 { println!("Analysing 1 found project..."); }
	else { println!("Analysing {} found projects...", projects.len()); }

	let name_len = projects.iter().map(|p| p.name.len()).max().unwrap();

	let mut total_size_src = 0;
	let mut total_size_deps = 0;
	let mut total_size_deps_untouchable = 0;

	let mut project_stats = Vec::new();
	let mut to_remove = Vec::new();

	for project in projects {
		print!("   - {}{}   ", project.name, " ".repeat(name_len - project.name.len()));
		let _ = stdout().flush();

		let stats = analyse(project);
		total_size_src += stats.size_src;

		if stats.size_deps > 0 {
			if stats.modified > 2592000 || settings.all {
				total_size_deps += stats.size_deps;
				for path in languages::get_paths(&stats.language) {
					to_remove.push(stats.path.clone().join(path));
				}
			} else {
				total_size_deps_untouchable += stats.size_deps;
			}

			project_stats.push(stats);
		}

		println!("OK");
	}

	if to_remove.len() == 0 {
		println!("No projects have directories that can be removed");
		println!("   This is likely because your projects were recently modified");
		println!("   Run the application with the --all flag to disregard file age");
		println!("   See --help for all flags");
		return;
	}

	// Show stats
	println!("About to clear {} out of the total {} across found projects",
		format_size(total_size_deps),
		format_size(total_size_src + total_size_deps + total_size_deps_untouchable));

	if total_size_deps_untouchable > 0 {
		println!("   Cannot remove {} because associated projects were used recently",
			format_size(total_size_deps_untouchable));
	}

	// Ask the user to confirm before deleting
	if settings.force == false {
		if to_remove.len() == 1 { print!("Directory"); }
		else { print!("Directories"); }
		println!(" that will be removed:");

		for path in &to_remove {
			println!("   - {}", path.display());
		}

		print!("   Permanently delete ");
		if to_remove.len() == 1 { print!("this directory?"); }
		else { print!("these {} directories?", to_remove.len()); }
		print!(" (y/n) ");

		let _ = stdout().flush();

		let mut input = String::new();
		stdin().read_line(&mut input).unwrap();
		if !input.contains("y") { return; }
	}

	println!("Removing directories...");

	let dir_len = to_remove.iter().map(|d| d.to_str().unwrap().len()).max().unwrap();

	for dir in to_remove {
		print!("   - {}{}   ", dir.display(), " ".repeat(dir_len - dir.to_str().unwrap().len()));
		let _ = stdout().flush();

		if let Err(err) = remove_dir_all(dir) {
			println!("ERR");
			println!("{}", err);
		} else {
			println!("OK");
		}
	}

	println!("Done");
}

fn format_size(size : u64) -> String {
	return size.file_size(options::CONVENTIONAL).unwrap();
}
