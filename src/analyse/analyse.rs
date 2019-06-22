use std::path::{Path, PathBuf};
use std::thread;
use std::time::{Duration, Instant};

use crossbeam::queue::SegQueue;
use crossbeam::thread::scope;

use crate::output::output;
use crate::Project;
use crate::settings::Settings;

use super::modified::filter_by_modified;

pub fn analyse(projects : SegQueue<Project>, settings : &Settings) {

	let filtered = filter_by_modified(projects);

	if filtered.len() == 0 {
		output().analyse_no_old_cleanables();
		return;
	}

//	let delete_dirs = find_dirs(cleanables, settings);
//
//	output().analyse_processing_done(delete_dirs.len());
//
//	while let Ok(p) = delete_dirs.pop() {
//		println!("{:?}", p);
//	}

//	return delete_dirs;
}
