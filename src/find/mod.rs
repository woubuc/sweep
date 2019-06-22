//! The find module contains everything to explore the configured paths and
//! discover cleanable projects and their dependency directories

pub use self::discover::discover;

mod discover;
mod identify;
mod ignore;

