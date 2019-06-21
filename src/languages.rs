/// The language of a project
pub enum Language {
	/// A Git repository, without a known language
	Git,

	/// A Java project
	Java,

	/// A NodeJS project
	NodeJS,

	/// A Rust project
	Rust,
}
