//! Properties whose value must be an IRI but whose shape names no target.
//!
//! These sit between the two paths the generator has: they are not literals, so
//! the value phase's datatype branch does not apply, and they name no shape, so
//! the linking phase has nothing to point them at. Before this was handled they
//! were silently dropped, which turned a required property into a missing one
//! and made the output fail a `sh:minCount` the schema plainly declared.

#![cfg(not(target_family = "wasm"))]

use rudof_generate::config::GeneratorConfig;
use rudof_generate::{DataGenerator, errors::Result};
use rudof_rdf::rdf_core::{NeighsRDF, RDFFormat};
use rudof_rdf::rdf_impl::{OxigraphInMemory, ReaderMode};
use std::path::PathBuf;
use tempfile::TempDir;

fn write(dir: &TempDir, name: &str, body: &str) -> PathBuf {
    let path = dir.path().join(name);
    std::fs::write(&path, body).expect("failed to write test schema");
    path
}

async fn generate(schema: &PathBuf, out: PathBuf) -> Result<OxigraphInMemory> {
    let mut config = GeneratorConfig::default();
    config.output.path = out.clone();
    config.generation.entity_count = 4;
    config.generation.seed = Some(3);

    let mut generator = DataGenerator::new(config)?;
    generator.load_schema_auto(schema).await?;
    generator.generate().await?;

    let turtle = std::fs::read_to_string(&out)?;
    Ok(
        OxigraphInMemory::from_str(&turtle, &RDFFormat::Turtle, None, &ReaderMode::Strict)
            .expect("generated output should parse as Turtle"),
    )
}

/// Objects of `predicate`, as strings.
fn objects_of(graph: &OxigraphInMemory, predicate_suffix: &str) -> Vec<String> {
    graph
        .triples()
        .expect("graph should be readable")
        .filter(|t| t.predicate.to_string().contains(predicate_suffix))
        .map(|t| t.object.to_string())
        .collect()
}

#[tokio::test]
async fn a_required_iri_property_is_emitted() -> Result<()> {
    let dir = tempfile::tempdir().unwrap();
    let schema = write(
        &dir,
        "iri.ttl",
        r#"
@prefix sh:  <http://www.w3.org/ns/shacl#> .
@prefix ex:  <http://example.org/> .
@prefix :    <http://weso.es/shapes/> .

:Widget a sh:NodeShape ;
    sh:targetClass ex:Widget ;
    sh:property [ sh:path ex:seeAlso ; sh:nodeKind sh:IRI ; sh:minCount 1 ; sh:maxCount 1 ] .
"#,
    );

    let graph = generate(&schema, dir.path().join("out.ttl")).await?;
    let values = objects_of(&graph, "seeAlso");

    assert_eq!(values.len(), 4, "every entity must carry the required property");
    Ok(())
}

#[tokio::test]
async fn the_value_of_an_iri_property_is_an_iri_not_a_literal() -> Result<()> {
    let dir = tempfile::tempdir().unwrap();
    let schema = write(
        &dir,
        "iri_kind.ttl",
        r#"
@prefix sh:  <http://www.w3.org/ns/shacl#> .
@prefix ex:  <http://example.org/> .
@prefix :    <http://weso.es/shapes/> .

:Widget a sh:NodeShape ;
    sh:targetClass ex:Widget ;
    sh:property [ sh:path ex:seeAlso ; sh:nodeKind sh:IRI ; sh:minCount 1 ; sh:maxCount 1 ] .
"#,
    );

    let graph = generate(&schema, dir.path().join("out.ttl")).await?;
    let values = objects_of(&graph, "seeAlso");

    assert!(!values.is_empty(), "expected generated values");
    for value in &values {
        assert!(
            value.starts_with('<') && value.ends_with('>'),
            "an sh:nodeKind sh:IRI property must not be given a literal, got {value}"
        );
    }
    Ok(())
}

#[tokio::test]
async fn minted_iris_are_distinct_per_subject() -> Result<()> {
    let dir = tempfile::tempdir().unwrap();
    let schema = write(
        &dir,
        "distinct.ttl",
        r#"
@prefix sh:  <http://www.w3.org/ns/shacl#> .
@prefix ex:  <http://example.org/> .
@prefix :    <http://weso.es/shapes/> .

:Widget a sh:NodeShape ;
    sh:targetClass ex:Widget ;
    sh:property [ sh:path ex:seeAlso ; sh:nodeKind sh:IRI ; sh:minCount 1 ; sh:maxCount 1 ] .
"#,
    );

    let graph = generate(&schema, dir.path().join("out.ttl")).await?;
    let mut values = objects_of(&graph, "seeAlso");
    let total = values.len();
    values.sort();
    values.dedup();

    assert_eq!(
        values.len(),
        total,
        "each subject should get its own minted IRI, not a shared one"
    );
    Ok(())
}

/// A property that declares a datatype keeps producing a literal; the IRI
/// branch must not capture it.
#[tokio::test]
async fn a_datatype_property_still_produces_a_literal() -> Result<()> {
    let dir = tempfile::tempdir().unwrap();
    let schema = write(
        &dir,
        "literal.ttl",
        r#"
@prefix sh:  <http://www.w3.org/ns/shacl#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .
@prefix ex:  <http://example.org/> .
@prefix :    <http://weso.es/shapes/> .

:Widget a sh:NodeShape ;
    sh:targetClass ex:Widget ;
    sh:property [ sh:path ex:label ; sh:datatype xsd:string ; sh:minCount 1 ; sh:maxCount 1 ] .
"#,
    );

    let graph = generate(&schema, dir.path().join("out.ttl")).await?;
    let values = objects_of(&graph, "label");

    assert!(!values.is_empty(), "expected generated values");
    for value in &values {
        assert!(
            value.starts_with('"'),
            "a datatype property must still yield a literal, got {value}"
        );
    }
    Ok(())
}
