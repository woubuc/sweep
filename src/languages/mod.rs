mod identify;
mod language;

pub use languages::identify::identify;
pub use languages::language::Language;

/// Node.js projects
pub const NODE : Language = Language {
	name: "Node.js",
	paths: &["node_modules", ".cache"]
};

/// Rust projects
pub const RUST : Language = Language {
	name: "Rust",
	paths: &["target"]
};

/// Java projects
pub const JAVA : Language = Language {
	name: "Java",
	paths: &["build", ".gradle"]
};
