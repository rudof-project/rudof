use crate::types::{NodeKind, Value};
use iri_s::IriS;
use itertools::Itertools;
use prefixmap::IriRef;
use rudof_rdf::rdf_core::term::Object;
use rudof_rdf::rdf_core::term::literal::{ConcreteLiteral, Lang};
use rudof_rdf::rdf_core::vocabs::ShaclVocab;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};

// TODO - For node expr only derive Debug (maybe)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ASTComponent {
    Class(Object),
    Datatype(IriRef),
    NodeKind(NodeKind),
    MinCount(isize),
    MaxCount(isize),
    MinExclusive(ConcreteLiteral),
    MaxExclusive(ConcreteLiteral),
    MinInclusive(ConcreteLiteral),
    MaxInclusive(ConcreteLiteral),
    MinLength(isize),
    MaxLength(isize),
    Pattern {
        pattern: String,
        flags: Option<String>,
    },
    UniqueLang(bool),
    LanguageIn(Vec<Lang>),
    Equals(IriRef),
    Disjoint(IriRef),
    LessThan(IriRef),
    LessThanOrEquals(IriRef),
    Or(Vec<Object>),
    And(Vec<Object>),
    Not(Object),
    Xone(Vec<Object>),
    Closed {
        is_closed: bool,
        ignored_properties: HashSet<IriS>,
    },
    Node(Object),
    HasValue(Value),
    In(Vec<Value>),
    QualifiedValueShape {
        shape: Object,
        q_min_count: Option<isize>,
        q_max_count: Option<isize>,
        disjoint: Option<bool>,
        siblings: Vec<Object>,
    },
    Deactivated(bool), // TODO - Replace with node expr
}

impl Display for ASTComponent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ASTComponent::Class(o) => write!(f, "class({o})"),
            ASTComponent::Datatype(iri) => write!(f, "datatype({iri})"),
            ASTComponent::NodeKind(node) => write!(f, "nodeKind({node})"),
            ASTComponent::MinCount(qty) => write!(f, "minCount({qty})"),
            ASTComponent::MaxCount(qty) => write!(f, "maxCount({qty})"),
            ASTComponent::MinExclusive(lit) => write!(f, "minExclusive({lit})"),
            ASTComponent::MaxExclusive(lit) => write!(f, "maxExclusive({lit})"),
            ASTComponent::MinInclusive(lit) => write!(f, "minInclusive({lit})"),
            ASTComponent::MaxInclusive(lit) => write!(f, "maxInclusive({lit})"),
            ASTComponent::MinLength(l) => write!(f, "minLength({l})"),
            ASTComponent::MaxLength(l) => write!(f, "maxLength({l})"),
            ASTComponent::Pattern { pattern, flags } => match flags {
                None => write!(f, "pattern({pattern})"),
                Some(flags) => write!(f, "pattern({pattern}, {flags})"),
            },
            ASTComponent::UniqueLang(l) => write!(f, "uniqueLang({l})"),
            ASTComponent::LanguageIn(l) => {
                let str = l.iter().map(|s| s.to_string()).join(", ");
                write!(f, "languageIn[{str}]")
            },
            ASTComponent::Equals(iri) => write!(f, "equals({iri})"),
            ASTComponent::Disjoint(iri) => write!(f, "disjoint({iri})"),
            ASTComponent::LessThan(iri) => write!(f, "lessThan({iri})"),
            ASTComponent::LessThanOrEquals(iri) => write!(f, "lessThanOrEquals({iri})"),
            ASTComponent::Or(obj) => {
                let str = obj.iter().map(|s| s.to_string()).join(", ");
                write!(f, "or[{str}]")
            },
            ASTComponent::And(obj) => {
                let str = obj.iter().map(|s| s.to_string()).join(", ");
                write!(f, "and[{str}]")
            },
            ASTComponent::Not(obj) => write!(f, "not({obj})"),
            ASTComponent::Xone(obj) => {
                let str = obj.iter().map(|s| s.to_string()).join(", ");
                write!(f, "xone[{str}]")
            },
            ASTComponent::Closed {
                is_closed,
                ignored_properties,
            } => write!(
                f,
                "closed({is_closed}{})",
                if ignored_properties.is_empty() {
                    "".to_string()
                } else {
                    format!(
                        ", Ignored props: [{}]",
                        ignored_properties.iter().map(|p| p.to_string()).join(", ")
                    )
                }
            ),
            ASTComponent::Node(obj) => write!(f, "node({obj})"),
            ASTComponent::HasValue(v) => write!(f, "hasValue({v})"),
            ASTComponent::In(v) => {
                let str = v.iter().map(|v| v.to_string()).join(", ");
                write!(f, "in[{str}]")
            },
            ASTComponent::QualifiedValueShape {
                shape,
                disjoint,
                siblings,
                q_max_count,
                q_min_count,
            } => {
                write!(
                    f,
                    "qualifiedValueShape(shape: {shape}, qualified_min_count: {q_min_count:?}, qualified_max_count: {q_max_count:?}, qualified_value_shape_disjoint: {disjoint:?}{}",
                    if siblings.is_empty() {
                        "".to_string()
                    } else {
                        format!(", siblings: [{}]", siblings.iter().map(|s| s.to_string()).join(", "))
                    }
                )
            },
            ASTComponent::Deactivated(b) => write!(f, "deactivated({b})"),
        }
    }
}

impl From<ASTComponent> for IriS {
    fn from(value: ASTComponent) -> Self {
        match value {
            ASTComponent::Class(_) => ShaclVocab::sh_class().clone(),
            ASTComponent::Datatype(_) => ShaclVocab::sh_datatype().clone(),
            ASTComponent::NodeKind(_) => ShaclVocab::sh_node_kind().clone(),
            ASTComponent::MinCount(_) => ShaclVocab::sh_min_count().clone(),
            ASTComponent::MaxCount(_) => ShaclVocab::sh_max_count().clone(),
            ASTComponent::MinExclusive(_) => ShaclVocab::sh_min_exclusive().clone(),
            ASTComponent::MaxExclusive(_) => ShaclVocab::sh_max_exclusive().clone(),
            ASTComponent::MinInclusive(_) => ShaclVocab::sh_min_inclusive().clone(),
            ASTComponent::MaxInclusive(_) => ShaclVocab::sh_max_inclusive().clone(),
            ASTComponent::MinLength(_) => ShaclVocab::sh_min_length().clone(),
            ASTComponent::MaxLength(_) => ShaclVocab::sh_max_length().clone(),
            ASTComponent::Pattern { .. } => ShaclVocab::sh_pattern().clone(),
            ASTComponent::UniqueLang(_) => ShaclVocab::sh_unique_lang().clone(),
            ASTComponent::LanguageIn(_) => ShaclVocab::sh_language_in().clone(),
            ASTComponent::Equals(_) => ShaclVocab::sh_equals().clone(),
            ASTComponent::Disjoint(_) => ShaclVocab::sh_disjoint().clone(),
            ASTComponent::LessThan(_) => ShaclVocab::sh_less_than().clone(),
            ASTComponent::LessThanOrEquals(_) => ShaclVocab::sh_less_than_or_equals().clone(),
            ASTComponent::Or(_) => ShaclVocab::sh_or().clone(),
            ASTComponent::And(_) => ShaclVocab::sh_and().clone(),
            ASTComponent::Not(_) => ShaclVocab::sh_not().clone(),
            ASTComponent::Xone(_) => ShaclVocab::sh_xone().clone(),
            ASTComponent::Closed { .. } => ShaclVocab::sh_closed().clone(),
            ASTComponent::Node(_) => ShaclVocab::sh_node().clone(),
            ASTComponent::HasValue(_) => ShaclVocab::sh_has_value().clone(),
            ASTComponent::In(_) => ShaclVocab::sh_in().clone(),
            ASTComponent::QualifiedValueShape { .. } => ShaclVocab::sh_qualified_value_shape().clone(),
            ASTComponent::Deactivated(_) => ShaclVocab::sh_deactivated().clone(),
        }
    }
}
