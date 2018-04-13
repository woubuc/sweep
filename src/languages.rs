//! The cleanup tool supports multiple programming languages, the specifics of
//! which are defined in this module.

#[derive(Copy)]
#[derive(Clone)]
#[derive(Eq)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Debug)]
pub enum Language {
	NODE,
	RUST
}

/// Gets the dependency paths for a language
pub fn get_paths<'a>(lang : &Language) -> Vec<&'a str> {
	match lang {
		Language::NODE => vec!["node_modules"],
		Language::RUST => vec!["target"]
	}
}
