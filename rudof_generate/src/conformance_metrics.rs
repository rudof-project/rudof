use crate::unified_constraints::{NodeKind, UnifiedConstraint, UnifiedConstraintModel, Value};
use crate::{DataGeneratorError, Result};
use oxrdf::{NamedOrBlankNode, Term};
use regex::Regex;
use rudof_rdf::rdf_core::NeighsRDF;
use rudof_rdf::rdf_impl::InMemoryGraph;
use serde::Serialize;
use std::collections::HashMap;

const RDF_TYPE: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#type";

#[derive(Debug, Clone, Serialize, Default)]
pub struct TranslationMetrics {
    pub original_schema_constraints: usize,
    pub represented_constraints_in_unified: usize,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct ConformanceMetrics {
    pub total_generated_triples: usize,
    pub valid_triples: usize,
    pub triple_validity_percentage: f64,
    pub original_schema_constraints: usize,
    pub represented_constraints_in_unified: usize,
    pub shape_translation_loss_percentage: f64,
}

impl ConformanceMetrics {
    pub fn from_graph_and_model(
        graph: &InMemoryGraph,
        model: &UnifiedConstraintModel,
        translation_metrics: TranslationMetrics,
    ) -> Result<Self> {
        let triples = graph
            .triples()
            .map_err(|e| DataGeneratorError::GraphGeneration(format!("Failed to iterate graph triples: {e}")))?
            .collect::<Vec<_>>();

        let total_generated_triples = triples.len();

        let mut subject_type: HashMap<NamedOrBlankNode, String> = HashMap::new();
        let mut outgoing_counts: HashMap<(NamedOrBlankNode, String), usize> = HashMap::new();

        for triple in &triples {
            if triple.predicate.as_str() == RDF_TYPE {
                if let Term::NamedNode(shape_node) = &triple.object {
                    subject_type.insert(triple.subject.clone(), shape_node.as_str().to_string());
                }
            } else {
                let key = (triple.subject.clone(), triple.predicate.as_str().to_string());
                *outgoing_counts.entry(key).or_insert(0) += 1;
            }
        }

        let mut valid_triples = 0usize;

        for triple in &triples {
            let is_valid = if triple.predicate.as_str() == RDF_TYPE {
                match &triple.object {
                    Term::NamedNode(shape_node) => model.shapes.contains_key(shape_node.as_str()),
                    _ => false,
                }
            } else {
                validate_non_type_triple(triple, model, &subject_type, &outgoing_counts)
            };

            if is_valid {
                valid_triples += 1;
            }
        }

        let triple_validity_percentage = if total_generated_triples == 0 {
            0.0
        } else {
            (valid_triples as f64 / total_generated_triples as f64) * 100.0
        };

        let original_schema_constraints = translation_metrics.original_schema_constraints;
        let represented_constraints_in_unified = translation_metrics
            .represented_constraints_in_unified
            .min(original_schema_constraints);

        let shape_translation_loss_percentage = if original_schema_constraints == 0 {
            0.0
        } else {
            100.0
                * (1.0
                    - (represented_constraints_in_unified as f64 / original_schema_constraints as f64))
        };

        Ok(Self {
            total_generated_triples,
            valid_triples,
            triple_validity_percentage,
            original_schema_constraints,
            represented_constraints_in_unified,
            shape_translation_loss_percentage,
        })
    }
}

fn validate_non_type_triple(
    triple: &oxrdf::Triple,
    model: &UnifiedConstraintModel,
    subject_type: &HashMap<NamedOrBlankNode, String>,
    outgoing_counts: &HashMap<(NamedOrBlankNode, String), usize>,
) -> bool {
    let Some(shape_id) = subject_type.get(&triple.subject) else {
        return false;
    };

    let Some(shape) = model.shapes.get(shape_id) else {
        return false;
    };

    let Some(property) = shape
        .properties
        .iter()
        .find(|p| p.property_iri == triple.predicate.as_str())
    else {
        return false;
    };

    let count_key = (triple.subject.clone(), triple.predicate.as_str().to_string());
    let count = outgoing_counts.get(&count_key).copied().unwrap_or(0);

    if let Some(max) = property.max_cardinality
        && count > max as usize
    {
        return false;
    }

    property
        .constraints
        .iter()
        .all(|c| evaluate_constraint(c, &triple.object, subject_type))
}

fn evaluate_constraint(
    constraint: &UnifiedConstraint,
    object: &Term,
    subject_type: &HashMap<NamedOrBlankNode, String>,
) -> bool {
    match constraint {
        UnifiedConstraint::Datatype(expected) => match object {
            Term::Literal(lit) => lit.datatype().as_str() == expected,
            _ => false,
        },
        UnifiedConstraint::ShapeReference(target_shape) => match object {
            Term::NamedNode(node) => subject_type
                .get(&NamedOrBlankNode::NamedNode(node.clone()))
                .map(|s| s == target_shape)
                .unwrap_or(false),
            Term::BlankNode(node) => subject_type
                .get(&NamedOrBlankNode::BlankNode(node.clone()))
                .map(|s| s == target_shape)
                .unwrap_or(false),
            _ => false,
        },
        UnifiedConstraint::NodeKind(node_kind) => match node_kind {
            NodeKind::Iri => matches!(object, Term::NamedNode(_)),
            NodeKind::BlankNode => matches!(object, Term::BlankNode(_)),
            NodeKind::Literal => matches!(object, Term::Literal(_)),
            NodeKind::BlankNodeOrIri => matches!(object, Term::NamedNode(_) | Term::BlankNode(_)),
            NodeKind::BlankNodeOrLiteral => matches!(object, Term::BlankNode(_) | Term::Literal(_)),
            NodeKind::IriOrLiteral => matches!(object, Term::NamedNode(_) | Term::Literal(_)),
        },
        UnifiedConstraint::Pattern(pattern) => match object {
            Term::Literal(lit) => Regex::new(pattern)
                .map(|re| re.is_match(lit.value()))
                .unwrap_or(false),
            _ => false,
        },
        UnifiedConstraint::MinInclusive(bound) => compare_numeric(object, bound, |v, b| v >= b),
        UnifiedConstraint::MaxInclusive(bound) => compare_numeric(object, bound, |v, b| v <= b),
        UnifiedConstraint::MinExclusive(bound) => compare_numeric(object, bound, |v, b| v > b),
        UnifiedConstraint::MaxExclusive(bound) => compare_numeric(object, bound, |v, b| v < b),
        UnifiedConstraint::MinLength(min) => match object {
            Term::Literal(lit) => lit.value().chars().count() >= *min as usize,
            _ => false,
        },
        UnifiedConstraint::MaxLength(max) => match object {
            Term::Literal(lit) => lit.value().chars().count() <= *max as usize,
            _ => false,
        },
        UnifiedConstraint::In(allowed) => allowed.iter().any(|value| value_matches_term(value, object)),
        UnifiedConstraint::HasValue(expected) => value_matches_term(expected, object),
    }
}

fn compare_numeric<F>(object: &Term, bound: &Value, cmp: F) -> bool
where
    F: Fn(f64, f64) -> bool,
{
    let Some(value_num) = term_as_f64(object) else {
        return false;
    };

    let Some(bound_num) = value_as_f64(bound) else {
        return false;
    };

    cmp(value_num, bound_num)
}

fn term_as_f64(term: &Term) -> Option<f64> {
    match term {
        Term::Literal(lit) => lit.value().parse::<f64>().ok(),
        _ => None,
    }
}

fn value_as_f64(value: &Value) -> Option<f64> {
    match value {
        Value::Literal(lexical, _) => lexical.parse::<f64>().ok(),
        _ => None,
    }
}

fn value_matches_term(value: &Value, term: &Term) -> bool {
    match (value, term) {
        (Value::Iri(expected), Term::NamedNode(actual)) => actual.as_str() == expected,
        (Value::BlankNode(expected), Term::BlankNode(actual)) => actual.as_str() == expected,
        (Value::Literal(expected_lex, expected_dt), Term::Literal(actual)) => {
            if actual.value() != expected_lex {
                return false;
            }

            if let Some(dt) = expected_dt {
                actual.datatype().as_str() == dt
            } else {
                true
            }
        },
        _ => false,
    }
}
