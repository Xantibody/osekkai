//! The reader state: what the reader already knows and what they will do
//! with the text. The binary owns only this vocabulary. A reader is declared
//! per document as a combination of values, never catalogued here, so adding
//! readers adds nothing to this file.
//!
//! Each value carries two kinds of effect. `guidance` needs the meaning of
//! the text and is printed for the skill to follow; `rules` can be judged
//! from characters and structure and are enabled in the lint.

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Knowledge {
    Known,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Purpose {
    /// 知っておく
    Inform,
    /// 判断する
    Decide,
    /// 手を動かす
    Act,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct State {
    pub people: Option<Knowledge>,
    pub system: Option<Knowledge>,
    pub cost: Option<Knowledge>,
    pub history: Option<Knowledge>,
    pub purpose: Option<Purpose>,
}

/// A dimension of the state and the values it can take.
pub struct Dimension {
    pub key: &'static str,
    pub label: &'static str,
    pub values: &'static [Value],
}

/// One value of a dimension with what it changes in the output.
pub struct Value {
    pub key: &'static str,
    pub label: &'static str,
    pub guidance: &'static str,
    pub rules: &'static [&'static str],
}

const KNOWN: &str = "known";
const UNKNOWN: &str = "unknown";

pub const DIMENSIONS: &[Dimension] = &[
    Dimension {
        key: "people",
        label: "人物",
        values: &[
            Value {
                key: KNOWN,
                label: "知っている",
                guidance: "名前だけで通じる。意見が割れていれば発言者を残す",
                rules: &[],
            },
            Value {
                key: UNKNOWN,
                label: "知らない",
                guidance: "名前を消し、何が言われたかだけ残す",
                rules: &["no-person-name"],
            },
        ],
    },
    Dimension {
        key: "system",
        label: "システム",
        values: &[
            Value {
                key: KNOWN,
                label: "知っている",
                guidance: "固有名詞のまま",
                rules: &[],
            },
            Value {
                key: UNKNOWN,
                label: "知らない",
                guidance: "一言説明を添える。説明しないと通じない細部は丸ごと消す",
                rules: &[],
            },
        ],
    },
    Dimension {
        key: "cost",
        label: "費用感",
        values: &[
            Value {
                key: KNOWN,
                label: "知っている",
                guidance: "案の金額だけ並べる。現状値は書かない",
                rules: &[],
            },
            Value {
                key: UNKNOWN,
                label: "知らない",
                guidance: "比で書く（今の半分ほど）",
                rules: &["no-absolute-amount"],
            },
        ],
    },
    Dimension {
        key: "history",
        label: "経緯",
        values: &[
            Value {
                key: KNOWN,
                label: "知っている",
                guidance: "結論と差分だけ",
                rules: &[],
            },
            Value {
                key: UNKNOWN,
                label: "知らない",
                guidance: "なぜ今この話をしているかを書く",
                rules: &[],
            },
        ],
    },
    Dimension {
        key: "purpose",
        label: "目的",
        values: &[
            Value {
                key: "inform",
                label: "知っておく",
                guidance: "何が起きているかを数項目。章なし。判断の材料は消す",
                rules: &["no-heading", "max-top-items"],
            },
            Value {
                key: "decide",
                label: "判断する",
                guidance: "決めてほしいことと期限を先頭。案と未確定事項を章に分ける",
                rules: &["decision-first", "require-heading"],
            },
            Value {
                key: "act",
                label: "手を動かす",
                guidance: "例題がまだない",
                rules: &[],
            },
        ],
    },
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    /// `key=value` was expected.
    Malformed(String),
    UnknownDimension(String),
    UnknownValue {
        dimension: String,
        value: String,
    },
    Duplicate(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Malformed(s) => write!(f, "「{s}」は key=value の形ではない"),
            Self::UnknownDimension(k) => write!(f, "「{k}」という次元は定義されていない"),
            Self::UnknownValue { dimension, value } => {
                write!(f, "「{dimension}」に「{value}」という値はない")
            }
            Self::Duplicate(k) => write!(f, "「{k}」が二度宣言されている"),
        }
    }
}

impl std::error::Error for ParseError {}

fn knowledge(dimension: &str, value: &str) -> Result<Knowledge, ParseError> {
    match value {
        KNOWN => Ok(Knowledge::Known),
        UNKNOWN => Ok(Knowledge::Unknown),
        _ => Err(ParseError::UnknownValue {
            dimension: dimension.to_owned(),
            value: value.to_owned(),
        }),
    }
}

impl State {
    /// Parse `people=unknown cost=known purpose=inform`. Dimensions left out
    /// stay undeclared and enable nothing.
    pub fn parse(decl: &str) -> Result<Self, ParseError> {
        let mut state = Self::default();
        for pair in decl.split_whitespace() {
            let Some((key, value)) = pair.split_once('=') else {
                return Err(ParseError::Malformed(pair.to_owned()));
            };
            state.set(key, value)?;
        }
        Ok(state)
    }

    fn set(&mut self, key: &str, value: &str) -> Result<(), ParseError> {
        fn put<T>(slot: &mut Option<T>, key: &str, v: T) -> Result<(), ParseError> {
            if slot.is_some() {
                return Err(ParseError::Duplicate(key.to_owned()));
            }
            *slot = Some(v);
            Ok(())
        }
        match key {
            "people" => put(&mut self.people, key, knowledge(key, value)?),
            "system" => put(&mut self.system, key, knowledge(key, value)?),
            "cost" => put(&mut self.cost, key, knowledge(key, value)?),
            "history" => put(&mut self.history, key, knowledge(key, value)?),
            "purpose" => {
                let purpose = match value {
                    "inform" => Purpose::Inform,
                    "decide" => Purpose::Decide,
                    "act" => Purpose::Act,
                    _ => {
                        return Err(ParseError::UnknownValue {
                            dimension: key.to_owned(),
                            value: value.to_owned(),
                        })
                    }
                };
                put(&mut self.purpose, key, purpose)
            }
            _ => Err(ParseError::UnknownDimension(key.to_owned())),
        }
    }

    /// The declared value of each dimension, in `DIMENSIONS` order.
    pub fn declared(&self) -> Vec<(&'static Dimension, &'static Value)> {
        DIMENSIONS
            .iter()
            .filter_map(|d| self.value_key(d.key).map(|k| (d, k)))
            .map(|(d, k)| {
                (
                    d,
                    d.values
                        .iter()
                        .find(|v| v.key == k)
                        .expect("declared value is defined"),
                )
            })
            .collect()
    }

    fn value_key(&self, dimension: &str) -> Option<&'static str> {
        fn k(v: Option<Knowledge>) -> Option<&'static str> {
            v.map(|v| match v {
                Knowledge::Known => KNOWN,
                Knowledge::Unknown => UNKNOWN,
            })
        }
        match dimension {
            "people" => k(self.people),
            "system" => k(self.system),
            "cost" => k(self.cost),
            "history" => k(self.history),
            "purpose" => self.purpose.map(|p| match p {
                Purpose::Inform => "inform",
                Purpose::Decide => "decide",
                Purpose::Act => "act",
            }),
            _ => None,
        }
    }
}

impl State {
    /// What the declared values ask of the writer: guidance to follow and
    /// the lint rules they switch on. Printed by `osekkai state show`.
    pub fn describe(&self) -> String {
        let declared = self.declared();
        if declared.is_empty() {
            return "宣言された次元がない。osekkai state list で次元と値を見る\n".to_owned();
        }
        let mut out = String::new();
        for (d, v) in declared {
            out.push_str(&format!(
                "{} = {}（{}={}）\n  {}\n",
                d.label, v.label, d.key, v.key, v.guidance
            ));
            if !v.rules.is_empty() {
                out.push_str(&format!("  lint: {}\n", v.rules.join(", ")));
            }
        }
        out
    }
}

/// Every dimension with its values, one line each. Printed by `osekkai state list`.
pub fn list() -> String {
    DIMENSIONS
        .iter()
        .map(|d| {
            let values: Vec<String> = d
                .values
                .iter()
                .map(|v| format!("{}（{}）", v.key, v.label))
                .collect();
            format!("{}（{}）: {}\n", d.key, d.label, values.join(" / "))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_declaration_sets_nothing() {
        assert_eq!(State::parse("").unwrap(), State::default());
    }

    #[test]
    fn parses_each_dimension() {
        let s =
            State::parse("people=unknown system=known cost=unknown history=known purpose=decide")
                .unwrap();
        assert_eq!(s.people, Some(Knowledge::Unknown));
        assert_eq!(s.system, Some(Knowledge::Known));
        assert_eq!(s.cost, Some(Knowledge::Unknown));
        assert_eq!(s.history, Some(Knowledge::Known));
        assert_eq!(s.purpose, Some(Purpose::Decide));
    }

    #[test]
    fn rejects_unknown_dimension_and_value() {
        assert_eq!(
            State::parse("mood=happy"),
            Err(ParseError::UnknownDimension("mood".into()))
        );
        assert_eq!(
            State::parse("purpose=known"),
            Err(ParseError::UnknownValue {
                dimension: "purpose".into(),
                value: "known".into()
            })
        );
        assert_eq!(
            State::parse("people"),
            Err(ParseError::Malformed("people".into()))
        );
        assert_eq!(
            State::parse("people=known people=unknown"),
            Err(ParseError::Duplicate("people".into()))
        );
    }

    #[test]
    fn declared_lists_values_with_their_effects() {
        let s = State::parse("purpose=inform people=unknown").unwrap();
        let declared = s.declared();
        let keys: Vec<_> = declared.iter().map(|(d, v)| (d.key, v.key)).collect();
        assert_eq!(keys, vec![("people", "unknown"), ("purpose", "inform")]);
        assert_eq!(declared[0].1.rules, &["no-person-name"]);
    }

    #[test]
    fn every_dimension_value_parses_back_to_itself() {
        for d in DIMENSIONS {
            for v in d.values {
                let s = State::parse(&format!("{}={}", d.key, v.key)).unwrap();
                assert_eq!(s.declared()[0].1.key, v.key, "{}={}", d.key, v.key);
            }
        }
    }
}
