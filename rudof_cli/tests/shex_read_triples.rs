#![cfg(not(target_family = "wasm"))]

use std::path::Path;
use std::process::{Command, Stdio};

const DATA: &str = r#"prefix : <http://a.example/>
:alice :name "Alice" .
:bob :knows :alice .
:carol :name "Carol" .
"#;

const SCHEMA: &str = r#"prefix : <http://a.example/>
:Person { :name . ; ^:knows . * }
"#;

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
    dir
}

const ALICE: [&str; 4] = ["-n", "<http://a.example/alice>", "-l", "<http://a.example/Person>"];

#[test]
fn statistics_prints_the_triples_read() {
    let dir = fixture();
    let mut args = vec!["shex-validate", "data.ttl", "-s", "schema.shex", "--statistics"];
    args.extend(ALICE);
    let stdout = rudof_in(dir.path(), &args);
    assert!(stdout.contains("Triples read: 2 (distinct: 2)"), "{stdout}");
}

#[test]
fn validated_graph_writes_the_triples_read() {
    let dir = fixture();
    let mut args = vec![
        "validate",
        "data.ttl",
        "-s",
        "schema.shex",
        "--validated-graph",
        "sub.nt",
        "--validated-graph-format",
        "ntriples",
    ];
    args.extend(ALICE);
    rudof_in(dir.path(), &args);
    let written = std::fs::read_to_string(dir.path().join("sub.nt")).unwrap();
    let mut lines: Vec<&str> = written.lines().collect();
    lines.sort();
    assert_eq!(
        lines,
        vec![
            r#"<http://a.example/alice> <http://a.example/name> "Alice" ."#,
            "<http://a.example/bob> <http://a.example/knows> <http://a.example/alice> .",
        ]
    );
}
