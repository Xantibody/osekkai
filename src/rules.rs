//! One file per rule; `run` is the only entry point.

use serde::Serialize;

use crate::doc::Document;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Finding {
    #[serde(rename = "ruleId")]
    pub rule: &'static str,
    pub line: usize,
    pub column: usize,
    pub message: String,
}

type Rule = fn(&Document, &mut Vec<Finding>);

const STRUCTURAL: &[Rule] = &[];

pub fn run(doc: &Document) -> Vec<Finding> {
    let mut out = Vec::new();
    for rule in STRUCTURAL {
        rule(doc, &mut out);
    }
    out
}
