//! Property graph output: the generated graph in rudof's `.pg` format and the
//! type map (`.map`) that associates every node and edge with its PG-Schema type,
//! the PG counterpart of the ShapeMap side output.
//!
//! The graph is read off the triples of a model translated by
//! [`crate::adapters::pgschema`]: an entity is a node, its key triples are its
//! record, and each edge triple is an edge. A referenced IRI that is not an
//! entity (a dangling reference) becomes a node labelled `Dangling`, with no
//! record (`.pg` has no syntax for a node with neither).

use crate::adapters::pgschema::{PG_NS, local_name, parse_edge_iri};
use crate::ir::UnifiedConstraintModel;
use oxrdf::{NamedOrBlankNode, Term, Triple};
use std::collections::{BTreeMap, BTreeSet};

const RDF_TYPE: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#type";
const XSD: &str = "http://www.w3.org/2001/XMLSchema#";

/// The `.pg` identifier of a node IRI (identifiers are `\w+`).
pub fn node_id(iri: &str) -> String {
    let local = iri.strip_prefix(PG_NS).unwrap_or(iri);
    let local = local.strip_prefix("type/").unwrap_or(local);
    local
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() || c == '_' { c } else { '_' })
        .collect()
}

/// The `.pg` identifier of the edge `source -label-> target`.
pub fn edge_id(source: &str, label: &str, target: &str) -> String {
    format!("{}__{label}__{}", node_id(source), node_id(target))
}

/// The name of a node type or edge type from its IRI.
pub fn type_name(iri: &str) -> String {
    local_name(iri, "type").unwrap_or(iri).to_string()
}

/// `(graph, type map)` for the triples of the `entities` (node IRI, shape id).
pub fn property_graph(
    triples: &[Triple],
    model: &UnifiedConstraintModel,
    entities: &[(String, String)],
) -> (String, String) {
    let mut by_subject: BTreeMap<&str, Vec<&Triple>> = BTreeMap::new();
    for t in triples {
        if let NamedOrBlankNode::NamedNode(s) = &t.subject {
            by_subject.entry(s.as_str()).or_default().push(t);
        }
    }
    let known: BTreeSet<&str> = entities.iter().map(|(n, _)| n.as_str()).collect();
    let (mut graph, mut map, mut edges) = (String::new(), Vec::new(), String::new());
    let mut dangling: BTreeSet<String> = BTreeSet::new();
    for (node, shape_id) in entities {
        let mut labels: BTreeSet<String> = model
            .shapes
            .get(shape_id)
            .into_iter()
            .flat_map(|s| s.labels.iter().chain(s.target_class.iter()))
            .filter_map(|l| local_name(l, "label").map(String::from))
            .collect();
        let mut record: BTreeMap<String, Vec<String>> = BTreeMap::new();
        for t in by_subject.get(node.as_str()).into_iter().flatten() {
            let predicate = t.predicate.as_str();
            match (&t.object, parse_edge_iri(predicate)) {
                (Term::NamedNode(o), _) if predicate == RDF_TYPE => {
                    labels.extend(local_name(o.as_str(), "label").map(String::from));
                },
                (Term::NamedNode(target), Some((edge_type, label))) => {
                    let id = edge_id(node, label, target.as_str());
                    edges.push_str(&format!(
                        "({}) -({id} {{{label}}})-> ({})\n",
                        node_id(node),
                        node_id(target.as_str())
                    ));
                    map.push(format!("{id}: {edge_type}"));
                    if !known.contains(target.as_str()) {
                        dangling.insert(node_id(target.as_str()));
                    }
                },
                (Term::Literal(lit), _) => {
                    record.entry(key_name(predicate)).or_default().push(value(lit));
                },
                _ => {},
            }
        }
        graph.push_str(&format!("({}", node_id(node)));
        if !labels.is_empty() {
            graph.push_str(&format!(" {{{}}}", labels.into_iter().collect::<Vec<_>>().join(", ")));
        }
        if !record.is_empty() {
            let entries: Vec<String> = record
                .into_iter()
                .map(|(key, values)| match values.as_slice() {
                    [one] => format!("{key}: {one}"),
                    many => format!("{key}: [{}]", many.join(", ")),
                })
                .collect();
            graph.push_str(&format!(" [ {} ]", entries.join(", ")));
        }
        graph.push_str(")\n");
        map.push(format!("{}: {}", node_id(node), type_name(shape_id)));
    }
    for id in dangling {
        graph.push_str(&format!("({id} {{Dangling}})\n"));
    }
    graph.push_str(&edges);
    (graph, map.join(",\n") + "\n")
}

/// A record key: the key IRI's name, or the IRI's last segment for a predicate
/// the schema does not declare.
fn key_name(predicate: &str) -> String {
    local_name(predicate, "key")
        .map(String::from)
        .unwrap_or_else(|| node_id(predicate.rsplit(['/', '#']).next().unwrap_or(predicate)))
}

/// A literal in `.pg` syntax: integers bare, dates `DATE "…"`, booleans
/// `TRUE`/`FALSE`, everything else a quoted string.
fn value(lit: &oxrdf::Literal) -> String {
    let lexical = lit.value();
    let quoted = || format!("\"{}\"", lexical.replace('"', "\\\""));
    match lit.datatype().as_str().strip_prefix(XSD) {
        Some("integer" | "int" | "long") if lexical.parse::<i32>().is_ok() => lexical.to_string(),
        Some("date") => format!("DATE {}", quoted()),
        Some("boolean") if lexical == "true" || lexical == "1" => "TRUE".to_string(),
        Some("boolean") if lexical == "false" || lexical == "0" => "FALSE".to_string(),
        _ => quoted(),
    }
}
