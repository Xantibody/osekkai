//! A term the reader does not know must not reach them as it stands. The
//! writer lists those terms in the declaration; whether each one is
//! reworded, replaced by its explanation or cut is the skill's call, so the
//! rule only finds the ones still in the text.

use markdown::mdast::Node;

use super::Finding;
use crate::doc::{plain_text, start, walk, Document};
use crate::state::State;

pub const ID: &str = "no-unknown-term";

pub fn check(doc: &Document, state: &State, out: &mut Vec<Finding>) {
    if state.unknown.is_empty() {
        return;
    }
    walk(&doc.root, &mut |node| {
        if !matches!(node, Node::Paragraph(_) | Node::Heading(_)) {
            return;
        }
        let text = plain_text(node);
        for term in state.unknown.iter().filter(|t| text.contains(t.as_str())) {
            let (line, column) = start(node);
            out.push(Finding {
                rule: ID,
                line,
                column,
                message: format!(
                    "読み手は「{term}」を知らない。言い換えるか、説明に置き換えるか、消す"
                ),
            });
        }
    });
}
