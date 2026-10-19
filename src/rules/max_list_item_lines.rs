//! One bullet is three lines at most. The rule counts the lines a reader
//! sees, not the lines in the source: the body lint joins soft breaks before
//! this rule runs, so a PR body is always one source line per bullet.

use markdown::mdast::Node;
use unicode_width::UnicodeWidthStr;

use super::Finding;
use crate::doc::{first_paragraph, start, walk, Document};

pub const ID: &str = "max-list-item-lines";
const MAX_LINES: usize = 3;
// AIDEV-NOTE: 40 full-width characters per rendered line is a guess at how a
// GitHub PR body wraps; plan 未決事項 4 decides the real unit.
const CELLS_PER_LINE: usize = 80;

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

fn rendered_lines(src: &str) -> usize {
    src.lines()
        .map(|l| l.trim().width().div_ceil(CELLS_PER_LINE).max(1))
        .sum()
}
