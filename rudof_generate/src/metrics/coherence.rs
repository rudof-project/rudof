//! Coherence (Duan et al., "Apples and oranges", SIGMOD 2011), the structuredness of a
//! graph: how close every instance of every type comes to having all the properties
//! of its type.

use oxrdf::Triple;
use std::collections::{HashMap, HashSet};

const RDF_TYPE: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#type";

/// Coherence of `triples`, with each node of `types` typed by its shape.
///
/// For a type `t` with instances `I(t)` and properties `P(t)` (the predicates its
/// instances use, `rdf:type` excluded), the coverage is
/// `CV(t) = Σ_{s ∈ I(t)} |P(s)| / (|P(t)| · |I(t)|)`, and the coherence is the average
/// of the coverages weighted by `|P(t)| + |I(t)|`. Types whose instances have no
/// property are left out. A graph with no such type has coherence 1.
pub fn coherence(triples: &[Triple], types: &[(String, String)]) -> f64 {
    let type_of: HashMap<&str, &str> = types.iter().map(|(n, t)| (n.as_str(), t.as_str())).collect();
    let mut properties: HashMap<&str, HashSet<&str>> = HashMap::new();
    for triple in triples {
        let predicate = triple.predicate.as_str();
        if predicate == RDF_TYPE {
            continue;
        }
        let subject = match &triple.subject {
            oxrdf::NamedOrBlankNode::NamedNode(n) => n.as_str(),
            _ => continue,
        };
        if type_of.contains_key(subject) {
            properties.entry(subject).or_default().insert(predicate);
        }
    }

    let mut instances: HashMap<&str, Vec<&str>> = HashMap::new();
    for (node, t) in &type_of {
        instances.entry(t).or_default().push(node);
    }

    let mut weighted = 0.0;
    let mut total_weight = 0.0;
    for nodes in instances.values() {
        let type_properties: HashSet<&str> = nodes
            .iter()
            .filter_map(|n| properties.get(n))
            .flat_map(|ps| ps.iter().copied())
            .collect();
        if type_properties.is_empty() {
            continue;
        }
        let used: usize = nodes.iter().map(|n| properties.get(n).map_or(0, HashSet::len)).sum();
        let coverage = used as f64 / (type_properties.len() * nodes.len()) as f64;
        let weight = (type_properties.len() + nodes.len()) as f64;
        weighted += weight * coverage;
        total_weight += weight;
    }
    if total_weight == 0.0 {
        1.0
    } else {
        weighted / total_weight
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use oxrdf::{Literal, NamedNode};

    fn triple(s: &str, p: &str) -> Triple {
        Triple::new(
            NamedNode::new_unchecked(s),
            NamedNode::new_unchecked(p),
            Literal::new_simple_literal("v"),
        )
    }

    #[test]
    fn full_coverage_is_one() {
        let triples = vec![triple("http://e/a", "http://e/p"), triple("http://e/b", "http://e/p")];
        let types = vec![
            ("http://e/a".to_string(), "T".to_string()),
            ("http://e/b".to_string(), "T".to_string()),
        ];
        assert_eq!(coherence(&triples, &types), 1.0);
    }

    #[test]
    fn half_coverage_is_weighted_by_properties_and_instances() {
        // P(T) = {p, q}, I(T) = {a, b}; a has both, b has p only: CV = 3/4.
        let triples = vec![
            triple("http://e/a", "http://e/p"),
            triple("http://e/a", "http://e/q"),
            triple("http://e/b", "http://e/p"),
        ];
        let types = vec![
            ("http://e/a".to_string(), "T".to_string()),
            ("http://e/b".to_string(), "T".to_string()),
        ];
        assert_eq!(coherence(&triples, &types), 0.75);
    }

    #[test]
    fn rdf_type_is_not_a_property() {
        let triples = vec![triple("http://e/a", "http://e/p"), triple("http://e/b", RDF_TYPE)];
        let types = vec![
            ("http://e/a".to_string(), "T".to_string()),
            ("http://e/b".to_string(), "T".to_string()),
        ];
        assert_eq!(coherence(&triples, &types), 0.5);
    }
}
