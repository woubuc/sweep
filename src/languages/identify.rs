use std::path::Path;

use super::*;

pub fn identify(p : &Path) -> Option<Language> {
	if p.join("package.json").exists() { return Some(NODE); }
	if p.join("Cargo.toml").exists() { return Some(RUST); }
	if p.join("pom.xml").exists() || p.join("gradlew").exists() { return Some(JAVA); }
	return None;
}
