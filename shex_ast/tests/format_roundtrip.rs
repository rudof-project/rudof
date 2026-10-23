//! Round trips of the schemas of the W3C ShExTest `schemas` manifest between
//! ShExC, ShExJ and ShExR:
//!
//! - ShExC -> ShExJ -> schema
//! - ShExC -> ShExR (Turtle) -> schema
//! - ShExC -> ShExC -> schema
//! - ShExR -> ShExC -> schema
//! - ShExJ -> ShExC -> schema
//!
//! Each round trip must give the schema it starts from. The comparison ignores
//! the base, the prefixes, the source IRI and the labels of blank nodes, and
//! the differences that ShExR can't keep:
//!
//! - the order of the `EXTRA` predicates (in RDF, a set of triples)
//! - which use of a labelled triple expression holds its definition and which
//!   ones are references (`$:e` and `&:e`): in RDF, all of them are the same
//!   node. Each reference is replaced by its definition, and a round trip must
//!   not define the same label twice
//! - relative `IMPORT`s, which RDF resolves against the base

mod common;

use common::{read_manifest, schemas_dir};
use rudof_iri::IriS;
use rudof_rdf::rdf_core::{BuildRDF, RDFFormat};
use rudof_rdf::rdf_impl::{OxigraphInMemory, ReaderMode};
use serde_json::Value;
use shex_ast::ShExFormatter;
use shex_ast::ast::Schema;
use shex_ast::compact::ShExParser;
use shex_ast::shexr::shexr_builder::ShExRBuilder;
use shex_ast::shexr::shexr_parser::ShExRParser;
use std::collections::{BTreeMap, HashMap};

const BASE: &str = "http://example.org/schemas/";
const TRIPLE_EXPR_TYPES: [&str; 3] = ["TripleConstraint", "EachOf", "OneOf"];

/// The schema as ShExJ, without base, prefixes and source IRI, with the
/// differences described in the module documentation removed, and with the
/// blank node labels renamed in order of appearance.
fn normalize(schema: &Schema) -> Value {
    let mut schema = schema.clone().with_base(None).with_prefixmap(None);
    schema.with_source_iri(&IriS::new_unchecked("http://example.org/source"));
    let mut value = serde_json::to_value(&schema).expect("schema to JSON");
    if let Some(Value::Array(imports)) = value.get_mut("imports") {
        for import in imports {
            if let Value::String(s) = import
                && !s.contains(':')
            {
                *s = format!("{BASE}{s}");
            }
        }
    }
    let mut defs = HashMap::new();
    collect_triple_expr_defs(&value, &mut defs);
    expand_triple_expr_refs(&mut value, &defs, 0);
    sort_extra(&mut value);
    let mut bnodes = HashMap::new();
    rename_bnodes(&mut value, &mut bnodes);
    value
}

fn is_triple_expr(value: &Value) -> bool {
    value
        .get("type")
        .and_then(Value::as_str)
        .is_some_and(|t| TRIPLE_EXPR_TYPES.contains(&t))
}

/// The labelled triple expressions, and how many times each label is defined
fn collect_triple_expr_defs(value: &Value, defs: &mut HashMap<String, (Value, usize)>) {
    match value {
        Value::Object(map) => {
            if is_triple_expr(value)
                && let Some(Value::String(id)) = map.get("id")
            {
                defs.entry(id.clone()).or_insert_with(|| (value.clone(), 0)).1 += 1;
            }
            map.values().for_each(|v| collect_triple_expr_defs(v, defs));
        },
        Value::Array(vs) => vs.iter().for_each(|v| collect_triple_expr_defs(v, defs)),
        _ => {},
    }
}

/// Replaces each reference to a triple expression (a string in place of a
/// triple expression) by its definition
fn expand_triple_expr_refs(value: &mut Value, defs: &HashMap<String, (Value, usize)>, depth: usize) {
    if depth > 50 {
        return;
    }
    let expand = |v: &mut Value| {
        if let Value::String(label) = v
            && let Some((def, _)) = defs.get(label.as_str())
        {
            *v = def.clone();
        }
    };
    match value {
        Value::Object(map) => {
            let is_shape = map.get("type").and_then(Value::as_str) == Some("Shape");
            if is_shape && let Some(expression) = map.get_mut("expression") {
                expand(expression);
            }
            if let Some(Value::Array(expressions)) = map.get_mut("expressions") {
                expressions.iter_mut().for_each(expand);
            }
            map.values_mut()
                .for_each(|v| expand_triple_expr_refs(v, defs, depth + 1));
        },
        Value::Array(vs) => vs.iter_mut().for_each(|v| expand_triple_expr_refs(v, defs, depth + 1)),
        _ => {},
    }
}

fn sort_extra(value: &mut Value) {
    match value {
        Value::Object(map) => {
            if let Some(Value::Array(extra)) = map.get_mut("extra") {
                extra.sort_by_key(|v| v.to_string());
            }
            map.values_mut().for_each(sort_extra);
        },
        Value::Array(vs) => vs.iter_mut().for_each(sort_extra),
        _ => {},
    }
}

fn rename_bnodes(value: &mut Value, bnodes: &mut HashMap<String, String>) {
    match value {
        Value::String(s) if s.starts_with("_:") => {
            let n = bnodes.len();
            *s = bnodes.entry(s.clone()).or_insert_with(|| format!("_:b{n}")).clone();
        },
        Value::Array(vs) => vs.iter_mut().for_each(|v| rename_bnodes(v, bnodes)),
        Value::Object(map) => map.values_mut().for_each(|v| rename_bnodes(v, bnodes)),
        _ => {},
    }
}

fn source() -> IriS {
    IriS::new_unchecked("http://example.org/source")
}

fn via_shexj(schema: &Schema) -> Result<Schema, String> {
    let json = serde_json::to_string_pretty(schema).map_err(|e| format!("to ShExJ: {e}"))?;
    serde_json::from_str(&json).map_err(|e| format!("from ShExJ: {e}\n{json}"))
}

fn via_shexr(schema: &Schema) -> Result<Schema, String> {
    let mut graph = OxigraphInMemory::empty();
    ShExRBuilder::schema_to_rdf(schema, &mut graph).map_err(|e| format!("to ShExR: {e}"))?;
    let mut turtle = Vec::new();
    graph
        .serialize(&RDFFormat::Turtle, &mut turtle)
        .map_err(|e| format!("serialize Turtle: {e}"))?;
    let turtle = String::from_utf8(turtle).unwrap();
    let graph = OxigraphInMemory::from_str(&turtle, &RDFFormat::Turtle, Some(BASE), &ReaderMode::Strict)
        .map_err(|e| format!("parse Turtle: {e}\n{turtle}"))?;
    ShExRParser::new(graph)
        .parse()
        .map_err(|e| format!("from ShExR: {e}\n{turtle}"))
}

fn via_shexc(schema: &Schema) -> Result<Schema, String> {
    let shexc = ShExFormatter::default().without_colors().format_schema(schema);
    ShExParser::parse(&shexc, None, &source()).map_err(|e| format!("from ShExC: {e}\n{shexc}"))
}

fn check(schema: &Schema, round_trip: fn(&Schema) -> Result<Schema, String>) -> Result<(), String> {
    let back = round_trip(schema)?;
    let mut defs = HashMap::new();
    collect_triple_expr_defs(&serde_json::to_value(&back).unwrap(), &mut defs);
    let mut twice: Vec<_> = defs
        .iter()
        .filter(|(_, (_, n))| *n > 1)
        .map(|(id, _)| id.clone())
        .collect();
    if !twice.is_empty() {
        twice.sort();
        return Err(format!("triple expressions defined more than once: {twice:?}"));
    }
    let (expected, found) = (normalize(schema), normalize(&back));
    if expected == found {
        Ok(())
    } else {
        Err(format!(
            "different schema\nexpected: {}\nfound:    {}",
            serde_json::to_string(&expected).unwrap(),
            serde_json::to_string(&found).unwrap()
        ))
    }
}

#[test]
fn shextest_schemas_round_trip_between_formats() {
    let dir = schemas_dir();
    let entries = read_manifest(&dir);
    assert!(entries.len() > 400, "expected the full shexTest schemas corpus");
    let base = IriS::new_unchecked(BASE);

    let mut failures: BTreeMap<&str, Vec<(String, String)>> = BTreeMap::new();
    for entry in &entries {
        let shex_path = dir.join(&entry.shex);
        if let Ok(shexc) = std::fs::read_to_string(&shex_path) {
            match ShExParser::parse(&shexc, Some(base.clone()), &source()) {
                Ok(schema) => {
                    for (name, round_trip) in [
                        ("ShExC -> ShExJ", via_shexj as fn(&Schema) -> Result<Schema, String>),
                        ("ShExC -> ShExR", via_shexr),
                        ("ShExC -> ShExC", via_shexc),
                    ] {
                        if let Err(e) = check(&schema, round_trip) {
                            failures.entry(name).or_default().push((entry.name.clone(), e));
                        }
                    }
                },
                Err(e) => failures
                    .entry("parse ShExC")
                    .or_default()
                    .push((entry.name.clone(), e.to_string())),
            }
        }
        let json_path = dir.join(&entry.json);
        if let Ok(json) = std::fs::read_to_string(&json_path) {
            match Schema::from_json_str(&json) {
                Ok(schema) => {
                    if let Err(e) = check(&schema, via_shexc) {
                        failures
                            .entry("ShExJ -> ShExC")
                            .or_default()
                            .push((entry.name.clone(), e));
                    }
                },
                Err(e) => failures
                    .entry("parse ShExJ")
                    .or_default()
                    .push((entry.name.clone(), e.to_string())),
            }
        }
        let ttl_path = dir.join(&entry.ttl);
        if let Ok(ttl) = std::fs::read_to_string(&ttl_path) {
            let parsed = OxigraphInMemory::from_str(&ttl, &RDFFormat::Turtle, Some(base.as_str()), &ReaderMode::Strict)
                .map_err(|e| e.to_string())
                .and_then(|graph| ShExRParser::new(graph).parse().map_err(|e| e.to_string()));
            match parsed {
                Ok(schema) => {
                    if let Err(e) = check(&schema, via_shexc) {
                        failures
                            .entry("ShExR -> ShExC")
                            .or_default()
                            .push((entry.name.clone(), e));
                    }
                },
                Err(e) => failures.entry("parse ShExR").or_default().push((entry.name.clone(), e)),
            }
        }
    }

    let mut report = String::new();
    for (name, fails) in &failures {
        report.push_str(&format!("\n== {name}: {} failures\n", fails.len()));
        for (entry, e) in fails {
            report.push_str(&format!(
                "-- {entry}: {}\n",
                e.lines().take(6).collect::<Vec<_>>().join("\n   ")
            ));
        }
    }
    if let Ok(path) = std::env::var("ROUNDTRIP_REPORT") {
        std::fs::write(path, &report).unwrap();
    }
    assert!(failures.is_empty(), "{report}");
}

/// A node constraint with nothing in it accepts any node: in ShExC, that is `.`
#[test]
fn empty_node_constraint_is_printed_as_a_dot() {
    let shexr = r#"
prefix sx: <http://www.w3.org/ns/shex#>
prefix : <http://example.org/>

<> a sx:Schema ; sx:shapes ( :Book ) .
:Book a sx:ShapeDecl ;
  sx:shapeExpr [ a sx:Shape ;
    sx:expression [ a sx:EachOf ;
      sx:expressions ( [ a sx:TripleConstraint ; sx:predicate :related ; sx:valueExpr [ a sx:NodeConstraint ] ] )
    ]
  ] .
"#;
    let graph = OxigraphInMemory::from_str(shexr, &RDFFormat::Turtle, Some(BASE), &ReaderMode::Strict).unwrap();
    let schema = ShExRParser::new(graph).parse().unwrap();
    let shexc = ShExFormatter::default().without_colors().format_schema(&schema);
    assert!(shexc.contains(":Book { :related . }"), "{shexc}");
    ShExParser::parse(&shexc, None, &source()).expect("the ShExC is valid");
}
