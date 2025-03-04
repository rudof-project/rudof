use iri_s::IriS;
use node_kind::NodeKind;
use srdf::lang::Lang;
use srdf::Rdf;

use crate::component::Component;
use crate::Schema;
use crate::*;

use super::compile_shape;
use super::compile_shapes;
use super::compiled_shacl_error::CompiledShaclError;
use super::convert_iri_ref;
use super::convert_value;
use super::shape::CompiledShape;

#[derive(Clone, Debug)]
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
    LanguageIn(LanguageIn),
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
    pub fn compile(component: Component, schema: &Schema) -> Result<Self, CompiledShaclError> {
        let component = match component {
            Component::Class(object) => {
                let class_rule = object.into();
                CompiledComponent::Class(Class::new(class_rule))
            }
            Component::Datatype(iri_ref) => {
                let iri_ref = convert_iri_ref::<R>(iri_ref)?;
                CompiledComponent::Datatype(Datatype::new(iri_ref))
            }
            Component::NodeKind(node_kind) => CompiledComponent::NodeKind(Nodekind::new(node_kind)),
            Component::MinCount(count) => CompiledComponent::MinCount(MinCount::new(count)),
            Component::MaxCount(count) => CompiledComponent::MaxCount(MaxCount::new(count)),
            Component::MinExclusive(literal) => {
                let literal: R::Literal = literal.clone().into();
                let term = literal.into();
                CompiledComponent::MinExclusive(MinExclusive::new(term))
            }
            Component::MaxExclusive(literal) => {
                let literal: R::Literal = literal.clone().into();
                let term = literal.into();
                CompiledComponent::MaxExclusive(MaxExclusive::new(term))
            }
            Component::MinInclusive(literal) => {
                let literal: R::Literal = literal.clone().into();
                let term = literal.into();
                CompiledComponent::MinInclusive(MinInclusive::new(term))
            }
            Component::MaxInclusive(literal) => {
                let literal: R::Literal = literal.clone().into();
                let term = literal.into();
                CompiledComponent::MaxInclusive(MaxInclusive::new(term))
            }
            Component::MinLength(length) => CompiledComponent::MinLength(MinLength::new(length)),
            Component::MaxLength(length) => CompiledComponent::MaxLength(MaxLength::new(length)),
            Component::Pattern { pattern, flags } => {
                CompiledComponent::Pattern(Pattern::new(pattern, flags))
            }
            Component::UniqueLang(lang) => CompiledComponent::UniqueLang(UniqueLang::new(lang)),
            Component::LanguageIn { langs } => {
                CompiledComponent::LanguageIn(LanguageIn::new(langs))
            }
            Component::Equals(iri_ref) => {
                let iri_ref = convert_iri_ref::<R>(iri_ref)?;
                CompiledComponent::Equals(Equals::new(iri_ref))
            }
            Component::Disjoint(iri_ref) => {
                let iri_ref = convert_iri_ref::<R>(iri_ref)?;
                CompiledComponent::Disjoint(Disjoint::new(iri_ref))
            }
            Component::LessThan(iri_ref) => {
                let iri_ref = convert_iri_ref::<R>(iri_ref)?;
                CompiledComponent::LessThan(LessThan::new(iri_ref))
            }
            Component::LessThanOrEquals(iri_ref) => {
                let iri_ref = convert_iri_ref::<R>(iri_ref)?;
                CompiledComponent::LessThanOrEquals(LessThanOrEquals::new(iri_ref))
            }
            Component::Or { shapes } => {
                CompiledComponent::Or(Or::new(compile_shapes::<R>(shapes, schema)?))
            }
            Component::And { shapes } => {
                CompiledComponent::And(And::new(compile_shapes::<R>(shapes, schema)?))
            }
            Component::Not { shape } => {
                let shape = compile_shape::<R>(shape, schema)?;
                CompiledComponent::Not(Not::new(shape))
            }
            Component::Xone { shapes } => {
                CompiledComponent::Xone(Xone::new(compile_shapes::<R>(shapes, schema)?))
            }
            Component::Closed {
                is_closed,
                ignored_properties,
            } => {
                let properties = ignored_properties
                    .into_iter()
                    .map(|prop| convert_iri_ref::<R>(prop))
                    .collect::<Result<Vec<_>, _>>()?;
                CompiledComponent::Closed(Closed::new(is_closed, properties))
            }
            Component::Node { shape } => {
                let shape = compile_shape::<R>(shape, schema)?;
                CompiledComponent::Node(Node::new(shape))
            }
            Component::HasValue { value } => {
                let term = convert_value::<R>(value)?;
                CompiledComponent::HasValue(HasValue::new(term))
            }
            Component::In { values } => {
                let terms = values
                    .into_iter()
                    .map(|value| convert_value::<R>(value))
                    .collect::<Result<Vec<_>, _>>()?;
                CompiledComponent::In(In::new(terms))
            }
            Component::QualifiedValueShape {
                shape,
                qualified_min_count,
                qualified_max_count,
                qualified_value_shapes_disjoint,
            } => {
                let shape = compile_shape::<R>(shape, schema)?;
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
#[derive(Clone, Debug)]
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
#[derive(Clone, Debug)]
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
#[derive(Clone, Debug)]
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
#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
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
#[derive(Clone, Debug)]
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
#[derive(Clone, Debug)]
pub struct Closed<R: Rdf> {
    is_closed: bool,
    ignored_properties: Vec<R::IRI>,
}

impl<R: Rdf> Closed<R> {
    pub fn new(is_closed: bool, ignored_properties: Vec<R::IRI>) -> Self {
        Closed {
            is_closed,
            ignored_properties,
        }
    }

    pub fn is_closed(&self) -> bool {
        self.is_closed
    }

    pub fn ignored_properties(&self) -> &Vec<R::IRI> {
        &self.ignored_properties
    }
}

/// sh:hasValue specifies the condition that at least one value node is equal to
///  the given RDF term.
///
/// https://www.w3.org/TR/shacl/#HasValueConstraintComponent
#[derive(Clone, Debug)]
pub struct HasValue<R: Rdf> {
    value: R::Term,
}

impl<R: Rdf> HasValue<R> {
    pub fn new(value: R::Term) -> Self {
        HasValue { value }
    }

    pub fn value(&self) -> &R::Term {
        &self.value
    }
}

/// sh:in specifies the condition that each value node is a member of a provided
/// SHACL list.
///
/// https://www.w3.org/TR/shacl/#InConstraintComponent
#[derive(Clone, Debug)]
pub struct In<R: Rdf> {
    values: Vec<R::Term>,
}

impl<R: Rdf> In<R> {
    pub fn new(values: Vec<R::Term>) -> Self {
        In { values }
    }

    pub fn values(&self) -> &Vec<R::Term> {
        &self.values
    }
}

/// sh:disjoint specifies the condition that the set of value nodes is disjoint
/// with the set of objects of the triples that have the focus node as subject
/// and the value of sh:disjoint as predicate.
///
/// https://www.w3.org/TR/shacl/#DisjointConstraintComponent
#[derive(Clone, Debug)]
pub struct Disjoint<R: Rdf> {
    iri_ref: R::IRI,
}

impl<R: Rdf> Disjoint<R> {
    pub fn new(iri_ref: R::IRI) -> Self {
        Disjoint { iri_ref }
    }

    pub fn iri_ref(&self) -> &R::IRI {
        &self.iri_ref
    }
}

/// sh:equals specifies the condition that the set of all value nodes is equal
/// to the set of objects of the triples that have the focus node as subject and
/// the value of sh:equals as predicate.
///
/// https://www.w3.org/TR/shacl/#EqualsConstraintComponent
#[derive(Clone, Debug)]
pub struct Equals<R: Rdf> {
    iri_ref: R::IRI,
}

impl<R: Rdf> Equals<R> {
    pub fn new(iri_ref: R::IRI) -> Self {
        Equals { iri_ref }
    }

    pub fn iri_ref(&self) -> &R::IRI {
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
#[derive(Clone, Debug)]
pub struct LessThanOrEquals<R: Rdf> {
    iri_ref: R::IRI,
}

impl<R: Rdf> LessThanOrEquals<R> {
    pub fn new(iri_ref: R::IRI) -> Self {
        LessThanOrEquals { iri_ref }
    }

    pub fn iri_ref(&self) -> &R::IRI {
        &self.iri_ref
    }
}

/// sh:lessThan specifies the condition that each value node is smaller than all
/// the objects of the triples that have the focus node as subject and the
/// value of sh:lessThan as predicate.
///
/// https://www.w3.org/TR/shacl/#LessThanConstraintComponent
#[derive(Clone, Debug)]
pub struct LessThan<R: Rdf> {
    iri_ref: R::IRI,
}

impl<R: Rdf> LessThan<R> {
    pub fn new(iri_ref: R::IRI) -> Self {
        LessThan { iri_ref }
    }

    pub fn iri_ref(&self) -> &R::IRI {
        &self.iri_ref
    }
}

/// sh:node specifies the condition that each value node conforms to the given
/// node shape.
///
/// https://www.w3.org/TR/shacl/#NodeShapeComponent
#[derive(Clone, Debug)]
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
#[derive(Clone, Debug)]
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
#[derive(Clone, Debug)]
pub struct LanguageIn {
    langs: Vec<Lang>,
}

impl LanguageIn {
    pub fn new(langs: Vec<Lang>) -> Self {
        LanguageIn { langs }
    }

    pub fn langs(&self) -> &Vec<Lang> {
        &self.langs
    }
}

/// sh:maxLength specifies the maximum string length of each value node that
/// satisfies the condition. This can be applied to any literals and IRIs, but
/// not to blank nodes.
///
/// https://www.w3.org/TR/shacl/#MaxLengthConstraintComponent
#[derive(Clone, Debug)]
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
#[derive(Clone, Debug)]
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
#[derive(Clone, Debug)]
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
#[derive(Clone, Debug)]
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
#[derive(Clone, Debug)]
pub struct Class<R: Rdf> {
    class_rule: R::Term,
}

impl<R: Rdf> Class<R> {
    pub fn new(class_rule: R::Term) -> Self {
        Class { class_rule }
    }

    pub fn class_rule(&self) -> &R::Term {
        &self.class_rule
    }
}

/// sh:datatype specifies a condition to be satisfied with regards to the
/// datatype of each value node.
///
/// https://www.w3.org/TR/shacl/#ClassConstraintComponent
#[derive(Clone, Debug)]
pub struct Datatype<R: Rdf> {
    datatype: R::IRI,
}

impl<R: Rdf> Datatype<R> {
    pub fn new(datatype: R::IRI) -> Self {
        Datatype { datatype }
    }

    pub fn datatype(&self) -> &R::IRI {
        &self.datatype
    }
}

/// sh:nodeKind specifies a condition to be satisfied by the RDF node kind of
/// each value node.
///
/// https://www.w3.org/TR/shacl/#NodeKindConstraintComponent
#[derive(Clone, Debug)]
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
#[derive(Clone, Debug)]
pub struct MaxExclusive<R: Rdf> {
    max_exclusive: R::Term,
}

impl<R: Rdf> MaxExclusive<R> {
    pub fn new(literal: R::Term) -> Self {
        MaxExclusive {
            max_exclusive: literal,
        }
    }

    pub fn max_exclusive(&self) -> &R::Term {
        &self.max_exclusive
    }
}

/// https://www.w3.org/TR/shacl/#MaxInclusiveConstraintComponent
#[derive(Clone, Debug)]
pub struct MaxInclusive<R: Rdf> {
    max_inclusive: R::Term,
}

impl<R: Rdf> MaxInclusive<R> {
    pub fn new(literal: R::Term) -> Self {
        MaxInclusive {
            max_inclusive: literal,
        }
    }

    pub fn max_inclusive(&self) -> &R::Term {
        &self.max_inclusive
    }
}

/// https://www.w3.org/TR/shacl/#MinExclusiveConstraintComponent
#[derive(Clone, Debug)]
pub struct MinExclusive<R: Rdf> {
    min_exclusive: R::Term,
}

impl<R: Rdf> MinExclusive<R> {
    pub fn new(literal: R::Term) -> Self {
        MinExclusive {
            min_exclusive: literal,
        }
    }

    pub fn min_exclusive(&self) -> &R::Term {
        &self.min_exclusive
    }
}

/// https://www.w3.org/TR/shacl/#MinInclusiveConstraintComponent
#[derive(Clone, Debug)]
pub struct MinInclusive<R: Rdf> {
    min_inclusive: R::Term,
}

impl<R: Rdf> MinInclusive<R> {
    pub fn new(literal: R::Term) -> Self {
        MinInclusive {
            min_inclusive: literal,
        }
    }

    pub fn min_inclusive(&self) -> &R::Term {
        &self.min_inclusive
    }
}

/// Serialize this into ContraintComponent IriS
impl<R: Rdf> From<&CompiledComponent<R>> for IriS {
    fn from(value: &CompiledComponent<R>) -> Self {
        let iri_str = match value {
            CompiledComponent::Class(_) => SH_CLASS_CONSTRAINT_COMPONENT_STR,
            CompiledComponent::Datatype(_) => SH_DATATYPE_CONSTRAINT_COMPONENT_STR,
            CompiledComponent::NodeKind(_) => SH_NODE_CONSTRAINT_COMPONENT_STR,
            CompiledComponent::MinCount(_) => SH_MIN_COUNT_CONSTRAINT_COMPONENT_STR,
            CompiledComponent::MaxCount(_) => SH_MAX_COUNT_CONSTRAINT_COMPONENT_STR,
            CompiledComponent::MinExclusive(_) => SH_MIN_EXCLUSIVE_CONSTRAINT_COMPONENT_STR,
            CompiledComponent::MaxExclusive(_) => SH_MAX_EXCLUSIVE_CONSTRAINT_COMPONENT_STR,
            CompiledComponent::MinInclusive(_) => SH_MIN_INCLUSIVE_CONSTRAINT_COMPONENT_STR,
            CompiledComponent::MaxInclusive(_) => SH_MAX_INCLUSIVE_CONSTRAINT_COMPONENT_STR,
            CompiledComponent::MinLength(_) => SH_MIN_LENGTH_CONSTRAINT_COMPONENT_STR,
            CompiledComponent::MaxLength(_) => SH_MAX_LENGTH_CONSTRAINT_COMPONENT_STR,
            CompiledComponent::Pattern(_) => SH_PATTERN_CONSTRAINT_COMPONENT_STR,
            CompiledComponent::UniqueLang(_) => SH_UNIQUE_LANG_CONSTRAINT_COMPONENT_STR,
            CompiledComponent::LanguageIn(_) => SH_LANGUAGE_IN_CONSTRAINT_COMPONENT_STR,
            CompiledComponent::Equals(_) => SH_EQUALS_CONSTRAINT_COMPONENT_STR,
            CompiledComponent::Disjoint(_) => SH_DISJOINT_CONSTRAINT_COMPONENT_STR,
            CompiledComponent::LessThan(_) => SH_LESS_THAN_CONSTRAINT_COMPONENT_STR,
            CompiledComponent::LessThanOrEquals(_) => {
                SH_LESS_THAN_OR_EQUALS_CONSTRAINT_COMPONENT_STR
            }
            CompiledComponent::Or(_) => SH_OR_CONSTRAINT_COMPONENT_STR,
            CompiledComponent::And(_) => SH_AND_CONSTRAINT_COMPONENT_STR,
            CompiledComponent::Not(_) => SH_NOT_CONSTRAINT_COMPONENT_STR,
            CompiledComponent::Xone(_) => SH_XONE_CONSTRAINT_COMPONENT_STR,
            CompiledComponent::Closed(_) => SH_CLOSED_CONSTRAINT_COMPONENT_STR,
            CompiledComponent::Node(_) => SH_NODE_CONSTRAINT_COMPONENT_STR,
            CompiledComponent::HasValue(_) => SH_HAS_VALUE_CONSTRAINT_COMPONENT_STR,
            CompiledComponent::In(_) => SH_IN_CONSTRAINT_COMPONENT_STR,
            CompiledComponent::QualifiedValueShape(_) => todo!(),
        };
        IriS::new_unchecked(iri_str)
    }
}
