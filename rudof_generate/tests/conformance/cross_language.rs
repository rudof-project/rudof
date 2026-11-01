//! One graph, three languages. A graph generated from a PG-Schema and broken once
//! is written both as a property graph and as RDF, and validated against the
//! PG-Schema and against ShEx and SHACL translations of it. Each output has its
//! own labels, because the languages place a reference failure differently: in a
//! property graph the failing edge is labelled and nothing propagates further; in
//! RDF the node that holds the reference fails, and so does every node that
//! reaches it. Both label sets must equal what the validators report.

use crate::common::*;
use rudof_generate::GeneratorConfig;
use rudof_generate::config::OutputFormat;
use serde_json::Value;
use std::collections::BTreeSet;

/// A shipped example schema, `examples/schemas/<relative>`.
fn example(relative: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("examples/schemas")
        .join(relative);
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()))
}

/// The running example, Person -worksFor-> Org -locatedIn-> City: two reference steps,
/// no recursion (SHACL leaves recursion undefined), as a PG-Schema and as its faithful
/// ShEx and SHACL translations (closed shapes, required keys).
fn chain_pgs() -> String {
    example("equivalence/references.pgs")
}

fn chain_shex() -> String {
    example("equivalence/references.shex")
}

fn chain_shacl() -> String {
    example("equivalence/references.ttl")
}

/// The SHACL a modeller would write for the same schema: shapes open, and keys without
/// `sh:minCount`, since SHACL properties are optional by default.
fn chain_shacl_idiomatic() -> String {
    example("cross-language/references-idiomatic.ttl")
}

struct Run {
    graph: String,
    type_map: String,
    pg_labels: Vec<Value>,
    rdf: String,
    rdf_shapemap: String,
    rdf_labels: Vec<Value>,
}

fn read_labels(path: &std::path::Path) -> Vec<Value> {
    std::fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("{}: {e}", path.display()))
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| serde_json::from_str(l).expect("label line is JSON"))
        .collect()
}

fn failing(labels: &[Value]) -> BTreeSet<String> {
    labels
        .iter()
        .filter(|l| l["kind"] != "trap")
        .map(|l| l["node"].as_str().unwrap().to_string())
        .collect()
}

/// `reading` is extra TOML for `[violations.rdf]` (how the RDF translation reads
/// the schema's defaults); `operators` restricts the operators when not empty.
async fn run(entities: usize, rate: f64, operators: &[&str], reading: &str) -> Run {
    let fixture = Fixture::pgschema(&chain_pgs());
    let dir = fixture.dir.path();
    let toml = format!(
        "version = 2\n[run]\nseed = 11\nscale = {{ entities = {entities} }}\n[violations]\nrate = {rate}\n\
         {}[violations.rdf]\n{reading}\n[output]\ndir = \"{}\"\nformats = [\"pg\", \"turtle\"]\nstats = false\n",
        if operators.is_empty() {
            String::new()
        } else {
            let quoted: Vec<String> = operators.iter().map(|o| format!("\"{o}\"")).collect();
            format!("operators = [{}]\n", quoted.join(", "))
        },
        dir.display().to_string().replace('\\', "/"),
    );
    let cfg = GeneratorConfig::from_toml_str(&toml).unwrap_or_else(|e| panic!("{e}\n{toml}"));
    assert_eq!(cfg.output.format, OutputFormat::Pg);
    let graph = generate(&fixture, cfg).await;
    Run {
        graph,
        type_map: std::fs::read_to_string(dir.join("data.typemap")).unwrap(),
        pg_labels: read_labels(&dir.join("labels.jsonl")),
        rdf: std::fs::read_to_string(dir.join("data.ttl")).unwrap(),
        rdf_shapemap: std::fs::read_to_string(dir.join("data.shapemap")).unwrap(),
        rdf_labels: read_labels(&dir.join("labels-rdf.jsonl")),
    }
}

/// IRIs as the validators print them: without angle brackets.
fn bare(set: &BTreeSet<String>) -> BTreeSet<String> {
    set.iter()
        .map(|n| n.trim_matches(|c| c == '<' || c == '>').to_string())
        .collect()
}

#[test]
fn the_rdf_translations_accept_a_conforming_chain() {
    let data = "@prefix k: <http://rudof.example/pg/key/> .
@prefix l: <http://rudof.example/pg/label/> .
<http://x/p> a l:Person ; k:name \"Ann\" ;
  <http://rudof.example/pg/edge/WorksForType/worksFor/conforming-source> <http://x/o> .
<http://x/o> a l:Org ; k:label \"ACME\" ;
  <http://rudof.example/pg/edge/LocatedInType/locatedIn/conforming-source> <http://x/c> .
<http://x/c> a l:City ; k:name \"Oviedo\" .";
    let map = "<http://x/p>@<http://rudof.example/pg/type/PersonType>,\n\
               <http://x/o>@<http://rudof.example/pg/type/OrgType>,\n\
               <http://x/c>@<http://rudof.example/pg/type/CityType>";
    let shex = validate_shex(data, &chain_shex(), map);
    assert!(shex.failing.is_empty(), "{}", shex.report);
    let shacl = validate_shacl(data, &chain_shacl());
    assert!(shacl.failing.is_empty(), "{}", shacl.report);
    // A city without a name fails, and so do the org and the person that reach it.
    let broken = data.replace("k:name \"Oviedo\"", "k:label \"Oviedo\"");
    let shacl = validate_shacl(&broken, &chain_shacl());
    assert_eq!(
        bare(&shacl.failing),
        BTreeSet::from(["http://x/p".into(), "http://x/o".into(), "http://x/c".into()]),
        "{}",
        shacl.report
    );
}

#[tokio::test]
async fn a_clean_graph_conforms_in_all_three_languages() {
    let r = run(60, 0.0, &[], "").await;
    assert!(r.pg_labels.is_empty() && r.rdf_labels.is_empty());
    let pg = validate_pgschema(&r.graph, &chain_pgs(), &r.type_map);
    assert!(pg.failing.is_empty(), "{}", pg.report);
    let shex = validate_shex(&r.rdf, &chain_shex(), &r.rdf_shapemap);
    assert!(shex.failing.is_empty(), "{}", shex.report);
    let shacl = validate_shacl(&r.rdf, &chain_shacl());
    assert!(shacl.failing.is_empty(), "{}", shacl.report);
}

#[tokio::test]
async fn one_broken_graph_is_labelled_for_pg_schema_and_for_rdf() {
    let r = run(300, 0.3, &[], "").await;

    let pg = validate_pgschema(&r.graph, &chain_pgs(), &r.type_map);
    assert_eq!(pg.failing, failing(&r.pg_labels), "{}", pg.report);

    let expected_rdf = bare(&failing(&r.rdf_labels));
    let shex = validate_shex(&r.rdf, &chain_shex(), &r.rdf_shapemap);
    assert_eq!(bare(&shex.failing), expected_rdf, "{}", shex.report);
    let shacl = validate_shacl(&r.rdf, &chain_shacl());
    assert_eq!(bare(&shacl.failing), expected_rdf, "{}", shacl.report);

    // The two semantics differ: in RDF a person fails when the city of its
    // organisation fails (two steps); in the property graph no person does.
    let induced_rdf: Vec<_> = r.rdf_labels.iter().filter(|l| l["kind"] == "induced").collect();
    assert!(
        induced_rdf
            .iter()
            .any(|l| l["shape"].as_str().unwrap().ends_with("PersonType")
                && l["cause"].as_str().is_some_and(|c| c.contains("OrgType"))),
        "expected a two-step induced failure: {induced_rdf:?}"
    );
    assert!(
        r.pg_labels
            .iter()
            .all(|l| l["kind"] != "induced" || l["node"].as_str().unwrap().contains("__")),
        "property graph induced failures are edges"
    );
}

/// Default closedness and default cardinality differ between the languages, so
/// the same mutation fails in PG-Schema and is harmless in idiomatic SHACL: an
/// undeclared key (records are closed, shapes are open) and a dropped key (keys
/// are required unless OPTIONAL, SHACL properties are optional).
#[tokio::test]
async fn language_defaults_decide_whether_the_same_mutation_fails() {
    let r = run(
        300,
        0.3,
        &["closed-extra", "drop-required", "dangling-reference"],
        "closed = false\nkeys_required = false",
    )
    .await;

    let pg = validate_pgschema(&r.graph, &chain_pgs(), &r.type_map);
    assert_eq!(pg.failing, failing(&r.pg_labels), "{}", pg.report);
    let shacl = validate_shacl(&r.rdf, &chain_shacl_idiomatic());
    assert_eq!(bare(&shacl.failing), bare(&failing(&r.rdf_labels)), "{}", shacl.report);

    // Every node broken in the property graph is a trap in the RDF reading.
    let broken_pg = r.pg_labels.iter().filter(|l| l["kind"] == "direct").count();
    assert!(broken_pg > 20, "too few broken nodes: {broken_pg}");
    let rdf_traps = r.rdf_labels.iter().filter(|l| l["kind"] == "trap").count();
    assert_eq!(rdf_traps, broken_pg, "{:?}", r.rdf_labels);
    assert!(shacl.failing.is_empty(), "{}", shacl.report);
}

/// A node type with two labels is typed with both in RDF, as a ShEx shape with two
/// `rdf:type` constraints or a SHACL shape with a target class and `sh:hasValue` is.
#[tokio::test]
async fn a_node_type_with_two_labels_is_typed_with_both_in_rdf() {
    let pgs = "CREATE NODE TYPE ( StaffType : Person & Employee { name: STRING })";
    let shacl = "@prefix sh: <http://www.w3.org/ns/shacl#> .
@prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
@prefix k: <http://rudof.example/pg/key/> .
@prefix l: <http://rudof.example/pg/label/> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .
<http://rudof.example/pg/type/StaffType> a sh:NodeShape ; sh:targetClass l:Person ;
  sh:closed true ; sh:ignoredProperties ( rdf:type ) ;
  sh:property [ sh:path rdf:type ; sh:hasValue l:Employee ] ;
  sh:property [ sh:path k:name ; sh:datatype xsd:string ; sh:minCount 1 ; sh:maxCount 1 ] .";
    let fixture = Fixture::pgschema(pgs);
    let dir = fixture.dir.path();
    let mut cfg = GeneratorConfig::from_toml_str("version = 2\n[run]\nseed = 3\nscale = { entities = 20 }\n").unwrap();
    cfg.output.path = dir.join("out.pg");
    cfg.output.format = OutputFormat::Pg;
    cfg.output.write_stats = false;
    cfg.output.shapemap = Some(dir.join("out.map"));
    cfg.output.rdf = Some(dir.join("out.ttl"));
    let graph = generate(&fixture, cfg).await;
    let pg = validate_pgschema(&graph, pgs, &std::fs::read_to_string(dir.join("out.map")).unwrap());
    assert!(pg.failing.is_empty(), "{}", pg.report);
    let rdf = std::fs::read_to_string(dir.join("out.ttl")).unwrap();
    assert_eq!(
        instances_of(&rdf, "http://rudof.example/pg/label/Employee").len(),
        20,
        "{rdf}"
    );
    let v = validate_shacl(&rdf, shacl);
    assert!(v.failing.is_empty(), "{}", v.report);
}
