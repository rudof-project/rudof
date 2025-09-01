use std::fmt::Display;

use super::compile_shape;
use super::compile_shapes;
use super::compiled_shacl_error::CompiledShaclError;
use super::convert_iri_ref;
use super::convert_value;
use super::shape::CompiledShape;
use iri_s::IriS;
use regex::Regex;
use shacl_ast::component::Component;
use shacl_ast::node_kind::NodeKind;
use shacl_ast::shacl_vocab::{
    sh_and, sh_class, sh_datatype, sh_disjoint, sh_equals, sh_has_value, sh_in, sh_language_in,
    sh_less_than, sh_less_than_or_equals, sh_max_count, sh_max_exclusive, sh_max_inclusive,
    sh_max_length, sh_min_count, sh_min_exclusive, sh_min_inclusive, sh_min_length, sh_node,
    sh_node_kind, sh_not, sh_or, sh_pattern, sh_qualified_value_shape, sh_unique_lang, sh_xone,
};
use shacl_ast::Schema;
use srdf::lang::Lang;
use srdf::RDFNode;
use srdf::Rdf;
use srdf::SLiteral;

#[derive(Debug, Clone)]
pub enum CompiledComponent {
    Class(Class),
    Datatype(Datatype),
    NodeKind(Nodekind),
    MinCount(MinCount),
    MaxCount(MaxCount),
    MinExclusive(MinExclusive),
    MaxExclusive(MaxExclusive),
    MinInclusive(MinInclusive),
    MaxInclusive(MaxInclusive),
    MinLength(MinLength),
    MaxLength(MaxLength),
    Pattern(Pattern),
    UniqueLang(UniqueLang),
    LanguageIn(LanguageIn),
    Equals(Equals),
    Disjoint(Disjoint),
    LessThan(LessThan),
    LessThanOrEquals(LessThanOrEquals),
    Or(Or),
    And(And),
    Not(Not),
    Xone(Xone),
    Node(Node),
    HasValue(HasValue),
    In(In),
    QualifiedValueShape(QualifiedValueShape),
}

impl CompiledComponent {
    pub fn compile<S: Rdf>(
        component: Component,
        schema: &Schema<S>,
    ) -> Result<Option<Self>, CompiledShaclError> {
        let component = match component {
            Component::Class(object) => {
                let class_rule = object;
                Some(CompiledComponent::Class(Class::new(class_rule)))
            }
            Component::Datatype(iri_ref) => {
                let iri_ref = convert_iri_ref(iri_ref)?;
                Some(CompiledComponent::Datatype(Datatype::new(iri_ref)))
            }
            Component::NodeKind(node_kind) => {
                Some(CompiledComponent::NodeKind(Nodekind::new(node_kind)))
            }
            Component::MinCount(count) => Some(CompiledComponent::MinCount(MinCount::new(count))),
            Component::MaxCount(count) => Some(CompiledComponent::MaxCount(MaxCount::new(count))),
            Component::MinExclusive(literal) => {
                Some(CompiledComponent::MinExclusive(MinExclusive::new(literal)))
            }
            Component::MaxExclusive(literal) => {
                Some(CompiledComponent::MaxExclusive(MaxExclusive::new(literal)))
            }
            Component::MinInclusive(literal) => {
                Some(CompiledComponent::MinInclusive(MinInclusive::new(literal)))
            }
            Component::MaxInclusive(literal) => {
                Some(CompiledComponent::MaxInclusive(MaxInclusive::new(literal)))
            }
            Component::MinLength(length) => {
                Some(CompiledComponent::MinLength(MinLength::new(length)))
            }
            Component::MaxLength(length) => {
                Some(CompiledComponent::MaxLength(MaxLength::new(length)))
            }
            Component::Pattern { pattern, flags } => {
                Some(CompiledComponent::Pattern(Pattern::new(pattern, flags)))
            }
            Component::UniqueLang(lang) => {
                Some(CompiledComponent::UniqueLang(UniqueLang::new(lang)))
            }
            Component::LanguageIn { langs } => {
                Some(CompiledComponent::LanguageIn(LanguageIn::new(langs)))
            }
            Component::Equals(iri_ref) => {
                let iri_ref = convert_iri_ref(iri_ref)?;
                Some(CompiledComponent::Equals(Equals::new(iri_ref)))
            }
            Component::Disjoint(iri_ref) => {
                let iri_ref = convert_iri_ref(iri_ref)?;
                Some(CompiledComponent::Disjoint(Disjoint::new(iri_ref)))
            }
            Component::LessThan(iri_ref) => {
                let iri_ref = convert_iri_ref(iri_ref)?;
                Some(CompiledComponent::LessThan(LessThan::new(iri_ref)))
            }
            Component::LessThanOrEquals(iri_ref) => {
                let iri_ref = convert_iri_ref(iri_ref)?;
                Some(CompiledComponent::LessThanOrEquals(LessThanOrEquals::new(
                    iri_ref,
                )))
            }
            Component::Or { shapes } => Some(CompiledComponent::Or(Or::new(compile_shapes::<S>(
                shapes, schema,
            )?))),
            Component::And { shapes } => Some(CompiledComponent::And(And::new(
                compile_shapes::<S>(shapes, schema)?,
            ))),
            Component::Not { shape } => {
                let shape = compile_shape::<S>(shape, schema)?;
                Some(CompiledComponent::Not(Not::new(shape)))
            }
            Component::Xone { shapes } => Some(CompiledComponent::Xone(Xone::new(
                compile_shapes::<S>(shapes, schema)?,
            ))),
            Component::Closed { .. } => None,
            Component::Node { shape } => {
                let shape = compile_shape::<S>(shape, schema)?;
                Some(CompiledComponent::Node(Node::new(shape)))
            }
            Component::HasValue { value } => {
                let term = convert_value(value)?;
                Some(CompiledComponent::HasValue(HasValue::new(term)))
            }
            Component::In { values } => {
                let terms = values
                    .into_iter()
                    .map(convert_value)
                    .collect::<Result<Vec<_>, _>>()?;
                Some(CompiledComponent::In(In::new(terms)))
            }
            Component::QualifiedValueShape {
                shape,
                qualified_min_count,
                qualified_max_count,
                qualified_value_shapes_disjoint,
            } => {
                let shape = compile_shape::<S>(shape, schema)?;
                Some(CompiledComponent::QualifiedValueShape(
                    QualifiedValueShape::new(
                        shape,
                        qualified_min_count,
                        qualified_max_count,
                        qualified_value_shapes_disjoint,
                    ),
                ))
            }
            Component::Deactivated(_b) => None,
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

/// sh:minCount specifies the minimum number of value nodes that satisfy the
/// condition. If the minimum cardinality value is 0 then this constraint is
/// always satisfied and so may be omitted.
///
/// - IRI: https://www.w3.org/TR/shacl/#MinCountConstraintComponent
/// - DEF: If the number of value nodes is less than $minCount, there is a
///   validation result.
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
pub struct And {
    shapes: Vec<CompiledShape>,
}

impl And {
    pub fn new(shapes: Vec<CompiledShape>) -> Self {
        And { shapes }
    }

    pub fn shapes(&self) -> &Vec<CompiledShape> {
        &self.shapes
    }
}

/// sh:not specifies the condition that each value node cannot conform to a
/// given shape. This is comparable to negation and the logical "not" operator.
///
/// https://www.w3.org/TR/shacl/#NotConstraintComponent
#[derive(Debug, Clone)]
pub struct Not {
    shape: Box<CompiledShape>,
}

impl Not {
    pub fn new(shape: CompiledShape) -> Self {
        Not {
            shape: Box::new(shape),
        }
    }

    pub fn shape(&self) -> &CompiledShape {
        &self.shape
    }
}

/// sh:or specifies the condition that each value node conforms to at least one
/// of the provided shapes. This is comparable to disjunction and the logical
/// "or" operator.
///
/// https://www.w3.org/TR/shacl/#AndConstraintComponent

#[derive(Debug, Clone)]
pub struct Or {
    shapes: Vec<CompiledShape>,
}

impl Or {
    pub fn new(shapes: Vec<CompiledShape>) -> Self {
        Or { shapes }
    }

    pub fn shapes(&self) -> &Vec<CompiledShape> {
        &self.shapes
    }
}

/// sh:or specifies the condition that each value node conforms to at least one
/// of the provided shapes. This is comparable to disjunction and the logical
/// "or" operator.
///
/// https://www.w3.org/TR/shacl/#XoneConstraintComponent
#[derive(Debug, Clone)]
pub struct Xone {
    shapes: Vec<CompiledShape>,
}

impl Xone {
    pub fn new(shapes: Vec<CompiledShape>) -> Self {
        Xone { shapes }
    }

    pub fn shapes(&self) -> &Vec<CompiledShape> {
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
#[derive(Debug, Clone)]
pub struct Closed {
    is_closed: bool,
    ignored_properties: Vec<IriS>,
}

impl Closed {
    pub fn new(is_closed: bool, ignored_properties: Vec<IriS>) -> Self {
        Closed {
            is_closed,
            ignored_properties,
        }
    }

    pub fn is_closed(&self) -> bool {
        self.is_closed
    }

    pub fn ignored_properties(&self) -> &Vec<IriS> {
        &self.ignored_properties
    }
}

/// sh:hasValue specifies the condition that at least one value node is equal to
///  the given RDF term.
///
/// https://www.w3.org/TR/shacl/#HasValueConstraintComponent
#[derive(Debug, Clone)]
pub struct HasValue {
    value: RDFNode,
}

impl HasValue {
    pub fn new(value: RDFNode) -> Self {
        HasValue { value }
    }

    pub fn value(&self) -> &RDFNode {
        &self.value
    }
}

/// sh:in specifies the condition that each value node is a member of a provided
/// SHACL list.
///
/// https://www.w3.org/TR/shacl/#InConstraintComponent
#[derive(Debug, Clone)]
pub struct In {
    values: Vec<RDFNode>,
}

impl In {
    pub fn new(values: Vec<RDFNode>) -> Self {
        In { values }
    }

    pub fn values(&self) -> &Vec<RDFNode> {
        &self.values
    }
}

/// sh:disjoint specifies the condition that the set of value nodes is disjoint
/// with the set of objects of the triples that have the focus node as subject
/// and the value of sh:disjoint as predicate.
///
/// https://www.w3.org/TR/shacl/#DisjointConstraintComponent
#[derive(Debug, Clone)]
pub struct Disjoint {
    iri: IriS,
}

impl Disjoint {
    pub fn new(iri: IriS) -> Self {
        Disjoint { iri }
    }

    pub fn iri(&self) -> &IriS {
        &self.iri
    }
}

/// sh:equals specifies the condition that the set of all value nodes is equal
/// to the set of objects of the triples that have the focus node as subject and
/// the value of sh:equals as predicate.
///
/// https://www.w3.org/TR/shacl/#EqualsConstraintComponent
#[derive(Debug, Clone)]
pub struct Equals {
    iri: IriS,
}

impl Equals {
    pub fn new(iri: IriS) -> Self {
        Equals { iri }
    }

    pub fn iri(&self) -> &IriS {
        &self.iri
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
pub struct LessThanOrEquals {
    iri: IriS,
}

impl LessThanOrEquals {
    pub fn new(iri: IriS) -> Self {
        LessThanOrEquals { iri }
    }

    pub fn iri(&self) -> &IriS {
        &self.iri
    }
}

/// sh:lessThan specifies the condition that each value node is smaller than all
/// the objects of the triples that have the focus node as subject and the
/// value of sh:lessThan as predicate.
///
/// https://www.w3.org/TR/shacl/#LessThanConstraintComponent
#[derive(Debug, Clone)]
pub struct LessThan {
    iri: IriS,
}

impl LessThan {
    pub fn new(iri: IriS) -> Self {
        LessThan { iri }
    }

    pub fn iri(&self) -> &IriS {
        &self.iri
    }
}

/// sh:node specifies the condition that each value node conforms to the given
/// node shape.
///
/// https://www.w3.org/TR/shacl/#NodeShapeComponent
#[derive(Debug, Clone)]
pub struct Node {
    shape: Box<CompiledShape>,
}

impl Node {
    pub fn new(shape: CompiledShape) -> Self {
        Node {
            shape: Box::new(shape),
        }
    }

    pub fn shape(&self) -> &CompiledShape {
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
#[derive(Debug, Clone)]
pub struct QualifiedValueShape {
    shape: Box<CompiledShape>,
    qualified_min_count: Option<isize>,
    qualified_max_count: Option<isize>,
    qualified_value_shapes_disjoint: Option<bool>,
}

impl QualifiedValueShape {
    pub fn new(
        shape: CompiledShape,
        qualified_min_count: Option<isize>,
        qualified_max_count: Option<isize>,
        qualified_value_shapes_disjoint: Option<bool>,
    ) -> Self {
        QualifiedValueShape {
            shape: Box::new(shape),
            qualified_min_count,
            qualified_max_count,
            qualified_value_shapes_disjoint,
        }
    }

    pub fn shape(&self) -> &CompiledShape {
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
#[derive(Debug, Clone)]
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

/// sh:property can be used to specify that each value node has a given property
/// shape.
///
/// https://www.w3.org/TR/shacl/#PropertyShapeComponent
#[derive(Debug, Clone)]
pub struct Pattern {
    pattern: String,
    flags: Option<String>,
    regex: Regex,
}

impl Pattern {
    pub fn new(pattern: String, flags: Option<String>) -> Self {
        let regex = if let Some(_flags) = &flags {
            Regex::new(&pattern).expect("Invalid regex pattern")
        } else {
            Regex::new(&pattern).expect("Invalid regex pattern")
        };
        Pattern {
            pattern,
            flags,
            regex,
        }
    }

    pub fn pattern(&self) -> &String {
        &self.pattern
    }

    pub fn flags(&self) -> &Option<String> {
        &self.flags
    }

    pub fn regex(&self) -> &Regex {
        &self.regex
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

/// The condition specified by sh:class is that each value node is a SHACL
/// instance of a given type.
///
/// https://www.w3.org/TR/shacl/#ClassConstraintComponent
#[derive(Debug, Clone)]
pub struct Class {
    class_rule: RDFNode,
}

impl Class {
    pub fn new(class_rule: RDFNode) -> Self {
        Class { class_rule }
    }

    pub fn class_rule(&self) -> &RDFNode {
        &self.class_rule
    }
}

/// sh:datatype specifies a condition to be satisfied with regards to the
/// datatype of each value node.
///
/// https://www.w3.org/TR/shacl/#ClassConstraintComponent
#[derive(Debug, Clone)]
pub struct Datatype {
    datatype: IriS,
}

impl Datatype {
    pub fn new(datatype: IriS) -> Self {
        Datatype { datatype }
    }

    pub fn datatype(&self) -> &IriS {
        &self.datatype
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

/// https://www.w3.org/TR/shacl/#MaxExclusiveConstraintComponent
#[derive(Debug, Clone)]
pub struct MaxExclusive {
    max_exclusive: SLiteral,
}

impl MaxExclusive {
    pub fn new(literal: SLiteral) -> Self {
        MaxExclusive {
            max_exclusive: literal,
        }
    }

    pub fn max_exclusive(&self) -> &SLiteral {
        &self.max_exclusive
    }
}

/// https://www.w3.org/TR/shacl/#MaxInclusiveConstraintComponent
#[derive(Debug, Clone)]
pub struct MaxInclusive {
    max_inclusive: SLiteral,
}

impl MaxInclusive {
    pub fn new(literal: SLiteral) -> Self {
        MaxInclusive {
            max_inclusive: literal,
        }
    }

    pub fn max_inclusive(&self) -> &SLiteral {
        &self.max_inclusive
    }
}

/// https://www.w3.org/TR/shacl/#MinExclusiveConstraintComponent
#[derive(Debug, Clone)]
pub struct MinExclusive {
    min_exclusive: SLiteral,
}

impl MinExclusive {
    pub fn new(literal: SLiteral) -> Self {
        MinExclusive {
            min_exclusive: literal,
        }
    }

    pub fn min_exclusive(&self) -> &SLiteral {
        &self.min_exclusive
    }
}

/// https://www.w3.org/TR/shacl/#MinInclusiveConstraintComponent
#[derive(Debug, Clone)]
pub struct MinInclusive {
    min_inclusive: SLiteral,
}

impl MinInclusive {
    pub fn new(literal: SLiteral) -> Self {
        MinInclusive {
            min_inclusive: literal,
        }
    }

    pub fn min_inclusive_value(&self) -> &SLiteral {
        &self.min_inclusive
    }
}

impl From<&CompiledComponent> for IriS {
    fn from(value: &CompiledComponent) -> Self {
        match value {
            CompiledComponent::Class(_) => sh_class().clone(),
            CompiledComponent::Datatype(_) => sh_datatype().clone(),
            CompiledComponent::NodeKind(_) => sh_node_kind().clone(),
            CompiledComponent::MinCount(_) => sh_min_count().clone(),
            CompiledComponent::MaxCount(_) => sh_max_count().clone(),
            CompiledComponent::MinExclusive(_) => sh_min_exclusive().clone(),
            CompiledComponent::MaxExclusive(_) => sh_max_exclusive().clone(),
            CompiledComponent::MinInclusive(_) => sh_min_inclusive().clone(),
            CompiledComponent::MaxInclusive(_) => sh_max_inclusive().clone(),
            CompiledComponent::MinLength(_) => sh_min_length().clone(),
            CompiledComponent::MaxLength(_) => sh_max_length().clone(),
            CompiledComponent::Pattern { .. } => sh_pattern().clone(),
            CompiledComponent::UniqueLang(_) => sh_unique_lang().clone(),
            CompiledComponent::LanguageIn { .. } => sh_language_in().clone(),
            CompiledComponent::Equals(_) => sh_equals().clone(),
            CompiledComponent::Disjoint(_) => sh_disjoint().clone(),
            CompiledComponent::LessThan(_) => sh_less_than().clone(),
            CompiledComponent::LessThanOrEquals(_) => sh_less_than_or_equals().clone(),
            CompiledComponent::Or { .. } => sh_or().clone(),
            CompiledComponent::And { .. } => sh_and().clone(),
            CompiledComponent::Not { .. } => sh_not().clone(),
            CompiledComponent::Xone { .. } => sh_xone().clone(),
            CompiledComponent::Node { .. } => sh_node().clone(),
            CompiledComponent::HasValue { .. } => sh_has_value().clone(),
            CompiledComponent::In { .. } => sh_in().clone(),
            CompiledComponent::QualifiedValueShape { .. } => sh_qualified_value_shape().clone(),
        }
    }
}

impl Display for CompiledComponent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompiledComponent::Class(cls) => write!(f, "Class {:}", cls.class_rule()),
            CompiledComponent::Datatype(dt) => write!(f, "Datatype: {}", dt.datatype()),
            CompiledComponent::NodeKind(nk) => write!(f, "NodeKind: {}", nk.node_kind()),
            CompiledComponent::MinCount(n) => write!(f, " {n}"),
            CompiledComponent::MaxCount(n) => write!(f, " {n}"),
            CompiledComponent::MinExclusive(n) => write!(f, " {n}"),
            CompiledComponent::MaxExclusive(n) => write!(f, " {n}"),
            CompiledComponent::MinInclusive(n) => write!(f, " {n}"),
            CompiledComponent::MaxInclusive(n) => write!(f, " {n}"),
            CompiledComponent::MinLength(n) => write!(f, " {n}"),
            CompiledComponent::MaxLength(n) => write!(f, " {n}"),
            CompiledComponent::Pattern(pat) => write!(f, " {pat}"),
            CompiledComponent::UniqueLang(ul) => write!(f, " {ul}"),
            CompiledComponent::LanguageIn(l) => write!(f, " {l}"),
            CompiledComponent::Equals(p) => write!(f, " {p}"),
            CompiledComponent::Disjoint(p) => write!(f, " {p}"),
            CompiledComponent::LessThan(p) => write!(f, " {p}"),
            CompiledComponent::LessThanOrEquals(p) => write!(f, " {p}"),
            CompiledComponent::Or { .. } => write!(f, "Or"),
            CompiledComponent::And { .. } => write!(f, "And"),
            CompiledComponent::Not { .. } => write!(f, "Not"),
            CompiledComponent::Xone { .. } => write!(f, "Xone"),
            CompiledComponent::Node { .. } => write!(f, "Node"),
            CompiledComponent::HasValue(value) => write!(f, " {}", value),
            CompiledComponent::In(vs) => write!(f, " {}", vs),
            CompiledComponent::QualifiedValueShape(qvs) => {
                write!(f, " {}", qvs)
            }
        }
    }
}

impl Display for MinCount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MinCount: {}", self.min_count())
    }
}

impl Display for MaxCount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MaxCount: {}", self.max_count())
    }
}

impl Display for And {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "And: {} shapes", self.shapes().len())
    }
}

impl Display for Not {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Not: {}", self.shape())
    }
}

impl Display for Or {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Or: {} shapes", self.shapes().len())
    }
}

impl Display for Equals {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Equals: {}", self.iri())
    }
}

impl Display for Disjoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Disjoint: {}", self.iri())
    }
}

impl Display for LessThan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LessThan: {}", self.iri())
    }
}

impl Display for LessThanOrEquals {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LessThanOrEquals: {}", self.iri())
    }
}

impl Display for MinInclusive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MinInclusive: {}", self.min_inclusive)
    }
}

impl Display for MaxInclusive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MaxInclusive: {}", self.max_inclusive())
    }
}

impl Display for MinExclusive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MinExclusive: {}", self.min_exclusive())
    }
}

impl Display for MaxExclusive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MaxExclusive: {}", self.max_exclusive())
    }
}

impl Display for MinLength {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MinLength: {}", self.min_length())
    }
}

impl Display for MaxLength {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MaxLength: {}", self.max_length())
    }
}

impl Display for In {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let values = self
            .values()
            .iter()
            .map(|v| format!("{v}"))
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "In: [{}]", values)
    }
}

impl Display for HasValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HasValue: {}", self.value())
    }
}

impl Display for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(flags) = &self.flags {
            write!(f, "Pattern: /{}/{}", self.pattern(), flags)
        } else {
            write!(f, "Pattern: /{}/", self.pattern())
        }
    }
}

impl Display for QualifiedValueShape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "QualifiedValueShape: shape: {}, qualifiedMinCount: {:?}, qualifiedMaxCount: {:?}, qualifiedValueShapesDisjoint: {:?}",
            self.shape(),
            self.qualified_min_count(),
            self.qualified_max_count(),
            self.qualified_value_shapes_disjoint()
        )
    }
}

impl Display for UniqueLang {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "UniqueLang: {}", self.unique_lang())
    }
}

impl Display for LanguageIn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let langs = self
            .langs()
            .iter()
            .map(|l| l.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "LanguageIn: [{}]", langs)
    }
}
