//! The analyse module contains all logic to analyse cleanable project directories
//! and determine which files and directories should be removed

mod analyse;
mod find_dirs;

pub use self::analyse::analyse;
