use std::fs;
use std::path::Path;

use tempdir::TempDir;

pub fn with_temp_dir<F : FnOnce(&Path)>(action : F) {
	let temp_dir = TempDir::new("project_cleanup_test")
		.expect("Could not create temp directory");

	action(temp_dir.path());

	temp_dir.close().expect("Could not delete temp directory");
}


pub fn create_file(dir : &Path, file_name : &str) {
	fs::write(dir.join(file_name), "test_file").expect("Could not write test file");
}

pub fn create_dir(dir : &Path, dir_name : &str) {
	fs::create_dir(dir.join(dir_name)).expect("Could not create test dir");
}
