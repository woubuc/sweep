use std::env;
use std::process::exit;
use std::path::{Path, PathBuf};

pub struct Args {
    pub dir : PathBuf,
    pub verbose : bool,
    pub dry_run : bool
}

impl Args {
    pub fn parse() -> Args {
        let exe = env::current_exe().unwrap();
        let exe = exe.to_str().unwrap();

        let mut args = Args::defaults();

        for entry in env::args() {
            if exe.contains(&entry) { continue; }

            if entry.eq("-v") || entry.contains("-verbose") {
                args.verbose = true;
                continue;
            }

            if entry.eq("-d") || entry.contains("-dryrun") {
                args.dry_run = true;
                continue;
            }

            if entry.eq("-h") || entry.contains("-help") {
                println!("");
                println!("Usage: node-cleanup [flags] [dir]");
                println!("");
                println!("dir: The root directory to search for Node.js projects (recursive)");
                println!("     If omitted, will use the current working directory");
                println!("");
                println!("flags:");
                println!("-d, --dryrun   Find projects but don't remove anything");
                println!("-h, --help     View this information");
                println!("-v, --verbose  Enable verbose logging");
                exit(0);
            }

            args.dir = Path::new(&entry).to_path_buf();
        }

        if args.verbose {
            println!("");
            println!("Verbose logging enabled");
            println!("Search directory: {}", args.dir.display());
            if args.dry_run { println!("Running in dry run mode (nothing will be removed)"); }
        }

        return args;
    }

    fn defaults() -> Args {
        let cwd = env::current_dir().unwrap().to_path_buf();

        return Args {
            dir: cwd,
            dry_run: false,
            verbose: false
        };
    }
}