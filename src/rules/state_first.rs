//! The reader state is decided before anything is written, so its
//! declaration is the first block of the document. This is the part of the
//! fixed order (declare-state → draft → lint → hand-over) that the artifact
//! itself can prove.

use super::Finding;
use crate::doc::Document;
use crate::Options;

pub const ID: &str = "state-first";
/// A declaration that exists but cannot be parsed.
pub const DECLARATION_ID: &str = "state-declaration";

pub fn check(doc: &Document, options: &Options, out: &mut Vec<Finding>) {
    let Some(decl) = &doc.declaration else {
        if options.require_state {
            out.push(Finding {
                rule: ID,
                line: 1,
                column: 1,
                message: "読み手のステートが宣言されていない。<!-- osekkai: unknown=... purpose=... --> を先頭に置く".to_owned(),
            });
        }
        return;
    };
    if let Err(e) = &decl.state {
        out.push(Finding {
            rule: DECLARATION_ID,
            line: decl.line,
            column: 1,
            message: format!(
                "ステートの宣言が読めない。{e}。`osekkai state list` で次元と値を見る"
            ),
        });
    }
    if decl.index != 0 {
        out.push(Finding {
            rule: ID,
            line: decl.line,
            column: 1,
            message:
                "ステートの宣言は本文の最初のブロックに置く。読み手を決めてから書く順序を文書に残す"
                    .to_owned(),
        });
    }
}

#[cfg(test)]
mod tests {
    use crate::{lint_with, Options};

    #[test]
    fn missing_declaration_is_reported_only_when_required() {
        let src = "- 箇条\n";
        assert!(lint_with(src, &Options::default()).is_empty());
        let found = lint_with(
            src,
            &Options {
                require_state: true,
            },
        );
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].rule, super::ID);
    }
}
