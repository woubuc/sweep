use walkdir::WalkDir;
use std::path::PathBuf;

pub struct Project {
    pub dir : PathBuf,

    size : u64,
    modules_size : u64,
    modified : u64
}

impl Project {
    pub fn new(dir : PathBuf) -> Project {
        let size = WalkDir::new(&dir).into_iter()
            .filter_map(|entry| entry.ok())
            .filter(|entry| !entry.path().to_str().unwrap().contains("node_modules"))
            .filter_map(|entry| entry.metadata().ok())
            .filter(|metadata| metadata.is_file())
            .fold(0, |acc, m| acc + m.len());

        let modules_size = WalkDir::new(&dir.join("node_modules")).into_iter()
            .filter_map(|entry| entry.ok())
            .filter_map(|entry| entry.metadata().ok())
            .filter(|metadata| metadata.is_file())
            .fold(0, |acc, m| acc + m.len());
            
        let modified = WalkDir::new(&dir).into_iter()
            .filter_map(|entry| entry.ok())
            .filter(|entry| !entry.path().to_str().unwrap().contains("node_modules"))
            .filter_map(|entry| entry.metadata().ok())
            .filter(|metadata| metadata.is_file())
            .filter_map(|metadata| metadata.modified().ok())
            .max().unwrap()
            .elapsed().expect("Could not get modified date")
            .as_secs();

        return Project {dir, size, modules_size, modified};
    }

    pub fn size(&self) -> u64 { self.size }
    pub fn modules_size(&self) -> u64 { self.modules_size }
    pub fn modified(&self) -> u64 { self.modified }
}