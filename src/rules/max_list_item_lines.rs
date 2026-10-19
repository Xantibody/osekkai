//! One bullet is three lines at most. The rule counts the lines a reader
//! sees, not the lines in the source: the body lint joins soft breaks before
//! this rule runs, so a PR body is always one source line per bullet.

use markdown::mdast::Node;

use super::Finding;
use crate::doc::{first_paragraph, rendered_lines, start, walk, Document};

pub const ID: &str = "max-list-item-lines";
const MAX_LINES: usize = 3;

pub fn check(doc: &Document, out: &mut Vec<Finding>) {
    walk(&doc.root, &mut |node| {
        let Node::ListItem(item) = node else { return };
        let Some(paragraph) = first_paragraph(item) else {
            return;
        };
        let lines = rendered_lines(doc.source_of(paragraph));
        if lines <= MAX_LINES {
            return;
        }
        let (line, column) = start(paragraph);
        out.push(Finding {
            rule: ID,
            line,
            column,
            message: format!(
                "1 箇条は {MAX_LINES} 行まで（見た目で {lines} 行）。圧縮するか、下位の箇条か節に分ける"
            ),
        });
    });
}
