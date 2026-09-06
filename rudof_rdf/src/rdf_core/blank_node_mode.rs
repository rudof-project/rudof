use std::collections::HashMap;
use std::fmt::Display;
use std::str::FromStr;

use oxrdf::{BlankNode, NamedOrBlankNode, Term, Triple};

use crate::rdf_core::RDFError;

/// Controls how blank node identifiers are written out during RDF serialization.
///
/// Turtle parsers (and other RDF parsers) commonly assign long, random-looking
/// identifiers to anonymous blank nodes (e.g. those written as `[...]` or `()`
/// in Turtle). Those identifiers are not meaningful to a reader and make
/// serialized output noisy. [`BlankNodeMode::Simplify`] rewrites them to short
/// sequential identifiers (`_:b0`, `_:b1`, ...) assigned in the order they are
/// first encountered while serializing.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum BlankNodeMode {
    /// Rewrite blank node identifiers to short sequential ids (`_:b0`, `_:b1`, ...).
    #[default]
    Simplify,

    /// Keep the blank node identifiers exactly as they are stored in the graph.
    Preserve,
}

impl FromStr for BlankNodeMode {
    type Err = RDFError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "simplify" => Ok(BlankNodeMode::Simplify),
            "preserve" => Ok(BlankNodeMode::Preserve),
            _ => Err(RDFError::NotSupportedRDFFormatError {
                format: format!("Unknown blank node mode: {s}. Expected 'simplify' or 'preserve'"),
            }),
        }
    }
}

impl Display for BlankNodeMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BlankNodeMode::Simplify => write!(f, "simplify"),
            BlankNodeMode::Preserve => write!(f, "preserve"),
        }
    }
}

/// Applies `mode` to `triples`, relabeling blank nodes when `mode` is
/// [`BlankNodeMode::Simplify`] and returning `triples` unchanged otherwise.
///
/// Blank nodes are renamed to `b0`, `b1`, ... in the order they are first
/// encountered while scanning `triples` in order, so the same source blank
/// node always maps to the same short identifier within one call.
pub fn apply_blank_node_mode(triples: Vec<Triple>, mode: BlankNodeMode) -> Vec<Triple> {
    if mode == BlankNodeMode::Preserve {
        return triples;
    }

    let mut relabeled: HashMap<BlankNode, BlankNode> = HashMap::new();
    let mut relabel = |b: &BlankNode| -> BlankNode {
        let next_id = relabeled.len();
        relabeled
            .entry(b.clone())
            .or_insert_with(|| BlankNode::new_unchecked(format!("b{next_id}")))
            .clone()
    };

    triples
        .into_iter()
        .map(|t| {
            let subject = match t.subject {
                NamedOrBlankNode::BlankNode(b) => NamedOrBlankNode::BlankNode(relabel(&b)),
                other => other,
            };
            let object = match t.object {
                Term::BlankNode(b) => Term::BlankNode(relabel(&b)),
                other => other,
            };
            Triple::new(subject, t.predicate, object)
        })
        .collect()
}
