//! How a declared cardinality interval is realised in the output.
//!
//! The two languages disagree about what an omitted cardinality means -- ShEx
//! defines `{1,1}`, SHACL treats a missing `sh:minCount` as optional -- and the
//! difference has to be settled in the converters rather than in the generator,
//! which sees only the normalised interval.

#![cfg(not(target_family = "wasm"))]

use rudof_generate::config::{CardinalityStrategy, GeneratorConfig};
use rudof_generate::{DataGenerator, errors::Result};
use rudof_rdf::rdf_core::{NeighsRDF, RDFFormat};
use rudof_rdf::rdf_impl::{OxigraphInMemory, ReaderMode};
use std::collections::HashMap;
use std::path::PathBuf;
use tempfile::TempDir;

fn write(dir: &TempDir, name: &str, body: &str) -> PathBuf {
    let path = dir.path().join(name);
    std::fs::write(&path, body).expect("failed to write test schema");
    path
}

/// How many times `predicate_suffix` appears on each subject, one entry per
/// subject in the graph, sorted.
///
/// The document is parsed rather than scanned: the serialiser groups statements
/// by subject and chooses freely which predicate opens a block, so a text scan
/// attributes values to the wrong subject.
fn counts_per_subject(turtle: &str, predicate_suffix: &str) -> Vec<usize> {
    let graph = OxigraphInMemory::from_str(turtle, &RDFFormat::Turtle, None, &ReaderMode::Strict)
        .expect("generated output should parse as Turtle");

    let mut per_subject: HashMap<String, usize> = HashMap::new();
    for triple in graph.triples().expect("graph should be readable") {
        let subject = triple.subject.to_string();
        per_subject.entry(subject.clone()).or_insert(0);
        if triple.predicate.to_string().contains(predicate_suffix) {
            *per_subject.get_mut(&subject).expect("subject just inserted") += 1;
        }
    }

    let mut counts: Vec<usize> = per_subject.into_values().collect();
    counts.sort_unstable();
    counts
}

async fn generate(schema: &PathBuf, out: PathBuf, strategy: CardinalityStrategy) -> Result<String> {
    let mut config = GeneratorConfig::default();
    config.output.path = out.clone();
    config.generation.entity_count = 4;
    config.generation.seed = Some(11);
    config.generation.cardinality_strategy = strategy;

    let mut generator = DataGenerator::new(config)?;
    generator.load_schema_auto(schema).await?;
    generator.generate().await?;
    Ok(std::fs::read_to_string(&out)?)
}

/// A SHACL property with no `sh:minCount` is optional, so the minimum strategy
/// may legitimately emit none of it. Defaulting the minimum to 1 instead made
/// an optional property mandatory.
#[tokio::test]
async fn shacl_property_without_min_count_is_optional() -> Result<()> {
    let dir = tempfile::tempdir().unwrap();
    let schema = write(
        &dir,
        "optional.ttl",
        r#"
@prefix sh:  <http://www.w3.org/ns/shacl#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .
@prefix ex:  <http://example.org/> .
@prefix :    <http://weso.es/shapes/> .

:Widget a sh:NodeShape ;
    sh:targetClass ex:Widget ;
    sh:property [ sh:path ex:optionalNote ; sh:datatype xsd:string ; sh:maxCount 1 ] .
"#,
    );

    let content = generate(&schema, dir.path().join("out.ttl"), CardinalityStrategy::Minimum).await?;
    let counts = counts_per_subject(&content, "optionalNote");

    assert!(
        counts.iter().all(|&n| n == 0),
        "a property with no sh:minCount must be treated as optional, got {counts:?}"
    );
    Ok(())
}

/// The same property with an explicit minimum must always be present.
#[tokio::test]
async fn shacl_property_with_min_count_is_always_emitted() -> Result<()> {
    let dir = tempfile::tempdir().unwrap();
    let schema = write(
        &dir,
        "required.ttl",
        r#"
@prefix sh:  <http://www.w3.org/ns/shacl#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .
@prefix ex:  <http://example.org/> .
@prefix :    <http://weso.es/shapes/> .

:Widget a sh:NodeShape ;
    sh:targetClass ex:Widget ;
    sh:property [ sh:path ex:requiredNote ; sh:datatype xsd:string ; sh:minCount 1 ; sh:maxCount 1 ] .
"#,
    );

    let content = generate(&schema, dir.path().join("out.ttl"), CardinalityStrategy::Minimum).await?;
    let counts = counts_per_subject(&content, "requiredNote");

    assert!(!counts.is_empty(), "expected generated entities");
    assert!(
        counts.iter().all(|&n| n == 1),
        "a property with sh:minCount 1 must always be present, got {counts:?}"
    );
    Ok(())
}

/// ShEx defines an omitted cardinality as exactly one, and the converter
/// normalises it, so the generator must emit it even under the minimum
/// strategy.
#[tokio::test]
async fn shex_property_without_cardinality_is_exactly_one() -> Result<()> {
    let dir = tempfile::tempdir().unwrap();
    let schema = write(
        &dir,
        "default.shex",
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

    // Checked under the maximum strategy, which is where an upper bound that
    // was lost in translation shows up: ShEx's default is exactly one, not one
    // or more.
    let content = generate(&schema, dir.path().join("out.ttl"), CardinalityStrategy::Maximum).await?;
    let counts = counts_per_subject(&content, "label");

    assert!(!counts.is_empty(), "expected generated entities");
    assert!(
        counts.iter().all(|&n| n == 1),
        "ShEx's default cardinality is exactly one, got {counts:?}"
    );
    Ok(())
}

/// The maximum strategy must not exceed a declared `sh:maxCount`.
#[tokio::test]
async fn maximum_strategy_respects_max_count() -> Result<()> {
    let dir = tempfile::tempdir().unwrap();
    let schema = write(
        &dir,
        "bounded.ttl",
        r#"
@prefix sh:  <http://www.w3.org/ns/shacl#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .
@prefix ex:  <http://example.org/> .
@prefix :    <http://weso.es/shapes/> .

:Widget a sh:NodeShape ;
    sh:targetClass ex:Widget ;
    sh:property [ sh:path ex:tag ; sh:datatype xsd:string ; sh:minCount 1 ; sh:maxCount 3 ] .
"#,
    );

    let content = generate(&schema, dir.path().join("out.ttl"), CardinalityStrategy::Maximum).await?;
    let counts = counts_per_subject(&content, "tag");

    assert!(!counts.is_empty(), "expected generated entities");
    assert!(
        counts.iter().all(|&n| (1..=3).contains(&n)),
        "realised cardinality must stay inside [1, 3], got {counts:?}"
    );
    Ok(())
}

/// A schema that sets no upper bound permits any number of values, so the
/// number the generator picks is a configuration choice rather than a
/// correctness one. It defaults to one and must be raisable.
#[tokio::test]
async fn the_unbounded_cap_is_configurable() -> Result<()> {
    let dir = tempfile::tempdir().unwrap();
    let schema = write(
        &dir,
        "unbounded.shex",
        r#"
PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>
PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>
PREFIX ex:  <http://example.org/>
PREFIX :    <http://weso.es/shapes/>

:Widget {
   rdf:type  [ex:Widget]  ;
   ex:tag    xsd:string  *
}
"#,
    );

    let run = async |cap: usize, name: &str| -> Result<Vec<usize>> {
        let mut config = GeneratorConfig::default();
        config.output.path = dir.path().join(name);
        config.generation.entity_count = 3;
        config.generation.seed = Some(5);
        config.generation.cardinality_strategy = CardinalityStrategy::Maximum;
        config.generation.unbounded_property_values = cap;

        let mut generator = DataGenerator::new(config.clone())?;
        generator.load_schema_auto(&schema).await?;
        generator.generate().await?;
        let content = std::fs::read_to_string(&config.output.path)?;
        Ok(counts_per_subject(&content, "tag"))
    };

    let few = run(2, "few.ttl").await?;
    let many = run(7, "many.ttl").await?;

    assert!(
        few.iter().all(|&n| n <= 2),
        "a cap of 2 must bound an unbounded property, got {few:?}"
    );
    assert!(
        many.iter().any(|&n| n > 2),
        "raising the cap must let more values through, got {many:?}"
    );
    Ok(())
}
