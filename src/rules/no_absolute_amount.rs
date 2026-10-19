//! A reader with no feel for the budget gets a ratio (今の半分ほど), not a
//! figure. Any number followed by a currency unit counts as a figure.

use std::sync::LazyLock;

use markdown::mdast::Node;
use regex::Regex;

use super::Finding;
use crate::doc::{plain_text, start, walk, Document};
use crate::state::{Knowledge, State};

pub const ID: &str = "no-absolute-amount";

static AMOUNT: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\d[\d,.]*\s*(万|億|円|USD|ドル|\$)").expect("valid regex"));

pub fn check(doc: &Document, state: &State, out: &mut Vec<Finding>) {
    if state.cost != Some(Knowledge::Unknown) {
        return;
    }
    walk(&doc.root, &mut |node| {
        if !matches!(node, Node::Paragraph(_) | Node::Heading(_)) {
            return;
        }
        let text = plain_text(node);
        let Some(m) = AMOUNT.find(&text) else { return };
        let (line, column) = start(node);
        out.push(Finding {
            rule: ID,
            line,
            column,
            message: format!(
                "読み手は費用感を知らない。「{}」を比（今の半分ほど）で書く",
                m.as_str().trim()
            ),
        });
    });
}
