//! A reader who has to decide gets the request and its deadline first. The
//! first block after the declaration and the title must carry both.

use std::sync::LazyLock;

use markdown::mdast::Node;
use regex::Regex;

use super::Finding;
use crate::doc::{plain_text, start, Document};
use crate::state::{Purpose, State};

pub const ID: &str = "decision-first";

// A bare 月 or 日 matched 毎月 and 今日, so a date needs a number in front of it.
static DEADLINE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"までに|前に|期限なし|[0-9０-９]+ ?[月日]|[上中下]旬|月末").expect("valid regex")
});
static REQUEST: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"ほしい|決め|ください").expect("valid regex"));

pub fn check(doc: &Document, state: &State, out: &mut Vec<Finding>) {
    if state.purpose != Some(Purpose::Decide) {
        return;
    }
    let Some(blocks) = doc.root.children() else {
        return;
    };
    let Some(first) = blocks.iter().find(|b| is_body(b)) else {
        return;
    };
    let text = match first {
        Node::List(list) => list.children.first().map(plain_text).unwrap_or_default(),
        other => plain_text(other),
    };
    let missing = match (DEADLINE.is_match(&text), REQUEST.is_match(&text)) {
        (true, true) => return,
        (false, true) => "期限",
        (true, false) => "決めてほしいこと",
        (false, false) => "決めてほしいことと期限",
    };
    let (line, column) = start(first);
    out.push(Finding {
        rule: ID,
        line,
        column,
        message: format!("判断する読み手には{missing}を先頭に置く。経緯や案はその後ろ"),
    });
}

/// The declaration and a title heading come before the body; everything else is body.
fn is_body(block: &Node) -> bool {
    match block {
        Node::Html(_) => false,
        Node::Heading(h) => h.depth != 1,
        _ => true,
    }
}
