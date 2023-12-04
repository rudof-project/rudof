use iri_s::IriS;
use shex_ast::Node;
use srdf::shacl_path::SHACLPath;
use srdf::SRDF;
use thiserror::Error;

/// A NodeSelector following [ShapeMap spec](https://shexspec.github.io/shape-map/#shapemap-structure) can be used to select RDF Nodes
///
#[derive(Debug)]
pub enum NodeSelector {
    Node(Node),
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
#[derive(Debug)]

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
