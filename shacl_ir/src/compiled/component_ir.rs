use std::fmt::Display;

use super::compile_shape;
use super::compile_shapes;
use super::compiled_shacl_error::CompiledShaclError;
use super::convert_iri_ref;
use super::convert_value;
use super::shape::ShapeIR;
use iri_s::IriS;
use shacl_ast::Schema;
use shacl_ast::component::Component;
use shacl_ast::node_kind::NodeKind;
use shacl_ast::shacl_vocab::{
    sh_and, sh_class, sh_datatype, sh_disjoint, sh_equals, sh_has_value, sh_in, sh_language_in,
    sh_less_than, sh_less_than_or_equals, sh_max_count, sh_max_exclusive, sh_max_inclusive,
    sh_max_length, sh_min_count, sh_min_exclusive, sh_min_inclusive, sh_min_length, sh_node,
    sh_node_kind, sh_not, sh_or, sh_pattern, sh_qualified_value_shape, sh_unique_lang, sh_xone,
};
use srdf::RDFNode;
use srdf::Rdf;
use srdf::SLiteral;
use srdf::SRegex;
use srdf::lang::Lang;

#[derive(Debug, Clone)]
pub enum ComponentIR {
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

impl ComponentIR {
    pub fn compile<S: Rdf>(
        component: Component,
        schema: &Schema<S>,
    ) -> Result<Option<Self>, Box<CompiledShaclError>> {
        let component = match component {
            Component::Class(object) => {
                let class_rule = object;
                Some(ComponentIR::Class(Class::new(class_rule)))
            }
            Component::Datatype(iri_ref) => {
                let iri_ref = convert_iri_ref(iri_ref)?;
                Some(ComponentIR::Datatype(Datatype::new(iri_ref)))
            }
            Component::NodeKind(node_kind) => Some(ComponentIR::NodeKind(Nodekind::new(node_kind))),
            Component::MinCount(count) => Some(ComponentIR::MinCount(MinCount::new(count))),
            Component::MaxCount(count) => Some(ComponentIR::MaxCount(MaxCount::new(count))),
            Component::MinExclusive(literal) => {
                Some(ComponentIR::MinExclusive(MinExclusive::new(literal)))
            }
            Component::MaxExclusive(literal) => {
                Some(ComponentIR::MaxExclusive(MaxExclusive::new(literal)))
            }
            Component::MinInclusive(literal) => {
                Some(ComponentIR::MinInclusive(MinInclusive::new(literal)))
            }
            Component::MaxInclusive(literal) => {
                Some(ComponentIR::MaxInclusive(MaxInclusive::new(literal)))
            }
            Component::MinLength(length) => Some(ComponentIR::MinLength(MinLength::new(length))),
            Component::MaxLength(length) => Some(ComponentIR::MaxLength(MaxLength::new(length))),
            Component::Pattern { pattern, flags } => {
                let pattern = Pattern::new(pattern, flags)?;
                Some(ComponentIR::Pattern(pattern))
            }
            Component::UniqueLang(lang) => Some(ComponentIR::UniqueLang(UniqueLang::new(lang))),
            Component::LanguageIn { langs } => {
                Some(ComponentIR::LanguageIn(LanguageIn::new(langs)))
            }
            Component::Equals(iri_ref) => {
                let iri_ref = convert_iri_ref(iri_ref)?;
                Some(ComponentIR::Equals(Equals::new(iri_ref)))
            }
            Component::Disjoint(iri_ref) => {
                let iri_ref = convert_iri_ref(iri_ref)?;
                Some(ComponentIR::Disjoint(Disjoint::new(iri_ref)))
            }
            Component::LessThan(iri_ref) => {
                let iri_ref = convert_iri_ref(iri_ref)?;
                Some(ComponentIR::LessThan(LessThan::new(iri_ref)))
            }
            Component::LessThanOrEquals(iri_ref) => {
                let iri_ref = convert_iri_ref(iri_ref)?;
                Some(ComponentIR::LessThanOrEquals(LessThanOrEquals::new(
                    iri_ref,
                )))
            }
            Component::Or { shapes } => Some(ComponentIR::Or(Or::new(compile_shapes::<S>(
                shapes, schema,
            )?))),
            Component::And { shapes } => Some(ComponentIR::And(And::new(compile_shapes::<S>(
                shapes, schema,
            )?))),
            Component::Not { shape } => {
                let shape = compile_shape::<S>(shape, schema)?;
                Some(ComponentIR::Not(Not::new(shape)))
            }
            Component::Xone { shapes } => Some(ComponentIR::Xone(Xone::new(compile_shapes::<S>(
                shapes, schema,
            )?))),
            Component::Closed { .. } => None,
            Component::Node { shape } => {
                let shape = compile_shape::<S>(shape, schema)?;
                Some(ComponentIR::Node(Node::new(shape)))
            }
            Component::HasValue { value } => {
                let term = convert_value(value)?;
                Some(ComponentIR::HasValue(HasValue::new(term)))
            }
            Component::In { values } => {
                let terms = values
                    .into_iter()
                    .map(convert_value)
                    .collect::<Result<Vec<_>, _>>()?;
                Some(ComponentIR::In(In::new(terms)))
            }
            Component::QualifiedValueShape {
                shape,
                q_min_count,
                q_max_count,
                disjoint,
                siblings,
            } => {
                let shape = compile_shape::<S>(shape, schema)?;
                let mut compiled_siblings = Vec::new();
                for sibling in siblings.iter() {
                    let compiled_sibling = compile_shape(sibling.clone(), schema)?;
                    compiled_siblings.push(compiled_sibling);
                }
                Some(ComponentIR::QualifiedValueShape(QualifiedValueShape::new(
                    shape,
                    q_min_count,
                    q_max_count,
                    disjoint,
                    compiled_siblings,
                )))
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
    shapes: Vec<ShapeIR>,
}

impl And {
    pub fn new(shapes: Vec<ShapeIR>) -> Self {
        And { shapes }
    }

    pub fn shapes(&self) -> &Vec<ShapeIR> {
        &self.shapes
    }
}

/// sh:not specifies the condition that each value node cannot conform to a
/// given shape. This is comparable to negation and the logical "not" operator.
///
/// https://www.w3.org/TR/shacl/#NotConstraintComponent
#[derive(Debug, Clone)]
pub struct Not {
    shape: Box<ShapeIR>,
}

impl Not {
    pub fn new(shape: ShapeIR) -> Self {
        Not {
            shape: Box::new(shape),
        }
    }

    pub fn shape(&self) -> &ShapeIR {
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
    shapes: Vec<ShapeIR>,
}

impl Or {
    pub fn new(shapes: Vec<ShapeIR>) -> Self {
        Or { shapes }
    }

    pub fn shapes(&self) -> &Vec<ShapeIR> {
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
    shapes: Vec<ShapeIR>,
}

impl Xone {
    pub fn new(shapes: Vec<ShapeIR>) -> Self {
        Xone { shapes }
    }

    pub fn shapes(&self) -> &Vec<ShapeIR> {
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
    shape: Box<ShapeIR>,
}

impl Node {
    pub fn new(shape: ShapeIR) -> Self {
        Node {
            shape: Box::new(shape),
        }
    }

    pub fn shape(&self) -> &ShapeIR {
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
    shape: Box<ShapeIR>,
    qualified_min_count: Option<isize>,
    qualified_max_count: Option<isize>,
    qualified_value_shapes_disjoint: Option<bool>,
    siblings: Vec<ShapeIR>,
}

impl QualifiedValueShape {
    pub fn new(
        shape: ShapeIR,
        qualified_min_count: Option<isize>,
        qualified_max_count: Option<isize>,
        qualified_value_shapes_disjoint: Option<bool>,
        siblings: Vec<ShapeIR>,
    ) -> Self {
        QualifiedValueShape {
            shape: Box::new(shape),
            qualified_min_count,
            qualified_max_count,
            qualified_value_shapes_disjoint,
            siblings,
        }
    }

    pub fn shape(&self) -> &ShapeIR {
        &self.shape
    }

    pub fn qualified_min_count(&self) -> Option<isize> {
        self.qualified_min_count
    }

    pub fn qualified_max_count(&self) -> Option<isize> {
        self.qualified_max_count
    }

    pub fn siblings(&self) -> &Vec<ShapeIR> {
        &self.siblings
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
    regex: SRegex,
}

impl Pattern {
    pub fn new(pattern: String, flags: Option<String>) -> Result<Self, CompiledShaclError> {
        let regex = SRegex::new(&pattern, flags.as_deref()).map_err(|e| {
            CompiledShaclError::InvalidRegex {
                pattern: pattern.clone(),
                flags: flags.clone(),
                error: e,
            }
        })?;
        Ok(Pattern {
            pattern,
            flags,
            regex,
        })
    }

    pub fn pattern(&self) -> &String {
        &self.pattern
    }

    pub fn flags(&self) -> &Option<String> {
        &self.flags
    }

    pub fn regex(&self) -> &SRegex {
        &self.regex
    }

    pub fn match_str(&self, str: &str) -> bool {
        self.regex().is_match(str)
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

impl From<&ComponentIR> for IriS {
    fn from(value: &ComponentIR) -> Self {
        match value {
            ComponentIR::Class(_) => sh_class().clone(),
            ComponentIR::Datatype(_) => sh_datatype().clone(),
            ComponentIR::NodeKind(_) => sh_node_kind().clone(),
            ComponentIR::MinCount(_) => sh_min_count().clone(),
            ComponentIR::MaxCount(_) => sh_max_count().clone(),
            ComponentIR::MinExclusive(_) => sh_min_exclusive().clone(),
            ComponentIR::MaxExclusive(_) => sh_max_exclusive().clone(),
            ComponentIR::MinInclusive(_) => sh_min_inclusive().clone(),
            ComponentIR::MaxInclusive(_) => sh_max_inclusive().clone(),
            ComponentIR::MinLength(_) => sh_min_length().clone(),
            ComponentIR::MaxLength(_) => sh_max_length().clone(),
            ComponentIR::Pattern { .. } => sh_pattern().clone(),
            ComponentIR::UniqueLang(_) => sh_unique_lang().clone(),
            ComponentIR::LanguageIn { .. } => sh_language_in().clone(),
            ComponentIR::Equals(_) => sh_equals().clone(),
            ComponentIR::Disjoint(_) => sh_disjoint().clone(),
            ComponentIR::LessThan(_) => sh_less_than().clone(),
            ComponentIR::LessThanOrEquals(_) => sh_less_than_or_equals().clone(),
            ComponentIR::Or { .. } => sh_or().clone(),
            ComponentIR::And { .. } => sh_and().clone(),
            ComponentIR::Not { .. } => sh_not().clone(),
            ComponentIR::Xone { .. } => sh_xone().clone(),
            ComponentIR::Node { .. } => sh_node().clone(),
            ComponentIR::HasValue { .. } => sh_has_value().clone(),
            ComponentIR::In { .. } => sh_in().clone(),
            ComponentIR::QualifiedValueShape { .. } => sh_qualified_value_shape().clone(),
        }
    }
}

impl Display for ComponentIR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ComponentIR::Class(cls) => write!(f, " {cls}"),
            ComponentIR::Datatype(dt) => write!(f, " {dt}"),
            ComponentIR::NodeKind(nk) => write!(f, " {nk}"),
            ComponentIR::MinCount(n) => write!(f, " {n}"),
            ComponentIR::MaxCount(n) => write!(f, " {n}"),
            ComponentIR::MinExclusive(n) => write!(f, " {n}"),
            ComponentIR::MaxExclusive(n) => write!(f, " {n}"),
            ComponentIR::MinInclusive(n) => write!(f, " {n}"),
            ComponentIR::MaxInclusive(n) => write!(f, " {n}"),
            ComponentIR::MinLength(n) => write!(f, " {n}"),
            ComponentIR::MaxLength(n) => write!(f, " {n}"),
            ComponentIR::Pattern(pat) => write!(f, " {pat}"),
            ComponentIR::UniqueLang(ul) => write!(f, " {ul}"),
            ComponentIR::LanguageIn(l) => write!(f, " {l}"),
            ComponentIR::Equals(p) => write!(f, " {p}"),
            ComponentIR::Disjoint(p) => write!(f, " {p}"),
            ComponentIR::LessThan(p) => write!(f, " {p}"),
            ComponentIR::LessThanOrEquals(p) => write!(f, " {p}"),
            ComponentIR::Or(or) => write!(f, " {or}"),
            ComponentIR::And(and) => write!(f, " {and}"),
            ComponentIR::Not(not) => write!(f, " {not}"),
            ComponentIR::Xone(xone) => write!(f, " {xone}"),
            ComponentIR::Node(node) => write!(f, " {node}"),
            ComponentIR::HasValue(value) => write!(f, " HasValue({value})"),
            ComponentIR::In(vs) => write!(f, " {}", vs),
            ComponentIR::QualifiedValueShape(qvs) => {
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

impl Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Class: {}", self.class_rule())
    }
}

impl Display for Datatype {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Datatype: {}", self.datatype())
    }
}

impl Display for Nodekind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NodeKind: {:?}", self.node_kind())
    }
}

impl Display for Xone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Xone [{}]",
            self.shapes()
                .iter()
                .map(|s| s.id().to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Node [{}]", self.shape.id())
    }
}

impl Display for And {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "And [{}]",
            self.shapes()
                .iter()
                .map(|s| s.id().to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

impl Display for Not {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Not [{}]", self.shape.id())
    }
}

impl Display for Or {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Or[{}]",
            self.shapes()
                .iter()
                .map(|s| s.id().to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
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
            "QualifiedValueShape: shape: {}, qualifiedMinCount: {:?}, qualifiedMaxCount: {:?}, qualifiedValueShapesDisjoint: {:?}{}",
            self.shape().id(),
            self.qualified_min_count(),
            self.qualified_max_count(),
            self.qualified_value_shapes_disjoint(),
            if self.siblings().is_empty() {
                "".to_string()
            } else {
                format!(
                    ", siblings: [{}]",
                    self.siblings()
                        .iter()
                        .map(|s| s.to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
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
