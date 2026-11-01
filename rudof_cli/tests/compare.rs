//! Integration tests for `compare --mode data` (RDF graph comparison), and
//! that `compare` still compares schemas by default.

#![cfg(not(target_family = "wasm"))]

use std::path::Path;
use std::process::{Command, Stdio};

struct Output {
    stdout: String,
    stderr: String,
    success: bool,
}

fn rudof(args: &[&str]) -> Output {
    let output = Command::new(env!("CARGO_BIN_EXE_rudof"))
        .args(args)
        .stdin(Stdio::null())
        .env_remove("FORCE_COLOR")
        .env_remove("CLICOLOR_FORCE")
        .output()
        .expect("failed to run rudof");
    Output {
        stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
        stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
        success: output.status.success(),
    }
}

fn fixture(name: &str) -> String {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
        .to_string_lossy()
        .into_owned()
}

/// Runs `rudof compare --mode data --data1 FIRST --data2 SECOND EXTRA...`
fn compare(first: &str, second: &str, extra: &[&str]) -> Output {
    let mut args = vec!["compare", "--mode", "data", "--data1", first, "--data2", second];
    args.extend_from_slice(extra);
    let out = rudof(&args);
    assert!(out.success, "rudof {args:?} failed:\n{}\n{}", out.stdout, out.stderr);
    out
}

#[test]
fn isomorphic_graphs_in_different_formats() {
    let (a, b) = (fixture("compare_alice.ttl"), fixture("compare_alice_renamed.nt"));
    let out = compare(&a, &b, &["--format2", "ntriples"]);
    assert!(out.stdout.starts_with("isomorphic:"), "{}", out.stdout);
    assert!(out.stdout.contains("(mode: injective)"), "{}", out.stdout);
}

#[test]
fn subgraph_and_supergraph() {
    let (small, big) = (fixture("compare_alice.ttl"), fixture("compare_alice_more.ttl"));
    assert!(compare(&small, &big, &[]).stdout.starts_with("subgraph:"));
    assert!(compare(&big, &small, &[]).stdout.starts_with("supergraph:"));
}

#[test]
fn mode_flag_changes_containment() {
    let (general, specific) = (fixture("compare_alice.ttl"), fixture("compare_alice_bob.ttl"));
    assert!(compare(&general, &specific, &[]).stdout.starts_with("different:"));
    let out = compare(&general, &specific, &["--subgraph-mode", "homomorphic"]);
    assert!(out.stdout.starts_with("different:"), "{}", out.stdout);
    let out = compare(&general, &specific, &["--subgraph-mode", "entailment"]);
    assert!(out.stdout.starts_with("subgraph:"), "{}", out.stdout);
    assert!(out.stdout.contains("(mode: entailment)"), "{}", out.stdout);
}

#[test]
fn json_result_format() {
    let (general, specific) = (fixture("compare_alice.ttl"), fixture("compare_alice_bob.ttl"));
    let out = compare(&general, &specific, &["--subgraph-mode", "entailment", "-r", "json"]);
    let json: serde_json::Value = serde_json::from_str(&out.stdout).expect("valid JSON");
    assert_eq!(json["relation"], "subgraph");
    assert_eq!(json["mode"], "entailment");
}

#[test]
fn unknown_mode_is_rejected() {
    let a = fixture("compare_alice.ttl");
    let out = rudof(&[
        "compare",
        "--mode",
        "data",
        "--data1",
        &a,
        "--data2",
        &a,
        "--subgraph-mode",
        "subset",
    ]);
    assert!(!out.success);
    assert!(out.stderr.contains("injective"), "{}", out.stderr);
}

#[test]
fn a_single_graph_needs_loaded_data() {
    let a = fixture("compare_alice.ttl");
    let out = rudof(&["compare", "--mode", "data", "--data2", &a]);
    assert!(!out.success);
    assert!(out.stderr.contains("loaded data"), "{}", out.stderr);
}

#[test]
fn schema_format_is_rejected_in_data_mode() {
    let a = fixture("compare_alice.ttl");
    let out = rudof(&[
        "compare",
        "--mode",
        "data",
        "--data1",
        &a,
        "--data2",
        &a,
        "--format1",
        "shexc",
    ]);
    assert!(!out.success);
    assert!(out.stderr.contains("not an RDF format"), "{}", out.stderr);
}

#[test]
fn schema_mode_is_still_the_default() {
    let schema = r#"PREFIX : <http://example.org/>
PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>
:Person { :name xsd:string }
"#;
    let dir = tempfile::tempdir().expect("temp dir");
    let path = dir.path().join("person.shex");
    std::fs::write(&path, schema).expect("write schema");
    let path = path.to_string_lossy().into_owned();
    let out = rudof(&[
        "compare",
        "--schema1",
        &path,
        "--schema2",
        &path,
        "--shape1",
        "http://example.org/Person",
        "--shape2",
        "http://example.org/Person",
    ]);
    assert!(out.success, "{}\n{}", out.stdout, out.stderr);
    assert!(out.stdout.contains("Shapes Comparison"), "{}", out.stdout);
}

#[test]
fn schema_mode_needs_schema1() {
    let a = fixture("compare_alice.ttl");
    let out = rudof(&["compare", "--schema2", &a]);
    assert!(!out.success);
    assert!(out.stderr.contains("--schema1"), "{}", out.stderr);
}
