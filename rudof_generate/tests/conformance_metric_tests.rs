//! The triple validity metric, which the generator reports about its own output.
//!
//! The metric resolves each subject to a shape through its `rdf:type`, and that
//! resolution has to follow the same rule generation uses. When entities began
//! to be typed with the class their shape targets rather than with the shape
//! IRI, the metric could no longer find any shape and reported that none of the
//! generated triples were valid -- the mirror image of the typing defect, in the
//! measurement rather than the data.

#![cfg(not(target_family = "wasm"))]

use rudof_generate::config::GeneratorConfig;
use rudof_generate::{DataGenerator, errors::Result};
use std::path::PathBuf;
use tempfile::TempDir;

fn write(dir: &TempDir, name: &str, body: &str) -> PathBuf {
    let path = dir.path().join(name);
    std::fs::write(&path, body).expect("failed to write test schema");
    path
}

/// Generate, then read back the conformance figures from the stats sidecar the
/// generator writes next to its output.
async fn validity_of(schema: &PathBuf, out: PathBuf) -> Result<(f64, f64)> {
    let mut config = GeneratorConfig::default();
    config.output.path = out.clone();
    config.generation.entity_count = 6;
    config.generation.seed = Some(17);

    let mut generator = DataGenerator::new(config)?;
    generator.load_schema_auto(schema).await?;
    generator.generate().await?;

    // `with_extension` replaces the suffix, so `out.ttl` becomes `out.stats.json`.
    let sidecar = out.with_extension("stats.json");
    let raw = std::fs::read_to_string(&sidecar).unwrap_or_else(|e| panic!("no stats at {sidecar:?}: {e}"));
    let parsed: serde_json::Value = serde_json::from_str(&raw).expect("stats should be JSON");
    let conformance = &parsed["conformance_metrics"];

    Ok((
        conformance["triple_validity_percentage"].as_f64().unwrap_or(-1.0),
        conformance["shape_translation_loss_percentage"]
            .as_f64()
            .unwrap_or(-1.0),
    ))
}

const TARGETED: &str = r#"
@prefix sh:  <http://www.w3.org/ns/shacl#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .
@prefix ex:  <http://example.org/> .
@prefix :    <http://weso.es/shapes/> .

:Widget a sh:NodeShape ;
    sh:targetClass ex:Widget ;
    sh:property [ sh:path ex:label ; sh:datatype xsd:string  ; sh:minCount 1 ; sh:maxCount 1 ] ;
    sh:property [ sh:path ex:count ; sh:datatype xsd:integer ; sh:minCount 1 ; sh:maxCount 1 ] .
"#;

/// A shape that declares a target class: entities carry that class, and the
/// metric has to resolve it back to the shape rather than look for the shape
/// IRI in the data.
#[tokio::test]
async fn validity_is_reported_for_shapes_with_a_target_class() -> Result<()> {
    let dir = tempfile::tempdir().unwrap();
    let schema = write(&dir, "targeted.ttl", TARGETED);

    let (validity, _loss) = validity_of(&schema, dir.path().join("out.ttl")).await?;

    assert!(
        validity > 99.0,
        "data generated from this schema satisfies it, so validity should be ~100%, got {validity}"
    );
    Ok(())
}

/// A shape with no target class is typed with its own IRI, which the metric
/// must keep resolving as it always did.
#[tokio::test]
async fn validity_is_reported_for_shapes_without_a_target_class() -> Result<()> {
    let dir = tempfile::tempdir().unwrap();
    let schema = write(
        &dir,
        "untargeted.ttl",
        r#"
@prefix sh:  <http://www.w3.org/ns/shacl#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .
@prefix ex:  <http://example.org/> .
@prefix :    <http://weso.es/shapes/> .

:Widget a sh:NodeShape ;
    sh:property [ sh:path ex:label ; sh:datatype xsd:string ; sh:minCount 1 ; sh:maxCount 1 ] .
"#,
    );

    let (validity, _loss) = validity_of(&schema, dir.path().join("out.ttl")).await?;

    assert!(
        validity > 99.0,
        "a shape without a target class is typed with its own IRI and must still \
         resolve, got {validity}"
    );
    Ok(())
}

/// A schema wholly inside the executable fragment loses nothing, and its output
/// should satisfy every constraint that was carried over.
#[tokio::test]
async fn a_fully_represented_schema_yields_full_validity() -> Result<()> {
    let dir = tempfile::tempdir().unwrap();
    let schema = write(&dir, "targeted.ttl", TARGETED);

    let (validity, loss) = validity_of(&schema, dir.path().join("out.ttl")).await?;

    assert_eq!(
        loss, 0.0,
        "this schema uses only represented constructs, got {loss}% loss"
    );
    assert!(
        validity > 99.0,
        "with nothing lost in translation the generated data should satisfy the \
         whole schema, got {validity}"
    );
    Ok(())
}
