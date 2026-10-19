//! A parsed Markdown document with the helpers rules need.

use markdown::mdast::Node;
use markdown::{to_mdast, ParseOptions};

pub struct Document {
    pub src: String,
    pub root: Node,
}

impl Document {
    pub fn parse(src: &str) -> Self {
        // GFM so tables, task lists and autolink literals parse the way GitHub renders PR bodies.
        let root = to_mdast(src, &ParseOptions::gfm()).expect("markdown has no syntax errors");
        Self {
            src: src.to_owned(),
            root,
        }
    }
}

/// 1-based start line and column of a node, (0, 0) when unknown.
pub fn start(node: &Node) -> (usize, usize) {
    node.position()
        .map(|p| (p.start.line, p.start.column))
        .unwrap_or((0, 0))
}

/// Depth-first visit of every node.
pub fn walk<'a>(node: &'a Node, f: &mut dyn FnMut(&'a Node)) {
    f(node);
    if let Some(children) = node.children() {
        for child in children {
            walk(child, f);
        }
    }
}

/// Concatenated text of a node, skipping code spans, raw HTML and links.
/// Links are skipped whole: an autolinked URL is a Text child of the Link,
/// and its colons and periods are not prose.
pub fn plain_text(node: &Node) -> String {
    fn collect(node: &Node, out: &mut String) {
        match node {
            Node::Text(t) => out.push_str(&t.value),
            Node::Link(_) => {}
            _ => {
                if let Some(children) = node.children() {
                    for child in children {
                        collect(child, out);
                    }
                }
            }
        }
    }
    let mut out = String::new();
    collect(node, &mut out);
    out
}

/// The paragraph that opens a list item, if any.
pub fn first_paragraph(item: &markdown::mdast::ListItem) -> Option<&Node> {
    item.children
        .iter()
        .find(|c| matches!(c, Node::Paragraph(_)))
}

impl Document {
    /// The source text a node was parsed from, empty when the position is unknown.
    pub fn source_of(&self, node: &Node) -> &str {
        node.position()
            .map(|p| &self.src[p.start.offset..p.end.offset])
            .unwrap_or("")
    }
}

// AIDEV-NOTE: 40 full-width characters per rendered line is a guess at how a
// GitHub PR body wraps; plan 未決事項 4 decides the real unit.
const CELLS_PER_LINE: usize = 80;

/// Lines a reader sees when `src` is rendered: each source line wrapped at
/// `CELLS_PER_LINE` display cells, never less than one.
pub fn rendered_lines(src: &str) -> usize {
    use unicode_width::UnicodeWidthStr;
    src.lines()
        .map(|l| l.trim().width().div_ceil(CELLS_PER_LINE).max(1))
        .sum()
}
