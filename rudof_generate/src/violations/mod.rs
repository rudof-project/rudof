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
/// Predicate used by `closed-extra`; no schema declares or ignores it.
pub const UNDECLARED_PROPERTY: &str = "http://rudof.example/violations#undeclaredProperty";

/// One ground-truth entry: a node a correct validator must report as failing
/// against `shape`.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct ViolationLabel {
    pub node: String,
    pub shape: String,
    /// `"direct"` (a constraint of this node was broken), `"induced"` (it
    /// references, through `property`, a node that fails) or `"trap"` (the node
    /// was changed in a way that keeps it valid; reporting it is a false positive).
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
    let mut graph = Indexed::new(std::mem::take(triples));

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
            if let Some(property) = apply(operator, &mut graph, node, shape, model, &mut rng) {
                if !operator.is_trap() {
                    failing.insert(node.clone());
                }
                labels.push(ViolationLabel {
                    node: node.clone(),
                    shape: shape_id.clone(),
                    kind: if operator.is_trap() { "trap" } else { "direct" },
                    property,
                    operator: Some(operator.name()),
                    cause: None,
                });
                break;
            }
        }
    }

    *triples = graph.into_triples();
    labels.extend(induced_failures(triples, model, entities, &mut failing));
    labels.extend(class_traps(triples, model, entities, &failing));
    Ok(labels)
}

/// Nodes whose `sh:class` value fails its own shape. The value keeps its type,
/// so the node conforms; reporting it means reading `sh:class` as `sh:node`.
fn class_traps(
    triples: &[Triple],
    model: &UnifiedConstraintModel,
    entities: &[(String, String)],
    failing: &BTreeSet<String>,
) -> Vec<ViolationLabel> {
    let shape_of: HashMap<&str, &str> = entities.iter().map(|(n, s)| (n.as_str(), s.as_str())).collect();
    let mut traps: BTreeMap<String, ViolationLabel> = BTreeMap::new();
    for t in triples {
        let (NamedOrBlankNode::NamedNode(s), Term::NamedNode(o)) = (&t.subject, &t.object) else {
            continue;
        };
        if failing.contains(s.as_str()) || !failing.contains(o.as_str()) || traps.contains_key(s.as_str()) {
            continue;
        }
        let Some((shape_id, shape)) = shape_of
            .get(s.as_str())
            .and_then(|id| model.shapes.get(*id).map(|shape| (*id, shape)))
        else {
            continue;
        };
        let by_class = shape.properties.iter().any(|p| {
            p.property_iri == t.predicate.as_str()
                && p.constraints.iter().any(|c| matches!(c, UnifiedConstraint::Class(_)))
        });
        if by_class {
            traps.insert(
                s.as_str().to_string(),
                ViolationLabel {
                    node: s.as_str().to_string(),
                    shape: shape_id.to_string(),
                    kind: "trap",
                    property: t.predicate.as_str().to_string(),
                    operator: Some("class-of-failing"),
                    cause: Some(o.as_str().to_string()),
                },
            );
        }
    }
    traps.into_values().collect()
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
    let mut tolerant: HashMap<(String, String), usize> = HashMap::new();
    for t in triples {
        let (NamedOrBlankNode::NamedNode(s), Term::NamedNode(o)) = (&t.subject, &t.object) else {
            continue;
        };
        let Some(shape) = shape_of.get(s.as_str()).and_then(|id| model.shapes.get(*id)) else {
            continue;
        };
        let reference = shape.properties.iter().find(|p| {
            p.property_iri == t.predicate.as_str()
                && p.constraints
                    .iter()
                    .any(|c| matches!(c, UnifiedConstraint::ShapeReference(_)))
        });
        if let Some(prop) = reference {
            // A tolerant reference (qualified value shape, ShEx EXTRA) fails only
            // when fewer than its minimum of conforming targets remain.
            if !prop.all_values_match {
                tolerant.insert(
                    (s.as_str().to_string(), t.predicate.as_str().to_string()),
                    prop.min_cardinality.unwrap_or(0) as usize,
                );
            }
            refs.push((
                s.as_str().to_string(),
                t.predicate.as_str().to_string(),
                o.as_str().to_string(),
            ));
        }
    }
    refs.sort();

    // Reverse index: target -> (referrer, property), then a worklist from the
    // directly failing nodes, so each reference is followed once.
    let mut referrers: HashMap<&str, Vec<(&str, &str)>> = HashMap::new();
    let mut targets: HashMap<(&str, &str), Vec<&str>> = HashMap::new();
    for (referrer, property, target) in &refs {
        referrers
            .entry(target.as_str())
            .or_default()
            .push((referrer.as_str(), property.as_str()));
        targets
            .entry((referrer.as_str(), property.as_str()))
            .or_default()
            .push(target.as_str());
    }
    let mut labels = Vec::new();
    let mut queue: std::collections::VecDeque<String> = failing.iter().cloned().collect();
    while let Some(target) = queue.pop_front() {
        let Some(sources) = referrers.get(target.as_str()) else {
            continue;
        };
        for (referrer, property) in sources {
            if failing.contains(*referrer) {
                continue;
            }
            if let Some(min) = tolerant.get(&(referrer.to_string(), property.to_string())) {
                // Only generated entities can conform: the no-data node of a trap
                // and a dangling target never do.
                let conforming = targets[&(*referrer, *property)]
                    .iter()
                    .filter(|t| shape_of.contains_key(**t) && !failing.contains(**t))
                    .count();
                if conforming >= *min {
                    continue;
                }
            }
            if failing.insert(referrer.to_string()) {
                labels.push(ViolationLabel {
                    node: referrer.to_string(),
                    shape: shape_of.get(referrer).copied().unwrap_or_default().to_string(),
                    kind: "induced",
                    property: property.to_string(),
                    operator: None,
                    cause: Some(target.clone()),
                });
                queue.push_back(referrer.to_string());
            }
        }
    }
    labels
}

/// The generated triples with an index by subject, so a mutation touches only
/// the triples of the entity it breaks.
struct Indexed {
    triples: Vec<Option<Triple>>,
    by_subject: HashMap<String, Vec<usize>>,
}

impl Indexed {
    fn new(triples: Vec<Triple>) -> Self {
        let mut by_subject: HashMap<String, Vec<usize>> = HashMap::new();
        for (i, t) in triples.iter().enumerate() {
            if let NamedOrBlankNode::NamedNode(s) = &t.subject {
                by_subject.entry(s.as_str().to_string()).or_default().push(i);
            }
        }
        Self {
            triples: triples.into_iter().map(Some).collect(),
            by_subject,
        }
    }

    fn values(&self, node: &str, property: &str) -> Vec<Term> {
        let mut values: Vec<Term> = self
            .by_subject
            .get(node)
            .into_iter()
            .flatten()
            .filter_map(|&i| self.triples[i].as_ref())
            .filter(|t| t.predicate.as_str() == property)
            .map(|t| t.object.clone())
            .collect();
        values.sort_by_key(|v| v.to_string());
        values
    }

    /// Remove the triples of `node` with `property` (and `object`, if given).
    fn remove(&mut self, node: &str, property: &str, object: Option<&Term>) {
        for &i in self.by_subject.get(node).into_iter().flatten() {
            let matches = self.triples[i]
                .as_ref()
                .is_some_and(|t| t.predicate.as_str() == property && object.is_none_or(|o| &t.object == o));
            if matches {
                self.triples[i] = None;
            }
        }
    }

    fn add(&mut self, triple: Triple) {
        if let NamedOrBlankNode::NamedNode(s) = &triple.subject {
            self.by_subject
                .entry(s.as_str().to_string())
                .or_default()
                .push(self.triples.len());
        }
        self.triples.push(Some(triple));
    }

    fn into_triples(self) -> Vec<Triple> {
        self.triples.into_iter().flatten().collect()
    }
}

fn rng_for(seed: Option<u64>, node: &str) -> StdRng {
    let mut hasher = DefaultHasher::new();
    seed.unwrap_or(0).hash(&mut hasher);
    "violation".hash(&mut hasher);
    node.hash(&mut hasher);
    StdRng::seed_from_u64(hasher.finish())
}

/// Try `operator` on `node`; on success return the property it broke. The
/// properties are tried from a random starting point, so every property the
/// operator applies to gets broken somewhere in the data.
fn apply(
    operator: ViolationOperator,
    triples: &mut Indexed,
    node: &str,
    shape: &UnifiedShape,
    model: &UnifiedConstraintModel,
    rng: &mut StdRng,
) -> Option<String> {
    if operator == ViolationOperator::ClosedExtra {
        if !shape.closed {
            return None;
        }
        Mutation::Add(vec![Term::Literal(Literal::new_simple_literal(
            "not allowed by the closed shape",
        ))])
        .apply(triples, node, UNDECLARED_PROPERTY, &[]);
        return Some(UNDECLARED_PROPERTY.to_string());
    }
    let mut props: Vec<&UnifiedPropertyConstraint> =
        shape.properties.iter().filter(|p| p.property_iri != RDF_TYPE).collect();
    props.sort_by(|a, b| a.property_iri.cmp(&b.property_iri));
    if !props.is_empty() {
        let start = rng.gen_range(0..props.len());
        props.rotate_left(start);
    }
    for prop in props {
        let values = triples.values(node, &prop.property_iri);
        let replacement: Option<Mutation> = match operator {
            ViolationOperator::DropRequired => {
                (prop.min_cardinality.unwrap_or(0) >= 1 && !values.is_empty()).then_some(Mutation::RemoveAll)
            },
            ViolationOperator::ExceedMax => exceed_max(prop, &values),
            ViolationOperator::WrongDatatype => wrong_datatype(prop, &values),
            ViolationOperator::IllTypedLiteral => ill_typed_literal(prop, &values),
            ViolationOperator::OutOfRange => out_of_range(prop, &values),
            ViolationOperator::NotInValueSet => not_in_value_set(prop, &values),
            ViolationOperator::PatternBreak => pattern_break(prop, &values),
            ViolationOperator::DanglingReference => dangling_reference(prop, &values, node, model),
            ViolationOperator::ClosedExtra => None,
            ViolationOperator::WrongLanguage => wrong_language(prop, &values),
            ViolationOperator::DuplicateLanguage => duplicate_language(prop, &values),
            ViolationOperator::TrapLanguageCase => relabel_language(prop, &values, |t| t.to_ascii_uppercase(), false),
            ViolationOperator::TrapLanguageSubtag => relabel_language(prop, &values, |t| format!("{t}-zz"), true),
            ViolationOperator::TrapNumericLexical => numeric_lexical_variant(prop, &values),
            ViolationOperator::LangStringForString => lang_string_for_string(prop, &values),
            ViolationOperator::DerivedDatatype => derived_datatype(prop, &values),
            ViolationOperator::LookalikeValue => lookalike_value(prop, &values),
            ViolationOperator::TrapDecimalLexical => decimal_lexical_variant(prop, &values),
            ViolationOperator::UnmatchedValue => prop
                .all_values_match
                .then(|| unmatched_value(prop, &values, node, model))
                .flatten(),
            // Every constraint on the path must tolerate the unmatched value.
            ViolationOperator::TrapExtraValue => shape
                .properties
                .iter()
                .filter(|p| p.property_iri == prop.property_iri)
                .all(|p| !p.all_values_match)
                .then(|| unmatched_value(prop, &values, node, model))
                .flatten(),
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
    fn apply(self, triples: &mut Indexed, node: &str, property: &str, values: &[Term]) {
        let subject = NamedOrBlankNode::NamedNode(NamedNode::new_unchecked(node));
        let predicate = NamedNode::new_unchecked(property);
        match self {
            Mutation::RemoveAll => triples.remove(node, property, None),
            Mutation::ReplaceFirst(term) => {
                triples.remove(node, property, Some(&values[0]));
                triples.add(Triple::new(subject, predicate, term));
            },
            Mutation::Add(terms) => {
                for term in terms {
                    triples.add(Triple::new(subject.clone(), predicate.clone(), term));
                }
            },
        }
    }
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
    Some(Mutation::ReplaceFirst(other_datatype_value(datatype_of(prop)?)))
}

/// A value whose datatype is not `datatype`.
fn other_datatype_value(datatype: &str) -> Term {
    if datatype == format!("{XSD}string") {
        typed("42", &format!("{XSD}integer"))
    } else {
        Term::Literal(Literal::new_simple_literal("not a value of the declared datatype"))
    }
}

/// A lexical form that is not valid for `datatype`, for the XSD datatypes that
/// have a lexical space to violate (strings accept every lexical form).
fn invalid_lexical_form(datatype: &str) -> Option<&'static str> {
    match datatype.strip_prefix(XSD)? {
        "integer" | "int" | "long" | "short" | "byte" | "nonNegativeInteger" | "positiveInteger" | "decimal"
        | "double" | "float" => Some("not-a-number"),
        "boolean" => Some("maybe"),
        "date" => Some("2024-13-45"),
        "dateTime" => Some("yesterday at noon"),
        _ => None,
    }
}

fn ill_typed_literal(prop: &UnifiedPropertyConstraint, values: &[Term]) -> Option<Mutation> {
    if values.is_empty() {
        return None;
    }
    let datatype = datatype_of(prop)?;
    let lexical = invalid_lexical_form(datatype)?;
    Some(Mutation::ReplaceFirst(typed(lexical, datatype)))
}

fn language_constraint(prop: &UnifiedPropertyConstraint) -> Option<(&[String], bool)> {
    prop.constraints.iter().find_map(|c| match c {
        UnifiedConstraint::Language { tags, ranges } if !tags.is_empty() => Some((tags.as_slice(), *ranges)),
        _ => None,
    })
}

fn tagged(lexical: &str, tag: &str) -> Term {
    Term::Literal(Literal::new_language_tagged_literal_unchecked(lexical, tag))
}

/// The first value as (lexical form, tag), if it is a language-tagged literal.
fn first_tagged(values: &[Term]) -> Option<(String, String)> {
    match values.first()? {
        Term::Literal(lit) => lit.language().map(|tag| (lit.value().to_string(), tag.to_string())),
        _ => None,
    }
}

fn wrong_language(prop: &UnifiedPropertyConstraint, values: &[Term]) -> Option<Mutation> {
    let (tags, ranges) = language_constraint(prop)?;
    let (lexical, _) = first_tagged(values)?;
    let outside = ["zz", "qaa", "qtz"]
        .into_iter()
        .find(|t| !crate::ir::language_matches(t, tags, ranges))?;
    Some(Mutation::ReplaceFirst(tagged(&lexical, outside)))
}

fn duplicate_language(prop: &UnifiedPropertyConstraint, values: &[Term]) -> Option<Mutation> {
    prop.constraints
        .iter()
        .any(|c| matches!(c, UnifiedConstraint::UniqueLang))
        .then_some(())?;
    let (_, tag) = first_tagged(values)?;
    // Stay within the maximum, so uniqueLang is the only constraint broken.
    if prop.max_cardinality.is_some_and(|max| values.len() + 1 > max as usize) {
        return None;
    }
    Some(Mutation::Add(vec![tagged("duplicate language", &tag)]))
}

/// Retag the first value with `retag(tag)`; used by the traps, which must keep
/// the node valid. `needs_ranges` limits the subtag trap to range matching.
fn relabel_language(
    prop: &UnifiedPropertyConstraint,
    values: &[Term],
    retag: impl Fn(&str) -> String,
    needs_ranges: bool,
) -> Option<Mutation> {
    let (tags, ranges) = language_constraint(prop)?;
    if needs_ranges && !ranges {
        return None;
    }
    let (lexical, tag) = first_tagged(values)?;
    let new_tag = retag(&tag);
    (new_tag != tag && crate::ir::language_matches(&new_tag, tags, ranges))
        .then(|| Mutation::ReplaceFirst(tagged(&lexical, &new_tag)))
}

fn has_range(prop: &UnifiedPropertyConstraint) -> bool {
    prop.constraints.iter().any(|c| {
        matches!(
            c,
            UnifiedConstraint::MinInclusive(_)
                | UnifiedConstraint::MaxInclusive(_)
                | UnifiedConstraint::MinExclusive(_)
                | UnifiedConstraint::MaxExclusive(_)
        )
    })
}

/// Constraints that compare a value's term (value sets, language tags) rather
/// than its lexical form or value; changing the term would break them too.
fn compares_terms(prop: &UnifiedPropertyConstraint) -> bool {
    prop.constraints.iter().any(|c| {
        matches!(
            c,
            UnifiedConstraint::In(_)
                | UnifiedConstraint::HasValue(_)
                | UnifiedConstraint::Language { .. }
                | UnifiedConstraint::UniqueLang
        )
    })
}

/// Constraints that read the lexical form; a lexical variant would be a real change there.
fn reads_lexical_form(prop: &UnifiedPropertyConstraint) -> bool {
    compares_terms(prop)
        || prop.constraints.iter().any(|c| {
            matches!(
                c,
                UnifiedConstraint::Pattern(_) | UnifiedConstraint::MinLength(_) | UnifiedConstraint::MaxLength(_)
            )
        })
}

/// The first value's lexical form, if it is a literal.
fn first_lexical(values: &[Term]) -> Option<&str> {
    match values.first()? {
        Term::Literal(lit) => Some(lit.value()),
        _ => None,
    }
}

/// Same integer value, non-canonical lexical form: a leading zero after the sign.
/// Only under numeric range facets, and never where a pattern, value set or
/// length facet reads the lexical form (there it would be a real change).
fn numeric_lexical_variant(prop: &UnifiedPropertyConstraint, values: &[Term]) -> Option<Mutation> {
    let datatype = datatype_of(prop)?;
    if datatype != format!("{XSD}integer") || !has_range(prop) || reads_lexical_form(prop) {
        return None;
    }
    let lexical = first_lexical(values)?;
    lexical.parse::<i64>().ok()?;
    let variant = match lexical.strip_prefix('-') {
        Some(digits) => format!("-0{digits}"),
        None => format!("0{}", lexical.trim_start_matches('+')),
    };
    Some(Mutation::ReplaceFirst(typed(&variant, datatype)))
}

/// Same decimal value with a trailing fractional zero (`3.5` -> `3.50`, `3` -> `3.0`).
fn decimal_lexical_variant(prop: &UnifiedPropertyConstraint, values: &[Term]) -> Option<Mutation> {
    let datatype = datatype_of(prop)?;
    if datatype != format!("{XSD}decimal") || !has_range(prop) || reads_lexical_form(prop) {
        return None;
    }
    let lexical = first_lexical(values)?;
    let is_decimal = !lexical.is_empty()
        && lexical.chars().all(|c| c.is_ascii_digit() || "+-.".contains(c))
        && lexical.parse::<f64>().is_ok();
    if !is_decimal {
        return None;
    }
    let variant = if lexical.contains('.') {
        format!("{lexical}0")
    } else {
        format!("{lexical}.0")
    };
    Some(Mutation::ReplaceFirst(typed(&variant, datatype)))
}

/// `"ABC"@en` where `xsd:string` is required. Patterns and length facets read the
/// lexical form, which is kept, so the datatype is the only constraint broken.
fn lang_string_for_string(prop: &UnifiedPropertyConstraint, values: &[Term]) -> Option<Mutation> {
    if datatype_of(prop)? != format!("{XSD}string") || compares_terms(prop) {
        return None;
    }
    Some(Mutation::ReplaceFirst(tagged(first_lexical(values)?, "en")))
}

/// A datatype derived from `datatype` by restriction, with the lexical forms it
/// accepts; `None` where no derived type keeps every value.
fn derived_datatype_of(datatype: &str, lexical: &str) -> Option<&'static str> {
    match datatype.strip_prefix(XSD)? {
        "integer" => lexical.parse::<i32>().is_ok().then_some("int"),
        "decimal" => lexical.parse::<i64>().is_ok().then_some("integer"),
        "string" => (!lexical.contains(|c: char| c.is_ascii_control())).then_some("normalizedString"),
        _ => None,
    }
}

fn derived_datatype(prop: &UnifiedPropertyConstraint, values: &[Term]) -> Option<Mutation> {
    let datatype = datatype_of(prop)?;
    if compares_terms(prop) {
        return None;
    }
    let lexical = first_lexical(values)?;
    let derived = derived_datatype_of(datatype, lexical)?;
    Some(Mutation::ReplaceFirst(typed(lexical, &format!("{XSD}{derived}"))))
}

/// The text of a value-set member, as `lookalike_value` compares it.
fn member_text(value: &Value) -> Option<&str> {
    match value {
        Value::Iri(iri) => Some(iri.as_str()),
        Value::Literal(lexical, _) => Some(lexical.as_str()),
        Value::BlankNode(_) => None,
    }
}

/// A term with the same text as the current value (a member of the value set) but
/// not equal to it: an IRI becomes a string, a typed literal or a language-tagged
/// string loses its datatype or tag, a string gains a tag.
fn lookalike_value(prop: &UnifiedPropertyConstraint, values: &[Term]) -> Option<Mutation> {
    let members = prop.constraints.iter().find_map(|c| match c {
        UnifiedConstraint::In(members) if !members.is_empty() => Some(members),
        _ => None,
    })?;
    // Any other constraint could be broken by the new term as well.
    if prop.constraints.iter().any(|c| !matches!(c, UnifiedConstraint::In(_))) {
        return None;
    }
    let (text, term) = match values.first()? {
        Term::NamedNode(iri) => (iri.as_str(), Term::Literal(Literal::new_simple_literal(iri.as_str()))),
        Term::Literal(lit) if lit.language().is_none() && lit.datatype().as_str() == format!("{XSD}string") => {
            (lit.value(), tagged(lit.value(), "en"))
        },
        Term::Literal(lit) => (lit.value(), Term::Literal(Literal::new_simple_literal(lit.value()))),
        _ => return None,
    };
    // With two members of the same text, the lookalike might be the other one.
    let same_text = members.iter().filter(|m| member_text(m) == Some(text)).count();
    (same_text == 1).then_some(Mutation::ReplaceFirst(term))
}

/// Add one value outside the property's value set, or of another datatype when
/// the datatype is its only value constraint. Where the maximum counts every
/// value (SHACL), it must leave room for the new one.
fn unmatched_value(
    prop: &UnifiedPropertyConstraint,
    values: &[Term],
    node: &str,
    model: &UnifiedConstraintModel,
) -> Option<Mutation> {
    let counted = !prop.counts_matching_values;
    if counted && prop.max_cardinality.is_some_and(|max| values.len() + 1 > max as usize) {
        return None;
    }
    if let [UnifiedConstraint::ShapeReference(target)] = prop.constraints.as_slice() {
        return requires_something(model, target).then(|| Mutation::Add(vec![no_data_node(node, "unmatched")]));
    }
    if prop.constraints.iter().all(|c| matches!(c, UnifiedConstraint::In(_))) {
        let members = prop.constraints.iter().find_map(|c| match c {
            UnifiedConstraint::In(members) if !members.is_empty() => Some(members),
            _ => None,
        })?;
        return Some(Mutation::Add(vec![outside_value_set(&members[0])?]));
    }
    if has_value_constraints(prop) {
        return None;
    }
    let datatype = datatype_of(prop)?;
    Some(Mutation::Add(vec![other_datatype_value(datatype)]))
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
    Some(Mutation::ReplaceFirst(outside_value_set(&members[0])?))
}

/// A term of the same kind as `member` that is not in its value set.
fn outside_value_set(member: &Value) -> Option<Term> {
    Some(match member {
        Value::Iri(iri) => Term::NamedNode(NamedNode::new_unchecked(format!("{iri}-not-in-set"))),
        Value::Literal(_, Some(datatype)) => typed("value outside the set", datatype),
        Value::Literal(_, None) => Term::Literal(Literal::new_simple_literal("value outside the set")),
        Value::BlankNode(_) => return None,
    })
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
    // A node with no data has no type either, so it always breaks `sh:class`.
    let breaks = prop.constraints.iter().find_map(|c| match c {
        UnifiedConstraint::ShapeReference(s) => Some(requires_something(model, s)),
        UnifiedConstraint::Class(_) => Some(true),
        _ => None,
    })?;
    // Where only conforming values are counted (a qualified value shape), losing
    // one is a violation only if fewer than the minimum remain.
    if !prop.all_values_match && values.len() > prop.min_cardinality.unwrap_or(0) as usize {
        return None;
    }
    breaks.then(|| Mutation::ReplaceFirst(no_data_node(node, "dangling")))
}

/// A node with no data violates `target` only if the target requires something.
fn requires_something(model: &UnifiedConstraintModel, target: &str) -> bool {
    model.shapes.get(target).is_some_and(|s| {
        s.properties
            .iter()
            .any(|p| p.property_iri != RDF_TYPE && p.min_cardinality.unwrap_or(0) >= 1)
    })
}

fn no_data_node(node: &str, suffix: &str) -> Term {
    Term::NamedNode(NamedNode::new_unchecked(format!("{node}/{suffix}")))
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
