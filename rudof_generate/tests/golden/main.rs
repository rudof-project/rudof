//! Golden-output tests: a fixed schema and seed must keep producing exactly the
//! same graph. They are the regression net for refactoring the generator: a
//! change of output is either a bug or a deliberate change, and a deliberate one
//! is recorded by regenerating the snapshots and reviewing the diff:
//!
//! ```sh
//! UPDATE_GOLDEN=1 cargo test -p rudof_generate --test golden
//! ```
//!
//! Snapshots are sorted N-Triples, so they do not depend on serializer ordering.
#![cfg(not(target_family = "wasm"))]

#[path = "../common/mod.rs"]
mod common;

use common::*;
use rudof_generate::config::OutputFormat;
use std::path::PathBuf;

const SHEX: &str = r#"PREFIX ex: <http://example.org/>
PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>
ex:PersonShape {
  a [ex:Person] ;
  ex:name xsd:string ;
  ex:age xsd:integer MININCLUSIVE 18 MAXINCLUSIVE 65 ;
  ex:code xsd:string /^[A-Z]{3}-[0-9]{4}$/ ;
  ex:status ["active" "retired"] ;
  ex:nick xsd:string {0,2} ;
  ex:worksFor @ex:OrgShape
}
ex:OrgShape { a [ex:Org] ; ex:label xsd:string }
"#;

const SHACL: &str = r#"@prefix sh: <http://www.w3.org/ns/shacl#> .
@prefix ex: <http://example.org/> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .
ex:PersonShape a sh:NodeShape ; sh:targetClass ex:Person ;
  sh:property [ sh:path ex:name ; sh:minCount 1 ; sh:maxCount 1 ; sh:datatype xsd:string ] ;
  sh:property [ sh:path ex:age ; sh:minCount 1 ; sh:datatype xsd:integer ; sh:minInclusive 18 ; sh:maxInclusive 65 ] ;
  sh:property [ sh:path ex:status ; sh:minCount 1 ; sh:in ( "active" "retired" ) ] ;
  sh:property [ sh:path ex:nick ; sh:maxCount 2 ; sh:datatype xsd:string ] ;
  sh:property [ sh:path ex:worksFor ; sh:minCount 1 ; sh:maxCount 1 ; sh:node ex:OrgShape ] .
ex:OrgShape a sh:NodeShape ; sh:targetClass ex:Org ;
  sh:property [ sh:path ex:label ; sh:minCount 1 ; sh:datatype xsd:string ] .
"#;

fn snapshot_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/golden/snapshots")
        .join(format!("{name}.nt"))
}

/// Generate N-Triples for `fixture` with `threads` workers, sorted line by line.
async fn sorted_ntriples(fixture: &Fixture, threads: usize) -> String {
    let mut cfg = config(&fixture.dir.path().join("out.nt"), 12, 2026);
    cfg.output.format = OutputFormat::NTriples;
    cfg.parallel.worker_threads = Some(threads);
    let data = generate(fixture, cfg).await;
    let mut lines: Vec<&str> = data.lines().filter(|l| !l.trim().is_empty()).collect();
    lines.sort_unstable();
    lines.join("\n") + "\n"
}

async fn check(name: &str, fixture: Fixture) {
    let actual = sorted_ntriples(&fixture, 1).await;
    let path = snapshot_path(name);
    if std::env::var_os("UPDATE_GOLDEN").is_some() {
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(&path, &actual).unwrap();
        return;
    }
    let expected = std::fs::read_to_string(&path)
        .unwrap_or_else(|_| panic!("missing snapshot {}; run with UPDATE_GOLDEN=1", path.display()))
        .replace("\r\n", "\n");
    assert!(
        actual == expected,
        "output of `{name}` changed; if intended, rerun with UPDATE_GOLDEN=1 and review the diff\n--- actual ---\n{actual}"
    );
}

#[tokio::test]
async fn shex_person_org_snapshot() {
    check("shex_person_org", Fixture::shex(SHEX)).await;
}

#[tokio::test]
async fn shacl_person_org_snapshot() {
    check("shacl_person_org", Fixture::shacl(SHACL)).await;
}

#[tokio::test]
async fn output_does_not_depend_on_the_number_of_threads() {
    let one = sorted_ntriples(&Fixture::shex(SHEX), 1).await;
    let many = sorted_ntriples(&Fixture::shex(SHEX), 8).await;
    assert_eq!(
        one, many,
        "a fixed seed must give the same graph whatever the thread count"
    );
}
