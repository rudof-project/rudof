//! Controlled, labelled constraint violations.
//!
//! Applied to a schema-valid generated graph: each selected entity has exactly
//! one constraint of its own shape broken by one [`ViolationOperator`], and
//! everything else about it is left intact. Breaking a node also breaks every
//! node that reaches it through shape references (ShEx `@S`, SHACL `sh:node`),
//! because those references require the target to conform. Those *induced*
//! failures are derived here as well, so the labels list exactly the nodes a
//! correct validator reports, which makes the output a validator benchmark with
//! ground truth.

use crate::config::{ViolationOperator, ViolationsConfig};
use crate::ir::{UnifiedConstraint, UnifiedConstraintModel, UnifiedPropertyConstraint, UnifiedShape, Value};
use crate::{DataGeneratorError, Result};
use oxrdf::{Literal, NamedNode, NamedOrBlankNode, Term, Triple};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde::Serialize;
use std::collections::hash_map::DefaultHasher;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::hash::{Hash, Hasher};

const RDF_TYPE: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#type";
const XSD: &str = "http://www.w3.org/2001/XMLSchema#";

/// One ground-truth entry: a node a correct validator must report as failing
/// against `shape`.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct ViolationLabel {
    pub node: String,
    pub shape: String,
    /// `"direct"` (a constraint of this node was broken) or `"induced"` (it
    /// references, through `property`, a node that fails).
    pub kind: &'static str,
    pub property: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator: Option<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
}

/// Break entities of `triples` according to `config` and return the labels.
///
/// `entities` pairs every generated node with the shape that generated it.
pub fn inject(
    triples: &mut Vec<Triple>,
    model: &UnifiedConstraintModel,
    entities: &[(String, String)],
    config: &ViolationsConfig,
    seed: Option<u64>,
) -> Result<Vec<ViolationLabel>> {
    let mut labels = Vec::new();
    let mut failing: BTreeSet<String> = BTreeSet::new();

    for (index, (node, shape_id)) in entities.iter().enumerate() {
        if !config.shapes.is_empty() && !config.shapes.contains(shape_id) {
            continue;
        }
        let Some(shape) = model.shapes.get(shape_id) else {
            continue;
        };
        let mut rng = rng_for(seed, node);
        if config.rate <= 0.0 || rng.r#gen::<f64>() >= config.rate {
            continue;
        }
        // Rotate the operator list so a run exercises all of them.
        let n = config.operators.len();
        for k in 0..n {
            let operator = config.operators[(index + k) % n];
            if let Some(property) = apply(operator, triples, node, shape, model) {
                failing.insert(node.clone());
                labels.push(ViolationLabel {
                    node: node.clone(),
                    shape: shape_id.clone(),
                    kind: "direct",
                    property,
                    operator: Some(operator.name()),
                    cause: None,
                });
                break;
            }
        }
    }

    labels.extend(induced_failures(triples, model, entities, &mut failing));
    Ok(labels)
}

/// Write labels as JSON Lines.
pub fn write_labels(path: &std::path::Path, labels: &[ViolationLabel]) -> Result<()> {
    let mut out = String::new();
    for label in labels {
        out.push_str(&serde_json::to_string(label).map_err(|e| DataGeneratorError::OutputWriting(e.to_string()))?);
        out.push('\n');
    }
    std::fs::write(path, out)?;
    Ok(())
}

/// Nodes that fail because a shape reference points at a failing node,
/// computed to a fixpoint over the generated graph.
fn induced_failures(
    triples: &[Triple],
    model: &UnifiedConstraintModel,
    entities: &[(String, String)],
    failing: &mut BTreeSet<String>,
) -> Vec<ViolationLabel> {
    let shape_of: HashMap<&str, &str> = entities.iter().map(|(n, s)| (n.as_str(), s.as_str())).collect();
    // (referrer, property, target) for every shape-reference triple.
    let mut refs: Vec<(String, String, String)> = Vec::new();
    for t in triples {
        let (NamedOrBlankNode::NamedNode(s), Term::NamedNode(o)) = (&t.subject, &t.object) else {
            continue;
        };
        let Some(shape) = shape_of.get(s.as_str()).and_then(|id| model.shapes.get(*id)) else {
            continue;
        };
        let is_ref = shape.properties.iter().any(|p| {
            p.property_iri == t.predicate.as_str()
                && p.constraints
                    .iter()
                    .any(|c| matches!(c, UnifiedConstraint::ShapeReference(_)))
        });
        if is_ref {
            refs.push((
                s.as_str().to_string(),
                t.predicate.as_str().to_string(),
                o.as_str().to_string(),
            ));
        }
    }
    refs.sort();

    let mut labels = Vec::new();
    loop {
        let mut added = false;
        for (referrer, property, target) in &refs {
            if failing.contains(target) && !failing.contains(referrer) {
                failing.insert(referrer.clone());
                labels.push(ViolationLabel {
                    node: referrer.clone(),
                    shape: shape_of.get(referrer.as_str()).copied().unwrap_or_default().to_string(),
                    kind: "induced",
                    property: property.clone(),
                    operator: None,
                    cause: Some(target.clone()),
                });
                added = true;
            }
        }
        if !added {
            return labels;
        }
    }
}

fn rng_for(seed: Option<u64>, node: &str) -> StdRng {
    let mut hasher = DefaultHasher::new();
    seed.unwrap_or(0).hash(&mut hasher);
    "violation".hash(&mut hasher);
    node.hash(&mut hasher);
    StdRng::seed_from_u64(hasher.finish())
}

/// Try `operator` on `node`; on success return the property it broke.
fn apply(
    operator: ViolationOperator,
    triples: &mut Vec<Triple>,
    node: &str,
    shape: &UnifiedShape,
    model: &UnifiedConstraintModel,
) -> Option<String> {
    let mut props: Vec<&UnifiedPropertyConstraint> =
        shape.properties.iter().filter(|p| p.property_iri != RDF_TYPE).collect();
    props.sort_by(|a, b| a.property_iri.cmp(&b.property_iri));
    for prop in props {
        let values = values_of(triples, node, &prop.property_iri);
        let replacement: Option<Mutation> = match operator {
            ViolationOperator::DropRequired => {
                (prop.min_cardinality.unwrap_or(0) >= 1 && !values.is_empty()).then_some(Mutation::RemoveAll)
            },
            ViolationOperator::ExceedMax => exceed_max(prop, &values),
            ViolationOperator::WrongDatatype => wrong_datatype(prop, &values),
            ViolationOperator::OutOfRange => out_of_range(prop, &values),
            ViolationOperator::NotInValueSet => not_in_value_set(prop, &values),
            ViolationOperator::PatternBreak => pattern_break(prop, &values),
            ViolationOperator::DanglingReference => dangling_reference(prop, &values, node, model),
        };
        if let Some(mutation) = replacement {
            mutation.apply(triples, node, &prop.property_iri, &values);
            return Some(prop.property_iri.clone());
        }
    }
    None
}

enum Mutation {
    RemoveAll,
    ReplaceFirst(Term),
    Add(Vec<Term>),
}

impl Mutation {
    fn apply(self, triples: &mut Vec<Triple>, node: &str, property: &str, values: &[Term]) {
        let subject = NamedOrBlankNode::NamedNode(NamedNode::new_unchecked(node));
        let predicate = NamedNode::new_unchecked(property);
        match self {
            Mutation::RemoveAll => triples.retain(|t| !(t.subject == subject && t.predicate == predicate)),
            Mutation::ReplaceFirst(term) => {
                let first = values[0].clone();
                triples.retain(|t| !(t.subject == subject && t.predicate == predicate && t.object == first));
                triples.push(Triple::new(subject, predicate, term));
            },
            Mutation::Add(terms) => {
                for term in terms {
                    triples.push(Triple::new(subject.clone(), predicate.clone(), term));
                }
            },
        }
    }
}

fn values_of(triples: &[Triple], node: &str, property: &str) -> Vec<Term> {
    let mut values: Vec<Term> = triples
        .iter()
        .filter(|t| matches!(&t.subject, NamedOrBlankNode::NamedNode(s) if s.as_str() == node))
        .filter(|t| t.predicate.as_str() == property)
        .map(|t| t.object.clone())
        .collect();
    values.sort_by_key(|v| v.to_string());
    values
}

fn datatype_of(prop: &UnifiedPropertyConstraint) -> Option<&str> {
    prop.constraints.iter().find_map(|c| match c {
        UnifiedConstraint::Datatype(d) => Some(d.trim_matches(['<', '>'])),
        _ => None,
    })
}

/// Constraints on a property's values other than its datatype.
fn has_value_constraints(prop: &UnifiedPropertyConstraint) -> bool {
    prop.constraints
        .iter()
        .any(|c| !matches!(c, UnifiedConstraint::Datatype(_)))
}

fn typed(lexical: &str, datatype: &str) -> Term {
    Term::Literal(Literal::new_typed_literal(lexical, NamedNode::new_unchecked(datatype)))
}

fn exceed_max(prop: &UnifiedPropertyConstraint, values: &[Term]) -> Option<Mutation> {
    let max = prop.max_cardinality? as usize;
    if has_value_constraints(prop) {
        return None;
    }
    let datatype = datatype_of(prop)?;
    let extra = (max + 1).checked_sub(values.len()).filter(|n| *n > 0)?;
    let make = |i: usize| -> Option<Term> {
        match datatype.strip_prefix(XSD)? {
            "string" => Some(typed(&format!("extra value {i}"), datatype)),
            "integer" | "int" | "long" | "decimal" => Some(typed(&format!("{}", 900_000 + i), datatype)),
            _ => None,
        }
    };
    let terms: Option<Vec<Term>> = (0..extra).map(make).collect();
    Some(Mutation::Add(terms?))
}

fn wrong_datatype(prop: &UnifiedPropertyConstraint, values: &[Term]) -> Option<Mutation> {
    if values.is_empty() || has_value_constraints(prop) {
        return None;
    }
    let datatype = datatype_of(prop)?;
    let term = if datatype == format!("{XSD}string") {
        typed("42", &format!("{XSD}integer"))
    } else {
        Term::Literal(Literal::new_simple_literal("not a value of the declared datatype"))
    };
    Some(Mutation::ReplaceFirst(term))
}

fn integer_bound(value: &Value) -> Option<i64> {
    match value {
        Value::Literal(lexical, _) => lexical.parse().ok(),
        _ => None,
    }
}

fn out_of_range(prop: &UnifiedPropertyConstraint, values: &[Term]) -> Option<Mutation> {
    if values.is_empty() {
        return None;
    }
    let datatype = datatype_of(prop).unwrap_or("http://www.w3.org/2001/XMLSchema#integer");
    prop.constraints.iter().find_map(|c| {
        let n = match c {
            UnifiedConstraint::MaxInclusive(v) => integer_bound(v)? + 1,
            UnifiedConstraint::MaxExclusive(v) => integer_bound(v)?,
            UnifiedConstraint::MinInclusive(v) => integer_bound(v)? - 1,
            UnifiedConstraint::MinExclusive(v) => integer_bound(v)?,
            _ => return None,
        };
        Some(Mutation::ReplaceFirst(typed(&n.to_string(), datatype)))
    })
}

fn not_in_value_set(prop: &UnifiedPropertyConstraint, values: &[Term]) -> Option<Mutation> {
    if values.is_empty() {
        return None;
    }
    let members = prop.constraints.iter().find_map(|c| match c {
        UnifiedConstraint::In(members) if !members.is_empty() => Some(members),
        _ => None,
    })?;
    let term = match &members[0] {
        Value::Iri(iri) => Term::NamedNode(NamedNode::new_unchecked(format!("{iri}-not-in-set"))),
        Value::Literal(_, Some(datatype)) => typed("value outside the set", datatype),
        Value::Literal(_, None) => Term::Literal(Literal::new_simple_literal("value outside the set")),
        Value::BlankNode(_) => return None,
    };
    Some(Mutation::ReplaceFirst(term))
}

fn pattern_break(prop: &UnifiedPropertyConstraint, values: &[Term]) -> Option<Mutation> {
    if values.is_empty() {
        return None;
    }
    let pattern = prop.constraints.iter().find_map(|c| match c {
        UnifiedConstraint::Pattern(p) => Some(p.as_str()),
        _ => None,
    })?;
    let regex = regex::Regex::new(pattern).ok()?;
    let candidate = ["~ breaks the pattern ~", "0", "", "zzzzzzzzzzzzzzzzzzzz"]
        .into_iter()
        .find(|c| !regex.is_match(c))?;
    let datatype = datatype_of(prop).unwrap_or("http://www.w3.org/2001/XMLSchema#string");
    Some(Mutation::ReplaceFirst(typed(candidate, datatype)))
}

fn dangling_reference(
    prop: &UnifiedPropertyConstraint,
    values: &[Term],
    node: &str,
    model: &UnifiedConstraintModel,
) -> Option<Mutation> {
    if values.is_empty() {
        return None;
    }
    let target = prop.constraints.iter().find_map(|c| match c {
        UnifiedConstraint::ShapeReference(s) => Some(s),
        _ => None,
    })?;
    // A node with no data violates the target only if the target requires something.
    let requires = model.shapes.get(target).is_some_and(|s| {
        s.properties
            .iter()
            .any(|p| p.property_iri != RDF_TYPE && p.min_cardinality.unwrap_or(0) >= 1)
    });
    requires.then(|| Mutation::ReplaceFirst(Term::NamedNode(NamedNode::new_unchecked(format!("{node}/dangling")))))
}

/// Number of labels per operator, for reports.
pub fn summary(labels: &[ViolationLabel]) -> BTreeMap<String, usize> {
    let mut counts = BTreeMap::new();
    for label in labels {
        let key = label.operator.unwrap_or("induced").to_string();
        *counts.entry(key).or_insert(0) += 1;
    }
    counts
}
