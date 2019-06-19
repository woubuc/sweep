use std::time::Instant;
use std::cmp;

use colored::Colorize;
use std::io::{ stdout, Write };

const MS_PER_STEP : u128 = 100;
const STEPS : [&'static str; 10] = ["⠸ ", "⠼ ", "⠴ ", "⠦ ", "⠧ ", "⠇ ", "⠏ ", "⠋ ", "⠙ ", "⠹ "];
const DONE : &'static str = "ok";

/// Shows a rotating spinner in the terminal
///
/// The spinner will rotate when it is updated, it will not be updated
/// automatically. Printing anything to the terminal will break the
/// spinner. Call `.finish()` to finish the spinner.
pub struct Spinner {
	step : usize,
	step_increased : Instant,
	text_length: usize,
}

impl Spinner {
	/// Starts a new spinner
	///
	/// # Arguments
	/// * `initial_text` - The initial text message to display after the spinner indicator
	pub fn new(initial_text : &'static str) -> Spinner {
		let mut spinner = Spinner {
			step: 0,
			step_increased: Instant::now(),
			text_length: 0,
		};

		spinner.update(initial_text);
		return spinner;
	}

	/// Updates the spinner with a new message
	///
	/// # Arguments
	/// * `text` - The text message to display after the spinner indicator
	pub fn update(&mut self, text : &str) {
		if self.step_increased.elapsed().as_millis() >= MS_PER_STEP {
			self.step = match self.step {
				9 => 0,
				_ => self.step + 1,
			};

			self.step_increased = Instant::now();
		}

		self.text_length = cmp::max(text.len(), self.text_length);

		let indicator : &'static str = STEPS[self.step];
		let spacer = " ".repeat(self.text_length - text.len());
		print!("\r  {} {}{}", indicator.cyan(), text, spacer);

		// Try to flush the stdout buffer to update the print line before the
		// process is done, but it's not a problem if it won't flush right away
		// so we can just capture the result without doing anything with it
		let _ = stdout().flush();
	}

	/// Finishes the spinner with a green checkmark
	///
	/// # Arguments
	/// * `text` - The completion message to display
	pub fn finish(&self, text : &str) {
		let spacer = " ".repeat(self.text_length - text.len());
		println!("\r  {} {}{}", DONE.green(), text, spacer);
		let _ = stdout().flush();
	}
}
