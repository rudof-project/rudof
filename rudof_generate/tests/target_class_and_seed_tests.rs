//! Regression tests for how entities are typed, and for seeded reproducibility.
//!
//! Both properties are relied on by anything that validates generated data
//! against the schema it came from: a graph whose entities are not typed with
//! the class their shape targets is not selected by that schema at all, and a
//! run that cannot be reproduced cannot be checked twice.

#![cfg(not(target_family = "wasm"))]

use rudof_generate::config::GeneratorConfig;
use rudof_generate::{DataGenerator, errors::Result};
use rudof_rdf::rdf_core::{NeighsRDF, RDFFormat};
use rudof_rdf::rdf_impl::{OxigraphInMemory, ReaderMode};
use std::path::PathBuf;
use tempfile::TempDir;

const RDF_TYPE: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#type";

fn write(temp_dir: &TempDir, name: &str, body: &str) -> PathBuf {
    let path = temp_dir.path().join(name);
    std::fs::write(&path, body).expect("failed to write test schema");
    path
}

/// Every `rdf:type` object in a Turtle document, taken from the `a` shorthand
/// as well as the full predicate.
fn types_in(turtle: &str) -> Vec<String> {
    let mut found = Vec::new();
    for token in turtle.split_whitespace().collect::<Vec<_>>().windows(2) {
        let (predicate, object) = (token[0], token[1]);
        let is_type = predicate == "a" || predicate.trim_matches(['<', '>']) == RDF_TYPE;
        if is_type && object.starts_with('<') {
            found.push(object.trim_matches(['<', '>', ';', '.']).to_string());
        }
    }
    found
}

async fn generate(schema: &PathBuf, out: PathBuf, entities: usize, seed: Option<u64>) -> Result<String> {
    let mut config = GeneratorConfig::default();
    config.output.path = out.clone();
    config.generation.entity_count = entities;
    config.generation.seed = seed;

    let mut generator = DataGenerator::new(config)?;
    generator.load_schema_auto(schema).await?;
    generator.generate().await?;
    Ok(std::fs::read_to_string(&out)?)
}

#[tokio::test]
async fn shacl_entities_are_typed_with_the_declared_target_class() -> Result<()> {
    let dir = tempfile::tempdir().unwrap();
    let schema = write(
        &dir,
        "target.ttl",
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

    let content = generate(&schema, dir.path().join("out.ttl"), 3, Some(1)).await?;
    let types = types_in(&content);

    assert!(
        types.iter().all(|t| t == "http://example.org/Widget"),
        "entities must carry the class the shape targets, found {types:?}"
    );
    assert!(!types.is_empty(), "expected at least one typed entity");
    Ok(())
}

#[tokio::test]
async fn shacl_falls_back_to_a_single_valued_rdf_type_constraint() -> Result<()> {
    // A shape may pin rdf:type with sh:in instead of declaring a target. That
    // says just as clearly what its instances carry, and a shape typed with its
    // own IRI would fail the constraint it declares.
    let dir = tempfile::tempdir().unwrap();
    let schema = write(
        &dir,
        "in.ttl",
        r#"
@prefix sh:  <http://www.w3.org/ns/shacl#> .
@prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .
@prefix ex:  <http://example.org/> .
@prefix :    <http://weso.es/shapes/> .

:Widget a sh:NodeShape ;
    sh:property [ sh:path rdf:type ; sh:in ( ex:Widget ) ; sh:minCount 1 ; sh:maxCount 1 ] ;
    sh:property [ sh:path ex:label ; sh:datatype xsd:string ; sh:minCount 1 ; sh:maxCount 1 ] .
"#,
    );

    let content = generate(&schema, dir.path().join("out.ttl"), 3, Some(1)).await?;
    let types = types_in(&content);

    assert!(
        types.iter().all(|t| t == "http://example.org/Widget"),
        "entities must carry the class sh:in pins rdf:type to, found {types:?}"
    );
    assert!(!types.is_empty(), "expected at least one typed entity");
    Ok(())
}

#[tokio::test]
async fn shex_entities_are_typed_from_the_rdf_type_value_set() -> Result<()> {
    let dir = tempfile::tempdir().unwrap();
    let schema = write(
        &dir,
        "target.shex",
        r#"
PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>
PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>
PREFIX ex:  <http://example.org/>
PREFIX :    <http://weso.es/shapes/>

:Widget {
   rdf:type  [ex:Widget]  ;
   ex:label  xsd:string
}
"#,
    );

    let content = generate(&schema, dir.path().join("out.ttl"), 3, Some(1)).await?;
    let types = types_in(&content);

    assert!(
        types.iter().all(|t| t == "http://example.org/Widget"),
        "ShEx entities must carry the class their rdf:type value set names, found {types:?}"
    );
    assert!(!types.is_empty(), "expected at least one typed entity");
    Ok(())
}

#[tokio::test]
async fn the_same_seed_reproduces_the_same_graph() -> Result<()> {
    let dir = tempfile::tempdir().unwrap();
    let schema = write(
        &dir,
        "seeded.ttl",
        r#"
@prefix sh:  <http://www.w3.org/ns/shacl#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .
@prefix ex:  <http://example.org/> .
@prefix :    <http://weso.es/shapes/> .

:Widget a sh:NodeShape ;
    sh:targetClass ex:Widget ;
    sh:property [ sh:path ex:label ; sh:datatype xsd:string ; sh:minCount 1 ; sh:maxCount 1 ] ;
    sh:property [ sh:path ex:count ; sh:datatype xsd:integer ; sh:minCount 1 ; sh:maxCount 1 ] .
"#,
    );

    // Compared as a parsed set of triples rather than as text. The serialiser
    // fixes neither the order of subject blocks nor which predicate opens one,
    // so two byte-different documents can carry exactly the same graph, and it
    // is the graph the seed is supposed to determine.
    let as_set = |turtle: &str| {
        let graph = OxigraphInMemory::from_str(turtle, &RDFFormat::Turtle, None, &ReaderMode::Strict)
            .expect("generated output should parse as Turtle");
        let mut triples: Vec<String> = graph
            .triples()
            .expect("graph should be readable")
            .map(|t| t.to_string())
            .collect();
        triples.sort();
        triples
    };

    let first = generate(&schema, dir.path().join("a.ttl"), 6, Some(42)).await?;
    let second = generate(&schema, dir.path().join("b.ttl"), 6, Some(42)).await?;
    let other = generate(&schema, dir.path().join("c.ttl"), 6, Some(99)).await?;

    assert_eq!(
        as_set(&first),
        as_set(&second),
        "the same seed must produce the same graph"
    );
    assert_ne!(
        as_set(&first),
        as_set(&other),
        "a different seed should produce a different graph"
    );
    Ok(())
}

/// A shape whose instances carry several types must keep its own identity.
///
/// Extracted schemas state one `rdf:type` constraint per class an instance
/// carries, so a shape that specialises another names both. Taking whichever
/// came first gave every such shape the same type and erased the distinction
/// between them -- three shapes collapsing into one -- which a SHACL
/// `sh:targetClass` for the same shape would not do.
#[tokio::test]
async fn a_shape_with_several_rdf_type_constraints_keeps_its_own_class() -> Result<()> {
    let dir = tempfile::tempdir().unwrap();
    let schema = write(
        &dir,
        "multi.shex",
        r#"
PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>
PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>
PREFIX ex:  <http://example.org/>
PREFIX :    <http://weso.es/shapes/>

:ResearchAssistant {
   rdf:type  [ex:GraduateStudent]  ;
   rdf:type  [ex:ResearchAssistant]  ;
   ex:label  xsd:string
}
"#,
    );

    let content = generate(&schema, dir.path().join("out.ttl"), 3, Some(1)).await?;
    let types = types_in(&content);

    assert!(!types.is_empty(), "expected at least one typed entity");
    assert!(
        types.iter().all(|t| t == "http://example.org/ResearchAssistant"),
        "the shape's own class must win over the one it specialises, found {types:?}"
    );
    Ok(())
}
