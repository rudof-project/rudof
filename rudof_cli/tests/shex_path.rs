//! The `shex-path` command: evaluating a ShExPath against a ShEx schema.

#![cfg(not(target_family = "wasm"))]

use serde_json::{Value, json};
use std::path::Path;
use std::process::{Command, Stdio};

struct Output {
    stdout: String,
    stderr: String,
    code: i32,
}

fn rudof_in(dir: &Path, args: &[&str]) -> Output {
    let output = Command::new(env!("CARGO_BIN_EXE_rudof"))
        .args(args)
        .current_dir(dir)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to spawn rudof")
        .wait_with_output()
        .expect("failed to wait for rudof");

    Output {
        stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
        stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
        code: output.status.code().unwrap_or(-1),
    }
}

/// Part of the example schema of the ShExPath spec.
const ISSUE_SHEX: &str = r#"PREFIX : <http://ex.example/ns#>
PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>
BASE <http://ex.example/schema>

<#IssueShape> {
  :name xsd:string MINLENGTH 4 ;
  :category ["bug" "feature request"] ;
  $<#reportedBy> :postedBy @<#UserShape>
}

<#UserShape> IRI
"#;

/// Runs `rudof shex-path` with `path` against the example schema.
fn shex_path(path: &str) -> Output {
    let dir = tempfile::tempdir().expect("tempdir");
    std::fs::write(dir.path().join("issue.shex"), ISSUE_SHEX).expect("write schema");
    rudof_in(dir.path(), &["shex-path", "-s", "issue.shex", "-p", path])
}

fn matches(output: &Output) -> Vec<Value> {
    match serde_json::from_str::<Value>(&output.stdout) {
        Ok(Value::Array(matches)) => matches,
        other => panic!("the output is not a JSON array: {other:?}\nstdout: {}", output.stdout),
    }
}

#[test]
fn prints_each_selected_element_with_its_location() {
    let output = shex_path("@<#IssueShape>~:category");
    assert_eq!(output.code, 0, "stderr: {}", output.stderr);
    let matches = matches(&output);
    assert_eq!(matches.len(), 1);
    assert_eq!(
        matches[0]["location"],
        json!("/shapes/0/shapeExpr/expression/expressions/1")
    );
    assert_eq!(matches[0]["value"]["predicate"], json!("http://ex.example/ns#category"));
}

#[test]
fn prints_atomic_values() {
    let output = shex_path("$<#reportedBy>/valueExpr");
    assert_eq!(output.code, 0, "stderr: {}", output.stderr);
    let matches = matches(&output);
    assert_eq!(matches.len(), 1);
    assert_eq!(matches[0]["value"], json!("http://ex.example/schema#UserShape"));
}

#[test]
fn prints_an_empty_array_when_nothing_is_selected() {
    let output = shex_path("@<#Nope>");
    assert_eq!(output.code, 0, "stderr: {}", output.stderr);
    assert!(matches(&output).is_empty());
}

#[test]
fn fails_on_a_path_that_is_not_a_shexpath() {
    let output = shex_path("/shapes/asdf");
    assert_ne!(output.code, 0);
    assert!(output.stderr.contains("ShExPath"), "stderr: {}", output.stderr);
}

#[test]
fn fails_when_an_assertion_does_not_hold() {
    let output = shex_path("@<#IssueShape>/expression/expressions/*[assert count() = 1]");
    assert_ne!(output.code, 0);
    assert!(output.stderr.contains("assertion"), "stderr: {}", output.stderr);
}
