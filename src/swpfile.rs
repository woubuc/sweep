use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::path::Path;

pub fn parse_swpfile<P: AsRef<Path>>(dir: P, filename : P) -> Result<Vec<String>> {
	let file = File::open(dir.as_ref().join(filename))?;
	let reader = BufReader::new(file);

	let mut paths = Vec::new();

	for line in reader.lines() {
		let line = line?.trim().to_owned();

		if line.len() < 1 {
			continue;
		}
		if line.starts_with('#') {
			continue;
		}
		if paths.contains(&line) {
			continue;
		}

		paths.push(line);
	}

	Ok(paths)
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::utils::test_utils;

	#[test]
	fn empty_file() {
		test_utils::with_temp_dir(|dir| {
			std::fs::write(dir.join(".swpfile"), "")
				.expect("Could not write test file");

			let dirs = parse_swpfile(dir, Path::new(".swpfile"))
				.expect("Error while reading .swpfile");

			assert_eq!(dirs.len(), 0);
		});
	}

	#[test]
	fn swpfile() {
		const FILE_CONTENTS: &'static str = r"
			# comment

			target
			test-directory/

			duplicate
			duplicate
			";

		test_utils::with_temp_dir(|dir| {
			std::fs::write(dir.join(".swpfile"), FILE_CONTENTS)
				.expect("Could not write test file");

			let dirs = parse_swpfile(dir, Path::new(".swpfile"))
				.expect("Error while reading .swpfile");

			assert_eq!(dirs.len(), 3);
		});
	}

	#[test]
	fn swpfile_cleanuprc() {
		test_utils::with_temp_dir(|dir| {
			std::fs::write(dir.join(".cleanuprc"), "target")
				.expect("Could not write test file");

			let dirs = parse_swpfile(dir, Path::new(".cleanuprc"))
				.expect("Error while reading .cleanuprc");

			assert_eq!(dirs.len(), 1);
		});
	}
}
