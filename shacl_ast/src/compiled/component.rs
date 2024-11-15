use std::str::FromStr;

use iri_s::IriS;
use node_kind::NodeKind;
use srdf::model::rdf::Literal;
use srdf::model::rdf::Object;
use srdf::model::rdf::Predicate;
use srdf::model::rdf::Rdf;
use vocab::*;

use crate::component::Component;
use crate::Schema;
use crate::*;

use super::compiled_shacl_error::CompiledShaclError;
use super::shape::CompiledShape;

#[derive(Debug)]
pub enum CompiledComponent<R: Rdf> {
    Class(Class<R>),
    Datatype(Datatype<R>),
    NodeKind(Nodekind),
    MinCount(MinCount),
    MaxCount(MaxCount),
    MinExclusive(MinExclusive<R>),
    MaxExclusive(MaxExclusive<R>),
    MinInclusive(MinInclusive<R>),
    MaxInclusive(MaxInclusive<R>),
    MinLength(MinLength),
    MaxLength(MaxLength),
    Pattern(Pattern),
    UniqueLang(UniqueLang),
    LanguageIn(LanguageIn<R>),
    Equals(Equals<R>),
    Disjoint(Disjoint<R>),
    LessThan(LessThan<R>),
    LessThanOrEquals(LessThanOrEquals<R>),
    Or(Or<R>),
    And(And<R>),
    Not(Not<R>),
    Xone(Xone<R>),
    Closed(Closed<R>),
    Node(Node<R>),
    HasValue(HasValue<R>),
    In(In<R>),
    QualifiedValueShape(QualifiedValueShape<R>),
}
impl<R: Rdf> CompiledComponent<R> {
    pub fn compile(
        component: Component<R>,
        schema: &Schema<R>,
    ) -> Result<Self, CompiledShaclError> {
        let component = match component {
            Component::Class(class) => class.into(),
            Component::Datatype(data_type) => data_type.into(),
            Component::NodeKind(node_kind) => node_kind.into(),
            Component::MinCount(min_count) => min_count.into(),
            Component::MaxCount(count) => count.into(),
            Component::MinExclusive(lit) => lit.into(),
            Component::MaxExclusive(lit) => lit.into(),
            Component::MinInclusive(lit) => lit.into(),
            Component::MaxInclusive(lit) => lit.into(),
            Component::MinLength(length) => length.into(),
            Component::MaxLength(length) => length.into(),
            Component::Pattern(pattern) => pattern.into(),
            Component::UniqueLang(lang) => lang.into(),
            Component::LanguageIn(langs) => langs.into(),
            Component::Equals(iri_ref) => iri_ref.into(),
            Component::Disjoint(iri_ref) => iri_ref.into(),
            Component::LessThan(iri_ref) => iri_ref.into(),
            Component::LessThanOrEquals(iri_ref) => iri_ref.into(),
            Component::Or(or) => or.into(),
            Component::And(and) => and.into(),
            Component::Not(not) => not.into(),
            Component::Xone(xone) => xone.into(),
            Component::Closed(closed) => closed.into(),
            Component::Node(node) => node.into(),
            Component::HasValue(has_value) => has_value.into(),
            Component::In(i) => i.into(),
            Component::QualifiedValueShape(qvs) => qvs.into(),
        };

        Ok(component)
    }
}

/// sh:maxCount specifies the maximum number of value nodes that satisfy the
/// condition.
///
/// - IRI: https://www.w3.org/TR/shacl/#MaxCountConstraintComponent
/// - DEF: If the number of value nodes is greater than $maxCount, there is a
///   validation result.
#[derive(Debug)]
pub struct MaxCount {
    max_count: usize,
}

impl MaxCount {
    pub fn new(max_count: isize) -> Self {
        MaxCount {
            max_count: max_count as usize,
        }
    }

    pub fn max_count(&self) -> usize {
        self.max_count
    }
}

/// sh:minCount specifies the minimum number of value nodes that satisfy the
/// condition. If the minimum cardinality value is 0 then this constraint is
/// always satisfied and so may be omitted.
///
/// - IRI: https://www.w3.org/TR/shacl/#MinCountConstraintComponent
/// - DEF: If the number of value nodes is less than $minCount, there is a
///   validation result.
#[derive(Debug)]
pub struct MinCount {
    min_count: usize,
}

impl MinCount {
    pub fn new(min_count: isize) -> Self {
        MinCount {
            min_count: min_count as usize,
        }
    }

    pub fn min_count(&self) -> usize {
        self.min_count
    }
}

/// sh:and specifies the condition that each value node conforms to all provided
/// shapes. This is comparable to conjunction and the logical "and" operator.
///
/// https://www.w3.org/TR/shacl/#AndConstraintComponent
#[derive(Debug)]
pub struct And<R: Rdf> {
    shapes: Vec<CompiledShape<R>>,
}

impl<R: Rdf> And<R> {
    pub fn new(shapes: Vec<CompiledShape<R>>) -> Self {
        And { shapes }
    }

    pub fn shapes(&self) -> &Vec<CompiledShape<R>> {
        &self.shapes
    }
}

/// sh:not specifies the condition that each value node cannot conform to a
/// given shape. This is comparable to negation and the logical "not" operator.
///
/// https://www.w3.org/TR/shacl/#NotConstraintComponent
#[derive(Debug)]
pub struct Not<R: Rdf> {
    shape: CompiledShape<R>,
}

impl<R: Rdf> Not<R> {
    pub fn new(shape: CompiledShape<R>) -> Self {
        Not { shape }
    }

    pub fn shape(&self) -> &CompiledShape<R> {
        &self.shape
    }
}

/// sh:or specifies the condition that each value node conforms to at least one
/// of the provided shapes. This is comparable to disjunction and the logical
/// "or" operator.
///
/// https://www.w3.org/TR/shacl/#AndConstraintComponent

#[derive(Debug)]
pub struct Or<R: Rdf> {
    shapes: Vec<CompiledShape<R>>,
}

impl<R: Rdf> Or<R> {
    pub fn new(shapes: Vec<CompiledShape<R>>) -> Self {
        Or { shapes }
    }

    pub fn shapes(&self) -> &Vec<CompiledShape<R>> {
        &self.shapes
    }
}

/// sh:or specifies the condition that each value node conforms to at least one
/// of the provided shapes. This is comparable to disjunction and the logical
/// "or" operator.
///
/// https://www.w3.org/TR/shacl/#XoneConstraintComponent
#[derive(Debug)]
pub struct Xone<R: Rdf> {
    shapes: Vec<CompiledShape<R>>,
}

impl<R: Rdf> Xone<R> {
    pub fn new(shapes: Vec<CompiledShape<R>>) -> Self {
        Xone { shapes }
    }

    pub fn shapes(&self) -> &Vec<CompiledShape<R>> {
        &self.shapes
    }
}

/// Closed Constraint Component.
///
/// The RDF data model offers a huge amount of flexibility. Any node can in
/// principle have values for any property. However, in some cases it makes
/// sense to specify conditions on which properties can be applied to nodes.
/// The SHACL Core language includes a property called sh:closed that can be
/// used to specify the condition that each value node has values only for
/// those properties that have been explicitly enumerated via the property
/// shapes specified for the shape via sh:property.
///
/// https://www.w3.org/TR/shacl/#ClosedConstraintComponent
#[derive(Debug)]
pub struct Closed<R: Rdf> {
    is_closed: bool,
    ignored_properties: Vec<Predicate<R>>,
}

impl<R: Rdf> Closed<R> {
    pub fn new(is_closed: bool, ignored_properties: Vec<Predicate<R>>) -> Self {
        Closed {
            is_closed,
            ignored_properties,
        }
    }

    pub fn is_closed(&self) -> bool {
        self.is_closed
    }

    pub fn ignored_properties(&self) -> &Vec<Predicate<R>> {
        &self.ignored_properties
    }
}

/// sh:hasValue specifies the condition that at least one value node is equal to
///  the given RDF term.
///
/// https://www.w3.org/TR/shacl/#HasValueConstraintComponent
#[derive(Debug)]
pub struct HasValue<R: Rdf> {
    value: Object<R>,
}

impl<R: Rdf> HasValue<R> {
    pub fn new(value: Object<R>) -> Self {
        HasValue { value }
    }

    pub fn value(&self) -> &Object<R> {
        &self.value
    }
}

/// sh:in specifies the condition that each value node is a member of a provided
/// SHACL list.
///
/// https://www.w3.org/TR/shacl/#InConstraintComponent
#[derive(Debug)]
pub struct In<R: Rdf> {
    values: Vec<Object<R>>,
}

impl<R: Rdf> In<R> {
    pub fn new(values: Vec<Object<R>>) -> Self {
        In { values }
    }

    pub fn values(&self) -> &Vec<Object<R>> {
        &self.values
    }
}

/// sh:disjoint specifies the condition that the set of value nodes is disjoint
/// with the set of objects of the triples that have the focus node as subject
/// and the value of sh:disjoint as predicate.
///
/// https://www.w3.org/TR/shacl/#DisjointConstraintComponent
#[derive(Debug)]
pub struct Disjoint<R: Rdf> {
    iri_ref: Predicate<R>,
}

impl<R: Rdf> Disjoint<R> {
    pub fn new(iri_ref: Predicate<R>) -> Self {
        Disjoint { iri_ref }
    }

    pub fn iri_ref(&self) -> &Predicate<R> {
        &self.iri_ref
    }
}

/// sh:equals specifies the condition that the set of all value nodes is equal
/// to the set of objects of the triples that have the focus node as subject and
/// the value of sh:equals as predicate.
///
/// https://www.w3.org/TR/shacl/#EqualsConstraintComponent
#[derive(Debug)]
pub struct Equals<R: Rdf> {
    iri_ref: Predicate<R>,
}

impl<R: Rdf> Equals<R> {
    pub fn new(iri_ref: Predicate<R>) -> Self {
        Equals { iri_ref }
    }

    pub fn iri_ref(&self) -> &Predicate<R> {
        &self.iri_ref
    }
}

/// LessThanOrEquals Constraint Component.
///
/// sh:lessThanOrEquals specifies the condition that each value node is smaller
/// than or equal to all the objects of the triples that have the focus node
/// as subject and the value of sh:lessThanOrEquals as predicate.
///
/// https://www.w3.org/TR/shacl/#LessThanOrEqualsConstraintComponent
#[derive(Debug)]
pub struct LessThanOrEquals<R: Rdf> {
    iri_ref: Predicate<R>,
}

impl<R: Rdf> LessThanOrEquals<R> {
    pub fn new(iri_ref: Predicate<R>) -> Self {
        LessThanOrEquals { iri_ref }
    }

    pub fn iri_ref(&self) -> &Predicate<R> {
        &self.iri_ref
    }
}

/// sh:lessThan specifies the condition that each value node is smaller than all
/// the objects of the triples that have the focus node as subject and the
/// value of sh:lessThan as predicate.
///
/// https://www.w3.org/TR/shacl/#LessThanConstraintComponent
#[derive(Debug)]
pub struct LessThan<R: Rdf> {
    iri_ref: Predicate<R>,
}

impl<R: Rdf> LessThan<R> {
    pub fn new(iri_ref: Predicate<R>) -> Self {
        LessThan { iri_ref }
    }

    pub fn iri_ref(&self) -> &Predicate<R> {
        &self.iri_ref
    }
}

/// sh:node specifies the condition that each value node conforms to the given
/// node shape.
///
/// https://www.w3.org/TR/shacl/#NodeShapeComponent
#[derive(Debug)]
pub struct Node<R: Rdf> {
    shape: CompiledShape<R>,
}

impl<R: Rdf> Node<R> {
    pub fn new(shape: CompiledShape<R>) -> Self {
        Node { shape }
    }

    pub fn shape(&self) -> &CompiledShape<R> {
        &self.shape
    }
}

/// QualifiedValueShape Constraint Component.
///
/// sh:qualifiedValueShape specifies the condition that a specified number of
///  value nodes conforms to the given shape. Each sh:qualifiedValueShape can
///  have: one value for sh:qualifiedMinCount, one value for
///  sh:qualifiedMaxCount or, one value for each, at the same subject.
///
/// https://www.w3.org/TR/shacl/#QualifiedValueShapeConstraintComponent
#[derive(Debug)]
pub struct QualifiedValueShape<R: Rdf> {
    shape: CompiledShape<R>,
    qualified_min_count: Option<isize>,
    qualified_max_count: Option<isize>,
    qualified_value_shapes_disjoint: Option<bool>,
}

impl<R: Rdf> QualifiedValueShape<R> {
    pub fn new(
        shape: CompiledShape<R>,
        qualified_min_count: Option<isize>,
        qualified_max_count: Option<isize>,
        qualified_value_shapes_disjoint: Option<bool>,
    ) -> Self {
        QualifiedValueShape {
            shape,
            qualified_min_count,
            qualified_max_count,
            qualified_value_shapes_disjoint,
        }
    }

    pub fn shape(&self) -> &CompiledShape<R> {
        &self.shape
    }

    pub fn qualified_min_count(&self) -> Option<isize> {
        self.qualified_min_count
    }

    pub fn qualified_max_count(&self) -> Option<isize> {
        self.qualified_max_count
    }

    pub fn qualified_value_shapes_disjoint(&self) -> Option<bool> {
        self.qualified_value_shapes_disjoint
    }
}

/// The condition specified by sh:languageIn is that the allowed language tags
/// for each value node are limited by a given list of language tags.
///
/// https://www.w3.org/TR/shacl/#LanguageInConstraintComponent
#[derive(Debug)]
pub struct LanguageIn<R: Rdf> {
    langs: Vec<Literal<R::Triple>>,
}

impl<R: Rdf> LanguageIn<R> {
    pub fn new(langs: Vec<Literal<R::Triple>>) -> Self {
        LanguageIn { langs }
    }

    pub fn langs(&self) -> &Vec<Literal<R::Triple>> {
        &self.langs
    }
}

/// sh:maxLength specifies the maximum string length of each value node that
/// satisfies the condition. This can be applied to any literals and IRIs, but
/// not to blank nodes.
///
/// https://www.w3.org/TR/shacl/#MaxLengthConstraintComponent
#[derive(Debug)]
pub struct MaxLength {
    max_length: isize,
}

impl MaxLength {
    pub fn new(max_length: isize) -> Self {
        MaxLength { max_length }
    }

    pub fn max_length(&self) -> isize {
        self.max_length
    }
}

/// sh:minLength specifies the minimum string length of each value node that
/// satisfies the condition. This can be applied to any literals and IRIs, but
/// not to blank nodes.
///
/// https://www.w3.org/TR/shacl/#MinLengthConstraintComponent
#[derive(Debug)]
pub struct MinLength {
    min_length: isize,
}

impl MinLength {
    pub fn new(min_length: isize) -> Self {
        MinLength { min_length }
    }

    pub fn min_length(&self) -> isize {
        self.min_length
    }
}

/// sh:property can be used to specify that each value node has a given property
/// shape.
///
/// https://www.w3.org/TR/shacl/#PropertyShapeComponent
#[derive(Debug)]
pub struct Pattern {
    pattern: String,
    flags: Option<String>,
}

impl Pattern {
    pub fn new(pattern: String, flags: Option<String>) -> Self {
        Pattern { pattern, flags }
    }

    pub fn pattern(&self) -> &String {
        &self.pattern
    }

    pub fn flags(&self) -> &Option<String> {
        &self.flags
    }
}

/// The property sh:uniqueLang can be set to true to specify that no pair of
///  value nodes may use the same language tag.
///
/// https://www.w3.org/TR/shacl/#UniqueLangConstraintComponent
#[derive(Debug)]
pub struct UniqueLang {
    unique_lang: bool,
}

impl UniqueLang {
    pub fn new(unique_lang: bool) -> Self {
        UniqueLang { unique_lang }
    }

    pub fn unique_lang(&self) -> bool {
        self.unique_lang
    }
}

/// The condition specified by sh:class is that each value node is a SHACL
/// instance of a given type.
///
/// https://www.w3.org/TR/shacl/#ClassConstraintComponent
#[derive(Debug)]
pub struct Class<R: Rdf> {
    class_rule: Object<R>,
}

impl<R: Rdf> Class<R> {
    pub fn new(class_rule: Object<R>) -> Self {
        Class { class_rule }
    }

    pub fn class_rule(&self) -> &Object<R> {
        &self.class_rule
    }
}

/// sh:datatype specifies a condition to be satisfied with regards to the
/// datatype of each value node.
///
/// https://www.w3.org/TR/shacl/#ClassConstraintComponent
#[derive(Debug)]
pub struct Datatype<R: Rdf> {
    datatype: Predicate<R>,
}

impl<R: Rdf> Datatype<R> {
    pub fn new(datatype: Predicate<R>) -> Self {
        Datatype { datatype }
    }

    pub fn datatype(&self) -> &Predicate<R> {
        &self.datatype
    }
}

/// sh:nodeKind specifies a condition to be satisfied by the RDF node kind of
/// each value node.
///
/// https://www.w3.org/TR/shacl/#NodeKindConstraintComponent
#[derive(Debug)]
pub struct Nodekind {
    node_kind: NodeKind,
}

impl Nodekind {
    pub fn new(node_kind: NodeKind) -> Self {
        Nodekind { node_kind }
    }

    pub fn node_kind(&self) -> &NodeKind {
        &self.node_kind
    }
}

/// https://www.w3.org/TR/shacl/#MaxExclusiveConstraintComponent
#[derive(Debug)]
pub struct MaxExclusive<R: Rdf> {
    max_exclusive: Object<R>,
}

impl<R: Rdf> MaxExclusive<R> {
    pub fn new(literal: Object<R>) -> Self {
        MaxExclusive {
            max_exclusive: literal,
        }
    }

    pub fn max_exclusive(&self) -> &Object<R> {
        &self.max_exclusive
    }
}

/// https://www.w3.org/TR/shacl/#MaxInclusiveConstraintComponent
#[derive(Debug)]
pub struct MaxInclusive<R: Rdf> {
    max_inclusive: Object<R>,
}

impl<R: Rdf> MaxInclusive<R> {
    pub fn new(literal: Object<R>) -> Self {
        MaxInclusive {
            max_inclusive: literal,
        }
    }

    pub fn max_inclusive(&self) -> &Object<R> {
        &self.max_inclusive
    }
}

/// https://www.w3.org/TR/shacl/#MinExclusiveConstraintComponent
#[derive(Debug)]
pub struct MinExclusive<R: Rdf> {
    min_exclusive: Object<R>,
}

impl<R: Rdf> MinExclusive<R> {
    pub fn new(literal: Object<R>) -> Self {
        MinExclusive {
            min_exclusive: literal,
        }
    }

    pub fn min_exclusive(&self) -> &Object<R> {
        &self.min_exclusive
    }
}

/// https://www.w3.org/TR/shacl/#MinInclusiveConstraintComponent
#[derive(Debug)]
pub struct MinInclusive<R: Rdf> {
    min_inclusive: Object<R>,
}

impl<R: Rdf> MinInclusive<R> {
    pub fn new(literal: Object<R>) -> Self {
        MinInclusive {
            min_inclusive: literal,
        }
    }

    pub fn min_inclusive(&self) -> &Object<R> {
        &self.min_inclusive
    }
}

impl<R: Rdf> From<CompiledComponent<R>> for IriS {
    fn from(value: CompiledComponent<R>) -> Self {
        match value {
            CompiledComponent::Class(_) => IriS::from_str(SH_CLASS_STR),
            CompiledComponent::Datatype(_) => IriS::from_str(SH_DATATYPE_STR),
            CompiledComponent::NodeKind(_) => IriS::from_str(SH_IRI_STR),
            CompiledComponent::MinCount(_) => IriS::from_str(SH_MIN_COUNT_STR),
            CompiledComponent::MaxCount(_) => IriS::from_str(SH_MAX_COUNT_STR),
            CompiledComponent::MinExclusive(_) => IriS::from_str(SH_MIN_EXCLUSIVE_STR),
            CompiledComponent::MaxExclusive(_) => IriS::from_str(SH_MAX_EXCLUSIVE_STR),
            CompiledComponent::MinInclusive(_) => IriS::from_str(SH_MIN_INCLUSIVE_STR),
            CompiledComponent::MaxInclusive(_) => IriS::from_str(SH_MAX_INCLUSIVE_STR),
            CompiledComponent::MinLength(_) => IriS::from_str(SH_MIN_LENGTH_STR),
            CompiledComponent::MaxLength(_) => IriS::from_str(SH_MAX_LENGTH_STR),
            CompiledComponent::Pattern { .. } => IriS::from_str(SH_PATTERN_STR),
            CompiledComponent::UniqueLang(_) => IriS::from_str(SH_UNIQUE_LANG_STR),
            CompiledComponent::LanguageIn { .. } => IriS::from_str(SH_LANGUAGE_IN_STR),
            CompiledComponent::Equals(_) => IriS::from_str(SH_EQUALS_STR),
            CompiledComponent::Disjoint(_) => IriS::from_str(SH_DISJOINT_STR),
            CompiledComponent::LessThan(_) => IriS::from_str(SH_LESS_THAN_STR),
            CompiledComponent::LessThanOrEquals(_) => IriS::from_str(SH_LESS_THAN_OR_EQUALS_STR),
            CompiledComponent::Or { .. } => IriS::from_str(SH_OR_STR),
            CompiledComponent::And { .. } => IriS::from_str(SH_AND_STR),
            CompiledComponent::Not { .. } => IriS::from_str(SH_NOT_STR),
            CompiledComponent::Xone { .. } => IriS::from_str(SH_XONE_STR),
            CompiledComponent::Closed { .. } => IriS::from_str(SH_CLOSED_STR),
            CompiledComponent::Node { .. } => IriS::from_str(SH_NODE_STR),
            CompiledComponent::HasValue { .. } => IriS::from_str(SH_HAS_VALUE_STR),
            CompiledComponent::In { .. } => IriS::from_str(SH_IN_STR),
            CompiledComponent::QualifiedValueShape { .. } => {
                IriS::from_str(SH_QUALIFIED_VALUE_SHAPE_STR)
            }
        }
        .unwrap()
    }
}
