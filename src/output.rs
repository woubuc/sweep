//! Contains text formatting functions and output helpers to avoid having this formatting
//! throughout all of the code

use std::path::{Path, PathBuf};

use crossterm::{ terminal, Colored, Color, Attribute };
use lazy_static::lazy_static;
use std::io::{ stdout, Write };
use std::fmt::Display;

lazy_static! {
	static ref OUTPUT_MANAGER : OutputManager = OutputManager::create();
}

pub fn output() -> &'static OutputManager { &OUTPUT_MANAGER }

const LABEL_WIDTH : usize = 12;

pub struct OutputManager {
	term_width : usize
}

impl OutputManager {
	fn create() -> OutputManager {
		OutputManager {
			term_width: get_term_width(),
		}
	}

	/// The title of the application
	pub fn main_title(&self) {
		println!("{}Project Cleanup v{}{}", Attribute::Bold, env!("CARGO_PKG_VERSION"), Attribute::Reset);
	}

	/// A path as configured through CLI arguments
	pub fn settings_path(&self, path : &Path) {
		self.println("Path", Color::Blue, path.to_str().unwrap_or(""));
	}

	/// Info message when there are no cleanable projects found
	pub fn main_no_cleanable_projects(&self) {
		println!("{}", wrap_style("No cleanable projects found", Colored::Fg(Color::Yellow)));
		println!("  Check your paths and try again.");
		println!("  See `{}` for more options", wrap_style("--help", Attribute::Bold));
	}

	/// Info message when there are cleanable projects found,
	/// but none of them have deletable directories
	pub fn main_no_deletable_directories(&self) {
		println!("{}", wrap_style("No cleanable directories found", Colored::Fg(Color::Yellow)));
		println!("  This is likely because your projects were recently modified");
		println!("  Run the application with `{}` to disregard file age", wrap_style("--all", Attribute::Bold));
		println!("  Try `{}` for more options", wrap_style("--help", Attribute::Bold));
	}

	/// Displays a list of found directories that can be deleted
	pub fn main_directories_list(&self, dirs : &Vec<PathBuf>) {
		let message = if dirs.len() == 1 {
			format!("Found 1 directory that can be deleted:")
		} else {
			format!("Found {} directories that can be deleted:", dirs.len())
		};

		self.println("Result", Color::Green, &message);

		for dir in dirs {
			self.println_plain(dir.to_str().unwrap_or(""));
		}
	}

	/// Shows a warning before deleting deletable directories
	pub fn main_delete(&self) {
		println!("{}{}{} DANGER {} {}{}{}\r",
				 " ".repeat(LABEL_WIDTH - 8),

				 Colored::Fg(Color::White),
				 Colored::Bg(Color::Red),
				 Attribute::Reset,

				 Colored::Fg(Color::Red),
				 "Above directories will be permanently deleted",
				 Attribute::Reset,
		);
	}

	/// Asks the user if he wants to continue and actually delete files
	pub fn main_delete_question(&self) {
		print!("{} Continue? (y/n): ", " ".repeat(LABEL_WIDTH));
		let _ = stdout().flush();
	}

	pub fn main_delete_invalid_answer(&self) {
		self.println_plain("Please answer either 'y' or 'n'");
	}

	pub fn discover_searching_path(&self, path : &Path) {
		self.print("Searching", Color::Cyan, path.to_str().unwrap_or(""));
	}

	pub fn discover_searching_retry(&self, tries : usize) {
		self.print("Searching", Color::Cyan, &".".repeat(tries));
	}

	pub fn discover_searching_error(&self, error : &str, path : &Path) {
		self.println("Error", Color::Red, error);
		self.println_plain(path.to_str().unwrap_or(""));
	}

	pub fn discover_searching_done(&self, total_paths : usize, discovered : usize) {
		self.println("Searched", Color::Green, &format!("{} directories searched", total_paths));
		self.println_plain(format!("{} cleanable projects found", discovered));
	}

	pub fn analyse_filter_by_modified_skip(&self) {
		self.println("Skip", Color::Yellow, "--all flag set, ignoring last used time");
	}

	pub fn analyse_filter_by_modified_path(&self, path : &Path) {
		self.print("Analysing", Color::Cyan, path.to_str().unwrap_or(""));
	}

	pub fn analyse_filter_by_modified_retry(&self, tries : usize) {
		self.print("Analysing", Color::Cyan, &".".repeat(tries));
	}

	pub fn analyse_filter_by_modified_done(&self, old_projects : usize, recent_projects : usize) {
		if recent_projects == 0 {
			self.println("Analysed", Color::Green, "All projects can be cleaned");
		} else if old_projects == 0 {
			self.println("Analysed", Color::Green, "All projects have been modified recently");
		} else {
			self.println("Analysed", Color::Green, &format!("{} of {} projects can be cleaned", old_projects, old_projects + recent_projects));
			self.println_plain(format!("{} projects have been modified recently", recent_projects));
		}
	}

	pub fn analyse_processing_done(&self, discovered : usize) {
		self.println("Analysed", Color::Green, &format!("{} unnecessary directories found in projects", discovered));
	}

	pub fn delete_path(&self, dir : &Path) {
		self.print("Deleting", Color::Cyan, dir.to_str().unwrap_or(""));
	}

	pub fn delete_complete(&self) {
		self.print("Deleted", Color::Green, "All directories deleted");
	}


	fn print<S : Into<String>>(&self, label : S, label_colour : Color, message : S) {
		let message = self.shorten(message.into());
		let label = label.into();

		print!("{}{}{}{} {}{}\r",
			   " ".repeat(LABEL_WIDTH - label.len()),

			   Colored::Fg(label_colour),
			   label,
			   Attribute::Reset,

			   message,

			   " ".repeat(self.term_width - LABEL_WIDTH - 1 - message.len())
		);

		let _ = stdout().flush();
	}

	fn print_plain<S : Into<String>>(&self, message : S) {
		let message = self.shorten(message.into());

		print!("{} {}{}\r",
				 " ".repeat(LABEL_WIDTH),
				 message,
				 " ".repeat(self.term_width - LABEL_WIDTH - 1 - message.len())
		);

		let _ = stdout().flush();
	}

	fn println<S : Into<String>>(&self, label : S, label_colour : Color, message : S) {
		self.print(label, label_colour, message);
		println!();
	}

	fn println_plain<S : Into<String>>(&self, message : S) {
		self.print_plain(message);
		println!();
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

/// Wraps a string in the given style attribute
///
/// # Arguments
/// `message` - Message to wrap
/// `style`   - Style attribute to apply
fn wrap_style<D : Display, S : AsRef<str> + Display>(message : S, style : D) -> String {
	format!("{}{}{}", style, message, Attribute::Reset)
}
