//! Run the ShExMap example manifests (`examples/shexmap/shexjs`, shex.js's, and
//! `examples/shexmap/pyshex`, PyShEx's: ambiguity, EXTENDS and inverse cases) through
//! `shex_ast::shexmap`: bind, compare with the bindings shex.js recorded, materialize,
//! compare with the expected output, and check the round-trip laws.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use oxrdf::dataset::CanonicalizationAlgorithm;
use oxrdf::{Graph, Triple};
use rudof_iri::IriS;
use rudof_rdf::rdf_core::term::Object;
use rudof_rdf::rdf_core::{NeighsRDF, RDFFormat};
use rudof_rdf::rdf_impl::{OxigraphInMemory, ReaderMode};
use serde_json::{Map, Value, json};
use shex_ast::shexmap::{
    Bindings, MaterializerOptions, NODE_KEY, ShExMapError, analyse, bind, bind_all, materialize, parse_term,
};
use shex_ast::{Schema, ShExParser, ShapeExprLabel};

const TURTLE_BASE: &str = "http://a.example/turtle/"; // the base shex.js's test runner parses data with
const SCHEMA_BASE: &str = "http://a.example/schema/";

fn examples_dir() -> PathBuf {
    Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../examples/shexmap")).to_path_buf()
}

#[derive(Clone)]
struct Entry {
    dir: PathBuf,
    json: Map<String, Value>,
}

impl Entry {
    fn label(&self) -> String {
        format!(
            "{} / {}",
            self.json["schemaLabel"].as_str().unwrap(),
            self.json["dataLabel"].as_str().unwrap()
        )
    }

    fn text(&self, key: &str) -> String {
        if let Some(s) = self.json.get(key).and_then(Value::as_str) {
            return s.to_string();
        }
        let name = self.json[&format!("{key}URL")].as_str().unwrap();
        std::fs::read_to_string(self.dir.join(name)).unwrap_or_else(|e| panic!("{name}: {e}"))
    }

    fn file(&self, key: &str) -> String {
        let name = self.json[key].as_str().unwrap();
        std::fs::read_to_string(self.dir.join(name)).unwrap_or_else(|e| panic!("{name}: {e}"))
    }

    fn query_map(&self) -> (String, Option<ShapeExprLabel>) {
        node_and_shape(self.json["queryMap"].as_str().unwrap())
    }

    fn output_shape_map(&self) -> (Object, Option<ShapeExprLabel>) {
        let (node, shape) = node_and_shape(self.json["outputShapeMap"].as_str().unwrap());
        (parse_term(&node).unwrap(), shape)
    }

    fn input_graph(&self) -> OxigraphInMemory {
        turtle(&self.text("data"))
    }

    fn focus(&self, graph: &OxigraphInMemory) -> Object {
        let (node, _) = self.query_map();
        if node.starts_with("_:") {
            return only_root(graph);
        }
        let data = self.text("data");
        let base = data
            .lines()
            .find_map(|l| {
                l.trim()
                    .strip_prefix("BASE")
                    .map(|r| r.trim().trim_matches(|c| c == '<' || c == '>').to_string())
            })
            .unwrap_or_else(|| TURTLE_BASE.to_string());
        let iri = node.trim_matches(|c| c == '<' || c == '>');
        Object::iri(IriS::from_str_base(iri, Some(&base)).unwrap())
    }

    fn input_schema(&self) -> Schema {
        shexc(&self.text("schema"))
    }

    fn output_schema(&self) -> Schema {
        shexc(&self.text("outputSchema"))
    }

    fn expected_output(&self, name: Option<&str>) -> OxigraphInMemory {
        let name = name.map_or_else(
            || self.json["expectedOutputDataURL"].as_str().unwrap().to_string(),
            str::to_string,
        );
        turtle(&std::fs::read_to_string(self.dir.join(&name)).unwrap_or_else(|e| panic!("{name}: {e}")))
    }
}

fn node_and_shape(shape_map: &str) -> (String, Option<ShapeExprLabel>) {
    let (node, shape) = shape_map.trim().rsplit_once('@').unwrap();
    let shape = if shape == "START" {
        None
    } else {
        let iri = shape.trim_matches(|c| c == '<' || c == '>');
        Some(ShapeExprLabel::iri(
            IriS::from_str_base(iri, Some(SCHEMA_BASE)).unwrap(),
        ))
    };
    (node.to_string(), shape)
}

fn manifest(sub: &str) -> Vec<Entry> {
    let dir = examples_dir().join(sub);
    let text = std::fs::read_to_string(dir.join("manifest.json")).unwrap();
    let entries: Vec<Value> = serde_json::from_str(&text).unwrap();
    entries
        .into_iter()
        .map(|e| Entry {
            dir: dir.clone(),
            json: e.as_object().unwrap().clone(),
        })
        .collect()
}

fn all_entries() -> Vec<Entry> {
    let mut all = manifest("shexjs");
    all.extend(manifest("pyshex"));
    all
}

fn shexc(text: &str) -> Schema {
    ShExParser::parse(
        text,
        Some(IriS::new_unchecked(SCHEMA_BASE)),
        &IriS::new_unchecked(SCHEMA_BASE),
    )
    .unwrap_or_else(|e| panic!("schema does not parse: {e}\n{text}"))
}

fn turtle(text: &str) -> OxigraphInMemory {
    OxigraphInMemory::from_str(text, &RDFFormat::Turtle, Some(TURTLE_BASE), &ReaderMode::Strict)
        .unwrap_or_else(|e| panic!("data does not parse: {e}\n{text}"))
}

/// The data's only root: the subject that is nobody's object.
fn only_root(graph: &OxigraphInMemory) -> Object {
    let triples: Vec<oxrdf::Triple> = graph.triples().unwrap().collect();
    let objects: BTreeSet<String> = triples.iter().map(|t| t.object.to_string()).collect();
    let roots: BTreeSet<String> = triples
        .iter()
        .filter(|t| !objects.contains(&t.subject.to_string()))
        .map(|t| t.subject.to_string())
        .collect();
    assert_eq!(roots.len(), 1, "expected one root");
    let root = roots.into_iter().next().unwrap();
    parse_term(&root).unwrap()
}

fn as_graph(g: &OxigraphInMemory) -> Graph {
    let mut out = Graph::new();
    for t in g.triples().unwrap() {
        let t: Triple = t;
        out.insert(&t);
    }
    out
}

fn isomorphic(got: &OxigraphInMemory, expected: &OxigraphInMemory) -> bool {
    let mut a = as_graph(got);
    let mut b = as_graph(expected);
    a.canonicalize(CanonicalizationAlgorithm::Unstable);
    b.canonicalize(CanonicalizationAlgorithm::Unstable);
    a == b
}

fn assert_isomorphic(got: &OxigraphInMemory, expected: &OxigraphInMemory, what: &str) {
    let mut a = as_graph(got);
    let mut b = as_graph(expected);
    a.canonicalize(CanonicalizationAlgorithm::Unstable);
    b.canonicalize(CanonicalizationAlgorithm::Unstable);
    if a != b {
        let mut got_s: Vec<String> = a.iter().map(|t| t.to_string()).collect();
        let mut exp_s: Vec<String> = b.iter().map(|t| t.to_string()).collect();
        got_s.sort();
        exp_s.sort();
        panic!(
            "{what}: graphs differ\n--- got ---\n{}\n--- expected ---\n{}",
            got_s.join("\n"),
            exp_s.join("\n")
        );
    }
}

/// The tree with blank-node values reduced to a marker (labels are the parser's).
fn with_bnode_marker(v: &Value) -> Value {
    match v {
        Value::Object(m) => Value::Object(
            m.iter()
                .map(|(k, x)| {
                    let x = match x {
                        Value::String(s) if s.starts_with("_:") => Value::String("_:".to_string()),
                        other => other.clone(),
                    };
                    (k.clone(), x)
                })
                .collect(),
        ),
        Value::Array(a) => Value::Array(a.iter().map(with_bnode_marker).collect()),
        other => other.clone(),
    }
}

/// `with_bnode_marker`, each list's iterations sorted: ShEx matches bags, and the order a
/// neighbourhood is listed in follows the nodes' labels, which the parser chooses (oxigraph
/// labels anonymous nodes at random; n3.js and rdflib number them in document order).
fn sorted_lists(v: &Value) -> Value {
    match v {
        Value::Array(a) => {
            let mut items: Vec<Value> = a.iter().map(sorted_lists).collect();
            if items.iter().all(|e| !e.is_array()) || items.iter().all(|e| e.is_array() && !a[0].is_object()) {
                items.sort_by_key(|e| e.to_string());
            }
            Value::Array(items)
        },
        other => with_bnode_marker(other),
    }
}

/// `with_bnode_marker`, without `@node`, each list's iterations sorted.
fn canonical(v: &Value) -> Value {
    match v {
        Value::Object(m) => Value::Object(
            m.iter()
                .filter(|(k, _)| k.as_str() != NODE_KEY)
                .map(|(k, x)| {
                    let x = match x {
                        Value::String(s) if s.starts_with("_:") => Value::String("_:".to_string()),
                        other => other.clone(),
                    };
                    (k.clone(), x)
                })
                .collect(),
        ),
        Value::Array(a) => {
            let mut items: Vec<Value> = a.iter().map(canonical).collect();
            items.sort_by_key(|e| e.to_string());
            Value::Array(items)
        },
        other => other.clone(),
    }
}

fn bindings_of(entry: &Entry) -> (OxigraphInMemory, Bindings) {
    let graph = entry.input_graph();
    let (_, start) = entry.query_map();
    let focus = entry.focus(&graph);
    let bindings = bind(&graph, &entry.input_schema(), &focus, start.as_ref(), false)
        .unwrap_or_else(|e| panic!("[{}] bind: {e}", entry.label()));
    (graph, bindings)
}

fn materialized(entry: &Entry, bindings: &Bindings) -> OxigraphInMemory {
    let (root, shape) = entry.output_shape_map();
    materialize(
        &entry.output_schema(),
        bindings,
        Some(&root),
        shape.as_ref(),
        MaterializerOptions::default(),
    )
    .unwrap_or_else(|e| panic!("[{}] materialize: {e}", entry.label()))
}

#[test]
fn bindings_match_shexjs() {
    // The same tree as shex.js writes, @node included, blank-node labels aside.
    for entry in all_entries() {
        let Some(name) = entry.json.get("expectedBindingsURL").and_then(Value::as_str) else {
            continue;
        };
        let expected: Value = serde_json::from_str(&entry.file("expectedBindingsURL")).unwrap();
        let (_, bindings) = bindings_of(&entry);
        assert_eq!(
            sorted_lists(&bindings.to_json()),
            sorted_lists(&expected),
            "[{}] differs from {name}:\n{}",
            entry.label(),
            bindings.dumps(true)
        );
    }
}

// examples whose input schema has a repeated shape-valued constraint, so their trees have lists
// of iterations, each of which must say which node it matched
const HAS_ITERATIONS: &[&str] = &[
    "BPPatient multi-bindings / simple",
    "BPPatient 2 levels / simple",
    "splits / Ann-phone-mbox",
    "splits / Ann-mbox-phone",
    "inverse in / Ann's heart rate",
    "inverse out / Ann's chart",
    "EXTENDS / clinic",
];
const BP: &str = "http://shex.io/extensions/Map/#BPDAM-";

#[test]
fn iterations_carry_the_node_they_matched() {
    for entry in all_entries() {
        let (_, bindings) = bindings_of(&entry);
        let found = bindings.tree.nodes();
        let label = entry.label();
        if !HAS_ITERATIONS.contains(&label.as_str()) {
            assert!(found.is_empty(), "[{label}] unexpected @node");
            continue;
        }
        assert!(!found.is_empty(), "[{label}] no @node");
        for d in &found {
            assert!(
                matches!(d.own[NODE_KEY], Object::Iri(_) | Object::BlankNode(_)),
                "[{label}] @node must be an IRI or a blank node"
            );
        }
        if label == "BPPatient 2 levels / simple" {
            // the constraint that made the list also bound the node itself: the two must agree
            let nodes: BTreeSet<&Object> = found.iter().map(|d| &d.own[NODE_KEY]).collect();
            let bound: BTreeSet<&Object> = found
                .iter()
                .map(|d| {
                    d.own
                        .get(&format!("{BP}reports"))
                        .or_else(|| d.own.get(&format!("{BP}bp")))
                        .unwrap()
                })
                .collect();
            assert_eq!(nodes, bound);
            assert_eq!(found.len(), 6); // two reports, four readings
        }
        if label == "BPPatient multi-bindings / simple" {
            assert!(found.iter().all(|d| d.own[NODE_KEY] == d.own[&format!("{BP}XXX")]));
        }
        if label == "inverse in / Ann's heart rate" {
            assert!(
                found
                    .iter()
                    .all(|d| matches!(&d.own[NODE_KEY], Object::Iri(i) if i.as_str().starts_with("http://")))
            );
        }
    }
}

#[test]
fn nodes_survive_json() {
    for entry in all_entries() {
        let (_, bindings) = bindings_of(&entry);
        let again = Bindings::loads(&bindings.dumps(false)).unwrap();
        assert_eq!(again.to_json(), bindings.to_json(), "[{}]", entry.label());
        assert_eq!(again.frames(), bindings.frames());
        assert!(bindings.frames().iter().all(|f| !f.contains_key(NODE_KEY)));
    }
}

#[test]
fn bindings_survive_a_round_trip_through_the_input_schema() {
    // bind after materialize after bind is bind: materializing bindings with the *input*
    // schema gives a graph that binds to the same bindings, i.e. a schema maps to itself.
    // Compared as the set of parses, since an ambiguous input may parse in another order.
    for entry in all_entries() {
        let label = entry.label();
        let graph = entry.input_graph();
        let (_, start) = entry.query_map();
        let focus = entry.focus(&graph);
        let schema = entry.input_schema();
        let before: BTreeSet<String> = bind_all(&graph, &schema, &focus, start.as_ref(), 20)
            .unwrap()
            .iter()
            .map(|b| canonical(&b.to_json()).to_string())
            .collect();
        let root = match &focus {
            Object::BlankNode(_) => Object::bnode("root".to_string()),
            other => other.clone(),
        };
        let bindings = bind(&graph, &schema, &focus, start.as_ref(), false).unwrap();
        let out: OxigraphInMemory = materialize(
            &schema,
            &bindings,
            Some(&root),
            start.as_ref(),
            MaterializerOptions::default(),
        )
        .unwrap_or_else(|e| panic!("[{label}] materialize with the input schema: {e}"));
        let after: BTreeSet<String> = bind_all(&out, &schema, &root, start.as_ref(), 20)
            .unwrap_or_else(|e| panic!("[{label}] bind the materialized graph: {e}"))
            .iter()
            .map(|b| canonical(&b.to_json()).to_string())
            .collect();
        assert_eq!(after, before, "[{label}]");
    }
}

/// The outputs an entry allows: the expected one, or, for an ambiguous input, any of the
/// alternatives (which parse `bind` returns first depends on the data's blank-node labels).
fn expected_outputs(entry: &Entry) -> Vec<String> {
    match entry.json.get("alternativeOutputDataURLs") {
        Some(Value::Array(a)) => a.iter().map(|v| v.as_str().unwrap().to_string()).collect(),
        _ => vec![entry.json["expectedOutputDataURL"].as_str().unwrap().to_string()],
    }
}

fn assert_one_of_expected(entry: &Entry, out: &OxigraphInMemory) {
    let names = expected_outputs(entry);
    if names.len() == 1 {
        assert_isomorphic(out, &entry.expected_output(Some(&names[0])), &entry.label());
        return;
    }
    assert!(
        names.iter().any(|n| isomorphic(out, &entry.expected_output(Some(n)))),
        "[{}] the output is none of {names:?}",
        entry.label()
    );
}

#[test]
fn output_matches_expected() {
    for entry in all_entries() {
        let (_, bindings) = bindings_of(&entry);
        let out = materialized(&entry, &bindings);
        assert_one_of_expected(&entry, &out);
    }
}

#[test]
fn bindings_survive_json_into_materialization() {
    for entry in all_entries() {
        let (_, bindings) = bindings_of(&entry);
        let again = Bindings::loads(&bindings.dumps(true)).unwrap();
        let out = materialized(&entry, &again);
        assert_one_of_expected(&entry, &out);
    }
}

#[test]
fn materialize_from_shexjs_bindings() {
    // Bindings JSON written by shex.js materializes to the same graph here.
    for entry in all_entries() {
        if entry.json.get("expectedBindingsURL").is_none() {
            continue;
        }
        let bindings = Bindings::loads(&entry.file("expectedBindingsURL")).unwrap();
        let out = materialized(&entry, &bindings);
        assert_isomorphic(&out, &entry.expected_output(None), &entry.label());
    }
}

#[test]
fn every_parse_and_its_output() {
    // bind_all finds each distinct parse; bind reports the count; strict refuses ambiguity.
    for entry in all_entries() {
        let label = entry.label();
        let graph = entry.input_graph();
        let (_, start) = entry.query_map();
        let focus = entry.focus(&graph);
        let schema = entry.input_schema();
        let alternatives = bind_all(&graph, &schema, &focus, start.as_ref(), 20).unwrap();
        let expected = expected_outputs(&entry);
        assert_eq!(alternatives.len(), expected.len(), "[{label}]");
        // each parse's output is one of the expected outputs, and each expected output is made
        let mut taken: Vec<bool> = vec![false; expected.len()];
        for bindings in &alternatives {
            let out = materialized(&entry, bindings);
            let hit = expected
                .iter()
                .enumerate()
                .find(|(i, n)| !taken[*i] && isomorphic(&out, &entry.expected_output(Some(n))))
                .map(|(i, _)| i)
                .unwrap_or_else(|| panic!("[{label}] a parse gives none of the remaining outputs {expected:?}"));
            taken[hit] = true;
        }
        assert_eq!(
            bind(&graph, &schema, &focus, start.as_ref(), false)
                .unwrap()
                .alternatives,
            expected.len()
        );
        if expected.len() > 1 {
            match bind(&graph, &schema, &focus, start.as_ref(), true) {
                Err(ShExMapError::Ambiguous { alternatives, .. }) => assert_eq!(alternatives.len(), expected.len()),
                other => panic!("[{label}] strict should refuse: {other:?}"),
            }
        }
    }
}

#[test]
fn schema_pairs_analyse_cleanly() {
    for entry in all_entries() {
        let (_, start) = entry.query_map();
        let (_, out_start) = entry.output_shape_map();
        let statics: Vec<String> = entry
            .json
            .get("staticVars")
            .and_then(Value::as_object)
            .map(|m| m.keys().cloned().collect())
            .unwrap_or_default();
        let report = analyse(
            &entry.input_schema(),
            &entry.output_schema(),
            start.as_ref(),
            out_start.as_ref(),
            &statics,
        )
        .unwrap_or_else(|e| panic!("[{}] analyse: {e}", entry.label()));
        assert!(report.ok(), "[{}]\n{report}", entry.label());
    }
}

#[test]
fn a_non_conformant_focus_is_refused() {
    let entry = &manifest("shexjs")[0];
    let graph = entry.input_graph();
    let schema = entry.input_schema();
    let err = bind(
        &graph,
        &schema,
        &Object::iri(IriS::new_unchecked("tag:nobody")),
        None,
        false,
    )
    .unwrap_err();
    assert!(matches!(err, ShExMapError::Validation { .. }), "{err}");
}

#[test]
fn static_variables_fill_optional_constraints() {
    let entry = &manifest("shexjs")[0]; // BP / simple: :someConstProp xsd:string? %Map:{ <http://abc.example/someConstant> %}
    let (_, bindings) = bindings_of(entry);
    let (root, shape) = entry.output_shape_map();
    let mut options = MaterializerOptions::default();
    options.static_vars.insert(
        "http://abc.example/someConstant".to_string(),
        parse_term("\"123-456\"").unwrap(),
    );
    let out: OxigraphInMemory =
        materialize(&entry.output_schema(), &bindings, Some(&root), shape.as_ref(), options).unwrap();
    let pred: oxrdf::NamedNode = IriS::new_unchecked("http://dam.example/med#someConstProp").into();
    assert!(out.triples_with_predicate(&pred).unwrap().next().is_some());
    let _ = json!({});
}
