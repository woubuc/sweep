use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::path::Path;

pub fn parse_cleanuprc<P: AsRef<Path>>(dir: P) -> Result<Vec<String>> {
    let file = File::open(dir.as_ref().join(".cleanuprc"))?;
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
            std::fs::write(dir.join(".cleanuprc"), "").expect("Could not write test file");

            let dirs = parse_cleanuprc(dir).expect("Error while reading .cleanuprc");
            assert_eq!(dirs.len(), 0);
        });
    }

    #[test]
    fn cleanuprc() {
        const FILE_CONTENTS: &'static str = r"
			# comment

			target
			test-directory/

			duplicate
			duplicate
			";

        test_utils::with_temp_dir(|dir| {
            std::fs::write(dir.join(".cleanuprc"), FILE_CONTENTS)
                .expect("Could not write test file");

            let dirs = parse_cleanuprc(dir).expect("Error while reading .cleanuprc");
            assert_eq!(dirs.len(), 3);
        });
    }
}
