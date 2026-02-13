use crate::object_value::ObjectValue;
use crate::shapemap::SHACLPathRef;
use crate::shapemap::ShapemapError;
use iri_s::IriS;
use prefixmap::IriRef;
use prefixmap::PrefixMap;
use prefixmap::PrefixMapError;
use serde::Serialize;
use rdf::rdf_core::{
    NeighsRDF, query::QueryRDF, term::literal::ConcreteLiteral
};
use std::fmt::Display;
use thiserror::Error;
use tracing::trace;

/// A NodeSelector following [ShapeMap spec](https://shexspec.github.io/shape-map/#shapemap-structure) can be used to select RDF Nodes
///
#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum NodeSelector {
    Node(ObjectValue),
    TriplePattern {
        subject: Pattern,
        path: SHACLPathRef,
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
    pub fn triple_pattern(subject: Pattern, path: SHACLPathRef, object: Pattern) -> NodeSelector {
        NodeSelector::TriplePattern {
            subject,
            path,
            object,
        }
    }
    pub fn iri_unchecked(str: &str) -> NodeSelector {
        NodeSelector::Node(ObjectValue::iri(IriS::new_unchecked(str)))
    }

    pub fn iri_ref(iri: IriRef) -> NodeSelector {
        NodeSelector::Node(ObjectValue::iri_ref(iri))
    }

    pub fn literal(lit: ConcreteLiteral) -> NodeSelector {
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
                let nodes = resolve_query(rdf, query)?;
                Ok(nodes)
            }
            NodeSelector::TriplePattern {
                subject,
                path,
                object,
            } => {
                let subj_str =
                    mk_vble(subject, &rdf.prefixmap().unwrap_or_default()).map_err(|e| {
                        ShapemapError::PrefixMapError {
                            node: subject.to_string(),
                            error: e.to_string(),
                        }
                    })?;
                let path_str = mk_path_str(path, &rdf.prefixmap().unwrap_or_default())?;
                let obj_str =
                    mk_vble(object, &rdf.prefixmap().unwrap_or_default()).map_err(|e| {
                        ShapemapError::PrefixMapError {
                            node: object.to_string(),
                            error: e.to_string(),
                        }
                    })?;
                let query = format!("SELECT ?focus WHERE {{ {subj_str} {path_str} {obj_str} }}");
                let nodes = resolve_query(rdf, &query)?;
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

fn mk_path_str(path: &SHACLPathRef, prefixmap: &PrefixMap) -> Result<String, ShapemapError> {
    match path {
        SHACLPathRef::Predicate { pred } => {
            let pred_resolved =
                prefixmap
                    .resolve_iriref(pred)
                    .map_err(|e| ShapemapError::PrefixMapError {
                        node: pred.to_string(),
                        error: e.to_string(),
                    })?;
            Ok(format!("<{pred_resolved}>"))
        }
        _ => todo!(),
    }
}

fn mk_vble(p: &Pattern, prefixmap: &PrefixMap) -> Result<String, PrefixMapError> {
    match p {
        Pattern::Node(object_value) => match object_value {
            ObjectValue::IriRef(iri_ref) => {
                let iri = prefixmap.resolve_iriref(iri_ref)?;
                Ok(format!("<{iri}>"))
            }
            ObjectValue::Literal(sliteral) => Ok(sliteral.to_string()),
        },
        Pattern::Wildcard => Ok("?any".to_string()),
        Pattern::Focus => Ok("?focus".to_string()),
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
                path,
                object,
            } => match (subject, path, object) {
                (Pattern::Focus, _pred, Pattern::Wildcard) => todo!(),
                (Pattern::Focus, _pred, Pattern::Node(_node)) => todo!(),
                (Pattern::Wildcard, _pred, Pattern::Focus) => todo!(),
                (Pattern::Node(_n), _pred, Pattern::Focus) => todo!(),
                (_, _, _) => todo!(),
            },
            NodeSelector::Sparql { .. } => todo!(),
            NodeSelector::Generic { .. } => todo!(),
        }
    }
}
#[derive(Debug, PartialEq, Clone, Serialize)]

pub enum Pattern {
    Node(ObjectValue),
    Wildcard,
    Focus,
}

impl Pattern {
    pub fn focus() -> Self {
        Pattern::Focus
    }

    pub fn wildcard() -> Self {
        Pattern::Wildcard
    }

    pub fn node(obj: ObjectValue) -> Self {
        Pattern::Node(obj)
    }

    pub fn prefixed(prefix: &str, local: &str) -> Self {
        Pattern::Node(ObjectValue::iri_ref(IriRef::prefixed(prefix, local)))
    }
}

impl Display for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Pattern::Node(object_value) => write!(f, "{object_value}"),
            Pattern::Wildcard => write!(f, "_"),
            Pattern::Focus => write!(f, "FOCUS"),
        }
    }
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
    trace!("Resolving SPARQL NodeSelector query for rdf\nQuery:\n{query}");
    let query_solutions =
        rdf.query_select(query)
            .map_err(|e| ShapemapError::NodeSelectorQueryError {
                query: query.to_string(),
                error: e.to_string(),
            })?;
    trace!(
        "SPARQL NodeSelector query solutions: {}",
        query_solutions.count()
    );
    for solution in query_solutions.iter() {
        let variables = solution.variables();
        trace!("SPARQL NodeSelector variables: {:?}", variables);
        if let Some(variable) = variables.first()
            && let Some(value) = solution.find_solution(variable)
        {
            results.push(value.clone())
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
                path: _,
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
