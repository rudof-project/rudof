//! Docker-dependent integration tests for the QLever backend.
//!
//! Gated behind the `qlever-docker-tests` feature so plain `cargo test` runs in environments without Docker. Enable with:
//!
//! ```bash
//! cargo test -p rudof_rdf --features qlever-docker-tests --lib rdf_impl::tests::qlever_docker -- --nocapture
//! ```

use crate::rdf_impl::{CliKind, Compression, QleverConfig, QleverGraphContainer, decompressor_probe, qlever_probe_cli};
use std::path::PathBuf;
use std::time::Duration;

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

/// End-to-end: stream a bzip2-compressed N-Triples dump into the index
/// builder via host-side decompressor + container stdin.
#[tokio::test]
async fn index_and_query_small_nt_bz2() {
    if decompressor_probe().for_compression(Compression::Bzip2).is_none() {
        eprintln!("skipping: no bzip2-family decompressor on PATH");
        return;
    }

    let tmp = tempfile::tempdir().unwrap();
    let nt_path = tmp.path().join("data.nt");
    let nt = "<http://example.org/a> <http://example.org/p> <http://example.org/b> .\n\
              <http://example.org/a> <http://example.org/p> <http://example.org/c> .\n";
    std::fs::write(&nt_path, nt).unwrap();

    // bzip2 compresses IN PLACE and removes the original, leaving data.nt.bz2.
    let status = tokio::process::Command::new("bzip2")
        .arg(&nt_path)
        .status()
        .await
        .expect("bzip2 must be on PATH for this test");
    assert!(status.success(), "bzip2 failed: {status}");
    let bz2_path = tmp.path().join("data.nt.bz2");
    assert!(bz2_path.exists(), "expected {} to exist", bz2_path.display());

    let index_dir: PathBuf = tmp.path().join("idx");
    let config = QleverConfig::default()
        .with_index_dir(&index_dir)
        .with_memory_max_size("1G");

    let graph = QleverGraphContainer::from_path(&bz2_path, config).await.unwrap();

    let exists = graph.query_ask_async("ASK { ?s ?p ?o }").await.unwrap();
    assert!(exists, "expected ASK over indexed compressed data to return true");

    let counted = graph
        .query_select_async("SELECT (COUNT(*) AS ?n) WHERE { ?s ?p ?o }")
        .await
        .unwrap();
    let term = counted
        .iter()
        .next()
        .expect("expected one solution")
        .find_solution("n")
        .expect("variable `n` missing");
    let s = format!("{term}");
    assert!(s.contains('2'), "expected COUNT(*) = 2, got: {s}");
}

/// Regression: when the container exits non-zero mid-stdin-stream
/// (simulating an OOM-kill of `IndexBuilderMain`), `build_index` must
/// return Err in seconds rather than hanging forever waiting for the
/// container to accept more stdin bytes.
#[tokio::test]
async fn stdin_stream_does_not_hang_when_container_oom_killed() {
    if decompressor_probe().for_compression(Compression::Bzip2).is_none() {
        eprintln!("skipping: no bzip2-family decompressor on PATH");
        return;
    }

    // Build a >2 MiB N-Triples file so the container has plenty to chew on.
    let tmp = tempfile::tempdir().unwrap();
    let nt_path = tmp.path().join("data.nt");
    {
        use std::io::Write;
        let mut f = std::fs::File::create(&nt_path).unwrap();
        for i in 0..200_000 {
            writeln!(
                f,
                "<http://example.org/s{i}> <http://example.org/p> <http://example.org/o{i}> ."
            )
            .unwrap();
        }
    }
    let status = tokio::process::Command::new("bzip2")
        .arg(&nt_path)
        .status()
        .await
        .expect("bzip2 on PATH");
    assert!(status.success());
    let bz2_path = tmp.path().join("data.nt.bz2");

    let index_dir: PathBuf = tmp.path().join("idx");
    let config = QleverConfig::default()
        .with_index_dir(&index_dir)
        .with_container_memory("4M")
        .with_container_memory_swap("4M");

    let fut = QleverGraphContainer::from_path(&bz2_path, config);
    let res = tokio::time::timeout(Duration::from_secs(60), fut).await;
    match res {
        Err(_) => panic!("build_index hung > 60s on dying container — regression of the OOM-stdin-hang bug"),
        Ok(Ok(_)) => panic!("build_index unexpectedly succeeded with a 4 MiB container cap"),
        Ok(Err(e)) => eprintln!("ok — got expected error in bounded time: {e}"),
    }
}

/// End-to-end: same as above but `.xz` instead of `.bz2`. Confirms the
/// strategy abstraction works end-to-end for both compression families.
#[tokio::test]
async fn index_and_query_small_nt_xz() {
    if decompressor_probe().for_compression(Compression::Xz).is_none() {
        eprintln!("skipping: no xz-family decompressor on PATH");
        return;
    }

    let tmp = tempfile::tempdir().unwrap();
    let nt_path = tmp.path().join("data.nt");
    let nt = "<http://example.org/a> <http://example.org/p> <http://example.org/b> .\n\
              <http://example.org/a> <http://example.org/p> <http://example.org/c> .\n";
    std::fs::write(&nt_path, nt).unwrap();

    let status = tokio::process::Command::new("xz")
        .arg(&nt_path)
        .status()
        .await
        .expect("xz must be on PATH for this test");
    assert!(status.success(), "xz failed: {status}");
    let xz_path = tmp.path().join("data.nt.xz");
    assert!(xz_path.exists(), "expected {} to exist", xz_path.display());

    let index_dir: PathBuf = tmp.path().join("idx");
    let config = QleverConfig::default()
        .with_index_dir(&index_dir)
        .with_memory_max_size("1G");

    let graph = QleverGraphContainer::from_path(&xz_path, config).await.unwrap();

    let exists = graph.query_ask_async("ASK { ?s ?p ?o }").await.unwrap();
    assert!(exists);

    let counted = graph
        .query_select_async("SELECT (COUNT(*) AS ?n) WHERE { ?s ?p ?o }")
        .await
        .unwrap();
    let term = counted
        .iter()
        .next()
        .expect("expected one solution")
        .find_solution("n")
        .expect("variable `n` missing");
    assert!(format!("{term}").contains('2'));
}
