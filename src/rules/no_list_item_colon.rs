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
        let Some(idx) = text.find([':', '：']) else {
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
