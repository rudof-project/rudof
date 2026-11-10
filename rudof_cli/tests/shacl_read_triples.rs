#![cfg(not(target_family = "wasm"))]

use std::path::Path;
use std::process::{Command, Stdio};

const DATA: &str = r#"prefix : <http://a.example/>
:alice :name "Alice" ; :age 30 .
:carol :name "Carol" .
"#;

const SHAPES: &str = r#"prefix : <http://a.example/>
prefix sh: <http://www.w3.org/ns/shacl#>
:Person sh:targetNode :alice ; sh:property [ sh:path :name ; sh:minCount 1 ] .
"#;

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
}

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
