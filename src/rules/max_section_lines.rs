//! A section that runs long gets split into chapters. A section is the text
//! directly under a heading, up to the next heading of any depth, so a title
//! is not charged for the chapters beneath it; the blocks before the first
//! heading form a section too.

use markdown::mdast::Node;

use super::Finding;
use crate::doc::{rendered_lines, start, Document};

pub const ID: &str = "max-section-lines";
// AIDEV-NOTE: 12 is four three-line bullets, a guess until an example shows
// where the author splits a chapter.
const MAX_LINES: usize = 12;

pub fn check(doc: &Document, out: &mut Vec<Finding>) {
    let Some(blocks) = doc.root.children() else {
        return;
    };
    let mut i = 0;
    while i < blocks.len() {
        let head = &blocks[i];
        let body_start = if is_heading(head) { i + 1 } else { i };
        let mut end = body_start;
        while end < blocks.len() && !is_heading(&blocks[end]) {
            end += 1;
        }
        let lines: usize = blocks[body_start..end]
            .iter()
            .map(|b| rendered_lines(doc.source_of(b)))
            .sum();
        if lines > MAX_LINES {
            let (line, column) = start(head);
            out.push(Finding {
                rule: ID,
                line,
                column,
                message: format!("1 節は {MAX_LINES} 行まで（見た目で {lines} 行）。章を分ける"),
            });
        }
        i = end.max(i + 1);
    }
}

fn is_heading(node: &Node) -> bool {
    matches!(node, Node::Heading(_))
}
