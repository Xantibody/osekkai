//! Every `tests/fixtures/**/*.md` is linted. A sidecar `<name>.expect` lists the
//! findings the file must produce, one `rule@line` per line; no sidecar means
//! the file must be clean. Accepted outputs from `.ai/refs/examples/` live
//! under `fixtures/examples/` and go through the same check.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

fn markdown_files(dir: &Path, out: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(dir).expect("fixtures dir") {
        let path = entry.expect("dir entry").path();
        if path.is_dir() {
            markdown_files(&path, out);
        } else if path.extension().is_some_and(|e| e == "md") {
            out.push(path);
        }
    }
}

fn expected(md: &Path) -> BTreeSet<String> {
    let sidecar = md.with_extension("expect");
    fs::read_to_string(sidecar)
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
fn fixtures_produce_exactly_the_expected_findings() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures");
    let mut files = Vec::new();
    markdown_files(&root, &mut files);
    files.sort();

    let mut failures = Vec::new();
    for md in &files {
        let src = fs::read_to_string(md).expect("fixture readable");
        let actual: BTreeSet<String> = osekkai::lint(&src)
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
