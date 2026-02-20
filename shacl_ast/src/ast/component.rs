use crate::{ShaclVocab, node_kind::NodeKind, value::Value};
use iri_s::{IriS, iri};
use itertools::Itertools;
use prefixmap::IriRef;
use rudof_rdf::rdf_core::{
    BuildRDF,
    term::{
        Object,
        literal::{ConcreteLiteral, Lang},
    },
};
use std::collections::HashSet;
use std::fmt::Display;

// #[derive(Debug)] // TODO - For Node Expr, do not clean
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Component {
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
    LanguageIn {
        langs: Vec<Lang>,
    },
    Equals(IriRef),
    Disjoint(IriRef),
    LessThan(IriRef),
    LessThanOrEquals(IriRef),
    Or {
        shapes: Vec<Object>,
    },
    And {
        shapes: Vec<Object>,
    },
    Not {
        shape: Object,
    },
    Xone {
        shapes: Vec<Object>,
    },
    Closed {
        is_closed: bool,
        ignored_properties: HashSet<IriS>,
    },
    Node {
        shape: Object,
    },
    HasValue {
        value: Value,
    },
    In {
        values: Vec<Value>,
    },
    QualifiedValueShape {
        shape: Object,
        q_min_count: Option<isize>,
        q_max_count: Option<isize>,
        disjoint: Option<bool>,
        siblings: Vec<Object>,
    },
    Deactivated(bool), // TODO - Change to Node Expr
}

impl Component {
    pub fn write<RDF>(&self, rdf_node: &Object, rdf: &mut RDF) -> Result<(), RDF::Err>
    where
        RDF: BuildRDF,
    {
        match self {
            Self::Class(rdf_node) => {
                Self::write_term(&rdf_node.clone().into(), ShaclVocab::SH_CLASS, rdf_node, rdf)?;
            },
            Self::Datatype(iri) => {
                Self::write_iri(iri, ShaclVocab::SH_DATATYPE, rdf_node, rdf)?;
            },
            Self::NodeKind(node_kind) => {
                let iri = match &node_kind {
                    NodeKind::Iri => ShaclVocab::SH_IRI,

                    _ => unimplemented!(),
                };

                Self::write_iri(&IriRef::Iri(iri!(iri)), ShaclVocab::SH_DATATYPE, rdf_node, rdf)?;
            },
            Self::MinCount(value) => {
                Self::write_integer(*value, ShaclVocab::SH_MIN_COUNT, rdf_node, rdf)?;
            },
            Self::MaxCount(value) => {
                Self::write_integer(*value, ShaclVocab::SH_MAX_COUNT, rdf_node, rdf)?;
            },
            Self::MinExclusive(value) => {
                Self::write_literal(value, ShaclVocab::SH_MIN_EXCLUSIVE, rdf_node, rdf)?;
            },
            Self::MaxExclusive(value) => {
                Self::write_literal(value, ShaclVocab::SH_MAX_EXCLUSIVE, rdf_node, rdf)?;
            },
            Self::MinInclusive(value) => {
                Self::write_literal(value, ShaclVocab::SH_MIN_INCLUSIVE, rdf_node, rdf)?;
            },
            Self::MaxInclusive(value) => {
                Self::write_literal(value, ShaclVocab::SH_MAX_INCLUSIVE, rdf_node, rdf)?;
            },
            Self::MinLength(value) => {
                Self::write_integer(*value, ShaclVocab::SH_MIN_LENGTH, rdf_node, rdf)?;
            },
            Self::MaxLength(value) => {
                Self::write_integer(*value, ShaclVocab::SH_MAX_LENGTH, rdf_node, rdf)?;
            },
            Self::Pattern { pattern, flags } => {
                Self::write_literal(&ConcreteLiteral::str(pattern), ShaclVocab::SH_PATTERN, rdf_node, rdf)?;
                if let Some(flags) = flags {
                    Self::write_literal(&ConcreteLiteral::str(flags), ShaclVocab::SH_FLAGS, rdf_node, rdf)?;
                }
            },
            Self::UniqueLang(value) => {
                Self::write_boolean(*value, ShaclVocab::SH_UNIQUE_LANG, rdf_node, rdf)?;
            },
            Self::LanguageIn { langs } => {
                langs.iter().try_for_each(|lang| {
                    Self::write_literal(
                        &ConcreteLiteral::str(&lang.to_string()),
                        ShaclVocab::SH_LANGUAGE_IN,
                        rdf_node,
                        rdf,
                    )
                })?;
            },
            Self::Equals(iri) => {
                Self::write_iri(iri, ShaclVocab::SH_EQUALS, rdf_node, rdf)?;
            },
            Self::Disjoint(iri) => {
                Self::write_iri(iri, ShaclVocab::SH_DISJOINT, rdf_node, rdf)?;
            },
            Self::LessThan(iri) => {
                Self::write_iri(iri, ShaclVocab::SH_LESS_THAN, rdf_node, rdf)?;
            },
            Self::LessThanOrEquals(iri) => {
                Self::write_iri(iri, ShaclVocab::SH_LESS_THAN_OR_EQUALS, rdf_node, rdf)?;
            },
            Self::Or { shapes } => {
                shapes
                    .iter()
                    .try_for_each(|shape| Self::write_term(&shape.clone().into(), ShaclVocab::SH_OR, rdf_node, rdf))?;
            },
            Self::And { shapes } => {
                shapes
                    .iter()
                    .try_for_each(|shape| Self::write_term(&shape.clone().into(), ShaclVocab::SH_AND, rdf_node, rdf))?;
            },
            Self::Not { shape } => {
                Self::write_term(&shape.clone().into(), ShaclVocab::SH_PATTERN, rdf_node, rdf)?;
            },
            Self::Xone { shapes } => {
                shapes.iter().try_for_each(|shape| {
                    Self::write_term(&shape.clone().into(), ShaclVocab::SH_XONE, rdf_node, rdf)
                })?;
            },
            Self::Closed {
                is_closed,
                ignored_properties,
            } => {
                Self::write_boolean(*is_closed, ShaclVocab::SH_CLOSED, rdf_node, rdf)?;

                ignored_properties.iter().try_for_each(|iri| {
                    let iri_ref = IriRef::Iri(iri.clone());
                    Self::write_iri(&iri_ref, ShaclVocab::SH_IGNORED_PROPERTIES, rdf_node, rdf)
                })?;
            },
            Self::Node { shape } => {
                Self::write_term(&shape.clone().into(), ShaclVocab::SH_NODE, rdf_node, rdf)?;
            },
            Self::HasValue { value } => match value {
                Value::Iri(iri) => {
                    Self::write_iri(iri, ShaclVocab::SH_HAS_VALUE, rdf_node, rdf)?;
                },
                Value::Literal(literal) => {
                    Self::write_literal(
                        &ConcreteLiteral::str(&literal.to_string()),
                        ShaclVocab::SH_HAS_VALUE,
                        rdf_node,
                        rdf,
                    )?;
                },
            },
            Self::In { values } => {
                // TODO: Review this code
                values.iter().try_for_each(|value| match value {
                    Value::Iri(iri) => Self::write_iri(iri, ShaclVocab::SH_IN, rdf_node, rdf),
                    Value::Literal(literal) => Self::write_literal(
                        &ConcreteLiteral::str(&literal.to_string()),
                        ShaclVocab::SH_IN,
                        rdf_node,
                        rdf,
                    ),
                })?;
            },
            Self::Deactivated(value) => {
                Self::write_boolean(*value, ShaclVocab::SH_DEACTIVATED, rdf_node, rdf)?;
                // TODO - For Node Expr, do not delete
                // if let NodeExpr::Literal(ConcreteLiteral::BooleanLiteral(lit)) = value {
                //     Self::write_boolean(*lit, ShaclVocab::SH_DEACTIVATED, rdf_node, rdf)
                // } else {
                //     todo!() // TODO - Launch error, since sh:deactivated only accepts boolean literals
                // }?
            },
            Self::QualifiedValueShape {
                shape,
                q_min_count,
                q_max_count,
                disjoint,
                ..
            } => {
                Self::write_term(
                    &shape.clone().into(),
                    ShaclVocab::SH_QUALIFIED_VALUE_SHAPE,
                    rdf_node,
                    rdf,
                )?;

                if let Some(value) = q_min_count {
                    Self::write_integer(*value, ShaclVocab::SH_QUALIFIED_MIN_COUNT, rdf_node, rdf)?;
                }

                if let Some(value) = q_max_count {
                    Self::write_integer(*value, ShaclVocab::SH_QUALIFIED_MAX_COUNT, rdf_node, rdf)?;
                }

                if let Some(value) = disjoint {
                    Self::write_boolean(*value, ShaclVocab::SH_QUALIFIED_MAX_COUNT, rdf_node, rdf)?;
                }
            },
        }
        Ok(())
    }

    fn write_integer<RDF>(value: isize, predicate: &str, rdf_node: &Object, rdf: &mut RDF) -> Result<(), RDF::Err>
    where
        RDF: BuildRDF,
    {
        let value: i128 = value.try_into().unwrap();
        let literal: RDF::Literal = value.into();
        Self::write_term(&literal.into(), predicate, rdf_node, rdf)
    }

    fn write_boolean<RDF>(value: bool, predicate: &str, rdf_node: &Object, rdf: &mut RDF) -> Result<(), RDF::Err>
    where
        RDF: BuildRDF,
    {
        let literal: RDF::Literal = value.into();
        Self::write_term(&literal.into(), predicate, rdf_node, rdf)
    }

    fn write_literal<RDF>(
        value: &ConcreteLiteral,
        predicate: &str,
        rdf_node: &Object,
        rdf: &mut RDF,
    ) -> Result<(), RDF::Err>
    where
        RDF: BuildRDF,
    {
        let literal: RDF::Literal = value.lexical_form().into();
        Self::write_term(&literal.into(), predicate, rdf_node, rdf)
    }

    fn write_iri<RDF>(value: &IriRef, predicate: &str, rdf_node: &Object, rdf: &mut RDF) -> Result<(), RDF::Err>
    where
        RDF: BuildRDF,
    {
        Self::write_term(&value.get_iri().unwrap().clone().into(), predicate, rdf_node, rdf)
    }

    fn write_term<RDF>(value: &RDF::Term, predicate: &str, rdf_node: &Object, rdf: &mut RDF) -> Result<(), RDF::Err>
    where
        RDF: BuildRDF,
    {
        let node: RDF::Subject = rdf_node.clone().try_into().map_err(|_| unreachable!())?;
        rdf.add_triple(node, iri!(predicate), value.clone())
    }

    pub fn closed(is_closed: bool, ignored_properties: HashSet<IriS>) -> Self {
        Component::Closed {
            is_closed,
            ignored_properties,
        }
    }
}

impl Display for Component {
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
            Component::Pattern { pattern, flags } => match flags {
                Some(flags) => write!(f, "pattern({pattern}, {flags})"),
                None => write!(f, "pattern({pattern})"),
            },
            Component::UniqueLang(ul) => write!(f, "uniqueLang({ul})"),
            Component::LanguageIn { .. } => todo!(),
            Component::Equals(e) => write!(f, "equals({e})"),
            Component::Disjoint(d) => write!(f, "disjoint({d})"),
            Component::LessThan(lt) => write!(f, "lessThan({lt})"),
            Component::LessThanOrEquals(lte) => write!(f, "lessThanOrEquals({lte})"),
            Component::Or { shapes } => {
                let str = shapes.iter().map(|s| s.to_string()).join(" ");
                write!(f, "or [{str}]")
            },
            Component::And { shapes } => {
                let str = shapes.iter().map(|s| s.to_string()).join(" ");
                write!(f, "and [{str}]")
            },
            Component::Not { shape } => {
                write!(f, "not [{shape}]")
            },
            Component::Xone { shapes } => {
                let str = shapes.iter().map(|s| s.to_string()).join(" ");
                write!(f, "xone [{str}]")
            },
            Component::Closed {
                is_closed,
                ignored_properties,
            } => {
                write!(
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
                )
            },
            Component::Node { shape } => write!(f, "node({shape})"),
            Component::HasValue { value } => write!(f, "hasValue({value})"),
            Component::In { values } => {
                let str = values.iter().map(|v| v.to_string()).join(" ");
                write!(f, "In [{str}]")
            },
            Component::QualifiedValueShape {
                shape,
                q_max_count,
                q_min_count,
                disjoint,
                siblings,
            } => write!(
                f,
                "QualifiedValueShape(shape: {shape}, qualified_min_count: {q_min_count:?}, qualified_max_count: {q_max_count:?}, qualified_value_shapes_disjoint: {disjoint:?}{})",
                if siblings.is_empty() {
                    "".to_string()
                } else {
                    format!(", siblings: [{}]", siblings.iter().map(|s| s.to_string()).join(", "))
                }
            ),
            Component::Deactivated(b) => write!(f, "deactivated({b})"),
        }
    }
}

impl From<Component> for IriS {
    fn from(value: Component) -> Self {
        match value {
            Component::Class(_) => ShaclVocab::sh_class().clone(),
            Component::Datatype(_) => ShaclVocab::sh_datatype().clone(),
            Component::NodeKind(_) => ShaclVocab::sh_iri().clone(),
            Component::MinCount(_) => ShaclVocab::sh_min_count().clone(),
            Component::MaxCount(_) => ShaclVocab::sh_max_count().clone(),
            Component::MinExclusive(_) => ShaclVocab::sh_min_exclusive().clone(),
            Component::MaxExclusive(_) => ShaclVocab::sh_max_exclusive().clone(),
            Component::MinInclusive(_) => ShaclVocab::sh_min_inclusive().clone(),
            Component::MaxInclusive(_) => ShaclVocab::sh_max_inclusive().clone(),
            Component::MinLength(_) => ShaclVocab::sh_min_length().clone(),
            Component::MaxLength(_) => ShaclVocab::sh_max_length().clone(),
            Component::Pattern { .. } => ShaclVocab::sh_pattern().clone(),
            Component::UniqueLang(_) => ShaclVocab::sh_unique_lang().clone(),
            Component::LanguageIn { .. } => ShaclVocab::sh_language_in().clone(),
            Component::Equals(_) => ShaclVocab::sh_equals().clone(),
            Component::Disjoint(_) => ShaclVocab::sh_disjoint().clone(),
            Component::LessThan(_) => ShaclVocab::sh_less_than().clone(),
            Component::LessThanOrEquals(_) => ShaclVocab::sh_less_than_or_equals().clone(),
            Component::Or { .. } => ShaclVocab::sh_or().clone(),
            Component::And { .. } => ShaclVocab::sh_and().clone(),
            Component::Not { .. } => ShaclVocab::sh_not().clone(),
            Component::Xone { .. } => ShaclVocab::sh_xone().clone(),
            Component::Closed { .. } => ShaclVocab::sh_closed().clone(),
            Component::Node { .. } => ShaclVocab::sh_node().clone(),
            Component::HasValue { .. } => ShaclVocab::sh_has_value().clone(),
            Component::In { .. } => ShaclVocab::sh_in().clone(),
            Component::QualifiedValueShape { .. } => ShaclVocab::sh_qualified_value_shape().clone(),
            Component::Deactivated(_) => ShaclVocab::sh_deactivated().clone(),
        }
    }
}

// TODO - For NodeExpr, do not delete
// impl<RDF: Rdf> Clone for Component<RDF> {
//     fn clone(&self) -> Self {
//         match self {
//             Component::Class(c) => Component::Class(c.clone()),
//             Component::Datatype(d) => Component::Datatype(d.clone()),
//             Component::NodeKind(n) => Component::NodeKind(n.clone()),
//             Component::MinCount(m) => Component::MinCount(*m),
//             Component::MaxCount(m) => Component::MaxCount(*m),
//             Component::MinExclusive(m) => Component::MinExclusive(m.clone()),
//             Component::MaxExclusive(m) => Component::MaxExclusive(m.clone()),
//             Component::MinInclusive(m) => Component::MinInclusive(m.clone()),
//             Component::MaxInclusive(m) => Component::MaxInclusive(m.clone()),
//             Component::MinLength(m) => Component::MinLength(m.clone()),
//             Component::MaxLength(m) => Component::MaxLength(m.clone()),
//             Component::Pattern { pattern, flags } => Component::Pattern {
//                 pattern: pattern.clone(),
//                 flags: flags.clone(),
//             },
//             Component::UniqueLang(u) => Component::UniqueLang(u.clone()),
//             Component::LanguageIn { langs } => Component::LanguageIn { langs: langs.clone() },
//             Component::Equals(e) => Component::Equals(e.clone()),
//             Component::Disjoint(d) => Component::Disjoint(d.clone()),
//             Component::LessThan(l) => Component::LessThan(l.clone()),
//             Component::LessThanOrEquals(l) => Component::LessThanOrEquals(l.clone()),
//             Component::Or { shapes } => Component::Or { shapes: shapes.clone() },
//             Component::And { shapes } => Component::And { shapes: shapes.clone() },
//             Component::Not { shape } => Component::Not { shape: shape.clone() },
//             Component::Xone { shapes } => Component::Xone { shapes: shapes.clone() },
//             Component::Closed {
//                 is_closed,
//                 ignored_properties,
//             } => Component::Closed {
//                 is_closed: *is_closed,
//                 ignored_properties: ignored_properties.clone(),
//             },
//             Component::Node { shape } => Component::Node { shape: shape.clone() },
//             Component::HasValue { value } => Component::HasValue { value: value.clone() },
//             Component::In { values } => Component::In { values: values.clone() },
//             Component::QualifiedValueShape {
//                 shape,
//                 disjoint,
//                 siblings,
//                 q_min_count,
//                 q_max_count,
//             } => Component::QualifiedValueShape {
//                 shape: shape.clone(),
//                 disjoint: *disjoint,
//                 siblings: siblings.clone(),
//                 q_min_count: *q_min_count,
//                 q_max_count: *q_max_count,
//             },
//             Component::Deactivated(d) => Component::Deactivated(d.clone()),
//         }
//     }
// }
//
// impl<RDF: Rdf> PartialEq for Component<RDF> {
//     fn eq(&self, other: &Self) -> bool {
//         match (self, other) {
//             (Component::Class(l), Component::Class(r)) => l == r,
//             (Component::Datatype(l), Component::Datatype(r)) => l == r,
//             (Component::NodeKind(l), Component::NodeKind(r)) => l == r,
//             (Component::MinCount(l), Component::MinCount(r)) => l == r,
//             (Component::MaxCount(l), Component::MaxCount(r)) => l == r,
//             (Component::MinExclusive(l), Component::MinExclusive(r)) => l == r,
//             (Component::MaxExclusive(l), Component::MaxExclusive(r)) => l == r,
//             (Component::MinInclusive(l), Component::MinInclusive(r)) => l == r,
//             (Component::MaxInclusive(l), Component::MaxInclusive(r)) => l == r,
//             (Component::MinLength(l), Component::MinLength(r)) => l == r,
//             (Component::MaxLength(l), Component::MaxLength(r)) => l == r,
//             (Component::Pattern { pattern: pl, flags: fl }, Component::Pattern { pattern: pr, flags: fr }) => {
//                 pl == pr && fl == fr
//             },
//             (Component::UniqueLang(l), Component::UniqueLang(r)) => l == r,
//             (Component::LanguageIn { langs: l }, Component::LanguageIn { langs: r }) => l == r,
//             (Component::Equals(l), Component::Equals(r)) => l == r,
//             (Component::Disjoint(l), Component::Disjoint(r)) => l == r,
//             (Component::LessThan(l), Component::LessThan(r)) => l == r,
//             (Component::LessThanOrEquals(l), Component::LessThanOrEquals(r)) => l == r,
//             (Component::Or { shapes: l }, Component::Or { shapes: r }) => l == r,
//             (Component::And { shapes: l }, Component::And { shapes: r }) => l == r,
//             (Component::Not { shape: l }, Component::Not { shape: r }) => l == r,
//             (Component::Xone { shapes: l }, Component::Xone { shapes: r }) => l == r,
//             (
//                 Component::Closed {
//                     is_closed: cl,
//                     ignored_properties: pl,
//                 },
//                 Component::Closed {
//                     is_closed: cr,
//                     ignored_properties: pr,
//                 },
//             ) => cl == cr && pl == pr,
//             (Component::Node { shape: l }, Component::Node { shape: r }) => l == r,
//             (Component::HasValue { value: l }, Component::HasValue { value: r }) => l == r,
//             (Component::In { values: l }, Component::In { values: r }) => l == r,
//             (
//                 Component::QualifiedValueShape {
//                     shape: sl,
//                     q_min_count: minl,
//                     q_max_count: maxl,
//                     disjoint: dl,
//                     siblings: sil,
//                 },
//                 Component::QualifiedValueShape {
//                     shape: sr,
//                     q_min_count: minr,
//                     q_max_count: maxr,
//                     disjoint: dr,
//                     siblings: sir,
//                 },
//             ) => sl == sr && minl == minr && maxl == maxr && dl == dr && sil == sir,
//             (Component::Deactivated(l), Component::Deactivated(r)) => l == r,
//             _ => false,
//         }
//     }
// }
