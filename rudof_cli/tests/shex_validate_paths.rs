//! `shex-validate` names each failure with a ShExPath of the schema, and `shex-path` selects with
//! it the triple constraint that failed.

#![cfg(not(target_family = "wasm"))]

use serde_json::Value;
use std::path::Path;
use std::process::{Command, Stdio};

const SCHEMA: &str = r#"PREFIX : <http://ex.example/ns#>
PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>
BASE <http://ex.example/schema>

<#IssueShape> {
  :name xsd:string ;
  :category ["bug" "feature request"] ;
  :category xsd:string ?
}
"#;

const DATA: &str = r#"prefix : <http://ex.example/ns#>
:issue1 :name "Crash" ; :category "question" .
"#;

const SHAPEMAP: &str = "<http://ex.example/ns#issue1>@<http://ex.example/schema#IssueShape>\n";

fn rudof_in(dir: &Path, args: &[&str]) -> String {
    let output = Command::new(env!("CARGO_BIN_EXE_rudof"))
        .args(args)
        .current_dir(dir)
        .stdin(Stdio::null())
        .output()
        .expect("failed to run rudof");
    assert!(
        output.status.success(),
        "rudof failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8_lossy(&output.stdout).into_owned()
}

fn fixture() -> tempfile::TempDir {
    let dir = tempfile::tempdir().expect("tempdir");
    std::fs::write(dir.path().join("data.ttl"), DATA).unwrap();
    std::fs::write(dir.path().join("schema.shex"), SCHEMA).unwrap();
    std::fs::write(dir.path().join("shapemap.sm"), SHAPEMAP).unwrap();
    dir
}

fn validate(dir: &Path, result_format: &str) -> String {
    validate_with(dir, &["-r", result_format])
}

fn validate_with(dir: &Path, options: &[&str]) -> String {
    let mut args = vec!["shex-validate", "data.ttl", "-s", "schema.shex", "-m", "shapemap.sm"];
    args.extend_from_slice(options);
    rudof_in(dir, &args)
}

/// Every `path` in a JSON value, at any depth.
fn paths(value: &Value, out: &mut Vec<String>) {
    match value {
        Value::Object(map) => {
            for (key, v) in map {
                match (key.as_str(), v) {
                    ("path", Value::String(path)) => out.push(path.clone()),
                    _ => paths(v, out),
                }
            }
        },
        Value::Array(vs) => vs.iter().for_each(|v| paths(v, out)),
        _ => {},
    }
}

#[test]
fn the_details_name_the_failing_triple_constraint() {
    let dir = fixture();
    let stdout = validate(dir.path(), "details");
    // The details are written below the table, each in one line, so the path is not cut. It is
    // the first of the two constraints on :category.
    assert!(
        stdout.contains(
            "\n  :issue1 does not conform to <#IssueShape>: property :category: expected exactly 1 value, \
             found 0 (1st :category constraint, at @<#IssueShape>~:category/self::[0])\n"
        ),
        "{stdout}"
    );
    // The row of the table sums it up, without repeating the node and the shape. How much of the
    // summary fits depends on the width of the terminal (80 columns when there is none, as in CI),
    // so only its start, which fits in the narrowest summary, is checked.
    assert!(stdout.contains("│ property :category"), "{stdout}");
}

#[test]
fn the_table_layout_writes_the_details_in_the_table() {
    let dir = fixture();
    let stdout = validate_with(dir.path(), &["-r", "details", "--details-layout", "table"]);
    assert!(stdout.contains("│ Details"), "{stdout}");
    assert!(!stdout.contains("│ Summary"), "{stdout}");
    // Nothing is written below the table.
    assert!(!stdout.contains("\n  :issue1 does not conform to"), "{stdout}");
    assert!(stdout.trim_end().ends_with('╯'), "{stdout}");
}

#[test]
fn the_json_path_selects_the_failing_triple_constraint() {
    let dir = fixture();
    let result: Value = serde_json::from_str(&validate(dir.path(), "json")).expect("JSON result");
    let mut found = Vec::new();
    paths(&result, &mut found);
    assert_eq!(found.len(), 1, "{result}");
    let path = &found[0];
    assert!(path.ends_with("~:category/self::[0]"), "{path}");

    let selected: Value =
        serde_json::from_str(&rudof_in(dir.path(), &["shex-path", "-s", "schema.shex", "-p", path])).expect("JSON");
    let selected = selected.as_array().expect("matches");
    assert_eq!(selected.len(), 1, "{selected:?}");
    assert_eq!(selected[0]["value"]["type"], "TripleConstraint");
    assert_eq!(selected[0]["value"]["predicate"], "http://ex.example/ns#category");
    assert!(selected[0]["value"]["valueExpr"]["values"].is_array(), "{selected:?}");
}
