//! The Settings struct parses and holds all options that can be configured
//! by the user through cli flags.

use out;

use std::env;
use std::path::{Path, PathBuf};
use std::process::exit;

/// Contains the application settings
pub struct Settings {
    /// The paths to search
    pub paths : Vec<PathBuf>,

	/// If true, remove even in recently used projects
	pub all : bool,

    /// If true, skip confirmation before removal
    pub force : bool,

    /// Enables or disables debug logging
    pub debug : bool
}

impl Settings {
    /// Creates a Settings object with the default settings
    fn defaults() -> Settings {
        Settings {
            paths: Vec::new(),
			all: false,
            force: false,
            debug: false
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
                out::help();
                exit(0);
            }

            // If boolean flags are given, enable the corresponding setting
            if match_flag(&flag, "-d", "-debug") { settings.debug = true; }
            else if match_flag(&flag, "-a", "-all") { settings.all = true; }
            else if match_flag(&flag, "-f", "-force") { settings.force = true; }

            // If the argument was not a flag, it's probably a path, so add it
            // to the settings if it was successfully parsed
            else if let Some(path) = parse_path(&flag) {
                settings.paths.push(path);
            }
        }

        // If debug mode is enabled, log the parsed settings
        if settings.debug {
            println!("Debug mode enabled");
			if settings.all { println!("    All flag enabled")}
            if settings.force { println!("    Force flag enabled"); }
        }

        // If no paths were given, use the current working directory as root
        if settings.paths.len() < 1 {
            if settings.debug { println!("    No paths given, using cwd"); }
            let cwd = env::current_dir().expect("Could not get working directory");
            settings.paths.push(cwd);
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
///
/// # Example
/// ```
/// let should_help = match_flag(&flag, "-h", "-help");
/// ```
fn match_flag(flag : &str, compact : &str, full : &str) -> bool {
    flag.eq(compact) || flag.contains(full)
}

/// Parses a flag into a complete, absolute directory path
///
/// # Arguments
/// * `flag` - The flag to parse
///
/// # Example
/// ```
/// let path = parse_path("/root");
/// ```
///
/// # Returns
/// The absolute PathBuf if a valid path was given, or None
fn parse_path(flag : &str) -> Option<PathBuf> {
    let path = Path::new(flag);

    if path.is_file() { return None; }
    return Some(path.to_path_buf());
}
