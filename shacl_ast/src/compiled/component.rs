use std::str::FromStr;

use compiled::compile_shape;
use compiled::compile_shapes;
use iri_s::IriS;
use node_kind::NodeKind;
use srdf::model::rdf::Rdf;
use srdf::model::rdf::TLiteral;
use srdf::model::rdf::TObject;
use srdf::model::rdf::TPredicate;
use vocab::*;

use crate::component::Component;
use crate::Schema;
use crate::*;

use super::compiled_shacl_error::CompiledShaclError;
use super::shape::CompiledShape;

#[derive(Debug, Clone)]
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

impl<R: Rdf + Clone> CompiledComponent<R> {
    pub fn compile(
        component: Component<R>,
        schema: &Schema<R>,
    ) -> Result<Self, CompiledShaclError> {
        let component = match component {
            Component::Class(class) => CompiledComponent::Class(class.into()),
            Component::Datatype(data_type) => CompiledComponent::Datatype(data_type.into()),
            Component::NodeKind(node_kind) => CompiledComponent::NodeKind(node_kind.into()),
            Component::MinCount(min_count) => CompiledComponent::MinCount(min_count.into()),
            Component::MaxCount(max_count) => CompiledComponent::MaxCount(max_count.into()),
            Component::MinExclusive(min) => CompiledComponent::MinExclusive(min.into()),
            Component::MaxExclusive(max) => CompiledComponent::MaxExclusive(max.into()),
            Component::MinInclusive(min) => CompiledComponent::MinInclusive(min.into()),
            Component::MaxInclusive(max) => CompiledComponent::MaxInclusive(max.into()),
            Component::MinLength(min) => CompiledComponent::MinLength(min.into()),
            Component::MaxLength(max) => CompiledComponent::MaxLength(max.into()),
            Component::Pattern(pattern) => CompiledComponent::Pattern(pattern.into()),
            Component::UniqueLang(lang) => CompiledComponent::UniqueLang(lang.into()),
            Component::LanguageIn(langs) => CompiledComponent::LanguageIn(langs.into()),
            Component::Equals(eq) => CompiledComponent::Equals(eq.into()),
            Component::Disjoint(disjoint) => CompiledComponent::Disjoint(disjoint.into()),
            Component::LessThan(lt) => CompiledComponent::LessThan(lt.into()),
            Component::LessThanOrEquals(lte) => CompiledComponent::LessThanOrEquals(lte.into()),
            Component::Or(or) => CompiledComponent::Or(or.compile(schema)?),
            Component::And(and) => CompiledComponent::And(and.compile(schema)?),
            Component::Not(not) => CompiledComponent::Not(not.compile(schema)?),
            Component::Xone(xone) => CompiledComponent::Xone(xone.compile(schema)?),
            Component::Closed(closed) => CompiledComponent::Closed(closed.into()),
            Component::Node(node) => CompiledComponent::Node(node.compile(schema)?),
            Component::HasValue(has_value) => CompiledComponent::HasValue(has_value.into()),
            Component::In(i) => CompiledComponent::In(i.into()),
            Component::QualifiedValueShape(q) => {
                CompiledComponent::QualifiedValueShape(q.compile(schema)?)
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
#[derive(Debug, Clone)]
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

impl From<ast::component::MaxCount> for MaxCount {
    fn from(value: ast::component::MaxCount) -> Self {
        MaxCount::new(value.max_count())
    }
}

/// sh:minCount specifies the minimum number of value nodes that satisfy the
/// condition. If the minimum cardinality value is 0 then this constraint is
/// always satisfied and so may be omitted.
///
/// - IRI: https://www.w3.org/TR/shacl/#MinCountConstraintComponent
/// - DEF: If the number of value nodes is less than $minCount, there is a
///   validation result.
#[derive(Debug, Clone)]
pub struct MinCount {
    min_count: isize,
}

impl MinCount {
    pub fn new(min_count: isize) -> Self {
        MinCount { min_count }
    }

    pub fn min_count(&self) -> isize {
        self.min_count
    }
}

impl From<ast::component::MinCount> for MinCount {
    fn from(value: ast::component::MinCount) -> Self {
        MinCount::new(value.min_count())
    }
}

/// sh:and specifies the condition that each value node conforms to all provided
/// shapes. This is comparable to conjunction and the logical "and" operator.
///
/// https://www.w3.org/TR/shacl/#AndConstraintComponent
#[derive(Debug, Clone)]
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

impl<R: Rdf + Clone> ast::component::And<R> {
    fn compile(&self, schema: &Schema<R>) -> Result<And<R>, CompiledShaclError> {
        Ok(And::new(compile_shapes(self.shapes().to_vec(), schema)?))
    }
}

/// sh:not specifies the condition that each value node cannot conform to a
/// given shape. This is comparable to negation and the logical "not" operator.
///
/// https://www.w3.org/TR/shacl/#NotConstraintComponent
#[derive(Debug, Clone)]
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

impl<R: Rdf + Clone> ast::component::Not<R> {
    fn compile(&self, schema: &Schema<R>) -> Result<Not<R>, CompiledShaclError> {
        Ok(Not::new(compile_shape(self.shape().clone(), schema)?))
    }
}

/// sh:or specifies the condition that each value node conforms to at least one
/// of the provided shapes. This is comparable to disjunction and the logical
/// "or" operator.
///
/// https://www.w3.org/TR/shacl/#AndConstraintComponent

#[derive(Debug, Clone)]
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

impl<R: Rdf + Clone> ast::component::Or<R> {
    fn compile(&self, schema: &Schema<R>) -> Result<Or<R>, CompiledShaclError> {
        Ok(Or::new(compile_shapes(self.shapes().to_vec(), schema)?))
    }
}

/// sh:or specifies the condition that each value node conforms to at least one
/// of the provided shapes. This is comparable to disjunction and the logical
/// "or" operator.
///
/// https://www.w3.org/TR/shacl/#XoneConstraintComponent
#[derive(Debug, Clone)]
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

impl<R: Rdf + Clone> ast::component::Xone<R> {
    fn compile(&self, schema: &Schema<R>) -> Result<Xone<R>, CompiledShaclError> {
        Ok(Xone::new(compile_shapes(self.shapes().to_vec(), schema)?))
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
#[derive(Debug, Clone)]
pub struct Closed<R: Rdf> {
    is_closed: bool,
    ignored_properties: Vec<TPredicate<R>>,
}

impl<R: Rdf> Closed<R> {
    pub fn new(is_closed: bool, ignored_properties: Vec<TPredicate<R>>) -> Self {
        Closed {
            is_closed,
            ignored_properties,
        }
    }

    pub fn is_closed(&self) -> bool {
        self.is_closed
    }

    pub fn ignored_properties(&self) -> &Vec<TPredicate<R>> {
        &self.ignored_properties
    }
}

impl<R: Rdf> From<ast::component::Closed<R>> for Closed<R> {
    fn from(value: ast::component::Closed<R>) -> Self {
        Closed::new(value.is_closed(), value.ignored_properties().to_vec())
    }
}

/// sh:hasValue specifies the condition that at least one value node is equal to
///  the given RDF term.
///
/// https://www.w3.org/TR/shacl/#HasValueConstraintComponent
#[derive(Debug, Clone)]
pub struct HasValue<R: Rdf> {
    value: TObject<R>,
}

impl<R: Rdf> HasValue<R> {
    pub fn new(value: TObject<R>) -> Self {
        HasValue { value }
    }

    pub fn value(&self) -> &TObject<R> {
        &self.value
    }
}

impl<R: Rdf> From<ast::component::HasValue<R>> for HasValue<R> {
    fn from(value: ast::component::HasValue<R>) -> Self {
        HasValue::new(value.value().clone())
    }
}

/// sh:in specifies the condition that each value node is a member of a provided
/// SHACL list.
///
/// https://www.w3.org/TR/shacl/#InConstraintComponent
#[derive(Debug, Clone)]
pub struct In<R: Rdf> {
    values: Vec<TObject<R>>,
}

impl<R: Rdf> In<R> {
    pub fn new(values: Vec<TObject<R>>) -> Self {
        In { values }
    }

    pub fn values(&self) -> &Vec<TObject<R>> {
        &self.values
    }
}

impl<R: Rdf> From<ast::component::In<R>> for In<R> {
    fn from(value: ast::component::In<R>) -> Self {
        In::new(value.values().to_vec())
    }
}

/// sh:disjoint specifies the condition that the set of value nodes is disjoint
/// with the set of objects of the triples that have the focus node as subject
/// and the value of sh:disjoint as predicate.
///
/// https://www.w3.org/TR/shacl/#DisjointConstraintComponent
#[derive(Debug, Clone)]
pub struct Disjoint<R: Rdf> {
    iri_ref: TPredicate<R>,
}

impl<R: Rdf> Disjoint<R> {
    pub fn new(iri_ref: TPredicate<R>) -> Self {
        Disjoint { iri_ref }
    }

    pub fn iri_ref(&self) -> &TPredicate<R> {
        &self.iri_ref
    }
}

impl<R: Rdf> From<ast::component::Disjoint<R>> for Disjoint<R> {
    fn from(value: ast::component::Disjoint<R>) -> Self {
        Disjoint::new(value.iri_ref().clone())
    }
}

/// sh:equals specifies the condition that the set of all value nodes is equal
/// to the set of objects of the triples that have the focus node as subject and
/// the value of sh:equals as predicate.
///
/// https://www.w3.org/TR/shacl/#EqualsConstraintComponent
#[derive(Debug, Clone)]
pub struct Equals<R: Rdf> {
    iri_ref: TPredicate<R>,
}

impl<R: Rdf> Equals<R> {
    pub fn new(iri_ref: TPredicate<R>) -> Self {
        Equals { iri_ref }
    }

    pub fn iri_ref(&self) -> &TPredicate<R> {
        &self.iri_ref
    }
}

impl<R: Rdf> From<ast::component::Equals<R>> for Equals<R> {
    fn from(value: ast::component::Equals<R>) -> Self {
        Equals::new(value.iri_ref().clone())
    }
}

/// LessThanOrEquals Constraint Component.
///
/// sh:lessThanOrEquals specifies the condition that each value node is smaller
/// than or equal to all the objects of the triples that have the focus node
/// as subject and the value of sh:lessThanOrEquals as predicate.
///
/// https://www.w3.org/TR/shacl/#LessThanOrEqualsConstraintComponent
#[derive(Debug, Clone)]
pub struct LessThanOrEquals<R: Rdf> {
    iri_ref: TPredicate<R>,
}

impl<R: Rdf> LessThanOrEquals<R> {
    pub fn new(iri_ref: TPredicate<R>) -> Self {
        LessThanOrEquals { iri_ref }
    }

    pub fn iri_ref(&self) -> &TPredicate<R> {
        &self.iri_ref
    }
}

impl<R: Rdf> From<ast::component::LessThanOrEquals<R>> for LessThanOrEquals<R> {
    fn from(value: ast::component::LessThanOrEquals<R>) -> Self {
        LessThanOrEquals::new(value.iri_ref().clone())
    }
}

/// sh:lessThan specifies the condition that each value node is smaller than all
/// the objects of the triples that have the focus node as subject and the
/// value of sh:lessThan as predicate.
///
/// https://www.w3.org/TR/shacl/#LessThanConstraintComponent
#[derive(Debug, Clone)]
pub struct LessThan<R: Rdf> {
    iri_ref: TPredicate<R>,
}

impl<R: Rdf> LessThan<R> {
    pub fn new(iri_ref: TPredicate<R>) -> Self {
        LessThan { iri_ref }
    }

    pub fn iri_ref(&self) -> &TPredicate<R> {
        &self.iri_ref
    }
}

impl<R: Rdf> From<ast::component::LessThan<R>> for LessThan<R> {
    fn from(value: ast::component::LessThan<R>) -> Self {
        LessThan::new(value.iri_ref().clone())
    }
}

/// sh:node specifies the condition that each value node conforms to the given
/// node shape.
///
/// https://www.w3.org/TR/shacl/#NodeShapeComponent
#[derive(Debug, Clone)]
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

impl<R: Rdf + Clone> ast::component::Node<R> {
    fn compile(&self, schema: &Schema<R>) -> Result<Node<R>, CompiledShaclError> {
        Ok(Node::new(compile_shape(self.shape().clone(), schema)?))
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
#[derive(Debug, Clone)]
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

impl<R: Rdf + Clone> ast::component::QualifiedValueShape<R> {
    fn compile(&self, schema: &Schema<R>) -> Result<QualifiedValueShape<R>, CompiledShaclError> {
        Ok(QualifiedValueShape::new(
            compile_shape(self.shape().clone(), schema)?,
            self.qualified_min_count(),
            self.qualified_max_count(),
            self.qualified_value_shapes_disjoint(),
        ))
    }
}

/// The condition specified by sh:languageIn is that the allowed language tags
/// for each value node are limited by a given list of language tags.
///
/// https://www.w3.org/TR/shacl/#LanguageInConstraintComponent
#[derive(Debug, Clone)]
pub struct LanguageIn<R: Rdf> {
    langs: Vec<TLiteral<R::Triple>>,
}

impl<R: Rdf> LanguageIn<R> {
    pub fn new(langs: Vec<TLiteral<R::Triple>>) -> Self {
        LanguageIn { langs }
    }

    pub fn langs(&self) -> &Vec<TLiteral<R::Triple>> {
        &self.langs
    }
}

impl<R: Rdf> From<ast::component::LanguageIn<R>> for LanguageIn<R> {
    fn from(value: ast::component::LanguageIn<R>) -> Self {
        LanguageIn::new(value.langs().to_vec())
    }
}

/// sh:maxLength specifies the maximum string length of each value node that
/// satisfies the condition. This can be applied to any literals and IRIs, but
/// not to blank nodes.
///
/// https://www.w3.org/TR/shacl/#MaxLengthConstraintComponent
#[derive(Debug, Clone)]
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

impl From<ast::component::MaxLength> for MaxLength {
    fn from(value: ast::component::MaxLength) -> Self {
        MaxLength::new(value.max_length())
    }
}

/// sh:minLength specifies the minimum string length of each value node that
/// satisfies the condition. This can be applied to any literals and IRIs, but
/// not to blank nodes.
///
/// https://www.w3.org/TR/shacl/#MinLengthConstraintComponent
#[derive(Debug, Clone)]
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

impl From<ast::component::MinLength> for MinLength {
    fn from(value: ast::component::MinLength) -> Self {
        MinLength::new(value.min_length())
    }
}

/// sh:property can be used to specify that each value node has a given property
/// shape.
///
/// https://www.w3.org/TR/shacl/#PropertyShapeComponent
#[derive(Debug, Clone)]
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

impl From<ast::component::Pattern> for Pattern {
    fn from(value: ast::component::Pattern) -> Self {
        Pattern::new(value.pattern().clone(), value.flags().clone())
    }
}

/// The property sh:uniqueLang can be set to true to specify that no pair of
///  value nodes may use the same language tag.
///
/// https://www.w3.org/TR/shacl/#UniqueLangConstraintComponent
#[derive(Debug, Clone)]
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

impl From<ast::component::UniqueLang> for UniqueLang {
    fn from(value: ast::component::UniqueLang) -> Self {
        UniqueLang::new(value.unique_lang())
    }
}

/// The condition specified by sh:class is that each value node is a SHACL
/// instance of a given type.
///
/// https://www.w3.org/TR/shacl/#ClassConstraintComponent
#[derive(Debug, Clone)]
pub struct Class<R: Rdf> {
    class_rule: TObject<R>,
}

impl<R: Rdf> Class<R> {
    pub fn new(class_rule: TObject<R>) -> Self {
        Class { class_rule }
    }

    pub fn class_rule(&self) -> &TObject<R> {
        &self.class_rule
    }
}

impl<R: Rdf> From<ast::component::Class<R>> for Class<R> {
    fn from(value: ast::component::Class<R>) -> Self {
        Class::new(value.class_rule().clone())
    }
}

/// sh:datatype specifies a condition to be satisfied with regards to the
/// datatype of each value node.
///
/// https://www.w3.org/TR/shacl/#ClassConstraintComponent
#[derive(Debug, Clone)]
pub struct Datatype<R: Rdf> {
    datatype: TPredicate<R>,
}

impl<R: Rdf> Datatype<R> {
    pub fn new(datatype: TPredicate<R>) -> Self {
        Datatype { datatype }
    }

    pub fn datatype(&self) -> &TPredicate<R> {
        &self.datatype
    }
}

impl<R: Rdf> From<ast::component::Datatype<R>> for Datatype<R> {
    fn from(value: ast::component::Datatype<R>) -> Self {
        Datatype::new(value.datatype().clone())
    }
}

/// sh:nodeKind specifies a condition to be satisfied by the RDF node kind of
/// each value node.
///
/// https://www.w3.org/TR/shacl/#NodeKindConstraintComponent
#[derive(Debug, Clone)]
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

impl From<ast::component::Nodekind> for Nodekind {
    fn from(value: ast::component::Nodekind) -> Self {
        Nodekind::new(value.node_kind().clone())
    }
}

/// https://www.w3.org/TR/shacl/#MaxExclusiveConstraintComponent
#[derive(Debug, Clone)]
pub struct MaxExclusive<R: Rdf> {
    max_exclusive: TObject<R>,
}

impl<R: Rdf> MaxExclusive<R> {
    pub fn new(literal: TObject<R>) -> Self {
        MaxExclusive {
            max_exclusive: literal,
        }
    }

    pub fn max_exclusive(&self) -> &TObject<R> {
        &self.max_exclusive
    }
}

impl<R: Rdf> From<ast::component::MaxExclusive<R>> for MaxExclusive<R> {
    fn from(value: ast::component::MaxExclusive<R>) -> Self {
        MaxExclusive::new(value.max_exclusive().clone())
    }
}

/// https://www.w3.org/TR/shacl/#MaxInclusiveConstraintComponent
#[derive(Debug, Clone)]
pub struct MaxInclusive<R: Rdf> {
    max_inclusive: TObject<R>,
}

impl<R: Rdf> MaxInclusive<R> {
    pub fn new(literal: TObject<R>) -> Self {
        MaxInclusive {
            max_inclusive: literal,
        }
    }

    pub fn max_inclusive(&self) -> &TObject<R> {
        &self.max_inclusive
    }
}

impl<R: Rdf> From<ast::component::MaxInclusive<R>> for MaxInclusive<R> {
    fn from(value: ast::component::MaxInclusive<R>) -> Self {
        MaxInclusive::new(value.max_inclusive().clone())
    }
}

/// https://www.w3.org/TR/shacl/#MinExclusiveConstraintComponent
#[derive(Debug, Clone)]
pub struct MinExclusive<R: Rdf> {
    min_exclusive: TObject<R>,
}

impl<R: Rdf> MinExclusive<R> {
    pub fn new(literal: TObject<R>) -> Self {
        MinExclusive {
            min_exclusive: literal,
        }
    }

    pub fn min_exclusive(&self) -> &TObject<R> {
        &self.min_exclusive
    }
}

impl<R: Rdf> From<ast::component::MinExclusive<R>> for MinExclusive<R> {
    fn from(value: ast::component::MinExclusive<R>) -> Self {
        MinExclusive::new(value.min_exclusive().clone())
    }
}

/// https://www.w3.org/TR/shacl/#MinInclusiveConstraintComponent
#[derive(Debug, Clone)]
pub struct MinInclusive<R: Rdf> {
    min_inclusive: TObject<R>,
}

impl<R: Rdf> MinInclusive<R> {
    pub fn new(literal: TObject<R>) -> Self {
        MinInclusive {
            min_inclusive: literal,
        }
    }

    pub fn min_inclusive(&self) -> &TObject<R> {
        &self.min_inclusive
    }
}

impl<R: Rdf> From<ast::component::MinInclusive<R>> for MinInclusive<R> {
    fn from(value: ast::component::MinInclusive<R>) -> Self {
        MinInclusive::new(value.min_inclusive().clone())
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
