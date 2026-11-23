//! Every `tests/records/**/*.md` is checked as a record of `~/.osekkai/`.
//! The file name matters, since the heading must match its slug. A sidecar
//! `<name>.expect` lists the findings, one `rule@line` per line; no sidecar
//! means the record must be clean.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

fn markdown_files(dir: &Path, out: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(dir).expect("records dir") {
        let path = entry.expect("dir entry").path();
        if path.is_dir() {
            markdown_files(&path, out);
        } else if path.extension().is_some_and(|e| e == "md") {
            out.push(path);
        }
    }
}

fn expected(md: &Path) -> BTreeSet<String> {
    fs::read_to_string(md.with_extension("expect"))
        .map(|s| {
            s.lines()
                .map(str::trim)
                .filter(|l| !l.is_empty())
                .map(String::from)
                .collect()
        })
        .unwrap_or_default()
}

#[test]
fn records_produce_exactly_the_expected_findings() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/records");
    let mut files = Vec::new();
    markdown_files(&root, &mut files);
    files.sort();

    let mut failures = Vec::new();
    for md in &files {
        let src = fs::read_to_string(md).expect("record readable");
        let name = md.file_name().unwrap().to_str().unwrap();
        let actual: BTreeSet<String> = osekkai::record::check(name, &src)
            .iter()
            .map(|f| format!("{}@{}", f.rule, f.line))
            .collect();
        let want = expected(md);
        if actual != want {
            failures.push(format!(
                "{}\n  expected: {:?}\n  actual:   {:?}",
                md.strip_prefix(&root).unwrap().display(),
                want,
                actual
            ));
        }
    }
    assert!(failures.is_empty(), "\n{}", failures.join("\n"));
}
