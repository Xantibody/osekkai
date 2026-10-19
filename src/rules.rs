//! One file per rule; `run` is the only entry point.

use serde::Serialize;

use crate::doc::Document;

mod max_list_block_items;
mod max_list_item_lines;
mod max_section_lines;
mod no_list_item_colon;
mod no_sentence_chain_in_list_item;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Finding {
    #[serde(rename = "ruleId")]
    pub rule: &'static str,
    pub line: usize,
    pub column: usize,
    pub message: String,
}

type Rule = fn(&Document, &mut Vec<Finding>);

const STRUCTURAL: &[Rule] = &[
    no_list_item_colon::check,
    no_sentence_chain_in_list_item::check,
    max_list_item_lines::check,
    max_list_block_items::check,
    max_section_lines::check,
];

pub fn run(doc: &Document) -> Vec<Finding> {
    let mut out = Vec::new();
    for rule in STRUCTURAL {
        rule(doc, &mut out);
    }
    out
}
