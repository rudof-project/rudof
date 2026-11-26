//! Data-driven suite for the subgraphs a ShEx validation can extract (issue #957).
//!
//! Each directory under `tests/subgraph/` is a case with:
//!
//! - `schema.shex`: the ShEx schema (ShExC),
//! - `data.ttl`: the RDF data (Turtle),
//! - `shapemap.sm`: the query shapemap, using the prefixes of the schema and the data,
//! - `valid.nt`: the expected *valid* subgraph: only the triples that some conformant
//!   `(node, shape)` pair of the shapemap matched, following the references it used,
//! - `visited.nt`: the expected *visited* subgraph: every triple the validator read
//!   while validating, conformant or not,
//! - `invalid.nt`: the expected *invalid* subgraph: the triples read that are not in
//!   the valid subgraph, so that `valid` and `invalid` split `visited`.
//!
//! Both expected graphs are compared as sets of N-Triples lines, so the cases avoid
//! blank nodes. Run with `BLESS_SUBGRAPH=1` to (re)write `visited.nt` from what the
//! validator currently reads, and review the diff before committing it.

use rudof_iri::IriS;
use rudof_rdf::rdf_core::RDFFormat;
use rudof_rdf::rdf_impl::{OxigraphInMemory, ReaderMode};
use rudof_rdf_sink::{GraphSink, TriplesSink};
use shex_ast::ir::{map_state::MapState, schema_ir::SchemaIR, semantic_actions_registry::SemanticActionsRegistry};
use shex_ast::{ResolveMethod, ShExParser, ShapeMapParser, ir::ast2ir::AST2IR};
use shex_validation::{Validator, ValidatorConfig};
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

const BASE: &str = "http://example.org/";

struct Case {
    dir: PathBuf,
}

impl Case {
    fn read(&self, file: &str) -> String {
        std::fs::read_to_string(self.dir.join(file))
            .unwrap_or_else(|e| panic!("{}: cannot read {file}: {e}", self.dir.display()))
    }

    fn name(&self) -> String {
        self.dir.file_name().unwrap().to_string_lossy().into_owned()
    }
}

fn cases() -> Vec<Case> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("subgraph");
    let mut dirs: Vec<PathBuf> = std::fs::read_dir(&root)
        .expect("tests/subgraph")
        .map(|e| e.unwrap().path())
        .filter(|p| p.is_dir())
        .collect();
    dirs.sort();
    assert!(!dirs.is_empty(), "no cases in {}", root.display());
    dirs.into_iter().map(|dir| Case { dir }).collect()
}

fn compile(src: &str, config: &ValidatorConfig) -> SchemaIR {
    let base = IriS::new_unchecked(BASE);
    let ast = ShExParser::parse(src, Some(base.clone()), &base).expect("parse schema");
    let mut map_state = MapState::default();
    let registry = SemanticActionsRegistry::default();
    registry.set_map_state(&mut map_state);
    let mut compiler = AST2IR::new(&ResolveMethod::default(), map_state);
    let mut compiled = SchemaIR::new(registry);
    compiler
        .compile(
            &ast,
            &base,
            &Some(base.clone()),
            &mut compiled,
            config.external_resolvers(),
        )
        .expect("compile to IR");
    compiled
}

/// The subgraphs one validation of `case` produced.
struct Subgraphs {
    valid: BTreeSet<String>,
    visited: BTreeSet<String>,
    invalid: BTreeSet<String>,
}

fn run(case: &Case) -> Subgraphs {
    let visited_sink = Arc::new(Mutex::new(GraphSink::new(OxigraphInMemory::new())));
    let valid_sink = Arc::new(Mutex::new(GraphSink::new(OxigraphInMemory::new())));
    let invalid_sink = Arc::new(Mutex::new(GraphSink::new(OxigraphInMemory::new())));
    let config = ValidatorConfig::default()
        .with_read_sink(TriplesSink::new(visited_sink.clone()))
        .with_valid_sink(TriplesSink::new(valid_sink.clone()))
        .with_invalid_sink(TriplesSink::new(invalid_sink.clone()));
    let schema = compile(&case.read("schema.shex"), &config);
    let graph = OxigraphInMemory::from_str(&case.read("data.ttl"), &RDFFormat::Turtle, None, &ReaderMode::Strict)
        .expect("parse data");
    let nodes_prefixmap = Some(graph.prefixmap().clone());
    let shapemap = ShapeMapParser::parse(
        &case.read("shapemap.sm"),
        &nodes_prefixmap,
        &None,
        &Some(schema.prefixmap()),
        &None,
    )
    .expect("parse shapemap");
    let validator = Validator::new(&schema, &config).expect("validator");
    validator
        .validate_shapemap(&shapemap, &graph, &schema, &nodes_prefixmap)
        .expect("validate");
    Subgraphs {
        valid: lines(&valid_sink.lock().unwrap()),
        visited: lines(&visited_sink.lock().unwrap()),
        invalid: lines(&invalid_sink.lock().unwrap()),
    }
}

fn lines(sink: &GraphSink) -> BTreeSet<String> {
    sink.graph().quads().map(|q| format!("{q} .")).collect()
}

fn expected(case: &Case, file: &str) -> BTreeSet<String> {
    case.read(file)
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .map(String::from)
        .collect()
}

fn show(set: &BTreeSet<String>) -> String {
    set.iter().map(|l| format!("  {l}\n")).collect()
}

/// Compares `actual` with the expected graph in `file` for every case, reporting
/// all the cases that differ at once.
fn check(file: &str, actual: impl Fn(&Subgraphs) -> &BTreeSet<String>) {
    let bless = file == "visited.nt" && std::env::var_os("BLESS_SUBGRAPH").is_some();
    let mut failures = Vec::new();
    for case in cases() {
        let subgraphs = run(&case);
        let actual = actual(&subgraphs);
        if bless {
            let body: String = actual.iter().map(|l| format!("{l}\n")).collect();
            std::fs::write(case.dir.join(file), body).unwrap();
            continue;
        }
        let expected = expected(&case, file);
        if &expected != actual {
            failures.push(format!(
                "case `{}` ({file}):\n missing:\n{} unexpected:\n{}",
                case.name(),
                show(&expected.difference(actual).cloned().collect()),
                show(&actual.difference(&expected).cloned().collect()),
            ));
        }
    }
    assert!(failures.is_empty(), "\n{}", failures.join("\n"));
}

#[test]
fn the_valid_subgraph_has_only_the_triples_of_conformant_pairs() {
    check("valid.nt", |s| &s.valid);
}

#[test]
fn the_visited_subgraph_has_every_triple_the_validator_read() {
    check("visited.nt", |s| &s.visited);
}

#[test]
fn the_invalid_subgraph_has_the_triples_read_outside_the_valid_subgraph() {
    check("invalid.nt", |s| &s.invalid);
}

#[test]
fn the_valid_and_invalid_subgraphs_split_the_visited_subgraph() {
    for case in cases() {
        let s = run(&case);
        assert!(
            s.valid.is_disjoint(&s.invalid),
            "case `{}`: valid and invalid overlap",
            case.name()
        );
        let union: BTreeSet<String> = s.valid.union(&s.invalid).cloned().collect();
        assert_eq!(union, s.visited, "case `{}`: valid + invalid != visited", case.name());
    }
}
