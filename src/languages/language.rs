/// The definition for a project language
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Language {
	pub name : &'static str,
	pub paths : &'static [&'static str]
}

impl Language {
	/// Gets the name of the language
	pub fn name(&self) -> &'static str { &self.name }

	/// Gets the paths that should be removed from a project
	///
	/// # Arguments
	/// * `path` - Path to the project directory
	///
	/// # Returns
	/// The path(s) that will be removed from the given project, or none if
	/// there are no paths to be removed
	pub fn get_paths(&self) -> &'static [&'static str] { self.paths }
}
