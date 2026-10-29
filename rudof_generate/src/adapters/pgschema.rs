//! PG-Schema → IR.
//!
//! A node type becomes a shape whose instances carry the type's labels and whose
//! record keys are literal properties; an edge type `(:S)-[E: l]->(:T)` becomes a
//! reference from `S` to `T`. Keys, labels, types and edge types are named by IRIs
//! under [`PG_NS`], so the same IR also yields an RDF view of the property graph,
//! and the PG sink ([`crate::sinks::pg`]) reads the names back from them.
//!
//! Edge endpoints come in two forms in rudof's PG-Schema syntax: `(:@T)` names a
//! node type, which the endpoint must conform to (a shape reference), and `(:L)`
//! a label spec. rudof's validator reads `(:L)` as label `L` with an empty closed
//! record, which no generated node with keys satisfies, so only `(:@T)` endpoints
//! are supported; an edge type with a label endpoint is left out.
//!
//! Supported: labels, `&` of types and references to types, closed and open
//! records, `OPTIONAL` keys, value cardinalities, and `CHECK` comparisons and
//! regular expressions. Unions (`|`, `||`), `ANY`, optional label specs and edge
//! types with required edge properties are left out and counted as lost in the
//! translation metrics; an edge type that is left out is simply not generated,
//! which keeps the data valid because PG-Schema states no edge counts.

use crate::ir::{UnifiedConstraint, UnifiedConstraintModel, UnifiedPropertyConstraint, UnifiedShape, Value};
use crate::metrics::conformance::TranslationMetrics;
use crate::{DataGeneratorError, Result};
use pgschema::boolean_expr::BooleanExpr;
use pgschema::card::{Card, Max};
use pgschema::label_property_spec::LabelPropertySpec;
use pgschema::parser::pgs_builder::PgsBuilder;
use pgschema::pgs::PropertyGraphSchema;
use pgschema::property_value_spec::{PropertyValue, PropertyValueSpec};
use pgschema::value::Value as PgValue;
use pgschema::value_type::ValueType;
use std::path::Path;

/// Namespace of the IRIs that name PG-Schema keys, labels, types and edge types.
pub const PG_NS: &str = "http://rudof.example/pg/";
const XSD: &str = "http://www.w3.org/2001/XMLSchema#";

pub fn type_iri(type_name: &str) -> String {
    format!("{PG_NS}type/{type_name}")
}

pub fn label_iri(label: &str) -> String {
    format!("{PG_NS}label/{label}")
}

pub fn key_iri(key: &str) -> String {
    format!("{PG_NS}key/{key}")
}

/// The IRI of edges of `edge_type` with `label`; `conforming_source` when the
/// edge type requires its source to conform to a node type (`(:@S)`), not only
/// to carry a label.
pub fn edge_iri(edge_type: &str, label: &str, conforming_source: bool) -> String {
    let suffix = if conforming_source { "/conforming-source" } else { "" };
    format!("{PG_NS}edge/{edge_type}/{label}{suffix}")
}

/// `(edge type, label)` of an IRI made by [`edge_iri`].
pub fn parse_edge_iri(iri: &str) -> Option<(&str, &str)> {
    let (edge_type, rest) = iri.strip_prefix(PG_NS)?.strip_prefix("edge/")?.split_once('/')?;
    Some((edge_type, rest.split('/').next()?))
}

/// Whether the edges with this IRI require their source to conform.
pub fn requires_conforming_source(iri: &str) -> bool {
    parse_edge_iri(iri).is_some() && iri.ends_with("/conforming-source")
}

/// What an edge endpoint requires: conformance to a node type, or a label.
enum Endpoint<'a> {
    Type(&'a str),
    Label(&'a str),
}

/// The name in an IRI made by [`type_iri`], [`label_iri`] or [`key_iri`].
pub fn local_name<'a>(iri: &'a str, kind: &str) -> Option<&'a str> {
    iri.strip_prefix(PG_NS)?.strip_prefix(kind)?.strip_prefix('/')
}

/// Whether a model was translated from a PG-Schema.
pub fn is_property_graph(model: &UnifiedConstraintModel) -> bool {
    model.shapes.keys().any(|id| id.starts_with(PG_NS))
}

pub struct PgSchemaToUnified;

impl Default for PgSchemaToUnified {
    fn default() -> Self {
        Self
    }
}

/// The parts of a node type once its spec is flattened.
#[derive(Default)]
struct NodeParts {
    labels: Vec<String>,
    keys: Vec<(String, ValueType, bool)>,
    closed: bool,
}

impl PgSchemaToUnified {
    pub async fn convert_file<P: AsRef<Path>>(&self, path: P) -> Result<(UnifiedConstraintModel, TranslationMetrics)> {
        let text = std::fs::read_to_string(path.as_ref())
            .map_err(|e| DataGeneratorError::Config(format!("Failed to read PG-Schema file: {e}")))?;
        self.convert_schema(&text)
    }

    pub fn convert_schema(&self, text: &str) -> Result<(UnifiedConstraintModel, TranslationMetrics)> {
        let schema = PgsBuilder::new()
            .parse_pgs(text)
            .map_err(|e| DataGeneratorError::Config(format!("Failed to parse PG-Schema: {e}")))?;
        Ok(self.convert_pgs(&schema))
    }

    pub fn convert_pgs(&self, schema: &PropertyGraphSchema) -> (UnifiedConstraintModel, TranslationMetrics) {
        let mut metrics = TranslationMetrics::default();
        let mut shapes: Vec<UnifiedShape> = Vec::new();
        for (type_name, spec) in schema.node_types() {
            let mut parts = NodeParts::default();
            let supported = flatten(spec, schema, &mut parts);
            metrics.original_schema_constraints += 1 + parts.keys.len();
            if !supported {
                tracing::warn!("PG-Schema node type {type_name} uses constructs the generator does not support");
                continue;
            }
            metrics.represented_constraints_in_unified += 1;
            let mut properties = Vec::new();
            for (key, value_type, required) in &parts.keys {
                match key_property(key, value_type, *required) {
                    Some(property) => {
                        metrics.represented_constraints_in_unified += 1;
                        properties.push(property);
                    },
                    None => tracing::warn!("PG-Schema key {type_name}.{key} has an unsupported value type"),
                }
            }
            parts.labels.dedup();
            shapes.push(UnifiedShape {
                id: type_iri(type_name),
                target_class: parts.labels.first().map(|l| label_iri(l)),
                properties,
                closed: parts.closed,
                ignored_properties: Vec::new(),
                labels: parts.labels.iter().map(|l| label_iri(l)).collect(),
            });
        }

        for (edge_type, edge) in schema.edge_types() {
            metrics.original_schema_constraints += 1;
            let (Some(source), Some(target)) = (endpoint(&edge.source), endpoint(&edge.target)) else {
                tracing::warn!("PG-Schema edge type {edge_type}: endpoints must name a node type or a label");
                continue;
            };
            let mut parts = NodeParts::default();
            let supported = flatten(edge.edge.label_property_spec(), schema, &mut parts);
            let required_property = parts.keys.iter().any(|(_, _, required)| *required);
            let ([label], true, false) = (parts.labels.as_slice(), supported, required_property) else {
                tracing::warn!(
                    "PG-Schema edge type {edge_type}: one label and no required edge property are supported"
                );
                continue;
            };
            let target = match target {
                Endpoint::Type(t) => UnifiedConstraint::ShapeReference(type_iri(t)),
                Endpoint::Label(_) => {
                    tracing::warn!("PG-Schema edge type {edge_type}: only node type endpoints (:@T) are supported");
                    continue;
                },
            };
            // The edge goes out of every node type that satisfies the source.
            let sources: Vec<&mut UnifiedShape> = match source {
                Endpoint::Type(t) => shapes.iter_mut().filter(|s| s.id == type_iri(t)).collect(),
                Endpoint::Label(l) => shapes.iter_mut().filter(|s| s.labels.contains(&label_iri(l))).collect(),
            };
            if sources.is_empty() {
                continue;
            }
            metrics.represented_constraints_in_unified += 1;
            for shape in sources {
                shape.properties.push(UnifiedPropertyConstraint {
                    property_iri: edge_iri(edge_type, label, matches!(source, Endpoint::Type(_))),
                    constraints: vec![target.clone()],
                    min_cardinality: None,
                    max_cardinality: None,
                    counts_matching_values: false,
                    all_values_match: true,
                });
            }
        }

        let mut model = UnifiedConstraintModel::new();
        for shape in shapes {
            model.add_shape(shape);
        }
        (model, metrics)
    }
}

/// The node type (`(:@T)`) or label (`(:L)`) an endpoint spec names.
fn endpoint(spec: &LabelPropertySpec) -> Option<Endpoint<'_>> {
    match spec {
        LabelPropertySpec::Ref(name) => Some(Endpoint::Type(name.as_str())),
        LabelPropertySpec::Label(label) => Some(Endpoint::Label(label.as_str())),
        _ => None,
    }
}

/// Collect the labels and keys of `spec`; false when part of it is unsupported.
fn flatten(spec: &LabelPropertySpec, schema: &PropertyGraphSchema, parts: &mut NodeParts) -> bool {
    match spec {
        LabelPropertySpec::Empty => true,
        LabelPropertySpec::Label(label) => {
            parts.labels.push(label.clone());
            true
        },
        LabelPropertySpec::Ref(name) => schema
            .get_node_semantics(name)
            .is_ok_and(|referenced| flatten(referenced, schema, parts)),
        LabelPropertySpec::And(a, b) => flatten(a, schema, parts) & flatten(b, schema, parts),
        LabelPropertySpec::Open(a) => flatten(a, schema, parts),
        LabelPropertySpec::Content(a, content) => {
            let (closed, value) = match content {
                PropertyValueSpec::Closed(value) => (true, value),
                PropertyValueSpec::Open(value) => (false, value),
            };
            parts.closed |= closed;
            flatten(a, schema, parts) & keys(value, parts)
        },
        LabelPropertySpec::Optional(_) | LabelPropertySpec::Or(_, _) => false,
    }
}

fn keys(value: &PropertyValue, parts: &mut NodeParts) -> bool {
    match value {
        PropertyValue::Empty => true,
        PropertyValue::EachOf(a, b) => keys(a, parts) & keys(b, parts),
        PropertyValue::Property(key, spec) => {
            parts.keys.push((key.str().to_string(), spec.to_value_type(), true));
            true
        },
        PropertyValue::OptionalProperty(key, spec) => {
            parts.keys.push((key.str().to_string(), spec.to_value_type(), false));
            true
        },
        PropertyValue::OneOf(_, _) => false,
    }
}

/// A key as a literal property. A required key must be present even when its
/// cardinality admits no values, so it gets at least one.
fn key_property(key: &str, value_type: &ValueType, required: bool) -> Option<UnifiedPropertyConstraint> {
    let mut constraints = Vec::new();
    let card = base_type(value_type, &mut constraints)?;
    let (min, max) = match card {
        Card::ZeroOrOne => (0, Some(1)),
        Card::One => (1, Some(1)),
        Card::ZeroOrMore => (0, None),
        Card::OneOrMore => (1, None),
        Card::Range(min, Max::Bounded(max)) => (min as u32, Some(max as u32)),
        Card::Range(min, Max::Unbounded) => (min as u32, None),
    };
    let min = if required { min.max(1) } else { 0 };
    Some(UnifiedPropertyConstraint {
        property_iri: key_iri(key),
        constraints,
        min_cardinality: Some(min),
        max_cardinality: max,
        counts_matching_values: false,
        all_values_match: true,
    })
}

/// The datatype (pushed onto `constraints`, with any `CHECK` facets) and the
/// cardinality of a value type.
fn base_type(value_type: &ValueType, constraints: &mut Vec<UnifiedConstraint>) -> Option<Card> {
    let (datatype, card) = match value_type {
        ValueType::StringType(card) => ("string", card),
        ValueType::IntegerType(card) => ("integer", card),
        ValueType::DateType(card) => ("date", card),
        ValueType::BoolType(card) => ("boolean", card),
        ValueType::Intersection(a, b) => {
            let card = base_type(a, constraints)?;
            let ValueType::Cond(condition) = b.as_ref() else {
                return None;
            };
            return facets(condition, constraints).then_some(card);
        },
        ValueType::Any | ValueType::Union(_, _) | ValueType::Cond(_) => return None,
    };
    constraints.push(UnifiedConstraint::Datatype(format!("{XSD}{datatype}")));
    Some(card.clone())
}

fn facets(condition: &BooleanExpr, constraints: &mut Vec<UnifiedConstraint>) -> bool {
    let bound = |v: &PgValue| match v {
        PgValue::Integer(n) => Some(Value::Literal(n.to_string(), Some(format!("{XSD}integer")))),
        _ => None,
    };
    let constraint = match condition {
        BooleanExpr::And(a, b) => return facets(a, constraints) && facets(b, constraints),
        BooleanExpr::True => return true,
        BooleanExpr::GreaterThan(v) => bound(v).map(UnifiedConstraint::MinExclusive),
        BooleanExpr::GreaterThanOrEqual(v) => bound(v).map(UnifiedConstraint::MinInclusive),
        BooleanExpr::LessThan(v) => bound(v).map(UnifiedConstraint::MaxExclusive),
        BooleanExpr::LessThanOrEqual(v) => bound(v).map(UnifiedConstraint::MaxInclusive),
        BooleanExpr::Regex(pattern) => Some(UnifiedConstraint::Pattern(pattern.clone())),
        _ => None,
    };
    match constraint {
        Some(c) => {
            constraints.push(c);
            true
        },
        None => false,
    }
}
