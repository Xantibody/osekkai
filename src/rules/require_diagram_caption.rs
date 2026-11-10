//! A diagram comes with a sentence or two on how to read it: what the boxes
//! and arrows stand for, what the colours mark. Without it the reader has to
//! guess what the picture is for. The caption is a paragraph right before or
//! right after the `mermaid` block; a list next to it is the section's
//! content, not a caption.

use markdown::mdast::Node;

use super::Finding;
use crate::doc::{start, walk, Document};

pub const ID: &str = "require-diagram-caption";

pub fn check(doc: &Document, out: &mut Vec<Finding>) {
    walk(&doc.root, &mut |node| {
        let Some(children) = node.children() else {
            return;
        };
        for (i, child) in children.iter().enumerate() {
            if !is_diagram(child) {
                continue;
            }
            let before = i.checked_sub(1).and_then(|j| children.get(j));
            let after = children.get(i + 1);
            if [before, after].into_iter().flatten().any(is_paragraph) {
                continue;
            }
            let (line, column) = start(child);
            out.push(Finding {
                rule: ID,
                line,
                column,
                message: "図の前か後に、読み方を 1〜2 文で書く".to_owned(),
            });
        }
    });
}

fn is_diagram(node: &Node) -> bool {
    matches!(node, Node::Code(code) if code.lang.as_deref() == Some("mermaid"))
}

fn is_paragraph(node: &Node) -> bool {
    matches!(node, Node::Paragraph(_))
}
