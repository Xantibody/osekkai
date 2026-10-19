//! A reader who does not know the people gets what was said, not who said
//! it. The heuristic is a short run of kanji followed by an honorific; 様 is
//! included and gives a false positive on お客様, which the fixture records.

use std::sync::LazyLock;

use markdown::mdast::Node;
use regex::Regex;

use super::Finding;
use crate::doc::{plain_text, start, walk, Document};
use crate::state::{Knowledge, State};

pub const ID: &str = "no-person-name";

static NAME: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\p{Han}{1,4}(さん|氏|様)").expect("valid regex"));

pub fn check(doc: &Document, state: &State, out: &mut Vec<Finding>) {
    if state.people != Some(Knowledge::Unknown) {
        return;
    }
    walk(&doc.root, &mut |node| {
        if !matches!(node, Node::Paragraph(_) | Node::Heading(_)) {
            return;
        }
        let text = plain_text(node);
        let Some(m) = NAME.find(&text) else { return };
        let (line, column) = start(node);
        out.push(Finding {
            rule: ID,
            line,
            column,
            message: format!(
                "読み手は人物を知らない。「{}」を消し、何が言われたかだけ残す",
                m.as_str()
            ),
        });
    });
}
