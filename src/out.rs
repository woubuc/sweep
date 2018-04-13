//! This module contains the multiline strings needed, to avoid them
//! messing up indentation when used in the codebase

/// Prints the application intro including title and version string
pub fn intro() {
    println!("
Project Cleanup
   v{}
   by woubuc", env!("CARGO_PKG_VERSION"));
}

/// Prints the CLI interface help text
pub fn help() {
    println!("
This utility cleans up build & dependency directories in old projects to
preserve disk space

USAGE:
   node-cleanup [FLAGS] <PATH>...

ARGS:
   PATH   One or more directories where the utility should start searching
         If omitted, will use the current working directory

FLAGS:
   -a, --all     Remove directories even in recently used projects
   -f, --force   Skip confirmation before removing directories

   -d, --debug   Enable debug output
   -h, --help    Shows this help

More info: github.com/woubuc/node-cleanup
Questions, bugs & other issues: github.com/woubuc/node-cleanup/issues");
}
