use std::ffi::OsStr;
use std::path::Path;

/// Checks if a file exists in a path
///
/// # Arguments
/// `path`     - The directory
/// `filename` - Filename to look up in `path`
///
/// # Returns
/// True if `filename` exists in `path`
pub fn exists_in_path(path: &Path, filename: &str) -> bool {
    path.clone().join(filename).exists()
}

/// Gets the filename of a path
///
/// # Arguments
/// `path` - The path
///
/// # Returns
/// The filename (last part) of the path, or an empty string if
/// the path could not be retrieved for any reason
pub fn file_name(path: &Path) -> &str {
    path.file_name()
        .unwrap_or_else(|| OsStr::new(""))
        .to_str()
        .unwrap_or("")
}

#[cfg(test)]
mod test {
    use crate::utils::test_utils;

    use super::*;

    #[test]
    fn file_exists() {
        test_utils::with_temp_dir(|dir| {
            test_utils::create_file(dir, "test_file.txt");

            assert_eq!(exists_in_path(&dir, "test_file.txt"), true);
        });
    }

    #[test]
    fn file_does_not_exist() {
        test_utils::with_temp_dir(|dir| {
            assert_eq!(exists_in_path(&dir, "test_file.txt"), false);
        });
    }

    #[test]
    fn directory_exists() {
        test_utils::with_temp_dir(|dir| {
            test_utils::create_dir(dir, "test_dir");

            assert_eq!(exists_in_path(&dir, "test_dir"), true);
        });
    }

    #[test]
    fn directory_does_not_exist() {
        test_utils::with_temp_dir(|dir| {
            assert_eq!(exists_in_path(&dir, "test_dir"), false);
        });
    }

    #[test]
    fn get_file_name() {
        test_utils::with_temp_dir(|dir| {
            test_utils::create_file(dir, "test_file.txt");

            assert_eq!(file_name(&dir.join("test_file.txt")), "test_file.txt");
        });
    }
}
