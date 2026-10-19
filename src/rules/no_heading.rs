//! A reader who only needs to know what is going on gets a few bullets and
//! no chapters. A single top-level heading is the title and stays.

use markdown::mdast::Node;

use super::Finding;
use crate::doc::{start, Document};
use crate::state::{Purpose, State};

pub const ID: &str = "no-heading";

pub fn check(doc: &Document, state: &State, out: &mut Vec<Finding>) {
    if state.purpose != Some(Purpose::Inform) {
        return;
    }
    let Some(blocks) = doc.root.children() else {
        return;
    };
    for block in blocks {
        let Node::Heading(h) = block else { continue };
        if h.depth == 1 {
            continue;
        }
        let (line, column) = start(block);
        out.push(Finding {
            rule: ID,
            line,
            column,
            message: "知っておくだけの読み手には章を立てない。何が起きているかを数項目の箇条にする"
                .to_owned(),
        });
    }
}
