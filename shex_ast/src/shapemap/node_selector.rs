use std::fmt::Display;

use crate::shapemap::ShapemapError;
use crate::{Node, object_value::ObjectValue};
use iri_s::IriS;
use prefixmap::IriRef;
use serde::Serialize;
use srdf::QueryRDF;
use srdf::SLiteral;
use srdf::shacl_path::SHACLPath;
use srdf::{NeighsRDF, VariableSolutionIndex};
use thiserror::Error;
use tracing::{info, trace};

/// A NodeSelector following [ShapeMap spec](https://shexspec.github.io/shape-map/#shapemap-structure) can be used to select RDF Nodes
///
#[derive(Debug, PartialEq, Clone, Serialize)]
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

    pub fn literal(lit: SLiteral) -> NodeSelector {
        NodeSelector::Node(ObjectValue::literal(lit))
    }

    pub fn prefixed(alias: &str, local: &str) -> NodeSelector {
        NodeSelector::Node(ObjectValue::prefixed(alias, local))
    }

    pub fn nodes<R>(&self, rdf: &R) -> Result<Vec<R::Term>, ShapemapError>
    where
        R: QueryRDF,
    {
        match self {
            NodeSelector::Node(value) => {
                let term = match value {
                    ObjectValue::IriRef(iri_ref) => {
                        let iri = iri_ref
                            .get_iri_prefixmap(&rdf.prefixmap().unwrap_or_default())
                            .map_err(|e| ShapemapError::ResolvingIriRef {
                                prefixmap: rdf.prefixmap().unwrap_or_default().to_string(),
                                iri_ref: iri_ref.to_string(),
                                error: e.to_string(),
                            })?;
                        let iri_term: R::Term = iri.into();
                        Ok(iri_term.clone())
                    }
                    ObjectValue::Literal(sliteral) => {
                        let lit: R::Literal = sliteral.clone().into();
                        Ok(lit.into())
                    }
                }?;
                Ok(vec![term])
            }
            NodeSelector::Sparql { query } => {
                trace!("Resolving SPARQL NodeSelector query: {}", query);
                let nodes = resolve_query(rdf, query)?;
                info!("SPARQL NodeSelector nodes resolve query: {:?}", nodes);
                Ok(nodes)
            }
            _ => todo!(),
        }
    }

    pub fn sparql(query: &str) -> NodeSelector {
        NodeSelector::Sparql {
            query: query.to_string(),
        }
    }
}

#[derive(Debug, Error)]
pub enum NodeSelectorError {}

impl NodeSelect for NodeSelector {
    fn select<S>(&self, _rdf: S) -> Result<Vec<S::Term>, NodeSelectorError>
    where
        S: NeighsRDF,
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
#[derive(Debug, PartialEq, Clone, Serialize)]

pub enum Pattern {
    Node(Node),
    Wildcard,
    Focus,
}

#[allow(dead_code)]
trait NodeSelect {
    fn select<S>(&self, rdf: S) -> Result<Vec<S::Term>, NodeSelectorError>
    where
        S: NeighsRDF;
}

fn resolve_query<R>(rdf: &R, query: &str) -> Result<Vec<R::Term>, ShapemapError>
where
    R: QueryRDF,
{
    let mut results = Vec::new();
    info!("Resolving SPARQL NodeSelector query for rdf");
    let query_solutions =
        rdf.query_select(query)
            .map_err(|e| ShapemapError::NodeSelectorQueryError {
                query: query.to_string(),
                error: e.to_string(),
            })?;
    info!(
        "SPARQL NodeSelector query solutions: {}",
        query_solutions.count()
    );
    for solution in query_solutions.iter() {
        let variables = solution.variables();
        info!("SPARQL NodeSelector variables: {:?}", variables);
        if let Some(variable) = variables.first() {
            if let Some(value) = solution.find_solution(variable) {
                results.push(value.clone())
            }
        }
    }
    Ok(results)
}

impl Display for NodeSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NodeSelector::Node(object_value) => {
                write!(f, "{object_value}")?;
                Ok(())
            }
            NodeSelector::TriplePattern {
                subject: _,
                pred: _,
                object: _,
            } => todo!(),
            NodeSelector::TriplePatternPath {
                subject: _,
                pred: _,
                object: _,
            } => todo!(),
            NodeSelector::Sparql { query } => {
                write!(f, "SPARQL \"\"\"{query}\"\"\"")?;
                Ok(())
            }
            NodeSelector::Generic { iri: _, param: _ } => todo!(),
        }
    }
}
