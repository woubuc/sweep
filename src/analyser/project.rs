use languages::Language;

use std::path::PathBuf;

/// The project data struct that includes the project stats, for use after the
/// project has been analysed
#[derive(Debug)]
pub struct Project {
	pub name : String,
	pub path : PathBuf,
	pub language : Language,
	pub modified : u64,
	pub size_src : u64,
	pub size_deps : u64
}
