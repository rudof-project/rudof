use std::fmt::Display;
use std::str::FromStr;

use iri_s::IriS;
use itertools::Itertools;
use srdf::model::rdf::Rdf;
use srdf::model::rdf::TLiteralRef;
use srdf::model::rdf::TObjectRef;
use srdf::model::rdf::TPredicateRef;
use srdf::model::Literal as _;

use crate::node_kind::NodeKind;
use crate::vocab::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Component<R: Rdf> {
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

impl<R: Rdf> Display for Component<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Component::Class(cls) => write!(f, "class({cls})"),
            Component::Datatype(dt) => write!(f, "datatype({dt})"),
            Component::NodeKind(nk) => write!(f, "nodeKind({nk})"),
            Component::MinCount(mc) => write!(f, "minCount({mc})"),
            Component::MaxCount(mc) => write!(f, "maxCount({mc})"),
            Component::MinExclusive(me) => write!(f, "minExclusive({me})"),
            Component::MaxExclusive(me) => write!(f, "maxExclusive({me})"),
            Component::MinInclusive(mi) => write!(f, "minInclusive({mi})"),
            Component::MaxInclusive(mi) => write!(f, "maxInclusive({mi})"),
            Component::MinLength(ml) => write!(f, "minLength({ml})"),
            Component::MaxLength(ml) => write!(f, "maxLength({ml})"),
            Component::Pattern(p) => write!(f, "pattern({p})"),
            Component::UniqueLang(ul) => write!(f, "uniqueLang({ul})"),
            Component::LanguageIn(li) => write!(f, "languageIn({li})"),
            Component::Equals(e) => write!(f, "equals({e})"),
            Component::Disjoint(d) => write!(f, "disjoint({d})"),
            Component::LessThan(lt) => write!(f, "uniqueLang({lt})"),
            Component::LessThanOrEquals(lte) => write!(f, "uniqueLang({lte})"),
            Component::Or(or) => write!(f, "or [{or}]"),
            Component::And(and) => write!(f, "and [{and}]"),
            Component::Not(not) => write!(f, "not [{not}]"),
            Component::Xone(xone) => write!(f, "xone [{xone}]"),
            Component::Closed(closed) => write!(f, "closed({closed})"),
            Component::Node(node) => write!(f, "node({node})"),
            Component::HasValue(hv) => write!(f, "hasValue({hv})"),
            Component::In(li) => write!(f, "languageIn({li})"),
            Component::QualifiedValueShape(qvs) => write!(f, "qualifiedValueShape({qvs})"),
        }
    }
}

impl<R: Rdf> From<Component<R>> for IriS {
    fn from(value: Component<R>) -> Self {
        match value {
            Component::Class(_) => IriS::from_str(SH_CLASS_STR),
            Component::Datatype(_) => IriS::from_str(SH_DATATYPE_STR),
            Component::NodeKind(_) => IriS::from_str(SH_IRI_STR),
            Component::MinCount(_) => IriS::from_str(SH_MIN_COUNT_STR),
            Component::MaxCount(_) => IriS::from_str(SH_MAX_COUNT_STR),
            Component::MinExclusive(_) => IriS::from_str(SH_MIN_EXCLUSIVE_STR),
            Component::MaxExclusive(_) => IriS::from_str(SH_MAX_EXCLUSIVE_STR),
            Component::MinInclusive(_) => IriS::from_str(SH_MIN_INCLUSIVE_STR),
            Component::MaxInclusive(_) => IriS::from_str(SH_MAX_INCLUSIVE_STR),
            Component::MinLength(_) => IriS::from_str(SH_MIN_LENGTH_STR),
            Component::MaxLength(_) => IriS::from_str(SH_MAX_LENGTH_STR),
            Component::Pattern { .. } => IriS::from_str(SH_PATTERN_STR),
            Component::UniqueLang(_) => IriS::from_str(SH_UNIQUE_LANG_STR),
            Component::LanguageIn { .. } => IriS::from_str(SH_LANGUAGE_IN_STR),
            Component::Equals(_) => IriS::from_str(SH_EQUALS_STR),
            Component::Disjoint(_) => IriS::from_str(SH_DISJOINT_STR),
            Component::LessThan(_) => IriS::from_str(SH_LESS_THAN_STR),
            Component::LessThanOrEquals(_) => IriS::from_str(SH_LESS_THAN_OR_EQUALS_STR),
            Component::Or { .. } => IriS::from_str(SH_OR_STR),
            Component::And { .. } => IriS::from_str(SH_AND_STR),
            Component::Not { .. } => IriS::from_str(SH_NOT_STR),
            Component::Xone { .. } => IriS::from_str(SH_XONE_STR),
            Component::Closed { .. } => IriS::from_str(SH_CLOSED_STR),
            Component::Node { .. } => IriS::from_str(SH_NODE_STR),
            Component::HasValue { .. } => IriS::from_str(SH_HAS_VALUE_STR),
            Component::In { .. } => IriS::from_str(SH_IN_STR),
            Component::QualifiedValueShape { .. } => IriS::from_str(SH_QUALIFIED_VALUE_SHAPE_STR),
        }
        .unwrap()
    }
}

/// sh:maxCount specifies the maximum number of value nodes that satisfy the
/// condition.
///
/// - IRI: https://www.w3.org/TR/shacl/#MaxCountConstraintComponent
/// - DEF: If the number of value nodes is greater than $maxCount, there is a
///   validation result.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct MaxCount {
    max_count: isize,
}

impl MaxCount {
    pub fn new(max_count: isize) -> Self {
        MaxCount { max_count }
    }

    pub fn max_count(&self) -> isize {
        self.max_count
    }
}

impl Display for MaxCount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{0}", self.max_count)
    }
}

/// sh:minCount specifies the minimum number of value nodes that satisfy the
/// condition. If the minimum cardinality value is 0 then this constraint is
/// always satisfied and so may be omitted.
///
/// - IRI: https://www.w3.org/TR/shacl/#MinCountConstraintComponent
/// - DEF: If the number of value nodes is less than $minCount, there is a
///   validation result.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
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

impl Display for MinCount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{0}", self.min_count)
    }
}

/// sh:and specifies the condition that each value node conforms to all provided
/// shapes. This is comparable to conjunction and the logical "and" operator.
///
/// https://www.w3.org/TR/shacl/#AndConstraintComponent
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct And<R: Rdf> {
    shapes: Vec<TObjectRef<R>>,
}

impl<R: Rdf> And<R> {
    pub fn new(shapes: Vec<TObjectRef<R>>) -> Self {
        And { shapes }
    }

    pub fn shapes(&self) -> &Vec<TObjectRef<R>> {
        &self.shapes
    }
}

impl<R: Rdf> Display for And<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{0}", self.shapes.iter().map(|s| s).join(" "))
    }
}

/// sh:not specifies the condition that each value node cannot conform to a
/// given shape. This is comparable to negation and the logical "not" operator.
///
/// https://www.w3.org/TR/shacl/#NotConstraintComponent
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Not<R: Rdf> {
    shape: TObjectRef<R>,
}

impl<R: Rdf> Not<R> {
    pub fn new(shape: TObjectRef<R>) -> Self {
        Not { shape }
    }

    pub fn shape(&self) -> &TObjectRef<R> {
        &self.shape
    }
}

impl<R: Rdf> Display for Not<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{0}", self.shape)
    }
}

/// sh:or specifies the condition that each value node conforms to at least one
/// of the provided shapes. This is comparable to disjunction and the logical
/// "or" operator.
///
/// https://www.w3.org/TR/shacl/#AndConstraintComponent

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Or<R: Rdf> {
    shapes: Vec<TObjectRef<R>>,
}

impl<R: Rdf> Or<R> {
    pub fn new(shapes: Vec<TObjectRef<R>>) -> Self {
        Or { shapes }
    }

    pub fn shapes(&self) -> &Vec<TObjectRef<R>> {
        &self.shapes
    }
}

impl<R: Rdf> Display for Or<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{0}", self.shapes.iter().map(|s| s).join(" "))
    }
}

/// sh:or specifies the condition that each value node conforms to at least one
/// of the provided shapes. This is comparable to disjunction and the logical
/// "or" operator.
///
/// https://www.w3.org/TR/shacl/#XoneConstraintComponent
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Xone<R: Rdf> {
    shapes: Vec<TObjectRef<R>>,
}

impl<R: Rdf> Xone<R> {
    pub fn new(shapes: Vec<TObjectRef<R>>) -> Self {
        Xone { shapes }
    }

    pub fn shapes(&self) -> &Vec<TObjectRef<R>> {
        &self.shapes
    }
}

impl<R: Rdf> Display for Xone<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{0}", self.shapes.iter().map(|s| s).join(" "))
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
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Closed<R: Rdf> {
    is_closed: bool,
    ignored_properties: Vec<TPredicateRef<R>>,
}

impl<R: Rdf> Closed<R> {
    pub fn new(is_closed: bool, ignored_properties: Vec<TPredicateRef<R>>) -> Self {
        Closed {
            is_closed,
            ignored_properties,
        }
    }

    pub fn is_closed(&self) -> bool {
        self.is_closed
    }

    pub fn ignored_properties(&self) -> &Vec<TPredicateRef<R>> {
        &self.ignored_properties
    }
}

impl<R: Rdf> Display for Closed<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "is closed? {}, [{}]",
            self.is_closed,
            self.ignored_properties().iter().map(|s| s).join(" - ")
        )
    }
}

/// sh:hasValue specifies the condition that at least one value node is equal to
///  the given RDF term.
///
/// https://www.w3.org/TR/shacl/#HasValueConstraintComponent
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct HasValue<R: Rdf> {
    value: TObjectRef<R>,
}

impl<R: Rdf> HasValue<R> {
    pub fn new(value: TObjectRef<R>) -> Self {
        HasValue { value }
    }

    pub fn value(&self) -> &TObjectRef<R> {
        &self.value
    }
}

impl<R: Rdf> Display for HasValue<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{0}", self.value)
    }
}

/// sh:in specifies the condition that each value node is a member of a provided
/// SHACL list.
///
/// https://www.w3.org/TR/shacl/#InConstraintComponent
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct In<R: Rdf> {
    values: Vec<TObjectRef<R>>,
}

impl<R: Rdf> In<R> {
    pub fn new(values: Vec<TObjectRef<R>>) -> Self {
        In { values }
    }

    pub fn values(&self) -> &Vec<TObjectRef<R>> {
        &self.values
    }
}

impl<R: Rdf> Display for In<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.values().iter().map(|s| s).join(" - "))
    }
}

/// sh:disjoint specifies the condition that the set of value nodes is disjoint
/// with the set of objects of the triples that have the focus node as subject
/// and the value of sh:disjoint as predicate.
///
/// https://www.w3.org/TR/shacl/#DisjointConstraintComponent
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Disjoint<R: Rdf> {
    iri_ref: TPredicateRef<R>,
}

impl<R: Rdf> Disjoint<R> {
    pub fn new(iri_ref: TPredicateRef<R>) -> Self {
        Disjoint { iri_ref }
    }

    pub fn iri_ref(&self) -> &TPredicateRef<R> {
        &self.iri_ref
    }
}

impl<R: Rdf> Display for Disjoint<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{0}", self.iri_ref)
    }
}

/// sh:equals specifies the condition that the set of all value nodes is equal
/// to the set of objects of the triples that have the focus node as subject and
/// the value of sh:equals as predicate.
///
/// https://www.w3.org/TR/shacl/#EqualsConstraintComponent
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Equals<R: Rdf> {
    iri_ref: TPredicateRef<R>,
}

impl<R: Rdf> Equals<R> {
    pub fn new(iri_ref: TPredicateRef<R>) -> Self {
        Equals { iri_ref }
    }

    pub fn iri_ref(&self) -> &TPredicateRef<R> {
        &self.iri_ref
    }
}

impl<R: Rdf> Display for Equals<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{0}", self.iri_ref)
    }
}

/// LessThanOrEquals Constraint Component.
///
/// sh:lessThanOrEquals specifies the condition that each value node is smaller
/// than or equal to all the objects of the triples that have the focus node
/// as subject and the value of sh:lessThanOrEquals as predicate.
///
/// https://www.w3.org/TR/shacl/#LessThanOrEqualsConstraintComponent
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct LessThanOrEquals<R: Rdf> {
    iri_ref: TPredicateRef<R>,
}

impl<R: Rdf> LessThanOrEquals<R> {
    pub fn new(iri_ref: TPredicateRef<R>) -> Self {
        LessThanOrEquals { iri_ref }
    }

    pub fn iri_ref(&self) -> &TPredicateRef<R> {
        &self.iri_ref
    }
}

impl<R: Rdf> Display for LessThanOrEquals<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{0}", self.iri_ref)
    }
}

/// sh:lessThan specifies the condition that each value node is smaller than all
/// the objects of the triples that have the focus node as subject and the
/// value of sh:lessThan as predicate.
///
/// https://www.w3.org/TR/shacl/#LessThanConstraintComponent
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct LessThan<R: Rdf> {
    iri_ref: TPredicateRef<R>,
}

impl<R: Rdf> LessThan<R> {
    pub fn new(iri_ref: TPredicateRef<R>) -> Self {
        LessThan { iri_ref }
    }

    pub fn iri_ref(&self) -> &TPredicateRef<R> {
        &self.iri_ref
    }
}

impl<R: Rdf> Display for LessThan<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{0}", self.iri_ref)
    }
}

/// sh:node specifies the condition that each value node conforms to the given
/// node shape.
///
/// https://www.w3.org/TR/shacl/#NodeShapeComponent
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Node<R: Rdf> {
    shape: TObjectRef<R>,
}

impl<R: Rdf> Node<R> {
    pub fn new(shape: TObjectRef<R>) -> Self {
        Node { shape }
    }

    pub fn shape(&self) -> &TObjectRef<R> {
        &self.shape
    }
}

impl<R: Rdf> Display for Node<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{0}", self.shape)
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
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct QualifiedValueShape<R: Rdf> {
    shape: TObjectRef<R>,
    qualified_min_count: Option<isize>,
    qualified_max_count: Option<isize>,
    qualified_value_shapes_disjoint: Option<bool>,
}

impl<R: Rdf> QualifiedValueShape<R> {
    pub fn new(
        shape: TObjectRef<R>,
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

    pub fn shape(&self) -> &TObjectRef<R> {
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

impl<R: Rdf> Display for QualifiedValueShape<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "shape: {0}, qualified_min_count: {1:?}, qualified_max_count: {2:?}, qualified_value_shapes_disjoint: {3:?}", self.shape, self.qualified_max_count, self.qualified_min_count, self.qualified_value_shapes_disjoint)
    }
}

/// The condition specified by sh:languageIn is that the allowed language tags
/// for each value node are limited by a given list of language tags.
///
/// https://www.w3.org/TR/shacl/#LanguageInConstraintComponent
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct LanguageIn<R: Rdf> {
    langs: Vec<TLiteralRef<R::Triple>>,
}

impl<R: Rdf> LanguageIn<R> {
    pub fn new(langs: Vec<TLiteralRef<R::Triple>>) -> Self {
        LanguageIn { langs }
    }

    pub fn langs(&self) -> &Vec<TLiteralRef<R::Triple>> {
        &self.langs
    }
}

impl<R: Rdf> Display for LanguageIn<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{0}",
            self.langs()
                .iter()
                .map(|s| s.as_string().unwrap())
                .join(" - ")
        )
    }
}

/// sh:maxLength specifies the maximum string length of each value node that
/// satisfies the condition. This can be applied to any literals and IRIs, but
/// not to blank nodes.
///
/// https://www.w3.org/TR/shacl/#MaxLengthConstraintComponent
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
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

impl Display for MaxLength {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{0}", self.max_length)
    }
}

/// sh:minLength specifies the minimum string length of each value node that
/// satisfies the condition. This can be applied to any literals and IRIs, but
/// not to blank nodes.
///
/// https://www.w3.org/TR/shacl/#MinLengthConstraintComponent
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
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

impl Display for MinLength {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{0}", self.min_length)
    }
}

/// sh:property can be used to specify that each value node has a given property
/// shape.
///
/// https://www.w3.org/TR/shacl/#PropertyShapeComponent
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
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

impl Display for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "pattern: {0}, flags: {1:?}", self.pattern, self.flags)
    }
}

/// The property sh:uniqueLang can be set to true to specify that no pair of
///  value nodes may use the same language tag.
///
/// https://www.w3.org/TR/shacl/#UniqueLangConstraintComponent
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
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

impl Display for UniqueLang {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.unique_lang)
    }
}

/// The condition specified by sh:class is that each value node is a SHACL
/// instance of a given type.
///
/// https://www.w3.org/TR/shacl/#ClassConstraintComponent
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Class<R: Rdf> {
    class_rule: TObjectRef<R>,
}

impl<R: Rdf> Class<R> {
    pub fn new(class_rule: TObjectRef<R>) -> Self {
        Class { class_rule }
    }

    pub fn class_rule(&self) -> &TObjectRef<R> {
        &self.class_rule
    }
}

impl<R: Rdf> Display for Class<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.class_rule)
    }
}

/// sh:datatype specifies a condition to be satisfied with regards to the
/// datatype of each value node.
///
/// https://www.w3.org/TR/shacl/#ClassConstraintComponent
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Datatype<R: Rdf> {
    datatype: TPredicateRef<R>,
}

impl<R: Rdf> Datatype<R> {
    pub fn new(datatype: TPredicateRef<R>) -> Self {
        Datatype { datatype }
    }

    pub fn datatype(&self) -> &TPredicateRef<R> {
        &self.datatype
    }
}

impl<R: Rdf> Display for Datatype<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.datatype())
    }
}

/// sh:nodeKind specifies a condition to be satisfied by the RDF node kind of
/// each value node.
///
/// https://www.w3.org/TR/shacl/#NodeKindConstraintComponent
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
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

impl Display for Nodekind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.node_kind)
    }
}

/// https://www.w3.org/TR/shacl/#MaxExclusiveConstraintComponent
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct MaxExclusive<R: Rdf> {
    max_exclusive: TObjectRef<R>,
}

impl<R: Rdf> MaxExclusive<R> {
    pub fn new(literal: TObjectRef<R>) -> Self {
        MaxExclusive {
            max_exclusive: literal,
        }
    }

    pub fn max_exclusive(&self) -> &TObjectRef<R> {
        &self.max_exclusive
    }
}

impl<R: Rdf> Display for MaxExclusive<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.max_exclusive)
    }
}

/// https://www.w3.org/TR/shacl/#MaxInclusiveConstraintComponent
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct MaxInclusive<R: Rdf> {
    max_inclusive: TObjectRef<R>,
}

impl<R: Rdf> MaxInclusive<R> {
    pub fn new(literal: TObjectRef<R>) -> Self {
        MaxInclusive {
            max_inclusive: literal,
        }
    }

    pub fn max_inclusive(&self) -> &TObjectRef<R> {
        &self.max_inclusive
    }
}

impl<R: Rdf> Display for MaxInclusive<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.max_inclusive)
    }
}

/// https://www.w3.org/TR/shacl/#MinExclusiveConstraintComponent
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct MinExclusive<R: Rdf> {
    min_exclusive: TObjectRef<R>,
}

impl<R: Rdf> MinExclusive<R> {
    pub fn new(literal: TObjectRef<R>) -> Self {
        MinExclusive {
            min_exclusive: literal,
        }
    }

    pub fn min_exclusive(&self) -> &TObjectRef<R> {
        &self.min_exclusive
    }
}

impl<R: Rdf> Display for MinExclusive<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.min_exclusive)
    }
}

/// https://www.w3.org/TR/shacl/#MinInclusiveConstraintComponent
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct MinInclusive<R: Rdf> {
    min_inclusive: TObjectRef<R>,
}

impl<R: Rdf> MinInclusive<R> {
    pub fn new(literal: TObjectRef<R>) -> Self {
        MinInclusive {
            min_inclusive: literal,
        }
    }

    pub fn min_inclusive(&self) -> &TObjectRef<R> {
        &self.min_inclusive
    }
}

impl<R: Rdf> Display for MinInclusive<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.min_inclusive)
    }
}
