//! One file per rule; `run` is the only entry point.

use serde::Serialize;

use crate::doc::Document;
use crate::state::State;
use crate::Options;

mod decision_first;
mod max_list_block_items;
mod max_list_item_lines;
mod max_section_lines;
mod max_top_items;
mod no_absolute_amount;
mod no_heading;
mod no_list_item_colon;
mod no_list_item_separator;
mod no_person_name;
mod no_sentence_chain_in_list_item;
mod postscript_last;
mod state_first;

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
    no_list_item_separator::check,
    no_sentence_chain_in_list_item::check,
    max_list_item_lines::check,
    max_list_block_items::check,
    max_section_lines::check,
    postscript_last::check,
];

/// Rules that a declared reader state switches on. Each one checks its own
/// trigger and stays silent for other states.
type StateRule = fn(&Document, &State, &mut Vec<Finding>);

const STATEFUL: &[StateRule] = &[
    no_person_name::check,
    no_absolute_amount::check,
    no_heading::check,
    max_top_items::check,
    decision_first::check,
];

pub fn run(doc: &Document, options: &Options) -> Vec<Finding> {
    let mut out = Vec::new();
    for rule in STRUCTURAL {
        rule(doc, &mut out);
    }
    state_first::check(doc, options, &mut out);
    if let Some(Ok(state)) = doc.declaration.as_ref().map(|d| &d.state) {
        for rule in STATEFUL {
            rule(doc, state, &mut out);
        }
    }
    out
}
