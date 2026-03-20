use crate::{node_kind::NodeKind, value::Value};
use iri_s::{iri, IriS};
use itertools::Itertools;
use prefixmap::IriRef;
use rudof_rdf::rdf_core::vocabs::ShaclVocab;
use rudof_rdf::rdf_core::{
    term::{
        literal::{ConcreteLiteral, Lang},
        Object,
    },
    BuildRDF,
};

// impl Component {
//     pub fn write<B: BuildRDF>(&self, rdf_node: &Object, rdf: &mut B) -> Result<(), B::Err> {
//         match self {
//             Self::Class(rdf_node) => {
//                 Self::write_term(&rdf_node.clone().into(), ShaclVocab::SH_CLASS, rdf_node, rdf)?;
//             },
//             Self::Datatype(iri) => {
//                 Self::write_iri(iri, ShaclVocab::SH_DATATYPE, rdf_node, rdf)?;
//             },
//             Self::NodeKind(node_kind) => {
//                 let iri = match &node_kind {
//                     NodeKind::Iri => ShaclVocab::SH_IRI,
//
//                     _ => unimplemented!(),
//                 };
//
//                 Self::write_iri(&IriRef::Iri(iri!(iri)), ShaclVocab::SH_DATATYPE, rdf_node, rdf)?;
//             },
//             Self::MinCount(value) => {
//                 Self::write_integer(*value, ShaclVocab::SH_MIN_COUNT, rdf_node, rdf)?;
//             },
//             Self::MaxCount(value) => {
//                 Self::write_integer(*value, ShaclVocab::SH_MAX_COUNT, rdf_node, rdf)?;
//             },
//             Self::MinExclusive(value) => {
//                 Self::write_literal(value, ShaclVocab::SH_MIN_EXCLUSIVE, rdf_node, rdf)?;
//             },
//             Self::MaxExclusive(value) => {
//                 Self::write_literal(value, ShaclVocab::SH_MAX_EXCLUSIVE, rdf_node, rdf)?;
//             },
//             Self::MinInclusive(value) => {
//                 Self::write_literal(value, ShaclVocab::SH_MIN_INCLUSIVE, rdf_node, rdf)?;
//             },
//             Self::MaxInclusive(value) => {
//                 Self::write_literal(value, ShaclVocab::SH_MAX_INCLUSIVE, rdf_node, rdf)?;
//             },
//             Self::MinLength(value) => {
//                 Self::write_integer(*value, ShaclVocab::SH_MIN_LENGTH, rdf_node, rdf)?;
//             },
//             Self::MaxLength(value) => {
//                 Self::write_integer(*value, ShaclVocab::SH_MAX_LENGTH, rdf_node, rdf)?;
//             },
//             Self::Pattern { pattern, flags } => {
//                 Self::write_literal(&ConcreteLiteral::str(pattern), ShaclVocab::SH_PATTERN, rdf_node, rdf)?;
//                 if let Some(flags) = flags {
//                     Self::write_literal(&ConcreteLiteral::str(flags), ShaclVocab::SH_FLAGS, rdf_node, rdf)?;
//                 }
//             },
//             Self::UniqueLang(value) => {
//                 Self::write_boolean(*value, ShaclVocab::SH_UNIQUE_LANG, rdf_node, rdf)?;
//             },
//             Self::LanguageIn(langs) => {
//                 langs.iter().try_for_each(|lang| {
//                     Self::write_literal(
//                         &ConcreteLiteral::str(&lang.to_string()),
//                         ShaclVocab::SH_LANGUAGE_IN,
//                         rdf_node,
//                         rdf,
//                     )
//                 })?;
//             },
//             Self::Equals(iri) => {
//                 Self::write_iri(iri, ShaclVocab::SH_EQUALS, rdf_node, rdf)?;
//             },
//             Self::Disjoint(iri) => {
//                 Self::write_iri(iri, ShaclVocab::SH_DISJOINT, rdf_node, rdf)?;
//             },
//             Self::LessThan(iri) => {
//                 Self::write_iri(iri, ShaclVocab::SH_LESS_THAN, rdf_node, rdf)?;
//             },
//             Self::LessThanOrEquals(iri) => {
//                 Self::write_iri(iri, ShaclVocab::SH_LESS_THAN_OR_EQUALS, rdf_node, rdf)?;
//             },
//             Self::Or(shapes) => {
//                 shapes
//                     .iter()
//                     .try_for_each(|shape| Self::write_term(&shape.clone().into(), ShaclVocab::SH_OR, rdf_node, rdf))?;
//             },
//             Self::And(shapes) => {
//                 shapes
//                     .iter()
//                     .try_for_each(|shape| Self::write_term(&shape.clone().into(), ShaclVocab::SH_AND, rdf_node, rdf))?;
//             },
//             Self::Not(shape) => {
//                 Self::write_term(&shape.clone().into(), ShaclVocab::SH_PATTERN, rdf_node, rdf)?;
//             },
//             Self::Xone(shapes) => {
//                 shapes.iter().try_for_each(|shape| {
//                     Self::write_term(&shape.clone().into(), ShaclVocab::SH_XONE, rdf_node, rdf)
//                 })?;
//             },
//             Self::Closed {
//                 is_closed,
//                 ignored_properties,
//             } => {
//                 Self::write_boolean(*is_closed, ShaclVocab::SH_CLOSED, rdf_node, rdf)?;
//
//                 ignored_properties.iter().try_for_each(|iri| {
//                     let iri_ref = IriRef::Iri(iri.clone());
//                     Self::write_iri(&iri_ref, ShaclVocab::SH_IGNORED_PROPERTIES, rdf_node, rdf)
//                 })?;
//             },
//             Self::Node(shape) => {
//                 Self::write_term(&shape.clone().into(), ShaclVocab::SH_NODE, rdf_node, rdf)?;
//             },
//             Self::HasValue(value) => match value {
//                 Value::Iri(iri) => {
//                     Self::write_iri(iri, ShaclVocab::SH_HAS_VALUE, rdf_node, rdf)?;
//                 },
//                 Value::Literal(literal) => {
//                     Self::write_literal(
//                         &ConcreteLiteral::str(&literal.to_string()),
//                         ShaclVocab::SH_HAS_VALUE,
//                         rdf_node,
//                         rdf,
//                     )?;
//                 },
//             },
//             Self::In(values) => {
//                 // TODO: Review this code
//                 values.iter().try_for_each(|value| match value {
//                     Value::Iri(iri) => Self::write_iri(iri, ShaclVocab::SH_IN, rdf_node, rdf),
//                     Value::Literal(literal) => Self::write_literal(
//                         &ConcreteLiteral::str(&literal.to_string()),
//                         ShaclVocab::SH_IN,
//                         rdf_node,
//                         rdf,
//                     ),
//                 })?;
//             },
//             Self::Deactivated(value) => {
//                 Self::write_boolean(*value, ShaclVocab::SH_DEACTIVATED, rdf_node, rdf)?;
//                 // TODO - For Node Expr, do not delete
//                 // if let NodeExpr::Literal(ConcreteLiteral::BooleanLiteral(lit)) = value {
//                 //     Self::write_boolean(*lit, ShaclVocab::SH_DEACTIVATED, rdf_node, rdf)
//                 // } else {
//                 //     todo!() // TODO - Launch error, since sh:deactivated only accepts boolean literals
//                 // }?
//             },
//             Self::QualifiedValueShape {
//                 shape,
//                 q_min_count,
//                 q_max_count,
//                 disjoint,
//                 ..
//             } => {
//                 Self::write_term(
//                     &shape.clone().into(),
//                     ShaclVocab::SH_QUALIFIED_VALUE_SHAPE,
//                     rdf_node,
//                     rdf,
//                 )?;
//
//                 if let Some(value) = q_min_count {
//                     Self::write_integer(*value, ShaclVocab::SH_QUALIFIED_MIN_COUNT, rdf_node, rdf)?;
//                 }
//
//                 if let Some(value) = q_max_count {
//                     Self::write_integer(*value, ShaclVocab::SH_QUALIFIED_MAX_COUNT, rdf_node, rdf)?;
//                 }
//
//                 if let Some(value) = disjoint {
//                     Self::write_boolean(*value, ShaclVocab::SH_QUALIFIED_MAX_COUNT, rdf_node, rdf)?;
//                 }
//             },
//         }
//         Ok(())
//     }
//
//     fn write_integer<B: BuildRDF>(value: isize, predicate: &str, rdf_node: &Object, rdf: &mut B) -> Result<(), B::Err> {
//         let value: i128 = value.try_into().unwrap();
//         let literal: B::Literal = value.into();
//         Self::write_term(&literal.into(), predicate, rdf_node, rdf)
//     }
//
//     fn write_boolean<B: BuildRDF>(value: bool, predicate: &str, rdf_node: &Object, rdf: &mut B) -> Result<(), B::Err> {
//         let literal: B::Literal = value.into();
//         Self::write_term(&literal.into(), predicate, rdf_node, rdf)
//     }
//
//     fn write_literal<B: BuildRDF>(
//         value: &ConcreteLiteral,
//         predicate: &str,
//         rdf_node: &Object,
//         rdf: &mut B,
//     ) -> Result<(), B::Err> {
//         let literal: B::Literal = value.lexical_form().into();
//         Self::write_term(&literal.into(), predicate, rdf_node, rdf)
//     }
//
//     fn write_iri<B: BuildRDF>(value: &IriRef, predicate: &str, rdf_node: &Object, rdf: &mut B) -> Result<(), B::Err> {
//         Self::write_term(&value.get_iri().unwrap().clone().into(), predicate, rdf_node, rdf)
//     }
//
//     fn write_term<B: BuildRDF>(value: &B::Term, predicate: &str, rdf_node: &Object, rdf: &mut B) -> Result<(), B::Err> {
//         let node: B::Subject = rdf_node.clone().try_into().map_err(|_| unreachable!())?;
//         rdf.add_triple(node, iri!(predicate), value.clone())
//     }
// }

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
