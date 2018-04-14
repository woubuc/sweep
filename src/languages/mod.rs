mod identify;
mod language;

pub use languages::identify::identify;
pub use languages::language::Language;

// Language definitions
pub const NODE : Language = Language {
	name: "Node.js",
	paths: &["node_modules", ".cache"]
};

pub const RUST : Language = Language {
	name: "Rust",
	paths: &["target"]
};
