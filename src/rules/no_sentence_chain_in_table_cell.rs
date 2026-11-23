//! A table cell holds one point, like a bullet. Once the bullets are
//! checked for chained sentences, a table is where the second sentence goes
//! instead, so a cell gets the same test as a bullet's first paragraph.

use markdown::mdast::Node;

use super::no_sentence_chain_in_list_item::chains;
use super::Finding;
use crate::doc::{plain_text, start, walk, Document};

pub const ID: &str = "no-sentence-chain-in-table-cell";

pub fn check(doc: &Document, out: &mut Vec<Finding>) {
    walk(&doc.root, &mut |node| {
        let Node::TableCell(_) = node else { return };
        if !chains(&plain_text(node)) {
            return;
        }
        let (line, column) = start(node);
        out.push(Finding {
            rule: ID,
            line,
            column,
            message: "表のセルの中で文を重ねない。1 つの点にするか、箇条に移す".to_owned(),
        });
    });
}
