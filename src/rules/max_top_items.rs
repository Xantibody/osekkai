//! A reader who only needs to know what is going on gets a handful of
//! top-level bullets. The limit is what the accepted third-party summary
//! used; nested detail is counted by max-list-block-items instead.

use markdown::mdast::Node;

use super::Finding;
use crate::doc::{start, Document};
use crate::state::{Purpose, State};

pub const ID: &str = "max-top-items";
const MAX_TOP_ITEMS: usize = 4;

pub fn check(doc: &Document, state: &State, out: &mut Vec<Finding>) {
    if state.purpose != Some(Purpose::Inform) {
        return;
    }
    let Some(blocks) = doc.root.children() else {
        return;
    };
    let mut seen = 0;
    for block in blocks {
        let Node::List(list) = block else { continue };
        for item in &list.children {
            seen += 1;
            if seen == MAX_TOP_ITEMS + 1 {
                let (line, column) = start(item);
                out.push(Finding {
                    rule: ID,
                    line,
                    column,
                    message: format!(
                        "知っておくだけの読み手には最上位の項目を {MAX_TOP_ITEMS} つまでにする。判断の材料は消す"
                    ),
                });
            }
        }
    }
}
