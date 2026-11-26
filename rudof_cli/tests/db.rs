//! End-to-end tests for the LadybugDB-backed commands:
//! `connect`, `ddl`, `load` and `query --dialect cypher`.
//!
//! These spawn the actual `rudof` binary the same way a user would, since
//! database creation, connection details persistence, and query execution
//! only show up end-to-end. See discussion #747 for the CLI design.

#![cfg(not(target_family = "wasm"))]

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

const DATA_TTL: &str = r#"@prefix : <http://example.org/> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

:alice a :User ;
    :name "Alice" ;
    :knows :bob .

:bob a :User ;
    :name "Bob" .
"#;

const SHAPES_TTL: &str = r#"@prefix : <http://example.org/> .
@prefix sh: <http://www.w3.org/ns/shacl#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

:UserShape a sh:NodeShape ;
    sh:targetClass :User ;
    sh:property [
        sh:path :name ;
        sh:datatype xsd:string ;
        sh:minCount 1 ;
    ] .
"#;

#[test]
fn ddl_cypher_dialect_emits_node_and_rel_tables() {
    let dir = tempfile::tempdir().expect("tempdir");
    std::fs::write(dir.path().join("data.ttl"), DATA_TTL).unwrap();

    let out = rudof_in(dir.path(), &["ddl", "data.ttl", "--dialect", "cypher"]);
    assert_eq!(out.code, 0, "stdout: {}\nstderr: {}", out.stdout, out.stderr);

    assert!(
        out.stdout
            .contains("CREATE NODE TABLE User (id STRING, knows STRING, name STRING, PRIMARY KEY(id));")
    );
    assert!(out.stdout.contains("CREATE REL TABLE knows (FROM User TO User);"));
    // No progress messages on stdout: the DDL must be pipeable.
    assert!(!out.stdout.contains("Derived"));
}

#[test]
fn ddl_gql_dialect_emits_graph_type() {
    let dir = tempfile::tempdir().expect("tempdir");
    std::fs::write(dir.path().join("data.ttl"), DATA_TTL).unwrap();

    let out = rudof_in(
        dir.path(),
        &["ddl", "data.ttl", "--dialect", "gql", "--graph-type-name", "social"],
    );
    assert_eq!(out.code, 0, "stdout: {}\nstderr: {}", out.stdout, out.stderr);

    assert!(out.stdout.contains("CREATE GRAPH TYPE social ("));
    assert!(
        out.stdout
            .contains("NODE TYPE User (id STRING, knows STRING, name STRING)")
    );
    assert!(out.stdout.contains("EDGE TYPE knows (FROM User TO User)"));
}

#[test]
fn connect_persists_connection_details() {
    let dir = tempfile::tempdir().expect("tempdir");

    let out = rudof_in(dir.path(), &["connect", "testdb.lbug"]);
    assert_eq!(out.code, 0, "stdout: {}\nstderr: {}", out.stdout, out.stderr);

    let contents = std::fs::read_to_string(dir.path().join(".rudof-connection.toml"))
        .expect("connection details file should exist");
    assert!(contents.contains("backend = \"lbug\""));
    assert!(contents.contains("read_only = false"));
    assert!(contents.contains("testdb.lbug"));
}

#[test]
fn connect_in_memory_does_not_persist() {
    let dir = tempfile::tempdir().expect("tempdir");

    let out = rudof_in(dir.path(), &["connect", "--in-memory"]);
    assert_eq!(out.code, 0, "stdout: {}\nstderr: {}", out.stdout, out.stderr);
    assert!(out.stdout.contains("in-memory"));
    assert!(
        !dir.path().join(".rudof-connection.toml").exists(),
        "in-memory connections must not be persisted"
    );
}

#[test]
fn load_validates_then_queries_roundtrip() {
    let dir = tempfile::tempdir().expect("tempdir");
    std::fs::write(dir.path().join("data.ttl"), DATA_TTL).unwrap();
    std::fs::write(dir.path().join("shapes.ttl"), SHAPES_TTL).unwrap();

    let connect = rudof_in(dir.path(), &["connect", "db.lbug"]);
    assert_eq!(connect.code, 0, "connect failed: {}", connect.stderr);

    let load = rudof_in(dir.path(), &["load", "data.ttl", "--shapes", "shapes.ttl"]);
    assert_eq!(load.code, 0, "load failed: {}\n{}", load.stdout, load.stderr);
    assert!(load.stdout.contains("SHACL validation PASSED"));
    assert!(load.stdout.contains("Inserted 2 node(s)"));

    // Query through the persisted connection details (no --db needed).
    let query = rudof_in(
        dir.path(),
        &["query", "--dialect", "cypher", "-q", "MATCH (n:User) RETURN n.name"],
    );
    assert_eq!(query.code, 0, "query failed: {}\n{}", query.stdout, query.stderr);
    assert!(query.stdout.contains("Alice"));
    assert!(query.stdout.contains("Bob"));
}

#[test]
fn query_cypher_dialect_reads_query_from_a_file() {
    // `-q`/`--query` is a shared `InputSpec` argument (file, URL, `-`, or
    // literal text) for both dialects, unlike the old `--cypher <STRING>`
    // flag it replaced, which only ever accepted inline text.
    let dir = tempfile::tempdir().expect("tempdir");
    std::fs::write(dir.path().join("data.ttl"), DATA_TTL).unwrap();
    std::fs::write(dir.path().join("query.cypher"), "MATCH (n:User) RETURN n.name\n").unwrap();

    rudof_in(dir.path(), &["connect", "db.lbug"]);
    let load = rudof_in(dir.path(), &["load", "data.ttl", "--skip-validation"]);
    assert_eq!(load.code, 0, "load failed: {}\n{}", load.stdout, load.stderr);

    let query = rudof_in(dir.path(), &["query", "--dialect", "cypher", "-q", "query.cypher"]);
    assert_eq!(query.code, 0, "query failed: {}\n{}", query.stdout, query.stderr);
    assert!(query.stdout.contains("Alice"));
    assert!(query.stdout.contains("Bob"));
}

#[test]
fn query_db_flag_without_cypher_dialect_reports_a_clear_error() {
    // `--db`/`--connection` only make sense for `--dialect cypher`; without
    // it they're silently meaningless for SPARQL, so this must be a clear
    // error instead.
    let dir = tempfile::tempdir().expect("tempdir");
    std::fs::write(dir.path().join("data.ttl"), DATA_TTL).unwrap();

    let out = rudof_in(
        dir.path(),
        &[
            "query",
            "--db",
            "db.lbug",
            "-q",
            "SELECT * WHERE { ?s ?p ?o }",
            "data.ttl",
        ],
    );
    assert_ne!(out.code, 0);
    assert!(out.stderr.contains("--dialect cypher"));
}

#[test]
fn load_aborts_on_shacl_violation() {
    let dir = tempfile::tempdir().expect("tempdir");
    std::fs::write(dir.path().join("shapes.ttl"), SHAPES_TTL).unwrap();
    std::fs::write(
        dir.path().join("bad.ttl"),
        "@prefix : <http://example.org/> .\n:carol a :User .\n",
    )
    .unwrap();

    rudof_in(dir.path(), &["connect", "db.lbug"]);

    let load = rudof_in(dir.path(), &["load", "bad.ttl", "--shapes", "shapes.ttl"]);
    assert_ne!(load.code, 0, "load must fail on non-conforming data");
    assert!(load.stdout.contains("SHACL validation FAILED"));
}

#[test]
fn full_workflow_connect_ddl_load_query_cypher() {
    // Regression coverage for the original PR #779 story end-to-end, now
    // routed through the unified `--backend` flag (see discussion #747):
    // connect to LadybugDB, preview the DDL derived from RDF data, validate
    // + load that data against SHACL shapes, then query it with Cypher.
    let dir = tempfile::tempdir().expect("tempdir");
    std::fs::write(dir.path().join("data.ttl"), DATA_TTL).unwrap();
    std::fs::write(dir.path().join("shapes.ttl"), SHAPES_TTL).unwrap();

    // 1. Connect, explicitly selecting the backend via the (now shared)
    //    --backend flag rather than the old --engine flag.
    let connect = rudof_in(dir.path(), &["connect", "db.lbug", "--backend", "lbug"]);
    assert_eq!(
        connect.code, 0,
        "connect failed: {}\n{}",
        connect.stdout, connect.stderr
    );
    let connection_contents = std::fs::read_to_string(dir.path().join(".rudof-connection.toml"))
        .expect("connection details file should exist");
    assert!(connection_contents.contains("backend = \"lbug\""));

    // 2. Derive and preview the DDL from the RDF data, stateless -- no
    //    database is touched yet.
    let ddl = rudof_in(dir.path(), &["ddl", "data.ttl", "--dialect", "cypher"]);
    assert_eq!(ddl.code, 0, "ddl failed: {}\n{}", ddl.stdout, ddl.stderr);
    assert!(
        ddl.stdout
            .contains("CREATE NODE TABLE User (id STRING, knows STRING, name STRING, PRIMARY KEY(id));")
    );

    // 3. Validate the same data against SHACL shapes and load it into the
    //    connected database (uses the connection details from step 1).
    let load = rudof_in(dir.path(), &["load", "data.ttl", "--shapes", "shapes.ttl"]);
    assert_eq!(load.code, 0, "load failed: {}\n{}", load.stdout, load.stderr);
    assert!(load.stdout.contains("SHACL validation PASSED"));
    assert!(load.stdout.contains("Inserted 2 node(s)"));

    // 4. Run a Cypher query against the loaded data through the same
    //    persisted connection.
    let query = rudof_in(
        dir.path(),
        &[
            "query",
            "--dialect",
            "cypher",
            "-q",
            "MATCH (n:User) RETURN n.name ORDER BY n.name",
        ],
    );
    assert_eq!(query.code, 0, "query failed: {}\n{}", query.stdout, query.stderr);
    assert!(query.stdout.contains("Alice"));
    assert!(query.stdout.contains("Bob"));
}

#[test]
fn load_requires_a_database() {
    let dir = tempfile::tempdir().expect("tempdir");
    std::fs::write(dir.path().join("data.ttl"), DATA_TTL).unwrap();

    let out = rudof_in(dir.path(), &["load", "data.ttl"]);
    assert_ne!(out.code, 0);
    assert!(out.stderr.contains("No database specified"));
    assert!(out.stderr.contains("rudof connect"));
}
