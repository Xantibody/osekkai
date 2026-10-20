//! The order the binary insists on. Four steps, nothing more: everything
//! between declaring the reader and linting the draft is the skill's to
//! arrange. A skill text carries one `<!-- osekkai-step: <id> -->` anchor per
//! step, and `check_anchors` keeps them in this order.

pub struct Step {
    pub id: &'static str,
    pub title: &'static str,
    pub detail: &'static str,
}

pub const STEPS: &[Step] = &[
    Step {
        id: "declare-state",
        title: "読み手のステートを決め、本文の先頭に宣言する",
        detail: "<!-- osekkai: unknown=... purpose=... --> を最初のブロックに置き、osekkai state show で guidance を読む",
    },
    Step {
        id: "draft",
        title: "本文を書く",
        detail: "主題の仮置き、骨組み、残す・消すの判断は skill の手順に従う",
    },
    Step {
        id: "lint",
        title: "osekkai lint が通るまで直す",
        detail: "hook も同じ検査を回し、指摘が残っていれば gh を止める",
    },
    Step {
        id: "hand-over",
        title: "渡す",
        detail: "配布先が宣言コメントを嫌うなら消してから渡す",
    },
];

const ANCHOR_PREFIX: &str = "<!-- osekkai-step:";

/// The steps as text, for the skill and the terminal.
pub fn describe() -> String {
    STEPS
        .iter()
        .enumerate()
        .map(|(i, s)| format!("{}. {} [{}]\n   {}\n", i + 1, s.title, s.id, s.detail))
        .collect()
}

/// Check that `text` carries every step anchor, in `STEPS` order.
pub fn check_anchors(text: &str) -> Result<(), String> {
    let found: Vec<&str> = text
        .match_indices(ANCHOR_PREFIX)
        .filter_map(|(i, _)| {
            let rest = &text[i + ANCHOR_PREFIX.len()..];
            rest.split("-->").next().map(str::trim)
        })
        .collect();
    let mut expected = STEPS.iter().map(|s| s.id);
    for id in &found {
        match expected.next() {
            Some(want) if want == *id => {}
            Some(want) => return Err(format!("アンカー「{id}」の前に「{want}」が要る")),
            None => return Err(format!("アンカー「{id}」は余分か、順序が違う")),
        }
    }
    match expected.next() {
        Some(missing) => Err(format!("アンカー「{missing}」が無い")),
        None => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn anchors(ids: &[&str]) -> String {
        ids.iter()
            .map(|id| format!("text\n<!-- osekkai-step: {id} -->\n"))
            .collect()
    }

    #[test]
    fn all_steps_in_order_pass() {
        let ids: Vec<&str> = STEPS.iter().map(|s| s.id).collect();
        assert_eq!(check_anchors(&anchors(&ids)), Ok(()));
    }

    #[test]
    fn missing_step_is_named() {
        let err = check_anchors(&anchors(&["declare-state", "draft", "hand-over"])).unwrap_err();
        assert!(err.contains("lint"), "{err}");
    }

    #[test]
    fn out_of_order_is_rejected() {
        let err =
            check_anchors(&anchors(&["draft", "declare-state", "lint", "hand-over"])).unwrap_err();
        assert!(err.contains("declare-state"), "{err}");
    }

    #[test]
    fn empty_text_lacks_the_first_step() {
        assert!(check_anchors("").unwrap_err().contains("declare-state"));
    }
}
