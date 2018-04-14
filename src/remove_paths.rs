use std::fs::remove_dir_all;
use std::path::PathBuf;

/// Removes the given paths
pub fn remove(paths : Vec<PathBuf>) {
	println!("Deleting directories...");
	let mut i = 0;

	for path in paths {
		if let Err(err) = remove_dir_all(path) {
			println!("Error while deleting directory: {}", err);
		}

		i += 1;
		if i % 10 == 0 {
			println!("  Deleted {} directories", i);
		}
	}

	println!("  Deleted {} directories", i);
}
