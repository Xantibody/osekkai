//! The reader state: what the reader already knows and what they will do
//! with the text. The binary owns only this vocabulary. A reader is declared
//! per document as a combination of values, never catalogued here, so adding
//! readers adds nothing to this file.
//!
//! Each value carries two kinds of effect. `guidance` needs the meaning of
//! the text and is printed for the skill to follow; `rules` can be judged
//! from characters and structure and are enabled in the lint.
//!
//! What the reader knows is one term list, not a dimension per kind of
//! thing. People, systems and amounts were once three `known`/`unknown`
//! dimensions; they were the examples the first two documents happened to
//! contain, and each new document would have added another.

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

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct State {
    /// Terms the reader does not know, as the writer listed them for this
    /// document. Which words those are cannot be enumerated here: a name,
    /// a product and a figure are all the same question to the reader.
    pub unknown: Vec<String>,
    /// Whether the reader knows why this topic is up now. Not a term, so
    /// it stays a dimension of its own.
    pub history: Option<Knowledge>,
    pub purpose: Option<Purpose>,
}

/// A dimension of the state and the values it can take.
pub struct Dimension {
    pub key: &'static str,
    pub label: &'static str,
    pub kind: Kind,
}

pub enum Kind {
    /// One of a fixed set of values.
    Choice(&'static [Value]),
    /// A comma-separated list the writer fills per document; every list
    /// carries the same effect.
    Terms(Value),
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
        key: "unknown",
        label: "知らない語",
        kind: Kind::Terms(Value {
            key: "<語>,<語>",
            label: "知らない語",
            guidance: "宣言した語を本文に出さない。言い換えるか、説明に置き換えるか、消す。名前を残して一言説明を添える語は宣言に入れない",
            rules: &["no-unknown-term"],
        }),
    },
    Dimension {
        key: "history",
        label: "経緯",
        kind: Kind::Choice(&[
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
        ]),
    },
    Dimension {
        key: "purpose",
        label: "目的",
        kind: Kind::Choice(&[
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
        ]),
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
    /// A term list with nothing in it.
    NoTerms(String),
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
            Self::NoTerms(k) => write!(f, "「{k}」に語がない。コンマ区切りで列挙する"),
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

/// `key=value` pairs of a declaration. A token without a key continues the
/// value before it, so a term such as 「840 万」 may hold a space.
fn pairs(decl: &str) -> Result<Vec<(&str, String)>, ParseError> {
    let mut out: Vec<(&str, String)> = Vec::new();
    for token in decl.split_whitespace() {
        let key = token
            .split_once('=')
            .filter(|(k, _)| !k.is_empty() && k.bytes().all(|b| b.is_ascii_lowercase()));
        match (key, out.last_mut()) {
            (Some((k, v)), _) => out.push((k, v.to_owned())),
            (None, Some((_, v))) => {
                v.push(' ');
                v.push_str(token);
            }
            (None, None) => return Err(ParseError::Malformed(token.to_owned())),
        }
    }
    Ok(out)
}

fn terms(dimension: &str, value: &str) -> Result<Vec<String>, ParseError> {
    let terms: Vec<String> = value
        .split([',', '、'])
        .map(str::trim)
        .filter(|t| !t.is_empty())
        .map(String::from)
        .collect();
    if terms.is_empty() {
        return Err(ParseError::NoTerms(dimension.to_owned()));
    }
    Ok(terms)
}

impl State {
    /// Parse `unknown=鈴木,APM history=known purpose=inform`. Dimensions
    /// left out stay undeclared and enable nothing.
    pub fn parse(decl: &str) -> Result<Self, ParseError> {
        let mut state = Self::default();
        for (key, value) in pairs(decl)? {
            state.set(key, &value)?;
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
            "unknown" => {
                if !self.unknown.is_empty() {
                    return Err(ParseError::Duplicate(key.to_owned()));
                }
                self.unknown = terms(key, value)?;
                Ok(())
            }
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
    pub fn declared(&self) -> Vec<Declared> {
        DIMENSIONS
            .iter()
            .filter_map(|d| match &d.kind {
                Kind::Terms(value) if d.key == "unknown" && !self.unknown.is_empty() => {
                    Some(Declared {
                        dimension: d,
                        value,
                        written: self.unknown.join(","),
                    })
                }
                Kind::Terms(_) => None,
                Kind::Choice(values) => {
                    let key = self.value_key(d.key)?;
                    let value = values
                        .iter()
                        .find(|v| v.key == key)
                        .expect("declared value is defined");
                    Some(Declared {
                        dimension: d,
                        value,
                        written: key.to_owned(),
                    })
                }
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

/// One declared dimension: its value's effects, and what the writer wrote
/// after the `=`.
pub struct Declared {
    pub dimension: &'static Dimension,
    pub value: &'static Value,
    pub written: String,
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
        for Declared {
            dimension: d,
            value: v,
            written,
        } in declared
        {
            let shown = match d.kind {
                Kind::Choice(_) => v.label.to_owned(),
                Kind::Terms(_) => written.replace(',', ", "),
            };
            out.push_str(&format!(
                "{} = {}（{}={}）\n  {}\n",
                d.label, shown, d.key, written, v.guidance
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
            let values = match d.kind {
                Kind::Choice(values) => values
                    .iter()
                    .map(|v| format!("{}（{}）", v.key, v.label))
                    .collect::<Vec<_>>()
                    .join(" / "),
                Kind::Terms(ref v) => format!("{}（コンマ区切りで列挙）", v.key),
            };
            format!("{}（{}）: {}\n", d.key, d.label, values)
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
        let s = State::parse("unknown=鈴木,APM history=known purpose=decide").unwrap();
        assert_eq!(s.unknown, vec!["鈴木", "APM"]);
        assert_eq!(s.history, Some(Knowledge::Known));
        assert_eq!(s.purpose, Some(Purpose::Decide));
    }

    #[test]
    fn a_term_may_hold_a_space_and_a_japanese_comma() {
        let s = State::parse("unknown=840 万,Grafana Cloud、鈴木 purpose=inform").unwrap();
        assert_eq!(s.unknown, vec!["840 万", "Grafana Cloud", "鈴木"]);
        assert_eq!(s.purpose, Some(Purpose::Inform));
    }

    #[test]
    fn rejects_an_empty_term_list() {
        assert_eq!(
            State::parse("unknown="),
            Err(ParseError::NoTerms("unknown".into()))
        );
        assert_eq!(
            State::parse("unknown=, ,"),
            Err(ParseError::NoTerms("unknown".into()))
        );
        assert_eq!(
            State::parse("unknown=a unknown=b"),
            Err(ParseError::Duplicate("unknown".into()))
        );
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
            State::parse("history"),
            Err(ParseError::Malformed("history".into()))
        );
        assert_eq!(
            State::parse("history=known history=unknown"),
            Err(ParseError::Duplicate("history".into()))
        );
    }

    #[test]
    fn declared_lists_values_with_their_effects() {
        let s = State::parse("purpose=inform unknown=鈴木,840 万").unwrap();
        let declared = s.declared();
        let keys: Vec<_> = declared
            .iter()
            .map(|d| (d.dimension.key, d.written.as_str()))
            .collect();
        assert_eq!(
            keys,
            vec![("unknown", "鈴木,840 万"), ("purpose", "inform")]
        );
        assert_eq!(declared[0].value.rules, &["no-unknown-term"]);
    }

    #[test]
    fn describe_shows_the_terms_as_written() {
        let s = State::parse("unknown=鈴木,840 万").unwrap();
        let text = s.describe();
        assert!(
            text.starts_with("知らない語 = 鈴木, 840 万（unknown=鈴木,840 万）\n"),
            "{text}"
        );
        assert!(text.contains("lint: no-unknown-term"), "{text}");
    }

    #[test]
    fn every_choice_value_parses_back_to_itself() {
        for d in DIMENSIONS {
            let Kind::Choice(values) = d.kind else {
                continue;
            };
            for v in values {
                let s = State::parse(&format!("{}={}", d.key, v.key)).unwrap();
                assert_eq!(s.declared()[0].value.key, v.key, "{}={}", d.key, v.key);
            }
        }
    }
}
