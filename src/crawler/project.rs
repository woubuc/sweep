use languages::Language;
use std::path::PathBuf;

/// The project data struct for the crawler
///
/// At this point in the application, project stats have not been loaded yet
/// and the crawler only has loads the below three properties.
#[derive(Debug)]
pub struct Project {
	/// Name of the directory
	pub name : String,

	/// Full path to the project directory
	pub path : PathBuf,

	/// The programming language or environment this project was written in
	pub language : Language
}
