//! Docker-dependent integration tests for the QLever backend.
//!
//! Gated behind the `qlever-docker-tests` feature so plain `cargo test` runs in environments without Docker. Enable with:
//!
//! ```bash
//! cargo test -p rudof_rdf --features qlever-docker-tests --lib rdf_impl::tests::qlever_docker -- --nocapture
//! ```

use crate::rdf_impl::{CliKind, QleverConfig, QleverGraphContainer, qlever_probe_cli};
use std::path::PathBuf;

/// `probe` should return either v1 or v2 against the real upstream image.
#[tokio::test]
async fn probe_detects_cli_kind() {
    let kind = qlever_probe_cli(&QleverConfig::default().image()).await.unwrap();
    assert!(matches!(kind, CliKind::V1 | CliKind::V2), "got: {kind:?}");
}

/// End-to-end: index a tiny Turtle file, start a server, run an ASK.
#[tokio::test]
async fn index_and_query_small_ttl() {
    let tmp = tempfile::tempdir().unwrap();
    let ttl_path = tmp.path().join("data.ttl");
    let ttl = "@prefix : <http://example.org/> .\n:a :p :b .\n:a :p :c .\n";
    std::fs::write(&ttl_path, ttl).unwrap();

    let index_dir: PathBuf = tmp.path().join("idx");

    let config = QleverConfig::default()
        .with_index_dir(&index_dir)
        .with_memory_max_size("1G");

    let graph = QleverGraphContainer::from_path(&ttl_path, config).await.unwrap();

    let exists = graph.query_ask_async("ASK { ?s ?p ?o }").await.unwrap();
    assert!(exists, "expected ASK over indexed data to return true");

    let nope = graph
        .query_ask_async("ASK { <http://nope.example.org/x> ?p ?o }")
        .await
        .unwrap();
    assert!(!nope, "ASK with no matching subject should be false");

    let counted = graph
        .query_select_async("SELECT (COUNT(*) AS ?n) WHERE { ?s ?p ?o }")
        .await
        .unwrap();
    let mut iter = counted.iter();
    let first = iter.next().expect("expected at least one solution");
    let term = first.find_solution("n").expect("variable `n` missing");
    let s = format!("{term}");
    assert!(s.contains('2'), "expected COUNT(*) = 2, got: {s}");
}
