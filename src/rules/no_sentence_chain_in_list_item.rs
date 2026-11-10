//! `- XXX。YYY。` chains sentences inside one bullet. A trailing period is
//! textlint's business (period-in-list-item); this rule looks for a period
//! with more text after it.
//!
//! The same chain hides in two more shapes: a second paragraph under the
//! bullet, and a 「、」 followed by a word that opens a new sentence
//! (「、なので」). Both are what a writer reaches for once the 「。」 is
//! flagged, so both count as a chain.

use markdown::mdast::Node;

use super::Finding;
use crate::doc::{plain_text, start, walk, Document};

pub const ID: &str = "no-sentence-chain-in-list-item";

/// Words that open a sentence of their own. A clause joined with 「が」 or
/// 「ため」 stays one sentence, so only words that read as a fresh start
/// after a 「、」 are listed.
const SENTENCE_OPENERS: &[&str] = &[
    "なので",
    "だから",
    "そのため",
    "ただし",
    "しかし",
    "ところが",
];

pub fn check(doc: &Document, out: &mut Vec<Finding>) {
    walk(&doc.root, &mut |node| {
        let Node::ListItem(item) = node else { return };
        let paragraphs: Vec<&Node> = item
            .children
            .iter()
            .filter(|c| matches!(c, Node::Paragraph(_)))
            .collect();
        let Some(first) = paragraphs.first() else {
            return;
        };
        if paragraphs.len() < 2 && !chains(&plain_text(first)) {
            return;
        }
        let (line, column) = start(first);
        out.push(Finding {
            rule: ID,
            line,
            column,
            message: "箇条の中で文を重ねない。1 つの文にするか、下位の箇条に分ける".to_owned(),
        });
    });
}

fn chains(text: &str) -> bool {
    let body = text.trim_end().trim_end_matches('。');
    body.contains('。')
        || SENTENCE_OPENERS
            .iter()
            .any(|w| body.contains(&format!("、{w}")))
}
