//! A list block stays small enough to take in at a glance. Two readings of
//! the author's OK / NG pair are both close, so either fires: the total
//! under one top-level item, and the sibling count shrinking with depth
//! (3 children, 2 grandchildren, 1 great-grandchild).

use markdown::mdast::{List, Node};

use super::Finding;
use crate::doc::{start, Document};

pub const ID: &str = "max-list-block-items";
const MAX_DESCENDANTS: usize = 5;
const SIBLING_BASE: usize = 4;

pub fn check(doc: &Document, out: &mut Vec<Finding>) {
    top_level_lists(&doc.root, &mut |list| visit(list, 0, out));
}

fn top_level_lists<'a>(node: &'a Node, f: &mut dyn FnMut(&'a List)) {
    match node {
        Node::List(list) => f(list),
        _ => {
            if let Some(children) = node.children() {
                for child in children {
                    top_level_lists(child, f);
                }
            }
        }
    }
}

fn visit(list: &List, depth: usize, out: &mut Vec<Finding>) {
    if depth > 0 {
        let limit = SIBLING_BASE.saturating_sub(depth);
        if list.children.len() > limit {
            let (line, column) = start(&list.children[0]);
            out.push(Finding {
                rule: ID,
                line,
                column,
                message: format!(
                    "深さ {depth} の箇条は {limit} 項目まで（{} 項目）。まとめるか、上の階層か節に分ける",
                    list.children.len()
                ),
            });
        }
    }
    for item in &list.children {
        let Node::ListItem(li) = item else { continue };
        if depth == 0 {
            let n = descendants(item);
            if n > MAX_DESCENDANTS {
                let (line, column) = start(item);
                out.push(Finding {
                    rule: ID,
                    line,
                    column,
                    message: format!(
                        "1 つの親の下は合計 {MAX_DESCENDANTS} 項目まで（{n} 項目）。まとめるか、節に分ける"
                    ),
                });
            }
        }
        for child in &li.children {
            if let Node::List(nested) = child {
                visit(nested, depth + 1, out);
            }
        }
    }
}

fn descendants(item: &Node) -> usize {
    let mut n = 0;
    if let Some(children) = item.children() {
        for child in children {
            if let Node::List(list) = child {
                for nested in &list.children {
                    n += 1 + descendants(nested);
                }
            }
        }
    }
    n
}
