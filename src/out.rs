use term_size::dimensions;
use std::cmp::min;
use std::io::{stdout, Write};

/// Gets the width of the terminal
pub fn width() -> usize {
	if let Some((w, _h)) = dimensions() { w } else { 80 }
}

/// A progress bar
pub struct Progress {
	total : usize,
	progress : usize,
	label : String,
	status : String
}

impl Progress {
	/// Initialises a progress bar
	///
	/// # Arguments
	/// * `total` - Total number of steps this progress bar represents
	/// * `label` - The label displayed in front of the progress bar
	pub fn new(mut total : usize, label : &str) -> Progress {
		if total < 1 { total = 1; }
		Progress {total, progress: 0, label: label.to_owned(), status: "".to_owned()}
	}

	/// Increases the progress with 1
	pub fn step(&mut self) -> usize {
		if self.progress < self.total { self.progress += 1; }
		self.update();
		return self.progress;
	}

	/// Sets the progress
	pub fn progress(&mut self, progress : usize) {
		self.progress = progress;
		if self.progress > self.total { self.progress = self.total; }
		self.update();
	}

	/// Sets the status text displayed after the progress bar
	pub fn status(&mut self, status : &str) {
		self.status = status.to_owned();
		self.update();
	}

	/// Finish the progress bar and move the cursor to a new line
	pub fn finish(&mut self) {
		self.status("OK");
		println!();
	}

	/// Finish the progress bar with an error and move the cursor to a new line
	pub fn error(&mut self) {
		self.status("ERR");
		println!();
	}

	/// Updates the terminal with the new values
	fn update(&self) {
		let w = width();
		print!("\r");

		let status_len = min(4, self.status.len());
		let status = format!("{}{}", &self.status[0..status_len], " ".repeat(4 - status_len));

		let progress_len = w - self.label.len() - 5;
		let progress_pct = self.progress * progress_len / self.total;
		let progress_bar = format!("{}{}", ".".repeat(progress_pct), " ".repeat(progress_len - progress_pct));

		print!("{} {} {}", self.label, progress_bar, status);
		let _ = stdout().flush();
	}
}

/// Generates a terminal control sequence
macro_rules! csi {
	($( $l:expr ),*) => { concat!("\x1B[", $( $l ),*) };
}

/// Hides the terminal cursor
pub fn hide_cursor() { print!("{}", csi!("?25l")) }

/// Shows the terminal cursor
pub fn show_cursor() { print!("{}", csi!("?25h")) }
