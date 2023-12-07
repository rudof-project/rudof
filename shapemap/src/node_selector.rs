use iri_s::IriS;
use prefixmap::IriRef;
use shex_ast::{object_value::ObjectValue, Node};
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

    pub fn prefixed(alias: &str, local: &str) -> NodeSelector {
        NodeSelector::Node(ObjectValue::prefixed(alias, local))
    }

    pub fn iter_node<S>(&self, rdf: &S) -> impl Iterator<Item=&ObjectValue> 
    where S: SRDF {
        match self {
            NodeSelector::Node(value) => std::iter::once(value),
            _ => todo!()
        }
    }
}

#[derive(Debug, Error)]
pub enum NodeSelectorError {}

impl NodeSelect for NodeSelector {
    fn select<S>(&self, rdf: S) -> Result<Vec<S::Term>, NodeSelectorError>
    where
        S: SRDF,
    {
        match self {
            NodeSelector::Node(node) => {
                todo!()
            }
            NodeSelector::TriplePattern {
                subject,
                pred,
                object,
            } => match (subject, pred, object) {
                (Pattern::Focus, pred, Pattern::Wildcard) => todo!(),
                (Pattern::Focus, pred, Pattern::Node(n)) => todo!(),
                (Pattern::Wildcard, pred, Pattern::Focus) => todo!(),
                (Pattern::Node(n), pred, Pattern::Focus) => todo!(),
                (_, _, _) => todo!(),
            },
            NodeSelector::TriplePatternPath {
                subject,
                pred,
                object,
            } => {
                todo!()
            }
            NodeSelector::Sparql { query } => todo!(),
            NodeSelector::Generic { iri, param } => todo!(),
        }
    }
}
#[derive(Debug, PartialEq)]

pub enum Pattern {
    Node(Node),
    Wildcard,
    Focus,
}

trait NodeSelect {
    fn select<S>(&self, rdf: S) -> Result<Vec<S::Term>, NodeSelectorError>
    where
        S: SRDF;
}
