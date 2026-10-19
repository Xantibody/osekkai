//! osekkai: guardrails for the summarize skill.
//!
//! The binary owns what can be verified from the artifact alone: the shape of
//! Markdown lists and sections, the reader state declared at the top of a
//! document, and the fixed order of steps. Everything that needs the meaning
//! of the text stays in the skill.

pub mod doc;
pub mod rules;

pub use doc::Document;
pub use rules::Finding;

/// Lint a Markdown source and return every finding, ordered by line.
pub fn lint(src: &str) -> Vec<Finding> {
    let doc = Document::parse(src);
    let mut out = rules::run(&doc);
    out.sort_by_key(|f| (f.line, f.column));
    out
}
