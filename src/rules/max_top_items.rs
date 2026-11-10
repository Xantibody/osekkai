//! Every section shows a handful of top-level bullets, and the detail goes
//! beneath them. The author cannot follow a text without a layer of major
//! items over the concrete ones; a flat run of bullets has no such layer, so
//! past the limit the bullets are grouped under parents or split into
//! sections. Headings reset the count, since a section is itself the upper
//! layer. The limit is what the accepted third-party summary used; nested
//! detail is counted by max-list-block-items instead.

use markdown::mdast::Node;

use super::Finding;
use crate::doc::{start, Document};

pub const ID: &str = "max-top-items";
const MAX_TOP_ITEMS: usize = 4;

pub fn check(doc: &Document, out: &mut Vec<Finding>) {
    let Some(blocks) = doc.root.children() else {
        return;
    };
    let mut seen = 0;
    for block in blocks {
        match block {
            Node::Heading(_) => seen = 0,
            Node::List(list) => {
                for item in &list.children {
                    seen += 1;
                    if seen == MAX_TOP_ITEMS + 1 {
                        let (line, column) = start(item);
                        out.push(Finding {
                            rule: ID,
                            line,
                            column,
                            message: format!(
                                "1 節の最上位の項目は {MAX_TOP_ITEMS} つまで。大項目で束ねるか、節に分ける"
                            ),
                        });
                    }
                }
            }
            _ => {}
        }
    }
}
