#![cfg(not(target_family = "wasm"))]

use std::path::Path;
use std::process::{Command, Stdio};

const DATA: &str = r#"prefix : <http://a.example/>
:alice :name "Alice" .
:bob :knows :alice .
:carol :name "Carol" .
"#;

const SCHEMA: &str = "prefix : <http://a.example/>
:Person { :name . ; ^:knows . * }
";

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

/// `--validated-graph` and `--validated-graph-format`, the names before
/// `--visited` and `--visited-format`, still work.
#[test]
fn the_old_validated_graph_options_are_aliases_of_visited() {
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
    lines.sort_unstable();
    assert_eq!(
        lines,
        vec![
            r#"<http://a.example/alice> <http://a.example/name> "Alice" ."#,
            "<http://a.example/bob> <http://a.example/knows> <http://a.example/alice> .",
        ]
    );
}

/// Issue #957: Bob has two names, so he does not conform to `:Person`.
const ISSUE_957_DATA: &str = r#"prefix : <http://a.example/>
:alice :name "Alice" ; :email "alice@example.org" .
:bob :name "Robert", "Bob" ; :email "bob@example.org" .
"#;

const ISSUE_957_SCHEMA: &str = "prefix : <http://a.example/>
prefix xsd: <http://www.w3.org/2001/XMLSchema#>
:Person { :name xsd:string }
";

const ISSUE_957_SHAPEMAP: &str = "<http://a.example/alice>@<http://a.example/Person>,\
<http://a.example/bob>@<http://a.example/Person>\n";

const ALICE_NAME: &str = r#"<http://a.example/alice> <http://a.example/name> "Alice" ."#;
const BOB_NAMES: [&str; 2] = [
    r#"<http://a.example/bob> <http://a.example/name> "Bob" ."#,
    r#"<http://a.example/bob> <http://a.example/name> "Robert" ."#,
];

/// Runs `command` on the issue #957 example with `extra` arguments and returns
/// its stdout and the directory it ran in.
fn issue_957_run_with(command: &str, extra: &[&str]) -> (String, tempfile::TempDir) {
    let dir = tempfile::tempdir().expect("tempdir");
    std::fs::write(dir.path().join("data.ttl"), ISSUE_957_DATA).unwrap();
    std::fs::write(dir.path().join("schema.shex"), ISSUE_957_SCHEMA).unwrap();
    std::fs::write(dir.path().join("map.sm"), ISSUE_957_SHAPEMAP).unwrap();
    let mut args = vec![command, "data.ttl", "-s", "schema.shex", "-m", "map.sm"];
    args.extend(extra);
    let stdout = rudof_in(dir.path(), &args);
    (stdout, dir)
}

fn issue_957_run(extra: &[&str]) -> (String, tempfile::TempDir) {
    issue_957_run_with("shex-validate", extra)
}

/// The sorted lines of the file `name` in `dir`.
fn sorted_lines(dir: &tempfile::TempDir, name: &str) -> Vec<String> {
    let written = std::fs::read_to_string(dir.path().join(name)).unwrap();
    let mut lines: Vec<String> = written.lines().map(String::from).collect();
    lines.sort();
    lines
}

#[test]
fn visited_writes_every_triple_read() {
    for command in ["shex-validate", "validate"] {
        let (_, dir) = issue_957_run_with(command, &["--visited", "visited.nt", "--visited-format", "ntriples"]);
        let mut expected = vec![ALICE_NAME.to_owned()];
        expected.extend(BOB_NAMES.map(String::from));
        assert_eq!(sorted_lines(&dir, "visited.nt"), expected, "{command}");
    }
}

#[test]
fn valid_writes_only_the_triples_of_conformant_nodes() {
    for command in ["shex-validate", "validate"] {
        let (_, dir) = issue_957_run_with(command, &["--valid", "valid.nt", "--valid-format", "ntriples"]);
        assert_eq!(sorted_lines(&dir, "valid.nt"), vec![ALICE_NAME], "{command}");
    }
}

#[test]
fn invalid_writes_the_triples_read_outside_the_valid_subgraph() {
    for command in ["shex-validate", "validate"] {
        let (_, dir) = issue_957_run_with(command, &["--invalid", "invalid.nt", "--invalid-format", "ntriples"]);
        assert_eq!(sorted_lines(&dir, "invalid.nt"), BOB_NAMES, "{command}");
    }
}

/// The three subgraphs from one run, each in its own format: N-Triples for the
/// visited one, Turtle (the default) for the valid one, N-Quads for the invalid one.
#[test]
fn visited_valid_and_invalid_are_written_at_once_each_in_its_format() {
    let (_, dir) = issue_957_run(&[
        "--visited",
        "visited.nt",
        "--visited-format",
        "ntriples",
        "--valid",
        "valid.ttl",
        "--invalid",
        "invalid.nq",
        "--invalid-format",
        "nquads",
    ]);
    assert_eq!(sorted_lines(&dir, "visited.nt").len(), 3);
    let valid = std::fs::read_to_string(dir.path().join("valid.ttl")).unwrap();
    assert!(valid.contains(r#":alice :name "Alice""#), "{valid}");
    assert!(!valid.contains(":bob"), "{valid}");
    assert_eq!(sorted_lines(&dir, "invalid.nq"), BOB_NAMES);
}

#[test]
fn subgraphs_in_turtle_use_the_prefixes_of_the_data() {
    for flag in ["--visited", "--valid", "--invalid"] {
        let (_, dir) = issue_957_run(&[flag, "sub.ttl"]);
        let written = std::fs::read_to_string(dir.path().join("sub.ttl")).unwrap();
        assert!(written.contains("prefix : <http://a.example/>"), "{flag}: {written}");
        assert!(written.contains(":name"), "{flag}: {written}");
        assert!(!written.contains("<http://a.example/name>"), "{flag}: {written}");
    }
}

#[test]
fn statistics_count_the_valid_and_invalid_triples_without_writing_a_graph() {
    let (stdout, dir) = issue_957_run(&["--statistics"]);
    assert!(stdout.contains("Triples read: 3 (distinct: 3)"), "{stdout}");
    assert!(stdout.contains("Valid triples: 1"), "{stdout}");
    assert!(stdout.contains("Invalid triples: 2"), "{stdout}");
    let files: Vec<_> = std::fs::read_dir(dir.path())
        .unwrap()
        .map(|e| e.unwrap().file_name())
        .collect();
    assert_eq!(files.len(), 3, "only the inputs, no graph written: {files:?}");
}

#[test]
fn statistics_are_the_same_when_subgraphs_are_also_written() {
    for flag in ["--visited", "--valid", "--invalid"] {
        let (stdout, dir) = issue_957_run(&["--statistics", flag, "sub.ttl"]);
        assert!(stdout.contains("Triples read: 3 (distinct: 3)"), "{flag}: {stdout}");
        assert!(stdout.contains("Valid triples: 1"), "{flag}: {stdout}");
        assert!(stdout.contains("Invalid triples: 2"), "{flag}: {stdout}");
        assert!(dir.path().join("sub.ttl").exists(), "{flag}");
    }
}
