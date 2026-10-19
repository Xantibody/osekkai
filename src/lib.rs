//! osekkai: guardrails for the summarize skill.
//!
//! The binary owns what can be verified from the artifact alone: the shape of
//! Markdown lists and sections, the reader state declared at the top of a
//! document, and the fixed order of steps. Everything that needs the meaning
//! of the text stays in the skill.

pub mod doc;
pub mod rules;
pub mod state;

pub use doc::Document;
pub use rules::Finding;
pub use state::State;

#[derive(Debug, Clone, Copy, Default)]
pub struct Options {
    /// Report a document that declares no reader state. Off by default so a
    /// body written without the skill is judged on its structure alone.
    pub require_state: bool,
}

/// Lint a Markdown source with default options.
pub fn lint(src: &str) -> Vec<Finding> {
    lint_with(src, &Options::default())
}

/// Lint a Markdown source and return every finding, ordered by line.
pub fn lint_with(src: &str, options: &Options) -> Vec<Finding> {
    let doc = Document::parse(src);
    let mut out = rules::run(&doc, options);
    out.sort_by_key(|f| (f.line, f.column));
    out
}
