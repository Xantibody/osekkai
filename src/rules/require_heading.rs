//! A reader who has to decide gets the options and the open questions as
//! separate chapters, so each can be found without reading the other.

use markdown::mdast::Node;

use super::Finding;
use crate::doc::{plain_text, Document};
use crate::state::{Purpose, State};

pub const ID: &str = "require-heading";
const REQUIRED: &[&str] = &["案", "未確定"];

pub fn check(doc: &Document, state: &State, out: &mut Vec<Finding>) {
    if state.purpose != Some(Purpose::Decide) {
        return;
    }
    let headings: Vec<String> = doc
        .root
        .children()
        .into_iter()
        .flatten()
        .filter(|b| matches!(b, Node::Heading(_)))
        .map(plain_text)
        .collect();
    let missing: Vec<&str> = REQUIRED
        .iter()
        .copied()
        .filter(|want| !headings.iter().any(|h| h.contains(want)))
        .collect();
    if missing.is_empty() {
        return;
    }
    out.push(Finding {
        rule: ID,
        line: 1,
        column: 1,
        message: format!(
            "判断する読み手には「{}」の章を分けて置く",
            missing.join("」「")
        ),
    });
}
