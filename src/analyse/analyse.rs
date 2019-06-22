use std::path::{Path, PathBuf};
use std::thread;
use std::time::{Duration, Instant};

use crossbeam::queue::SegQueue;
use crossbeam::thread::scope;

use crate::output::output;
use crate::settings::Settings;

use super::find_dirs::find_dirs;

pub fn analyse(cleanables : SegQueue<PathBuf>, settings : &Settings) {



//	let delete_dirs = find_dirs(cleanables, settings);
//
//	output().analyse_processing_done(delete_dirs.len());
//
//	while let Ok(p) = delete_dirs.pop() {
//		println!("{:?}", p);
//	}

//	return delete_dirs;
}
