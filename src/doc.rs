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
