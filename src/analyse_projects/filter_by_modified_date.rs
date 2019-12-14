use std::cmp::max;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

use crossbeam::queue::SegQueue;
use yansi::Color;

use crate::output;
use crate::utils::file_utils::file_name;
use crate::utils::process_queue;
use crate::Project;

const ALWAYS_IGNORE_DIRS: [&'static str; 3] = [".idea", ".vscode", ".git"];

pub fn filter_by_modified_date(projects: SegQueue<Project>) -> SegQueue<Project> {
    let old_projects = SegQueue::new();
    let recent_projects = AtomicUsize::new(0);

    process_queue(
        max(2, num_cpus::get()),
        &projects,
        |project| {
            let paths = SegQueue::new();
            let modified = SegQueue::new();

            find_modified_date_of_directory(&project, project.root(), &paths, &modified);

            process_queue(
                max(8, num_cpus::get() * 2),
                &paths,
                |path| {
                    output::print("Analysing", Color::Cyan, path.to_str().unwrap_or(""));
                    find_modified_date_of_directory(&project, &path, &paths, &modified);
                },
                |_| (),
            );

            let time_since_last_modified = {
                let mut vec = Vec::with_capacity(modified.len());
                while let Ok(m) = modified.pop() {
                    if let Some(m) = m {
                        vec.push(m);
                    }
                }

                vec.into_iter().min().unwrap_or(0)
            };

            // Count the total size (depending on modified date)
            if time_since_last_modified > 2_592_000 {
                old_projects.push(project);
            } else {
                recent_projects.fetch_add(1, Ordering::SeqCst);
            }
        },
        |tries| {
            output::print("Analysing", Color::Cyan, &".".repeat(tries));
        },
    );

    let recent_project_count = recent_projects.into_inner();
    let old_project_count = old_projects.len();

    if recent_project_count == 0 {
        output::println("Analysed", Color::Green, "All projects can be cleaned");
    } else if old_project_count == 0 {
        output::println(
            "Analysed",
            Color::Green,
            "All projects have been modified recently",
        );
    } else {
        let message = format!(
            "{} of {} projects can be cleaned",
            old_project_count,
            old_project_count + recent_project_count
        );
        output::println("Analysed", Color::Green, &message);
        output::println_info(format!(
            "{} projects have been modified recently",
            recent_project_count
        ));
    }

    return old_projects;
}

fn find_modified_date_of_directory(
    project: &Project,
    path: &Path,
    paths: &SegQueue<PathBuf>,
    modified: &SegQueue<Option<u64>>,
) {
    let (dirs, files) = {
        let mut dirs = Vec::new();
        let mut files = Vec::new();

        let read_dir = path.read_dir().expect("Could not read directory"); // TODO add better error handling
        for entry in read_dir.filter_map(|e| e.ok()) {
            if entry.file_type().unwrap().is_dir() {
                dirs.push(entry.path());
            } else {
                files.push(entry);
            }
        }
        (dirs, files)
    };

    modified.push(
        files
            .iter()
            .filter_map(|p| p.metadata().ok())
            .filter_map(|d| d.modified().ok())
            .filter_map(|m| m.elapsed().ok())
            .map(|e| e.as_secs())
            .min(),
    );

    for dir in dirs {
        if ALWAYS_IGNORE_DIRS.contains(&file_name(&dir)) {
            continue;
        }

        if project.is_cleanable_dir(&dir) {
            continue;
        }

        paths.push(dir);
    }
}
