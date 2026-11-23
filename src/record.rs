//! A record in `~/.osekkai/`: how one summary came to be, written by the
//! skill as it works. The binary checks the record's own shape at
//! hand-over, never the conversation that produced it.

use crate::rules::Finding;

pub const HEADING: &str = "record-heading";
pub const HEADER: &str = "record-header";
pub const LOG: &str = "record-log";
pub const DELIVERED: &str = "record-delivered";
pub const ROUND: &str = "record-round";

const HEADER_KEYS: &[&str] = &["reader", "source", "destination"];

/// Findings for a record named `file_name` (`YYYY-MM-DD-<slug>.md`).
pub fn check(file_name: &str, src: &str) -> Vec<Finding> {
    let lines = Lines::new(src);
    let mut out = Vec::new();
    heading(file_name, src, &mut out);
    header(&lines, &mut out);
    log(&lines, &mut out);
    delivered(&lines, &mut out);
    rounds(&lines, &mut out);
    out.sort_by_key(|f| (f.line, f.column));
    out
}

/// Each `## Round <n>` runs to the next round or the end, and holds the
/// reader's words, its own log with a lint line, and the text handed over
/// again in full.
fn rounds(lines: &Lines, out: &mut Vec<Finding>) {
    let starts: Vec<usize> = lines.find(is_round).collect();
    for (i, &start) in starts.iter().enumerate() {
        let end = starts.get(i + 1).copied().unwrap_or(lines.last() + 1);
        let first =
            |want: &str| (start + 1..end).find(|&n| !lines.is_fenced(n) && lines.get(n) == want);
        let log = first("### Log");
        let delivered = first("### Delivered");
        let has = |prefix: &str, from: usize, to: usize| {
            (from..to).any(|n| lines.get(n).starts_with(prefix))
        };
        let after = |from: usize| {
            [log, delivered]
                .into_iter()
                .flatten()
                .filter(|&n| n > from)
                .min()
        };
        let mut missing = Vec::new();
        let header_end = [log, delivered].into_iter().flatten().min().unwrap_or(end);
        if !has("- feedback", start + 1, header_end) {
            missing.push("「- feedback:」");
        }
        match log {
            None => missing.push("### Log"),
            Some(l) if !has("- lint:", l + 1, after(l).unwrap_or(end)) => {
                missing.push("### Log の「- lint:」")
            }
            Some(_) => {}
        }
        match delivered {
            None => missing.push("### Delivered"),
            Some(d) => lint_texts(lines, d + 1, after(d).unwrap_or(end), out),
        }
        if let (Some(l), Some(d)) = (log, delivered) {
            if l > d {
                missing.push("### Delivered より前の ### Log");
            }
        }
        if missing.is_empty() {
            continue;
        }
        out.push(Finding {
            rule: ROUND,
            line: start,
            column: 1,
            message: format!("{} に{} が要る", lines.get(start), missing.join("、")),
        });
    }
}

/// `## Delivered` runs to the first `## Round <n>` or the end. Only those
/// two lines bound it, so the delivered text's own `##` headings stay
/// inside. It holds one text, or one `### <reader>` per reader, each
/// opening with its declaration; each text passes the lint as the skill
/// runs it.
fn delivered(lines: &Lines, out: &mut Vec<Finding>) {
    let Some(start) = lines.find(|t| t == "## Delivered").next() else {
        out.push(Finding {
            rule: DELIVERED,
            line: 1,
            column: 1,
            message: "## Delivered の節が要る。渡した本文を宣言ごと全文で置く".to_owned(),
        });
        return;
    };
    let misplaced_log = lines.find(|t| t == "## Log").find(|&n| n > start);
    if let Some(line) = misplaced_log {
        out.push(Finding {
            rule: LOG,
            line,
            column: 1,
            message: "## Log は ## Delivered の前に置く".to_owned(),
        });
    }
    let end = lines
        .find(is_round)
        .chain(misplaced_log)
        .filter(|&n| n > start)
        .min()
        .unwrap_or(lines.last() + 1);
    if (start + 1..end).all(|n| lines.get(n).trim().is_empty()) {
        out.push(Finding {
            rule: DELIVERED,
            line: start,
            column: 1,
            message: "## Delivered が空。渡した本文を宣言ごと全文で置く".to_owned(),
        });
        return;
    }
    lint_texts(lines, start + 1, end, out);
}

/// Lint the text, or each reader's text, between `from` and `to`.
fn lint_texts(lines: &Lines, from: usize, to: usize, out: &mut Vec<Finding>) {
    let readers: Vec<usize> = (from..to)
        .filter(|&n| {
            !lines.is_fenced(n)
                && lines.get(n).starts_with("### ")
                && declaration_follows(lines, n + 1, to)
        })
        .collect();
    if readers.is_empty() {
        lint_text(lines, from, to, out);
        return;
    }
    for (i, &head) in readers.iter().enumerate() {
        let next = readers.get(i + 1).copied().unwrap_or(to);
        lint_text(lines, head + 1, next, out);
    }
}

fn declaration_follows(lines: &Lines, from: usize, to: usize) -> bool {
    (from..to)
        .map(|n| lines.get(n))
        .find(|t| !t.trim().is_empty())
        .is_some_and(|t| t.starts_with("<!-- osekkai:"))
}

fn lint_text(lines: &Lines, from: usize, to: usize, out: &mut Vec<Finding>) {
    let text: String = (from..to).map(|n| format!("{}\n", lines.get(n))).collect();
    let options = crate::Options {
        require_state: true,
    };
    out.extend(
        crate::lint_with(&text, &options)
            .into_iter()
            .map(|f| Finding {
                line: f.line + from - 1,
                ..f
            }),
    );
}

fn is_round(line: &str) -> bool {
    line.strip_prefix("## Round ")
        .is_some_and(|n| !n.is_empty() && n.bytes().all(|b| b.is_ascii_digit()))
}

/// The record's lines, 1-based, with the lines inside a fenced code block
/// marked: a delivered text may quote a record, and its quoted headings are
/// not this record's structure.
struct Lines<'a> {
    text: Vec<&'a str>,
    fenced: Vec<bool>,
}

impl<'a> Lines<'a> {
    fn new(src: &'a str) -> Self {
        let text: Vec<&str> = src.lines().collect();
        let mut fenced = Vec::with_capacity(text.len());
        // The fence that opened the current block: ``` or ~~~, as CommonMark
        // allows both and only the same kind closes it.
        let mut open: Option<&str> = None;
        for line in &text {
            let trimmed = line.trim_start();
            let fence = ["```", "~~~"].into_iter().find(|f| trimmed.starts_with(f));
            fenced.push(open.is_some() || fence.is_some());
            match (open, fence) {
                (None, Some(f)) => open = Some(f),
                (Some(o), Some(f)) if o == f => open = None,
                _ => {}
            }
        }
        Self { text, fenced }
    }

    fn is_fenced(&self, line: usize) -> bool {
        self.fenced[line - 1]
    }

    /// Line numbers (1-based) of the lines outside fences for which `f` holds.
    fn find(&self, f: impl Fn(&str) -> bool) -> std::vec::IntoIter<usize> {
        self.text
            .iter()
            .zip(&self.fenced)
            .enumerate()
            .filter(|(_, (t, fenced))| !**fenced && f(t.trim_end()))
            .map(|(i, _)| i + 1)
            .collect::<Vec<_>>()
            .into_iter()
    }

    fn get(&self, line: usize) -> &str {
        self.text[line - 1].trim_end()
    }

    fn last(&self) -> usize {
        self.text.len().max(1)
    }
}

/// `- reader:`, `- source:` and `- destination:` stand between the heading
/// and the first `##`.
fn header(lines: &Lines, out: &mut Vec<Finding>) {
    let end = lines
        .find(|t| t.starts_with("## "))
        .next()
        .unwrap_or(lines.last());
    let missing: Vec<&str> = HEADER_KEYS
        .iter()
        .copied()
        .filter(|key| !(1..end).any(|n| lines.get(n).starts_with(&format!("- {key}:"))))
        .collect();
    if missing.is_empty() {
        return;
    }
    out.push(Finding {
        rule: HEADER,
        line: end,
        column: 1,
        message: format!(
            "最初の ## の前に「- {}:」の行が要る",
            missing.join(":」「- ")
        ),
    });
}

/// `## Log` holds the attempts and a `- lint:` line, which tells a clean
/// run from a skipped one.
fn log(lines: &Lines, out: &mut Vec<Finding>) {
    let Some(start) = lines.find(|t| t == "## Log").next() else {
        let line = lines.find(|t| t.starts_with("## ")).next().unwrap_or(1);
        out.push(Finding {
            rule: LOG,
            line,
            column: 1,
            message: "## Log の節が要る".to_owned(),
        });
        return;
    };
    let end = lines
        .find(|t| t.starts_with("## "))
        .find(|&n| n > start)
        .unwrap_or(lines.last() + 1);
    if (start + 1..end).any(|n| lines.get(n).starts_with("- lint:")) {
        return;
    }
    out.push(Finding {
        rule: LOG,
        line: start,
        column: 1,
        message: "## Log に「- lint:」の行が要る。指摘がなくても、通ったことを書く".to_owned(),
    });
}

/// The first line is `# <slug>`: the file name without its date and `.md`.
fn heading(file_name: &str, src: &str, out: &mut Vec<Finding>) {
    let want = format!("# {}", slug(file_name));
    if src.lines().next().map(str::trim_end) == Some(want.as_str()) {
        return;
    }
    out.push(Finding {
        rule: HEADING,
        line: 1,
        column: 1,
        message: format!("記録の見出しは「{want}」。ファイル名から日付と .md を除いた slug"),
    });
}

fn slug(file_name: &str) -> &str {
    let stem = file_name.strip_suffix(".md").unwrap_or(file_name);
    match stem.get(..11) {
        Some(prefix) if is_date_prefix(prefix) => &stem[11..],
        _ => stem,
    }
}

/// `YYYY-MM-DD-`.
fn is_date_prefix(prefix: &str) -> bool {
    prefix.bytes().enumerate().all(|(i, b)| match i {
        4 | 7 | 10 => b == b'-',
        _ => b.is_ascii_digit(),
    })
}
