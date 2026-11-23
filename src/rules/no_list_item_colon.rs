//! `- XX: xxx` builds a hierarchy inside one bullet with a colon. The head
//! belongs in the bullet and the rest in a nested bullet, or the two become
//! one sentence.

use markdown::mdast::Node;

use super::Finding;
use crate::doc::{first_paragraph, plain_text, start, walk, Document};

pub const ID: &str = "no-list-item-colon";

pub fn check(doc: &Document, out: &mut Vec<Finding>) {
    walk(&doc.root, &mut |node| {
        let Node::ListItem(item) = node else { return };
        let Some(paragraph) = first_paragraph(item) else {
            return;
        };
        let text = plain_text(paragraph);
        let Some(idx) = separator(&text) else {
            return;
        };
        let (line, column) = start(paragraph);
        out.push(Finding {
            rule: ID,
            line,
            column,
            message: format!(
                "箇条の中をコロンで区切らない。「{}」の後ろは下位の箇条に分けるか、1 つの文にする",
                text[..idx].trim()
            ),
        });
    });
}

/// The byte index of the first colon that joins a head to a body. A colon
/// between two ASCII digits is a time or a ratio (「13:17」「3:2」), not a join.
fn separator(text: &str) -> Option<usize> {
    let bytes = text.as_bytes();
    text.match_indices([':', '：']).map(|(i, _)| i).find(|&i| {
        let digit_before = i > 0 && bytes[i - 1].is_ascii_digit();
        let digit_after = bytes.get(i + 1).is_some_and(u8::is_ascii_digit);
        !(text[i..].starts_with(':') && digit_before && digit_after)
    })
}
