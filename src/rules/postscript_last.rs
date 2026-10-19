//! What strays from the main line goes after everything else, outside the
//! list, as a paragraph that opens with 蛇足. Anywhere else it competes with
//! the point.

use markdown::mdast::Node;

use super::Finding;
use crate::doc::{plain_text, start, walk, Document};

pub const ID: &str = "postscript-last";
const MARKER: &str = "蛇足";

pub fn check(doc: &Document, out: &mut Vec<Finding>) {
    let Some(blocks) = doc.root.children() else {
        return;
    };
    let last = blocks.len().saturating_sub(1);
    for (i, block) in blocks.iter().enumerate() {
        if is_postscript(block) && i != last {
            report(block, "蛇足は最後に置く。後ろにある本筋の前に出ている", out);
        }
    }
    walk(&doc.root, &mut |node| {
        let Node::ListItem(item) = node else { return };
        for child in &item.children {
            if is_postscript(child) {
                report(child, "蛇足は箇条の外に出し、最後の段落にする", out);
            }
        }
    });
}

fn is_postscript(node: &Node) -> bool {
    matches!(node, Node::Paragraph(_)) && plain_text(node).trim_start().starts_with(MARKER)
}

fn report(node: &Node, message: &str, out: &mut Vec<Finding>) {
    let (line, column) = start(node);
    out.push(Finding {
        rule: ID,
        line,
        column,
        message: message.to_owned(),
    });
}
