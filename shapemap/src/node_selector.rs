use iri_s::IriS;
use prefixmap::IriRef;
use shex_ast::{object_value::ObjectValue, Node};
use srdf::literal::Literal;
use srdf::shacl_path::SHACLPath;
use srdf::SRDF;
use thiserror::Error;

/// A NodeSelector following [ShapeMap spec](https://shexspec.github.io/shape-map/#shapemap-structure) can be used to select RDF Nodes
///
#[derive(Debug, PartialEq)]
pub enum NodeSelector {
    Node(ObjectValue),
    TriplePattern {
        subject: Pattern,
        pred: IriS,
        object: Pattern,
    },
    TriplePatternPath {
        subject: Pattern,
        pred: SHACLPath,
        object: Pattern,
    },
    Sparql {
        query: String,
    },
    Generic {
        iri: IriS,
        param: String,
    },
}

impl NodeSelector {
    pub fn iri_unchecked(str: &str) -> NodeSelector {
        NodeSelector::Node(ObjectValue::iri(IriS::new_unchecked(str)))
    }

    pub fn iri_ref(iri: IriRef) -> NodeSelector {
        NodeSelector::Node(ObjectValue::iri_ref(iri))
    }

    pub fn literal(lit: Literal) -> NodeSelector {
        NodeSelector::Node(ObjectValue::literal(lit))
    }

    pub fn prefixed(alias: &str, local: &str) -> NodeSelector {
        NodeSelector::Node(ObjectValue::prefixed(alias, local))
    }

    pub fn iter_node<S>(&self, _rdf: &S) -> impl Iterator<Item = &ObjectValue>
    where
        S: SRDF,
    {
        match self {
            NodeSelector::Node(value) => std::iter::once(value),
            _ => todo!(),
        }
    }
}

#[derive(Debug, Error)]
pub enum NodeSelectorError {}

impl NodeSelect for NodeSelector {
    fn select<S>(&self, _rdf: S) -> Result<Vec<S::Term>, NodeSelectorError>
    where
        S: SRDF,
    {
        match self {
            NodeSelector::Node(_node) => {
                todo!()
            }
            NodeSelector::TriplePattern {
                subject,
                pred,
                object,
            } => match (subject, pred, object) {
                (Pattern::Focus, _pred, Pattern::Wildcard) => todo!(),
                (Pattern::Focus, _pred, Pattern::Node(_node)) => todo!(),
                (Pattern::Wildcard, _pred, Pattern::Focus) => todo!(),
                (Pattern::Node(_n), _pred, Pattern::Focus) => todo!(),
                (_, _, _) => todo!(),
            },
            NodeSelector::TriplePatternPath { .. } => todo!(),
            NodeSelector::Sparql { .. } => todo!(),
            NodeSelector::Generic { .. } => todo!(),
        }
    }
}
#[derive(Debug, PartialEq)]

pub enum Pattern {
    Node(Node),
    Wildcard,
    Focus,
}

#[allow(dead_code)]
trait NodeSelect {
    fn select<S>(&self, rdf: S) -> Result<Vec<S::Term>, NodeSelectorError>
    where
        S: SRDF;
}
