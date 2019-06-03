use std::{ env, process };
use std::path::{ Path, PathBuf };

/// The documentation text displayed when using the --help flag
const HELP : &str = "
This utility cleans up build & dependency directories in old projects to
preserve disk space

USAGE:
    node-cleanup [FLAGS] <PATH>...

ARGS:
    <PATH>...   One or more directories where the utility should start searching
                If omitted, will use the current working directory

FLAGS:
    -a, --all     Remove directories even in recently used projects
    -f, --force   Skip confirmation before removing directories

    -h, --help    Shows this help

More info: github.com/woubuc/node-cleanup
Questions, bugs & other issues: github.com/woubuc/node-cleanup/issues";

/// Contains the user-configurable settings for the application
pub struct Settings {
	/// The paths to search
	pub paths : Vec<PathBuf>,

	/// If true, remove even in recently used projects
	pub all : bool,

	/// If true, skip confirmation before removal
	pub force : bool
}

impl Settings {
	/// Creates a Settings object with the default settings
	fn defaults() -> Settings {
		Settings {
			paths: Vec::new(),
			all: false,
			force: false
		}
	}

	/// Parses CLI arguments into a Settings object
	///
	/// # Arguments
	/// * `args` - The CLI arguments
	///
	/// # Example
	/// ```
	/// let settings = Settings::from_args(std::env::args());
	/// ```
	pub fn from_args(args : env::Args) -> Settings {
		// Create a settings object with the default settings
		let mut settings = Settings::defaults();

		// Go through all arguments one by one
		for flag in args {
			// If the help flag is given, immediately show the help info and
			// then exit the application
			if match_flag(&flag, "-h", "-help") {
				println!("{}", HELP);
				process::exit(0);
			}

			// If boolean flags are given, enable the corresponding setting
			else if match_flag(&flag, "-a", "-all") { settings.all = true; }
			else if match_flag(&flag, "-f", "-force") { settings.force = true; }

			// If the argument was not a flag, it's probably a path, so add it
			// to the settings if it was successfully parsed
			else if let Some(path) = parse_path(&flag) {
				settings.paths.push(path);
			}
		}

		// If no paths were given, use the current directory as root
		if settings.paths.len() == 0 {
			settings.paths.push(".".into());
		}

		// Return the completed settings object
		return settings;
	}
}

/// Checks if an arg flag matches the given flag
///
/// # Arguments
/// * `flag`    - The flag
/// * `compact` - Compact flag notation
/// * `full`    - Full flag notation
fn match_flag(flag : &str, compact : &str, full : &str) -> bool {
	flag.eq(compact) || flag.contains(full)
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
