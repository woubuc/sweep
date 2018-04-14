use languages::*;

use std::path::Path;

pub fn identify(p : &Path) -> Option<Language> {
	if p.join("package.json").exists() { return Some(NODE); }
	if p.join("Cargo.toml").exists() { return Some(RUST); }
	return None;
}
