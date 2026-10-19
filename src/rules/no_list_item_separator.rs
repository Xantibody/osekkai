//! `foo — 説明` squeezes a heading and a body into one bullet. Ported from
//! the dotfiles textlint rule of the same name; the bold-label form is left
//! to textlint's no-ai-list-formatting there, and to no-list-item-colon here.

use std::sync::LazyLock;

use markdown::mdast::Node;
use regex::Regex;

use super::Finding;
use crate::doc::{first_paragraph, start, walk, Document};

pub const ID: &str = "no-list-item-separator";

static SEPARATOR: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(?:\[[ xX]\]\s*)?(.{1,60}?)\s+([—–-])\s+").expect("valid regex")
});

pub fn check(doc: &Document, out: &mut Vec<Finding>) {
    walk(&doc.root, &mut |node| {
        let Node::ListItem(item) = node else { return };
        let Some(paragraph) = first_paragraph(item) else {
            return;
        };
        let first_line = doc.source_of(paragraph).lines().next().unwrap_or("");
        let Some(m) = SEPARATOR.captures(first_line) else {
            return;
        };
        let (line, column) = start(paragraph);
        out.push(Finding {
            rule: ID,
            line,
            column,
            message: format!(
                "「{}」の後ろを「{}」で区切らず、1 つの文にするか下位の箇条に分ける",
                &m[1], &m[2]
            ),
        });
    });
}
