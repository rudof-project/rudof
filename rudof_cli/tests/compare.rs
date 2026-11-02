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

// Diffs

#[test]
fn show_diff_lists_the_triples_only_in_each_graph() {
    let (bob, robert) = (fixture("compare_bob.ttl"), fixture("compare_robert.ttl"));
    let out = compare(&bob, &robert, &["--show-diff"]);
    let expected = "different: neither the first graph nor the second graph is contained in the other (mode: injective)\n\
                    PREFIX : <http://example.org/>\n\
                    Only in the first graph (1):\n  _:b :name \"Bob\" .\n\
                    Only in the second graph (1):\n  _:c :name \"Robert\" .\n\
                    Blank nodes of the first graph matched in the second graph:\n  _:b = _:c\n";
    assert_eq!(out.stdout, expected);
}

#[test]
fn show_diff_of_a_subgraph() {
    let (small, big) = (fixture("compare_alice.ttl"), fixture("compare_alice_more.ttl"));
    let out = compare(&small, &big, &["--show-diff"]);
    assert!(out.stdout.contains("Only in the first graph (0):"), "{}", out.stdout);
    assert!(
        out.stdout
            .contains("Only in the second graph (1):\n  :alice :name \"Alice\" .\n"),
        "{}",
        out.stdout
    );
    assert!(!out.stdout.contains("approximate"), "{}", out.stdout);
}

#[test]
fn show_diff_as_json() {
    let (bob, robert) = (fixture("compare_bob.ttl"), fixture("compare_robert.ttl"));
    let out = compare(&bob, &robert, &["--show-diff", "-r", "json"]);
    let json: serde_json::Value = serde_json::from_str(&out.stdout).expect("valid JSON");
    assert_eq!(json["relation"], "different");
    assert_eq!(json["exact"], true);
    assert_eq!(
        json["only_in_first"],
        serde_json::json!(["_:b <http://example.org/name> \"Bob\""])
    );
    assert_eq!(
        json["only_in_second"],
        serde_json::json!(["_:c <http://example.org/name> \"Robert\""])
    );
    assert_eq!(json["first_to_second"], serde_json::json!([["_:b", "_:c"]]));
}

#[test]
fn show_diff_with_entailment_mode() {
    let (general, specific) = (fixture("compare_alice.ttl"), fixture("compare_alice_bob.ttl"));
    let out = compare(&general, &specific, &["--show-diff", "--subgraph-mode", "entailment"]);
    assert!(out.stdout.starts_with("subgraph:"), "{}", out.stdout);
    assert!(out.stdout.contains("Only in the second graph (0):"), "{}", out.stdout);
    assert!(out.stdout.contains(" = :bob\n"), "{}", out.stdout);
}

#[test]
fn approximate_diffs_are_reported() {
    let dir = tempfile::tempdir().expect("temp dir");
    let write = |name: &str, content: String| {
        let path = dir.path().join(name);
        std::fs::write(&path, format!("@prefix : <http://example.org/> .\n{content}")).expect("write");
        path.to_string_lossy().into_owned()
    };
    let lookalikes = |p: &str| (0..20).map(|i| format!("_:{p}{i} :p :o .\n")).collect::<String>();
    let first = write("first.ttl", lookalikes("a") + "_:a0 :q 1 .\n_:a1 :r 1 .\n");
    let second = write("second.ttl", lookalikes("b") + "_:b7 :q 2 .\n_:b3 :r 2 .\n");
    let out = compare(&first, &second, &["--show-diff", "--exact-search-limit", "10"]);
    assert!(out.stdout.contains("The diff is approximate"), "{}", out.stdout);
    let out = compare(&first, &second, &["--show-diff"]);
    assert!(!out.stdout.contains("approximate"), "{}", out.stdout);
}

// The mode is inferred from the options

/// Runs `rudof compare ARGS...` and returns its output, whether it succeeds or not
fn compare_raw(args: &[&str]) -> Output {
    let mut all = vec!["compare"];
    all.extend_from_slice(args);
    rudof(&all)
}

#[test]
fn data_options_imply_the_data_mode() {
    let (small, big) = (fixture("compare_alice.ttl"), fixture("compare_alice_more.ttl"));
    let out = compare_raw(&["--data1", &small, "--data2", &big]);
    assert!(out.success, "{}", out.stderr);
    assert!(out.stdout.starts_with("subgraph:"), "{}", out.stdout);
}

#[test]
fn schema_options_imply_the_schema_mode() {
    let dir = tempfile::tempdir().expect("temp dir");
    let path = dir.path().join("person.shex");
    std::fs::write(
        &path,
        "PREFIX : <http://example.org/>\nPREFIX xsd: <http://www.w3.org/2001/XMLSchema#>\n:Person { :name xsd:string }\n",
    )
    .expect("write schema");
    let path = path.to_string_lossy().into_owned();
    let label = "http://example.org/Person";
    let out = compare_raw(&[
        "--schema1",
        &path,
        "--schema2",
        &path,
        "--shape1",
        label,
        "--shape2",
        label,
    ]);
    assert!(out.success, "{}", out.stderr);
    assert!(out.stdout.contains("Shapes Comparison"), "{}", out.stdout);
}

#[test]
fn schema_and_data_options_cannot_be_mixed() {
    let a = fixture("compare_alice.ttl");
    let out = compare_raw(&["--schema1", &a, "--data2", &a]);
    assert!(!out.success);
    assert!(out.stderr.contains("not both"), "{}", out.stderr);
}

#[test]
fn explicit_mode_must_match_the_options() {
    let a = fixture("compare_alice.ttl");
    let out = compare_raw(&["--mode", "schema", "--data1", &a, "--data2", &a]);
    assert!(!out.success);
    assert!(
        out.stderr.contains("--data1 and --data2 compare RDF data"),
        "{}",
        out.stderr
    );
    let out = compare_raw(&["--mode", "data", "--schema1", &a, "--schema2", &a]);
    assert!(!out.success);
    assert!(
        out.stderr.contains("--schema1 and --schema2 compare schemas"),
        "{}",
        out.stderr
    );
}

#[test]
fn compare_needs_some_inputs() {
    let out = compare_raw(&[]);
    assert!(!out.success);
    assert!(out.stderr.contains("--schema1 and --schema2"), "{}", out.stderr);
    assert!(out.stderr.contains("--data1 and --data2"), "{}", out.stderr);
}
