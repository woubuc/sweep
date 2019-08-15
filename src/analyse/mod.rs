//! The analyse module contains all logic to analyse cleanable project directories
//! and determine which files and directories should be removed

pub use self::analyse::analyse;

mod analyse;
mod modified;

