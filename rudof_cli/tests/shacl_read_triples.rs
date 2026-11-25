#![cfg(not(target_family = "wasm"))]

use std::path::Path;
use std::process::{Command, Stdio};

const DATA: &str = r#"prefix : <http://a.example/>
:alice :name "Alice" ; :age 30 .
:carol :name "Carol" .
"#;

const SHAPES: &str = "prefix : <http://a.example/>
prefix sh: <http://www.w3.org/ns/shacl#>
:Person sh:targetNode :alice ; sh:property [ sh:path :name ; sh:minCount 1 ] .
";

fn rudof_in(dir: &Path, args: &[&str]) -> (bool, String, String) {
    let output = Command::new(env!("CARGO_BIN_EXE_rudof"))
        .args(args)
        .current_dir(dir)
        .stdin(Stdio::null())
        .output()
        .expect("failed to run rudof");
    (
        output.status.success(),
        String::from_utf8_lossy(&output.stdout).into_owned(),
        String::from_utf8_lossy(&output.stderr).into_owned(),
    )
}

fn fixture() -> tempfile::TempDir {
    let dir = tempfile::tempdir().expect("tempdir");
    std::fs::write(dir.path().join("data.ttl"), DATA).unwrap();
    std::fs::write(dir.path().join("shapes.ttl"), SHAPES).unwrap();
    dir
}

#[test]
fn statistics_prints_the_triples_read_on_a_line_of_its_own() {
    let dir = fixture();
    let (ok, stdout, stderr) = rudof_in(
        dir.path(),
        &["shacl-validate", "data.ttl", "-s", "shapes.ttl", "--statistics"],
    );
    assert!(ok, "{stderr}");
    assert!(stdout.lines().any(|l| l == "Triples read: 1 (distinct: 1)"), "{stdout}");
    assert!(stdout.lines().any(|l| l == "Valid triples: 1"), "{stdout}");
    assert!(stdout.lines().any(|l| l == "Invalid triples: 0"), "{stdout}");
}

/// Through the old `--validated-graph` names, aliases of `--visited`.
#[test]
fn validated_graph_writes_the_triples_read() {
    let dir = fixture();
    let (ok, _, stderr) = rudof_in(
        dir.path(),
        &[
            "validate",
            "-M",
            "shacl",
            "data.ttl",
            "-s",
            "shapes.ttl",
            "--schema-format",
            "turtle",
            "--validated-graph",
            "sub.nt",
            "--validated-graph-format",
            "ntriples",
        ],
    );
    assert!(ok, "{stderr}");
    let written = std::fs::read_to_string(dir.path().join("sub.nt")).unwrap();
    assert_eq!(
        written.lines().collect::<Vec<_>>(),
        vec![r#"<http://a.example/alice> <http://a.example/name> "Alice" ."#]
    );
}

#[test]
fn the_sparql_mode_cannot_tell_the_triples_read() {
    let dir = fixture();
    let (ok, _, stderr) = rudof_in(
        dir.path(),
        &[
            "shacl-validate",
            "data.ttl",
            "-s",
            "shapes.ttl",
            "--mode",
            "sparql",
            "--statistics",
        ],
    );
    assert!(!ok);
    assert!(stderr.contains("native engine"), "{stderr}");
}

#[test]
fn visited_in_turtle_uses_the_prefixes_of_the_data() {
    let dir = fixture();
    let (ok, _, stderr) = rudof_in(
        dir.path(),
        &["shacl-validate", "data.ttl", "-s", "shapes.ttl", "--visited", "sub.ttl"],
    );
    assert!(ok, "{stderr}");
    let written = std::fs::read_to_string(dir.path().join("sub.ttl")).unwrap();
    assert!(written.contains("prefix : <http://a.example/>"), "{written}");
    assert!(written.contains(":alice"), "{written}");
    assert!(!written.contains("<http://a.example/alice>"), "{written}");
}

/// Issue #972: the valid and invalid subgraphs of a SHACL validation.
const SUBGRAPH_SHAPES: &str = "prefix : <http://a.example/>
prefix sh: <http://www.w3.org/ns/shacl#>
prefix xsd: <http://www.w3.org/2001/XMLSchema#>
:Person sh:targetNode :alice, :carol ;
  sh:property [ sh:path :name ; sh:minCount 1 ] ;
  sh:property [ sh:path :age ; sh:minCount 1 ; sh:datatype xsd:integer ] .
";

fn subgraph_fixture() -> tempfile::TempDir {
    let dir = fixture();
    std::fs::write(dir.path().join("shapes.ttl"), SUBGRAPH_SHAPES).unwrap();
    dir
}

fn sorted_lines(dir: &Path, name: &str) -> Vec<String> {
    let text = std::fs::read_to_string(dir.join(name)).unwrap();
    let mut lines: Vec<String> = text.lines().map(str::to_owned).collect();
    lines.sort();
    lines
}

#[test]
fn valid_and_invalid_split_the_triples_read_by_the_conformant_nodes() {
    let dir = subgraph_fixture();
    let (ok, stdout, stderr) = rudof_in(
        dir.path(),
        &[
            "shacl-validate",
            "data.ttl",
            "-s",
            "shapes.ttl",
            "--statistics",
            "--valid",
            "valid.nt",
            "--valid-format",
            "ntriples",
            "--invalid",
            "invalid.nt",
            "--invalid-format",
            "ntriples",
        ],
    );
    assert!(ok, "{stderr}");
    assert_eq!(
        sorted_lines(dir.path(), "valid.nt"),
        vec![
            r#"<http://a.example/alice> <http://a.example/age> "30"^^<http://www.w3.org/2001/XMLSchema#integer> ."#,
            r#"<http://a.example/alice> <http://a.example/name> "Alice" ."#,
        ]
    );
    // :carol has no :age, so the triple read about her name is invalid
    assert_eq!(
        sorted_lines(dir.path(), "invalid.nt"),
        vec![r#"<http://a.example/carol> <http://a.example/name> "Carol" ."#]
    );
    assert!(stdout.lines().any(|l| l == "Triples read: 3 (distinct: 3)"), "{stdout}");
    assert!(stdout.lines().any(|l| l == "Valid triples: 2"), "{stdout}");
    assert!(stdout.lines().any(|l| l == "Invalid triples: 1"), "{stdout}");
}

#[test]
fn validate_writes_the_shacl_subgraphs() {
    let dir = subgraph_fixture();
    let (ok, _, stderr) = rudof_in(
        dir.path(),
        &[
            "validate",
            "-M",
            "shacl",
            "data.ttl",
            "-s",
            "shapes.ttl",
            "--schema-format",
            "turtle",
            "--valid",
            "valid.ttl",
            "--invalid",
            "invalid.ttl",
        ],
    );
    assert!(ok, "{stderr}");
    let valid = std::fs::read_to_string(dir.path().join("valid.ttl")).unwrap();
    assert!(valid.contains("prefix : <http://a.example/>"), "{valid}");
    assert!(valid.contains(":alice"), "{valid}");
    assert!(!valid.contains(":carol"), "{valid}");
    let invalid = std::fs::read_to_string(dir.path().join("invalid.ttl")).unwrap();
    assert!(invalid.contains(":carol"), "{invalid}");
}

#[test]
fn the_sparql_mode_cannot_tell_the_subgraphs() {
    let dir = subgraph_fixture();
    let (ok, _, stderr) = rudof_in(
        dir.path(),
        &[
            "shacl-validate",
            "data.ttl",
            "-s",
            "shapes.ttl",
            "--mode",
            "sparql",
            "--valid",
            "v.ttl",
        ],
    );
    assert!(!ok);
    assert!(stderr.contains("native engine"), "{stderr}");
}
