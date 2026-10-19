//! `- XXX。YYY。` chains sentences inside one bullet. A trailing period is
//! textlint's business (period-in-list-item); this rule looks for a period
//! with more text after it.

use markdown::mdast::Node;

use super::Finding;
use crate::doc::{first_paragraph, plain_text, start, walk, Document};

pub const ID: &str = "no-sentence-chain-in-list-item";

pub fn check(doc: &Document, out: &mut Vec<Finding>) {
    walk(&doc.root, &mut |node| {
        let Node::ListItem(item) = node else { return };
        let Some(paragraph) = first_paragraph(item) else {
            return;
        };
        let text = plain_text(paragraph);
        let body = text.trim_end().trim_end_matches('。');
        if !body.contains('。') {
            return;
        }
        let (line, column) = start(paragraph);
        out.push(Finding {
            rule: ID,
            line,
            column,
            message: "箇条の中で「。」で文を重ねない。1 つの文にするか、下位の箇条に分ける"
                .to_owned(),
        });
    });
}
