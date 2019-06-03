use std::fs::remove_dir_all;
use std::path::PathBuf;
use crate::spinner::Spinner;

/// Removes the given paths
pub fn remove(paths : Vec<PathBuf>) {
	println!("Deleting selected directories");
	let mut spinner = Spinner::new("Deleting directories...");
	let mut i = 0;

	for path in paths {
		if let Err(err) = remove_dir_all(path) {
			println!("Error while deleting directory: {}", err);
		}

		i += 1;
		spinner.update(format!("Deleted {} directories", i).as_str());
	}

	spinner.finish(format!("Deleted {} directories", i).as_str());
}
