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
mod no_heading;
mod no_list_item_colon;
mod no_list_item_separator;
mod no_sentence_chain_in_list_item;
mod no_sentence_chain_in_table_cell;
mod no_unknown_term;
mod require_diagram_caption;
mod require_heading;
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
    no_sentence_chain_in_table_cell::check,
    max_list_item_lines::check,
    max_list_block_items::check,
    max_section_lines::check,
    max_top_items::check,
    require_diagram_caption::check,
];

/// Rules that a declared reader state switches on. Each one checks its own
/// trigger and stays silent for other states.
type StateRule = fn(&Document, &State, &mut Vec<Finding>);

const STATEFUL: &[StateRule] = &[
    no_unknown_term::check,
    no_heading::check,
    decision_first::check,
    require_heading::check,
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

/// The density thresholds, one rule per line, so a writer can plan the
/// grouping before the lint runs. The numbers stay in the rule files.
pub fn limits() -> String {
    [
        (max_list_item_lines::ID, max_list_item_lines::limit()),
        (max_list_block_items::ID, max_list_block_items::limit()),
        (max_section_lines::ID, max_section_lines::limit()),
        (max_top_items::ID, max_top_items::limit()),
    ]
    .iter()
    .map(|(id, limit)| format!("{id}: {limit}\n"))
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn limits_name_every_density_rule_with_its_threshold() {
        let text = limits();
        for want in [
            "max-list-item-lines: 1 箇条は 3 行まで",
            "max-list-block-items: 1 つの親の下は合計 5 項目まで",
            "深さ 1 は 3 項目、深さ 2 は 2 項目、深さ 3 は 1 項目まで",
            "max-section-lines: 1 節は 13 行まで",
            "max-top-items: 1 節の最上位の項目は 4 つまで",
        ] {
            assert!(text.contains(want), "{want} missing from:\n{text}");
        }
    }
}
