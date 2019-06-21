//! The find module contains everything to explore the configured paths and
//! discover cleanable directories

mod discover;
mod identify;
mod ignore;

pub use self::discover::discover;
