//! Equivalence by generation. The same constructs written in ShEx, SHACL and
//! PG-Schema must give the same graph from the same seed, triple for triple, and that
//! graph must be valid in the three languages. The schemas are the shipped examples in
//! `examples/schemas/equivalence/<group>.{pgs,shex,ttl}`; the RDF ones are written over
//! the RDF encoding of the property graph (keys, labels and edges as IRIs under
//! `http://rudof.example/pg/`), which is how a property graph and an RDF graph are
//! compared at all. L1 is the common fragment of Ahmetaj et al. (WWW 2025), L2 the
//! constructs outside it that the three languages share.

use crate::common::*;
use rudof_generate::GeneratorConfig;
use rudof_generate::config::OutputFormat;
use std::collections::BTreeSet;
use std::path::PathBuf;

const ENTITIES: usize = 120;
const SEED: u64 = 13;

/// The shipped schema `examples/schemas/equivalence/<group>.<extension>`.
fn example(group: &str, extension: &str) -> String {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("examples/schemas/equivalence")
        .join(format!("{group}.{extension}"));
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()))
}

/// The triples of a Turtle document, as N-Triples lines.
fn triples(turtle: &str) -> BTreeSet<String> {
    oxttl::TurtleParser::new()
        .for_slice(turtle.as_bytes())
        .map(|t| t.expect("generated Turtle parses").to_string())
        .collect()
}

/// Generate from a ShEx or SHACL fixture; return the Turtle and the ShapeMap written.
async fn generate_rdf(fixture: &Fixture) -> (String, String) {
    let map = fixture.dir.path().join("out.shapemap");
    let mut cfg = config(&fixture.output(), ENTITIES, SEED);
    cfg.output.shapemap = Some(map.clone());
    let data = generate(fixture, cfg).await;
    (data, std::fs::read_to_string(map).unwrap())
}

/// Check one group: the same graph from the three schemas, valid in the three languages.
async fn assert_equivalent(group: &str) {
    let (pgs, shex_schema, shacl_schema) = (example(group, "pgs"), example(group, "shex"), example(group, "ttl"));

    // PG-Schema: the property graph, its type map, and the same graph as RDF.
    let pg_fixture = Fixture::pgschema(&pgs);
    let dir = pg_fixture.dir.path();
    let mut cfg = GeneratorConfig::default();
    cfg.generation.entity_count = ENTITIES;
    cfg.generation.seed = Some(SEED);
    cfg.output.path = dir.join("out.pg");
    cfg.output.format = OutputFormat::Pg;
    cfg.output.write_stats = false;
    cfg.output.shapemap = Some(dir.join("out.map"));
    cfg.output.rdf = Some(dir.join("out.ttl"));
    cfg.output.rdf_shapemap = Some(dir.join("out.shapemap"));
    let pg_graph = generate(&pg_fixture, cfg).await;
    let type_map = std::fs::read_to_string(dir.join("out.map")).unwrap();
    let pg_rdf = std::fs::read_to_string(dir.join("out.ttl")).unwrap();
    let pg_shapemap = std::fs::read_to_string(dir.join("out.shapemap")).unwrap();

    let (shex_rdf, shex_map) = generate_rdf(&Fixture::shex(&shex_schema)).await;
    let (shacl_rdf, _) = generate_rdf(&Fixture::shacl(&shacl_schema)).await;

    // The same data from the three languages.
    let base = triples(&pg_rdf);
    assert!(base.len() > ENTITIES, "{group}: too few triples: {}", base.len());
    assert_eq!(
        triples(&shex_rdf),
        base,
        "{group}: ShEx and PG-Schema generate different graphs"
    );
    assert_eq!(
        triples(&shacl_rdf),
        base,
        "{group}: SHACL and PG-Schema generate different graphs"
    );
    assert_eq!(shex_map.lines().count(), pg_shapemap.lines().count());

    // Valid in the three languages.
    let pg = validate_pgschema(&pg_graph, &pgs, &type_map);
    assert!(
        pg.failing.is_empty() && pg.conformant > 0,
        "{group} PG-Schema: {}",
        pg.report
    );
    let shex = validate_shex(&pg_rdf, &shex_schema, &pg_shapemap);
    assert!(
        shex.failing.is_empty() && shex.conformant > 0,
        "{group} ShEx: {}",
        shex.report
    );
    let shacl = validate_shacl(&pg_rdf, &shacl_schema);
    assert!(shacl.failing.is_empty(), "{group} SHACL: {}", shacl.report);
}

#[tokio::test]
async fn l1_keys_generate_the_same_valid_data_in_three_languages() {
    assert_equivalent("keys").await;
}

#[tokio::test]
async fn l1_facets_generate_the_same_valid_data_in_three_languages() {
    assert_equivalent("facets").await;
}

#[tokio::test]
async fn l2_labels_generate_the_same_valid_data_in_three_languages() {
    assert_equivalent("labels").await;
}

#[tokio::test]
async fn l2_references_generate_the_same_valid_data_in_three_languages() {
    assert_equivalent("references").await;
}

#[tokio::test]
async fn ldbc_snb_generates_the_same_valid_data_in_three_languages() {
    assert_equivalent("ldbc-snb").await;
}
