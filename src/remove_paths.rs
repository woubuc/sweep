use out::Progress;

use std::fs::remove_dir_all;
use std::path::PathBuf;

/// Removes the given paths
pub fn remove(paths : Vec<PathBuf>) {
	let mut progress = Progress::new(paths.len(), "Deleting directories");

	for path in paths {
		if let Err(err) = remove_dir_all(path) {
			progress.error();
			println!("{}", err);
		}

		progress.step();
	}

	progress.finish();
}
