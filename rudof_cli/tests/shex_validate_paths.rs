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
    rudof_in(
        dir,
        &[
            "shex-validate",
            "data.ttl",
            "-s",
            "schema.shex",
            "-m",
            "shapemap.sm",
            "-r",
            result_format,
        ],
    )
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
    // The table wraps long lines: read its cells as one text.
    let text: String = stdout.chars().filter(|c| !c.is_whitespace() && *c != '│').collect();
    // the first of the two constraints on :category
    assert!(
        text.contains("predicate:categoryat@<#IssueShape>~:category/self::[0]required"),
        "{stdout}"
    );
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
