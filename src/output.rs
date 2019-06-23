//! Contains text formatting functions and output helpers to avoid having this formatting
//! throughout all of the code

use std::path::Path;

use crossterm::terminal;
use colored::*; // TODO replace colored with crossterm colours
use lazy_static::lazy_static;
use std::io::{stdout, Write};

lazy_static! {
	static ref OUTPUT_MANAGER : OutputManager = OutputManager::create();
}

pub fn output() -> &'static OutputManager { &OUTPUT_MANAGER }

const LABEL_WIDTH : usize = 12;

pub struct OutputManager {
	colours : bool,
	term_width : usize
}

// TODO find a better solution for lacking colour support than having every output line twice in here

impl OutputManager {
	fn create() -> OutputManager {
		OutputManager {
			colours: try_init_colours(),
			term_width: get_term_width(),
		}
	}

	pub fn main_title(&self) {
		if self.colours {
			println!("{}", format!("Project Cleanup v{}", env!("CARGO_PKG_VERSION")).as_str().bold());
		} else {
			println!("Project Cleanup v{}", env!("CARGO_PKG_VERSION"));
		}
	}

	pub fn main_input_path(&self, path : &Path) {
		if self.colours {
			self.println("Path".blue(), path.to_str().unwrap_or(""));
		} else {
			self.println("Path", path.to_str().unwrap_or(""));
		}
	}

	pub fn main_no_cleanables_found(&self) {
		if self.colours {
			println!("{}", "No cleanable directories found".yellow());
			println!("  Check your paths and try again.");
			println!("  See `{}` for more options", "--help".bold());
		} else {
			println!("No cleanable directories found");
			println!("  Check your paths and try again.");
			println!("  See `--help` for more options");
		}
	}

	pub fn discover_searching_path(&self, path : &Path) {
		if self.colours {
			self.print("Searching".cyan().bold(), path.to_str().unwrap_or(""));
		} else {
			self.print("Searching", path.to_str().unwrap_or(""));
		}
	}

	pub fn discover_searching_retry(&self, tries : usize) {
		if self.colours {
			self.print("Searching".cyan().bold(), ".".repeat(tries));
		} else {
			self.print("Searching", ".".repeat(tries));
		}
	}

	pub fn discover_searching_error(&self, error : &str, path : &Path) {
		if self.colours {
			self.println("Error".red().bold(), error);
		} else {
			self.println("Error", error);
		}

		self.println_plain(path.to_str().unwrap_or(""));
	}

	pub fn discover_searching_done(&self, total_paths : usize, discovered : usize) {
		if self.colours {
			self.println("Searched".green().bold(), format!("{} directories searched", total_paths));
		} else {
			self.println("Searched", format!("{} directories searched", total_paths));
		}

		self.println_plain(format!("{} cleanable projects found", discovered));
	}

	pub fn analyse_filter_by_modified_skip(&self) {
		if self.colours {
			self.print("Skip".yellow().bold(), "--all flag set, skipping analysis");
		} else {
			self.print("Skip", "--all flag set, skipping analysis");
		}
	}

	pub fn analyse_filter_by_modified_path(&self, path : &Path) {
		if self.colours {
			self.print("Analysing".cyan().bold(), path.to_str().unwrap_or(""));
		} else {
			self.print("Analysing", path.to_str().unwrap_or(""));
		}
	}

	pub fn analyse_filter_by_modified_retry(&self, tries : usize) {
		if self.colours {
			self.print("Analysing".cyan().bold(), ".".repeat(tries));
		} else {
			self.print("Analysing", ".".repeat(tries));
		}
	}

	pub fn analyse_filter_by_modified_done(&self, old_projects : usize, recent_projects : usize) {
		if self.colours {
			if recent_projects == 0 {
				self.println("Analysed".green().bold(), "All projects can be cleaned");
			} else if old_projects == 0 {
				self.println("Analysed".green().bold(), "All projects have been modified recently");
			} else {
				self.println("Analysed".green().bold(), format!("{} of {} projects can be cleaned", old_projects, old_projects + recent_projects));
				self.println_plain(format!("{} projects have been modified recently", recent_projects));
			}
		} else {
			if recent_projects == 0 {
				self.println("Analysed", "All projects can be cleaned");
			} else if old_projects == 0 {
				self.println("Analysed", "All projects have been modified recently");
			} else {
				self.println("Analysed", format!("{} of {} projects can be cleaned", old_projects, old_projects + recent_projects));
				self.println_plain(format!("{} projects have been modified recently", recent_projects));
			}
		}
	}

	pub fn analyse_no_old_cleanables(&self) {
		if self.colours {
			println!("{}", "No cleanable directories found".yellow());
			println!("  This is likely because your projects were recently modified");
			println!("  Run the application with `{}` to disregard file age", "--all".bold());
			println!("  Try `{}` for more options", "--help".bold());
		} else {
			println!("No cleanable directories found");
			println!("  This is likely because your projects were recently modified");
			println!("  Run the application with `--all` to disregard file age");
			println!("  Try `--help` for more options");
		}
	}

	pub fn analyse_processing_done(&self, discovered : usize) {
		if self.colours {
			self.println("Analysed".green().bold(), format!("{} unnecessary directories found in projects", discovered));
		} else {
			self.println("Analysed", format!("{} unnecessary directories found in projects", discovered));
		}
	}


	fn print<L : Into<ColoredString>, S : Into<String>>(&self, label : L, message : S) {
		let message = self.shorten(message.into());
		let label = label.into();

		print!("{}{} {}{}\r",
				 " ".repeat(LABEL_WIDTH - label.len()),
				 label,
				 message,
				" ".repeat(self.term_width - LABEL_WIDTH - 1 - message.len())
		);

		let _ = stdout().flush();
	}

	fn println<L : Into<ColoredString>,S : Into<String>>(&self, label : L, message : S) {
		let message = self.shorten(message.into());
		let label = label.into();

		println!("{}{} {}{}",
			   " ".repeat(LABEL_WIDTH - label.len()),
			   label,
			   message,
			   " ".repeat(self.term_width - LABEL_WIDTH - 1 - message.len())
		);
	}

	fn println_plain<S : Into<String>>(&self, message : S) {
		let message = self.shorten(message.into());

		println!("{} {}{}",
				 " ".repeat(LABEL_WIDTH),
				 message,
				 " ".repeat(self.term_width - LABEL_WIDTH - 1 - message.len())
		);
	}

	/// Shortens a message by omitting the middle part and replacing it with '...'
	///
	/// If the given message is shorter than the available width, the
	/// original message will be returned
	fn shorten(&self, message : String) -> String {
		let max_width = self.term_width - LABEL_WIDTH - 1;
		let len = message.len();

		if len <= max_width {
			return message;
		}

		let break_index = max_width / 4;

		return [
			message.chars().take(break_index).collect(),
			"...".to_owned(),
			message.chars().skip(len - max_width + break_index + 3).collect()
		].join("");
	}
}

/// Gets the width of the current terminal
fn get_term_width() -> usize {
	let (term_width, _) = terminal().terminal_size();
	return term_width as usize;
}


#[cfg(windows)]
fn try_init_colours() -> bool {
	// On Windows, we need to enable the virtual terminal
	// to allow for proper colour support. Other platforms
	// should support ansi colouring without a problem.
	match colored::control::set_virtual_terminal(true) {
		Ok(_) => true,
		Err(_) => false,
	}
}

#[cfg(not(windows))]
fn try_init_colours() -> bool { true }
