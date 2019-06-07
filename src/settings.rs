use std::env::Args;
use std::path::{ Path, PathBuf };

use colored::*;
use regex::Regex;

/// The documentation text displayed when using the --help flag
const HELP : &str = "This utility cleans up build & dependency directories in old projects to
preserve disk space

USAGE:
    node-cleanup [OPTIONS] <PATH>...

ARGS:
    <PATH>...   One or more directories to start searching

OPTIONS:
    -a, --all           Remove directories even in recently used projects
    -f, --force         Skip confirmation before removing directories

	-i, --ignore <PAT>  Skips directories that match the given string or regex pattern anywhere in their path

    -h, --help          Shows this help
    -v, --version       Displays the current application version

More info: github.com/woubuc/node-cleanup
Questions, bugs & other issues: github.com/woubuc/node-cleanup/issues";

pub enum ParseResult {
	/// If all is well, the parse result should return with the settings object
	Ok(Settings),

	/// If a non-functional argument is given (e.g. --help), the result will
	/// return Done
	Done,

	/// If an invalid argument is given, the result will be Errored. An
	/// error message will have been printed.
	Errored,
}

/// Contains the user-configurable settings for the application
#[derive(Debug)]
pub struct Settings {
	/// The paths to search
	pub paths : Vec<PathBuf>,

	/// If true, remove even in recently used projects
	pub all : bool,

	/// If true, skip confirmation before removal
	pub force : bool,

	/// Directory name pattern to ignore
	pub ignore : Option<Regex>,
}

impl Settings {
	/// Creates a Settings object with the default settings
	fn defaults() -> Settings {
		Settings {
			paths: Vec::new(),
			all: false,
			force: false,
			ignore: None,
		}
	}

	/// Parses CLI arguments into a Settings object
	///
	/// # Arguments
	/// * `args` - The CLI arguments
	///
	/// # Returns
	/// If all is well, this function should return Ok(settings)
	/// If the settings could not be parsed,
	///
	/// # Example
	/// ```
	/// let settings = Settings::from_args(std::env::args());
	/// ```
	pub fn from_args(mut args : Args) -> ParseResult {
		// Create a settings object with the default settings
		let mut settings = Settings::defaults();

		// Go through all arguments one by one
		while args.len() > 0 {
			let flag = args.next();
			if flag.is_none() { break }
			let flag = flag.unwrap();

			// If the version flag is given, just exit because the version
			// gets printed at the start of the program by default
			if match_flag(&flag, "-v", "--version") {
				return ParseResult::Done;
			}

			// If the help flag is given, immediately show the help info and
			// then exit the application
			if match_flag(&flag, "-h", "--help") {
				println!("{}", HELP);
				return ParseResult::Done;
			}

			// If boolean flags are given, enable the corresponding setting
			if match_flag(&flag, "-a", "--all") {
				settings.all = true;
				continue;
			}

			if match_flag(&flag, "-f", "--force") {
				settings.force = true;
				continue;
			}

			// Construct regex pattern for ignore flag
			if match_flag(&flag, "-i", "--ignore") {
				let pattern = args.next();
				if pattern.is_none() {
					println!("{} No pattern given for --ignore flag", "!".bold().red());
					return ParseResult::Errored;
				}

				let regex = Regex::new(pattern.unwrap().as_str());
				if regex.is_err() {
					println!("{} Ignore pattern invalid: {}", "!".bold().red(), regex.err().unwrap());
					return ParseResult::Errored;
				}

				settings.ignore = Some(regex.unwrap());
				continue;
			}

			// If the argument was not a flag, it's probably a path, so add it
			// to the settings if it was successfully parsed
			if let Some(path) = parse_path(&flag) {
				settings.paths.push(path);
			}
		}

		// If no paths were given, use the current directory as root
		if settings.paths.len() == 0 {
			settings.paths.push(".".into());
		}

		// Return the completed settings object
		return ParseResult::Ok(settings);
	}
}

/// Checks if an arg flag matches the given flag
///
/// # Arguments
/// * `flag`    - The flag
/// * `compact` - Compact flag notation
/// * `full`    - Full flag notation
fn match_flag(flag : &str, compact : &str, full : &str) -> bool {
	flag.eq(compact) || flag.eq(full)
}

/// Parses a flag into a complete, absolute directory path
///
/// # Arguments
/// * `flag` - The flag to parse
///
/// # Returns
/// The absolute PathBuf if a valid path was given, or None
fn parse_path(flag : &str) -> Option<PathBuf> {
	let path = Path::new(flag);

	if path.is_file() { return None; }
	return Some(path.to_path_buf());
}
