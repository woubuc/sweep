extern crate walkdir;
extern crate humansize;

mod args;
mod project;
mod time_ago;

use args::Args;
use project::Project;
use time_ago::time_ago;

use std::ffi::OsStr;
use std::path::PathBuf;
use std::fs::{DirEntry, remove_dir_all};
use humansize::{FileSize, file_size_opts as options};

fn main() {
    let args = Args::parse();
    
    println!("");
    println!("Searching for Node.js projects in {}", args.dir.display());

    let projects = read_dir(args.dir);

    println!("");
    match projects.len() {
        0 => println!("No active projects found"),
        1 => println!("Found 1 project:"),
        _ => println!("Found {} projects:", projects.len())
    }

    let mut total_size : u64 = 0;
    let mut total_modules_size : u64 = 0;
    let mut total_cleanup_size : u64 = 0;

    for project in projects {
        let filename = project.dir.file_name()
            .unwrap_or(OsStr::new("")).to_str()
            .unwrap_or("[unknown project name]");

        println!("- Name: {}", filename);
        println!("  Path: {}", project.dir.display());
        println!("  Size: {}", project.size().file_size(options::CONVENTIONAL).unwrap());
        println!("  Modified: {}", time_ago(project.modified()));

        total_size += project.size() + project.modules_size();
        total_modules_size += project.modules_size();

        println!("");
        println!("  Modules: {}", project.dir.join("node_modules").display());
        println!("  Modules directory size: {}", project.modules_size().file_size(options::CONVENTIONAL).unwrap());

        if project.modified() < 2592000 {
            println!("  Project was modified recently, directory will not be removed");
            continue;
        }
        
        total_cleanup_size += project.modules_size();
        
        if args.dry_run {
            println!("  Directory will be removed when running without --dryrun");
        } else {
            println!("  Removing the node_modules directory...");

            if let Err(err) = remove_dir_all(project.dir.join("node_modules")) {
                println!("  ERROR An error occured: {}", err);
            } else {
                println!("  Done");
            }
        }

        println!("");
    }

    println!("---------------------------------------");

    let percentage = (total_cleanup_size as f64 / total_size as f64 * 100.0).round() as u64;

    if args.dry_run {
        println!("Total size of all projects: {}", format_size(total_size));
        println!("Total size of node_modules directories: {}", format_size(total_modules_size));
        println!("                      in old projects: {}", format_size(total_cleanup_size));
        println!("");
        println!("Reduce disk usage of your projects by {}%", percentage);
        println!("Run without --dryrun to remove node_modules directories in old projects");
    } else {
        println!("Total size of all projects: {}", format_size(total_size));
        println!("Total size of removed node_modules directories: {}", format_size(total_cleanup_size));
        println!("Total project size after node_modules removal: {}", format_size(total_size - total_cleanup_size));
        println!("");
        println!("Reduced disk usage of your projects by {}%", percentage);
    }
}

fn format_size(size : u64) -> String {
    return size.file_size(options::CONVENTIONAL).unwrap();
}

fn read_dir(dir : PathBuf) -> Vec<Project> {
    let mut projects : Vec<Project> = Vec::new();

    let contents = dir.read_dir();
    if let Err(err) = contents {
        println!("---------------------------------------");
        println!("ERROR Could not read directory contents");
        println!("      {}", dir.display());
        println!("      {}", err);
        println!("---------------------------------------");
        return projects;
    }

    for entry in contents.unwrap() {
        if entry.is_err() { continue; }
        let entry = entry.unwrap();

        if is_file(&entry) { continue; }
        if is_dotfile(&entry) { continue; }

        if !is_node_project(&entry) {
            projects.append(&mut read_dir(entry.path()));
            continue;
        }

        println!("Found project: {}", entry.path().display());
        projects.push(Project::new(entry.path().to_path_buf()));
    }

    return projects;
}

/// Checks if a DirEntry is a file or a symlink
/// 
/// Returns true if the entry is a file, false if not or
///         if the type could not be determined
fn is_file(entry : &DirEntry) -> bool {
    if let Ok(file_type) = entry.file_type() {
        return file_type.is_file() || file_type.is_symlink();
    }

    return false;
}

/// Checks if a DirEntry is a dotfile or -directory
/// 
/// Returns true if the entry is a dotfile or -directory,
///         false if not or if the file name could not be determined
fn is_dotfile(entry : &DirEntry) -> bool {
    if let Some(file_name) = entry.file_name().to_str() {
        return file_name.starts_with(".");
    }
    return false;
}

/// Determines if the given directory contains a Node.js project
/// 
/// Returns true if all of the following are true:
/// - The directory contains a package.json file
/// - The directory contains a node_modules directory
fn is_node_project(entry : &DirEntry) -> bool {
    let path = entry.path();
    if path.join("package.json").exists() == false { return false; }
    if path.join("node_modules").exists() == false { return false; }

    return true;
}