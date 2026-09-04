//! Does the generated data satisfy the schema it was generated from?
//!
//! This is the property the generator exists to provide, and until now nothing
//! asserted it. The existing suite checks that output is produced and parses,
//! which is why a run could type every entity with the wrong IRI, emit two
//! values for a property capped at one, and drop required properties entirely,
//! all while the tests stayed green.
//!
//! The checker below covers the core constraints -- cardinality, node kind
//! and datatype -- read from the SHACL shapes graph rather than from
//! the generator's own intermediate representation. It is deliberately not a
//! complete SHACL implementation: the point is to close the loop between input
//! and output, not to reimplement validation.

#![cfg(not(target_family = "wasm"))]

use rudof_generate::config::{CardinalityStrategy, GeneratorConfig};
use rudof_generate::{DataGenerator, errors::Result};
use rudof_rdf::rdf_core::{NeighsRDF, RDFFormat};
use rudof_rdf::rdf_impl::{OxigraphInMemory, ReaderMode};
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use tempfile::TempDir;

const RDF_TYPE: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#type";

/// One property constraint, as the shapes graph states it.
#[derive(Debug, Clone)]
struct PropertyRule {
    path: String,
    min_count: Option<usize>,
    max_count: Option<usize>,
    node_kind_iri: bool,
    datatype: Option<String>,
}

/// One node shape: the class it targets and the rules its instances must meet.
#[derive(Debug, Clone)]
struct ShapeRule {
    target_class: String,
    properties: Vec<PropertyRule>,
}

fn write(dir: &TempDir, name: &str, body: &str) -> PathBuf {
    let path = dir.path().join(name);
    std::fs::write(&path, body).expect("failed to write test schema");
    path
}

fn strip(iri: &str) -> String {
    iri.trim_matches(['<', '>']).to_string()
}

/// Read the shapes graph with the same parser the generator uses, so the test
/// and the code under test agree about what the file says.
fn parse_shapes(turtle: &str) -> Vec<ShapeRule> {
    let graph = OxigraphInMemory::from_str(turtle, &RDFFormat::Turtle, None, &ReaderMode::Strict)
        .expect("shapes graph should parse");
    let sh = |name: &str| format!("http://www.w3.org/ns/shacl#{name}");

    // Collect every (subject, predicate) -> objects, which is all the shape
    // structure this checker needs.
    let mut by_subject: HashMap<String, HashMap<String, Vec<String>>> = HashMap::new();
    for triple in graph.triples().expect("shapes graph should be readable") {
        by_subject
            .entry(triple.subject.to_string())
            .or_default()
            .entry(strip(&triple.predicate.to_string()))
            .or_default()
            .push(triple.object.to_string());
    }

    let one = |props: &HashMap<String, Vec<String>>, key: &str| -> Option<String> {
        props.get(key).and_then(|v| v.first()).cloned()
    };

    let mut shapes = Vec::new();
    for (subject, props) in &by_subject {
        let Some(target) = one(props, &sh("targetClass")) else {
            continue;
        };
        let mut properties = Vec::new();
        for property_shape in props.get(&sh("property")).cloned().unwrap_or_default() {
            let Some(rule_props) = by_subject.get(&property_shape) else {
                continue;
            };
            let Some(path) = one(rule_props, &sh("path")) else {
                continue;
            };
            let number = |key: &str| -> Option<usize> {
                one(rule_props, &sh(key))
                    .and_then(|v| v.split('"').nth(1).map(str::to_string).or(Some(v)))
                    .and_then(|v| v.parse::<usize>().ok())
            };
            properties.push(PropertyRule {
                path: strip(&path),
                min_count: number("minCount"),
                max_count: number("maxCount"),
                node_kind_iri: one(rule_props, &sh("nodeKind"))
                    .map(|nk| strip(&nk).ends_with("#IRI"))
                    .unwrap_or(false),
                datatype: one(rule_props, &sh("datatype")).map(|d| strip(&d)),
            });
        }
        let _ = subject;
        shapes.push(ShapeRule {
            target_class: strip(&target),
            properties,
        });
    }
    shapes
}

/// The outcome of checking a graph: what failed, and how much was actually
/// looked at.
///
/// The second number matters as much as the first. A shape selects its focus
/// nodes by target class, so if the generated entities carry some other type
/// nothing is selected, nothing is checked, and an empty failure list means the
/// schema never engaged with the data rather than that the data was good. Every
/// assertion below therefore requires coverage as well as silence.
struct Conformance {
    failures: Vec<String>,
    subjects_checked: usize,
    subjects_total: usize,
}

fn conformance_failures(graph: &OxigraphInMemory, shapes: &[ShapeRule]) -> Conformance {
    let mut by_subject: HashMap<String, Vec<(String, String)>> = HashMap::new();
    for triple in graph.triples().expect("graph should be readable") {
        by_subject
            .entry(triple.subject.to_string())
            .or_default()
            .push((strip(&triple.predicate.to_string()), triple.object.to_string()));
    }

    let mut failures = Vec::new();
    let mut subjects_checked = 0usize;
    let subjects_total = by_subject.len();
    for (subject, triples) in &by_subject {
        let types: HashSet<String> = triples
            .iter()
            .filter(|(p, _)| p == RDF_TYPE)
            .map(|(_, o)| strip(o))
            .collect();

        if shapes.iter().any(|s| types.contains(&s.target_class)) {
            subjects_checked += 1;
        }

        for shape in shapes {
            if !types.contains(&shape.target_class) {
                continue;
            }
            for rule in &shape.properties {
                if rule.path == RDF_TYPE {
                    continue;
                }
                let values: Vec<&String> = triples
                    .iter()
                    .filter(|(p, _)| *p == rule.path)
                    .map(|(_, o)| o)
                    .collect();

                if let Some(min) = rule.min_count
                    && values.len() < min
                {
                    failures.push(format!(
                        "{subject} has {} of <{}>, minCount is {min}",
                        values.len(),
                        rule.path
                    ));
                }
                if let Some(max) = rule.max_count
                    && values.len() > max
                {
                    failures.push(format!(
                        "{subject} has {} of <{}>, maxCount is {max}",
                        values.len(),
                        rule.path
                    ));
                }
                for value in &values {
                    if rule.node_kind_iri && !value.starts_with('<') {
                        failures.push(format!("{subject} <{}> {value} is not an IRI", rule.path));
                    }
                    if let Some(datatype) = &rule.datatype
                        && !value.starts_with('"')
                    {
                        failures.push(format!("{subject} <{}> {value} is not a {datatype} literal", rule.path));
                    }
                }
            }
        }
    }
    failures.sort();
    Conformance {
        failures,
        subjects_checked,
        subjects_total,
    }
}

/// Assert that the graph both was checked and passed.
fn assert_conforms(result: &Conformance, context: &str) {
    assert!(
        result.subjects_checked > 0,
        "{context}: no generated entity was selected by any shape, so nothing was \
         validated -- the entities are not typed with the class their shape targets"
    );
    assert!(
        result.failures.is_empty(),
        "{context}: {} of {} subjects checked, {} violation(s):\n  {}",
        result.subjects_checked,
        result.subjects_total,
        result.failures.len(),
        result.failures.join("\n  ")
    );
}

async fn generate(schema: &PathBuf, out: PathBuf, entities: usize) -> Result<OxigraphInMemory> {
    let mut config = GeneratorConfig::default();
    config.output.path = out.clone();
    config.generation.entity_count = entities;
    config.generation.seed = Some(2024);
    config.generation.cardinality_strategy = CardinalityStrategy::Maximum;

    let mut generator = DataGenerator::new(config)?;
    generator.load_schema_auto(schema).await?;
    generator.generate().await?;

    let turtle = std::fs::read_to_string(&out)?;
    Ok(
        OxigraphInMemory::from_str(&turtle, &RDFFormat::Turtle, None, &ReaderMode::Strict)
            .expect("generated output should parse as Turtle"),
    )
}

/// A schema exercising every route through the generator at once: literal
/// properties, a property whose range is another shape, a property constrained
/// only to be an IRI, and both an optional and a bounded-multiple property.
const MIXED_SCHEMA: &str = r#"
@prefix sh:  <http://www.w3.org/ns/shacl#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .
@prefix ex:  <http://example.org/> .
@prefix :    <http://weso.es/shapes/> .

:Department a sh:NodeShape ;
    sh:targetClass ex:Department ;
    sh:property [ sh:path ex:name ; sh:datatype xsd:string ; sh:minCount 1 ; sh:maxCount 1 ] .

:Person a sh:NodeShape ;
    sh:targetClass ex:Person ;
    sh:property [ sh:path ex:name      ; sh:datatype xsd:string  ; sh:minCount 1 ; sh:maxCount 1 ] ;
    sh:property [ sh:path ex:age       ; sh:datatype xsd:integer ; sh:minCount 1 ; sh:maxCount 1 ] ;
    sh:property [ sh:path ex:worksFor  ; sh:node :Department     ; sh:minCount 1 ; sh:maxCount 1 ] ;
    sh:property [ sh:path ex:homepage  ; sh:nodeKind sh:IRI      ; sh:minCount 1 ; sh:maxCount 1 ] ;
    sh:property [ sh:path ex:nickname  ; sh:datatype xsd:string  ; sh:maxCount 1 ] ;
    sh:property [ sh:path ex:tag       ; sh:datatype xsd:string  ; sh:minCount 1 ; sh:maxCount 3 ] .
"#;

#[tokio::test]
async fn generated_data_conforms_to_the_schema_it_came_from() -> Result<()> {
    let dir = tempfile::tempdir().unwrap();
    let schema = write(&dir, "mixed.ttl", MIXED_SCHEMA);

    let graph = generate(&schema, dir.path().join("out.ttl"), 40).await?;
    let result = conformance_failures(&graph, &parse_shapes(MIXED_SCHEMA));

    assert_conforms(&result, "generated data must satisfy its own schema");
    Ok(())
}

/// The same property, checked twice: the linking phase owns shape-valued
/// properties, so the value phase must not also emit one.
#[tokio::test]
async fn a_shape_valued_property_is_emitted_exactly_once() -> Result<()> {
    let dir = tempfile::tempdir().unwrap();
    let schema = write(&dir, "mixed.ttl", MIXED_SCHEMA);

    let graph = generate(&schema, dir.path().join("out.ttl"), 40).await?;
    let mut counts: HashMap<String, usize> = HashMap::new();
    for triple in graph.triples().expect("graph should be readable") {
        if triple.predicate.to_string().contains("worksFor") {
            *counts.entry(triple.subject.to_string()).or_insert(0) += 1;
        }
    }

    assert!(!counts.is_empty(), "expected the shape-valued property to be emitted");
    let over: Vec<_> = counts.iter().filter(|(_, n)| **n > 1).collect();
    assert!(
        over.is_empty(),
        "sh:maxCount 1 must mean one value, found subjects with more: {over:?}"
    );
    Ok(())
}

/// Conformance must not depend on the entity count, which decides how many
/// targets the linking phase has to choose from.
#[tokio::test]
async fn conformance_holds_at_several_scales() -> Result<()> {
    let dir = tempfile::tempdir().unwrap();
    let schema = write(&dir, "mixed.ttl", MIXED_SCHEMA);
    let shapes = parse_shapes(MIXED_SCHEMA);

    for entities in [2, 9, 50] {
        let graph = generate(&schema, dir.path().join(format!("out{entities}.ttl")), entities).await?;
        let result = conformance_failures(&graph, &shapes);
        assert_conforms(&result, &format!("at {entities} entities"));
    }
    Ok(())
}
