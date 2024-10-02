use crate::component::Component;
use crate::node_kind::NodeKind;
use crate::Schema;

use super::compile_shape;
use super::compile_shapes;
use super::compiled_shacl_error::CompiledShaclError;
use super::convert_iri_ref;
use super::convert_lang;
use super::convert_value;
use super::shape::CompiledShape;
use srdf::RDFNode;
use srdf::SRDFBasic;

#[derive(Hash, PartialEq, Eq)]
pub enum CompiledComponent<S: SRDFBasic> {
    Class(Class<S>),
    Datatype(Datatype<S>),
    NodeKind(Nodekind),
    MinCount(MinCount),
    MaxCount(MaxCount),
    MinExclusive(MinExclusive<S>),
    MaxExclusive(MaxExclusive<S>),
    MinInclusive(MinInclusive<S>),
    MaxInclusive(MaxInclusive<S>),
    MinLength(MinLength),
    MaxLength(MaxLength),
    Pattern(Pattern),
    UniqueLang(UniqueLang),
    LanguageIn(LanguageIn<S>),
    Equals(Equals<S>),
    Disjoint(Disjoint<S>),
    LessThan(LessThan<S>),
    LessThanOrEquals(LessThanOrEquals<S>),
    Or(Or<S>),
    And(And<S>),
    Not(Not<S>),
    Xone(Xone<S>),
    Closed(Closed<S>),
    Node(Node<S>),
    HasValue(HasValue<S>),
    In(In<S>),
    QualifiedValueShape(QualifiedValueShape<S>),
}

impl<S: SRDFBasic> CompiledComponent<S> {
    pub fn compile(component: Component, schema: &Schema) -> Result<Self, CompiledShaclError> {
        let component = match component {
            Component::Class(object) => {
                let class_rule = S::object_as_term(&object);
                CompiledComponent::Class(Class::new(class_rule))
            }
            Component::Datatype(iri_ref) => {
                let iri_ref = convert_iri_ref::<S>(iri_ref)?;
                CompiledComponent::Datatype(Datatype::new(iri_ref))
            }
            Component::NodeKind(node_kind) => CompiledComponent::NodeKind(Nodekind::new(node_kind)),
            Component::MinCount(count) => CompiledComponent::MinCount(MinCount::new(count)),
            Component::MaxCount(count) => CompiledComponent::MaxCount(MaxCount::new(count)),
            Component::MinExclusive(literal) => {
                let term = S::object_as_term(&RDFNode::literal(literal));
                CompiledComponent::MinExclusive(MinExclusive::new(term))
            }
            Component::MaxExclusive(literal) => {
                let term = S::object_as_term(&RDFNode::literal(literal));
                CompiledComponent::MaxExclusive(MaxExclusive::new(term))
            }
            Component::MinInclusive(literal) => {
                let term = S::object_as_term(&RDFNode::literal(literal));
                CompiledComponent::MinInclusive(MinInclusive::new(term))
            }
            Component::MaxInclusive(literal) => {
                let term = S::object_as_term(&RDFNode::literal(literal));
                CompiledComponent::MaxInclusive(MaxInclusive::new(term))
            }
            Component::MinLength(length) => CompiledComponent::MinLength(MinLength::new(length)),
            Component::MaxLength(length) => CompiledComponent::MaxLength(MaxLength::new(length)),
            Component::Pattern { pattern, flags } => {
                CompiledComponent::Pattern(Pattern::new(pattern, flags))
            }
            Component::UniqueLang(lang) => CompiledComponent::UniqueLang(UniqueLang::new(lang)),
            Component::LanguageIn { langs } => {
                let literals = langs
                    .into_iter()
                    .map(|lang| convert_lang::<S>(lang))
                    .collect::<Result<Vec<_>, _>>()?;
                CompiledComponent::LanguageIn(LanguageIn::new(literals))
            }
            Component::Equals(iri_ref) => {
                let iri_ref = convert_iri_ref::<S>(iri_ref)?;
                CompiledComponent::Equals(Equals::new(iri_ref))
            }
            Component::Disjoint(iri_ref) => {
                let iri_ref = convert_iri_ref::<S>(iri_ref)?;
                CompiledComponent::Disjoint(Disjoint::new(iri_ref))
            }
            Component::LessThan(iri_ref) => {
                let iri_ref = convert_iri_ref::<S>(iri_ref)?;
                CompiledComponent::LessThan(LessThan::new(iri_ref))
            }
            Component::LessThanOrEquals(iri_ref) => {
                let iri_ref = convert_iri_ref::<S>(iri_ref)?;
                CompiledComponent::LessThanOrEquals(LessThanOrEquals::new(iri_ref))
            }
            Component::Or { shapes } => {
                CompiledComponent::Or(Or::new(compile_shapes::<S>(shapes, schema)?))
            }
            Component::And { shapes } => {
                CompiledComponent::And(And::new(compile_shapes::<S>(shapes, schema)?))
            }
            Component::Not { shape } => {
                let shape = compile_shape::<S>(shape, schema)?;
                CompiledComponent::Not(Not::new(shape))
            }
            Component::Xone { shapes } => {
                CompiledComponent::Xone(Xone::new(compile_shapes::<S>(shapes, schema)?))
            }
            Component::Closed {
                is_closed,
                ignored_properties,
            } => {
                let properties = ignored_properties
                    .into_iter()
                    .map(|prop| convert_iri_ref::<S>(prop))
                    .collect::<Result<Vec<_>, _>>()?;
                CompiledComponent::Closed(Closed::new(is_closed, properties))
            }
            Component::Node { shape } => {
                let shape = compile_shape::<S>(shape, schema)?;
                CompiledComponent::Node(Node::new(shape))
            }
            Component::HasValue { value } => {
                let term = convert_value::<S>(value)?;
                CompiledComponent::HasValue(HasValue::new(term))
            }
            Component::In { values } => {
                let terms = values
                    .into_iter()
                    .map(|value| convert_value::<S>(value))
                    .collect::<Result<Vec<_>, _>>()?;
                CompiledComponent::In(In::new(terms))
            }
            Component::QualifiedValueShape {
                shape,
                qualified_min_count,
                qualified_max_count,
                qualified_value_shapes_disjoint,
            } => {
                let shape = compile_shape::<S>(shape, schema)?;
                CompiledComponent::QualifiedValueShape(QualifiedValueShape::new(
                    shape,
                    qualified_min_count,
                    qualified_max_count,
                    qualified_value_shapes_disjoint,
                ))
            }
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
#[derive(Hash, PartialEq, Eq)]
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
#[derive(Hash, PartialEq, Eq)]
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
#[derive(Hash, PartialEq, Eq)]
pub struct And<S: SRDFBasic> {
    shapes: Vec<CompiledShape<S>>,
}

impl<S: SRDFBasic> And<S> {
    pub fn new(shapes: Vec<CompiledShape<S>>) -> Self {
        And { shapes }
    }

    pub fn shapes(&self) -> &Vec<CompiledShape<S>> {
        &self.shapes
    }
}

/// sh:not specifies the condition that each value node cannot conform to a
/// given shape. This is comparable to negation and the logical "not" operator.
///
/// https://www.w3.org/TR/shacl/#NotConstraintComponent
#[derive(Hash, PartialEq, Eq)]
pub struct Not<S: SRDFBasic> {
    shape: CompiledShape<S>,
}

impl<S: SRDFBasic> Not<S> {
    pub fn new(shape: CompiledShape<S>) -> Self {
        Not { shape }
    }

    pub fn shape(&self) -> &CompiledShape<S> {
        &self.shape
    }
}

/// sh:or specifies the condition that each value node conforms to at least one
/// of the provided shapes. This is comparable to disjunction and the logical
/// "or" operator.
///
/// https://www.w3.org/TR/shacl/#AndConstraintComponent
#[derive(Hash, PartialEq, Eq)]
pub struct Or<S: SRDFBasic> {
    shapes: Vec<CompiledShape<S>>,
}

impl<S: SRDFBasic> Or<S> {
    pub fn new(shapes: Vec<CompiledShape<S>>) -> Self {
        Or { shapes }
    }

    pub fn shapes(&self) -> &Vec<CompiledShape<S>> {
        &self.shapes
    }
}

/// sh:or specifies the condition that each value node conforms to at least one
/// of the provided shapes. This is comparable to disjunction and the logical
/// "or" operator.
///
/// https://www.w3.org/TR/shacl/#XoneConstraintComponent
#[derive(Hash, PartialEq, Eq)]
pub struct Xone<S: SRDFBasic> {
    shapes: Vec<CompiledShape<S>>,
}

impl<S: SRDFBasic> Xone<S> {
    pub fn new(shapes: Vec<CompiledShape<S>>) -> Self {
        Xone { shapes }
    }

    pub fn shapes(&self) -> &Vec<CompiledShape<S>> {
        &self.shapes
    }
}

/// The RDF data model offers a huge amount of flexibility. Any node can in
/// principle have values for any property. However, in some cases it makes
/// sense to specify conditions on which properties can be applied to nodes.
/// The SHACL Core language includes a property called sh:closed that can be
/// used to specify the condition that each value node has values only for
/// those properties that have been explicitly enumerated via the property
/// shapes specified for the shape via sh:property.
///
/// https://www.w3.org/TR/shacl/#InConstraintComponent
#[derive(Hash, PartialEq, Eq)]
pub struct Closed<S: SRDFBasic> {
    is_closed: bool,
    ignored_properties: Vec<S::IRI>,
}

impl<S: SRDFBasic> Closed<S> {
    pub fn new(is_closed: bool, ignored_properties: Vec<S::IRI>) -> Self {
        Closed {
            is_closed,
            ignored_properties,
        }
    }

    pub fn is_closed(&self) -> bool {
        self.is_closed
    }

    pub fn ignored_properties(&self) -> &Vec<S::IRI> {
        &self.ignored_properties
    }
}

/// sh:hasValue specifies the condition that at least one value node is equal to
///  the given RDF term.
///
/// https://www.w3.org/TR/shacl/#HasValueConstraintComponent
#[derive(Hash, PartialEq, Eq)]
pub struct HasValue<S: SRDFBasic> {
    value: S::Term,
}

impl<S: SRDFBasic> HasValue<S> {
    pub fn new(value: S::Term) -> Self {
        HasValue { value }
    }

    pub fn value(&self) -> &S::Term {
        &self.value
    }
}

/// sh:in specifies the condition that each value node is a member of a provided
/// SHACL list.
///
/// https://www.w3.org/TR/shacl/#InConstraintComponent
#[derive(Hash, PartialEq, Eq)]
pub struct In<S: SRDFBasic> {
    values: Vec<S::Term>,
}

impl<S: SRDFBasic> In<S> {
    pub fn new(values: Vec<S::Term>) -> Self {
        In { values }
    }

    pub fn values(&self) -> &Vec<S::Term> {
        &self.values
    }
}

/// sh:disjoint specifies the condition that the set of value nodes is disjoint
/// with the set of objects of the triples that have the focus node as subject
/// and the value of sh:disjoint as predicate.
///
/// https://www.w3.org/TR/shacl/#DisjointConstraintComponent
#[derive(Hash, PartialEq, Eq)]
pub struct Disjoint<S: SRDFBasic> {
    iri_ref: S::IRI,
}

impl<S: SRDFBasic> Disjoint<S> {
    pub fn new(iri_ref: S::IRI) -> Self {
        Disjoint { iri_ref }
    }

    pub fn iri_ref(&self) -> &S::IRI {
        &self.iri_ref
    }
}

/// sh:equals specifies the condition that the set of all value nodes is equal
/// to the set of objects of the triples that have the focus node as subject and
/// the value of sh:equals as predicate.
///
/// https://www.w3.org/TR/shacl/#EqualsConstraintComponent
#[derive(Hash, PartialEq, Eq)]
pub struct Equals<S: SRDFBasic> {
    iri_ref: S::IRI,
}

impl<S: SRDFBasic> Equals<S> {
    pub fn new(iri_ref: S::IRI) -> Self {
        Equals { iri_ref }
    }

    pub fn iri_ref(&self) -> &S::IRI {
        &self.iri_ref
    }
}

/// sh:lessThanOrEquals specifies the condition that each value node is smaller
/// than or equal to all the objects of the triples that have the focus node
/// as subject and the value of sh:lessThanOrEquals as predicate.
///
/// https://www.w3.org/TR/shacl/#LessThanOrEqualsConstraintComponent
#[derive(Hash, PartialEq, Eq)]
pub struct LessThanOrEquals<S: SRDFBasic> {
    iri_ref: S::IRI,
}

impl<S: SRDFBasic> LessThanOrEquals<S> {
    pub fn new(iri_ref: S::IRI) -> Self {
        LessThanOrEquals { iri_ref }
    }

    pub fn iri_ref(&self) -> &S::IRI {
        &self.iri_ref
    }
}

/// sh:lessThan specifies the condition that each value node is smaller than all
/// the objects of the triples that have the focus node as subject and the
/// value of sh:lessThan as predicate.
///
/// https://www.w3.org/TR/shacl/#LessThanConstraintComponent
#[derive(Hash, PartialEq, Eq)]
pub struct LessThan<S: SRDFBasic> {
    iri_ref: S::IRI,
}

impl<S: SRDFBasic> LessThan<S> {
    pub fn new(iri_ref: S::IRI) -> Self {
        LessThan { iri_ref }
    }

    pub fn iri_ref(&self) -> &S::IRI {
        &self.iri_ref
    }
}

/// sh:node specifies the condition that each value node conforms to the given
/// node shape.
///
/// https://www.w3.org/TR/shacl/#NodeShapeComponent
#[derive(Hash, PartialEq, Eq)]
pub struct Node<S: SRDFBasic> {
    shape: CompiledShape<S>,
}

impl<S: SRDFBasic> Node<S> {
    pub fn new(shape: CompiledShape<S>) -> Self {
        Node { shape }
    }

    pub fn shape(&self) -> &CompiledShape<S> {
        &self.shape
    }
}

/// sh:qualifiedValueShape specifies the condition that a specified number of
///  value nodes conforms to the given shape. Each sh:qualifiedValueShape can
///  have: one value for sh:qualifiedMinCount, one value for
///  sh:qualifiedMaxCount or, one value for each, at the same subject.
///
/// https://www.w3.org/TR/shacl/#QualifiedValueShapeConstraintComponent
#[derive(Hash, PartialEq, Eq)]
pub struct QualifiedValueShape<S: SRDFBasic> {
    shape: CompiledShape<S>,
    qualified_min_count: Option<isize>,
    qualified_max_count: Option<isize>,
    qualified_value_shapes_disjoint: Option<bool>,
}

impl<S: SRDFBasic> QualifiedValueShape<S> {
    pub fn new(
        shape: CompiledShape<S>,
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

    pub fn shape(&self) -> &CompiledShape<S> {
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
#[derive(Hash, PartialEq, Eq)]
pub struct LanguageIn<S: SRDFBasic> {
    langs: Vec<S::Literal>,
}

impl<S: SRDFBasic> LanguageIn<S> {
    pub fn new(langs: Vec<S::Literal>) -> Self {
        LanguageIn { langs }
    }

    pub fn langs(&self) -> &Vec<S::Literal> {
        &self.langs
    }
}

/// sh:maxLength specifies the maximum string length of each value node that
/// satisfies the condition. This can be applied to any literals and IRIs, but
/// not to blank nodes.
///
/// https://www.w3.org/TR/shacl/#MaxLengthConstraintComponent
#[derive(Hash, PartialEq, Eq)]
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
#[derive(Hash, PartialEq, Eq)]
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
#[derive(Hash, PartialEq, Eq)]
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
#[derive(Hash, PartialEq, Eq)]
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
#[derive(Hash, PartialEq, Eq)]
pub struct Class<S: SRDFBasic> {
    class_rule: S::Term,
}

impl<S: SRDFBasic> Class<S> {
    pub fn new(class_rule: S::Term) -> Self {
        Class { class_rule }
    }

    pub fn class_rule(&self) -> &S::Term {
        &self.class_rule
    }
}

/// sh:datatype specifies a condition to be satisfied with regards to the
/// datatype of each value node.
///
/// https://www.w3.org/TR/shacl/#ClassConstraintComponent
#[derive(Hash, PartialEq, Eq)]
pub struct Datatype<S: SRDFBasic> {
    datatype: S::IRI,
}

impl<S: SRDFBasic> Datatype<S> {
    pub fn new(datatype: S::IRI) -> Self {
        Datatype { datatype }
    }

    pub fn datatype(&self) -> &S::IRI {
        &self.datatype
    }
}

/// sh:nodeKind specifies a condition to be satisfied by the RDF node kind of
/// each value node.
///
/// https://www.w3.org/TR/shacl/#NodeKindConstraintComponent
#[derive(Hash, PartialEq, Eq)]
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
#[derive(Hash, PartialEq, Eq)]
pub struct MaxExclusive<S: SRDFBasic> {
    max_exclusive: S::Term,
}

impl<S: SRDFBasic> MaxExclusive<S> {
    pub fn new(literal: S::Term) -> Self {
        MaxExclusive {
            max_exclusive: literal,
        }
    }

    pub fn max_exclusive(&self) -> &S::Term {
        &self.max_exclusive
    }
}

/// https://www.w3.org/TR/shacl/#MaxInclusiveConstraintComponent
#[derive(Hash, PartialEq, Eq)]
pub struct MaxInclusive<S: SRDFBasic> {
    max_inclusive: S::Term,
}

impl<S: SRDFBasic> MaxInclusive<S> {
    pub fn new(literal: S::Term) -> Self {
        MaxInclusive {
            max_inclusive: literal,
        }
    }

    pub fn max_inclusive(&self) -> &S::Term {
        &self.max_inclusive
    }
}

/// https://www.w3.org/TR/shacl/#MinExclusiveConstraintComponent
#[derive(Hash, PartialEq, Eq)]
pub struct MinExclusive<S: SRDFBasic> {
    min_exclusive: S::Term,
}

impl<S: SRDFBasic> MinExclusive<S> {
    pub fn new(literal: S::Term) -> Self {
        MinExclusive {
            min_exclusive: literal,
        }
    }

    pub fn min_exclusive(&self) -> &S::Term {
        &self.min_exclusive
    }
}

/// https://www.w3.org/TR/shacl/#MinInclusiveConstraintComponent
#[derive(Hash, PartialEq, Eq)]
pub struct MinInclusive<S: SRDFBasic> {
    min_inclusive: S::Term,
}

impl<S: SRDFBasic> MinInclusive<S> {
    pub fn new(literal: S::Term) -> Self {
        MinInclusive {
            min_inclusive: literal,
        }
    }

    pub fn min_inclusive(&self) -> &S::Term {
        &self.min_inclusive
    }
}
