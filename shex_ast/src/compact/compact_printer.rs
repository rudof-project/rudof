use crate::{BNode, ShapeExprLabel, object_value::ObjectValue};
use colored::*;
use prefixmap::{IriRef, PrefixMap};
use pretty::{Arena, DocAllocator, DocBuilder};
use rudof_iri::IriS;
use rudof_rdf::rdf_core::term::literal::{ConcreteLiteral, NumericLiteral};
use std::borrow::Cow;

pub(crate) fn pp_object_value<'a, A>(
    v: &ObjectValue,
    doc: &'a Arena<'a, A>,
    prefixmap: &PrefixMap,
) -> DocBuilder<'a, Arena<'a, A>, A> {
    match v {
        ObjectValue::IriRef(i) => pp_iri_ref(i, doc, prefixmap),
        ObjectValue::Literal(ConcreteLiteral::NumericLiteral(num)) => pp_numeric_literal(num, doc),
        ObjectValue::Literal(ConcreteLiteral::BooleanLiteral(b)) => doc.text(if *b { "true" } else { "false" }),
        ObjectValue::Literal(ConcreteLiteral::StringLiteral { lexical_form, lang }) => {
            let str = pp_quoted_string(lexical_form);
            match lang {
                Some(lang) => doc.text(format!("{str}@{lang}")),
                None => doc.text(str),
            }
        },
        ObjectValue::Literal(
            ConcreteLiteral::DatatypeLiteral { lexical_form, datatype }
            | ConcreteLiteral::WrongDatatypeLiteral {
                lexical_form, datatype, ..
            },
        ) => doc.text(format!(
            "{}^^{}",
            pp_quoted_string(lexical_form),
            pp_iri_ref_str(datatype, prefixmap)
        )),
        ObjectValue::Literal(ConcreteLiteral::DatetimeLiteral(dt)) => doc.text(format!(
            "{}^^<http://www.w3.org/2001/XMLSchema#dateTime>",
            pp_quoted_string(&dt.to_string())
        )),
    }
}

/// Quotes and escapes a string as a ShExC `STRING_LITERAL2` (`"..."`).
/// `"`, `\`, and literal newlines/carriage-returns can't appear bare inside
/// one and must be written using their ECHAR escape sequence.
fn pp_quoted_string(lexical_form: &str) -> String {
    let escaped = lexical_form
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r");
    format!("\"{escaped}\"")
}

fn pp_iri_ref_str(value: &IriRef, prefixmap: &PrefixMap) -> String {
    match value {
        IriRef::Iri(iri) => prefixmap.qualify(iri),
        IriRef::Prefixed { prefix, local } => format!("{prefix}:{local}"),
    }
}

pub(crate) fn pp_label<'a, A>(
    label: &ShapeExprLabel,
    doc: &'a Arena<'a, A>,
    prefixmap: &PrefixMap,
    keyword_color: Option<Color>,
) -> DocBuilder<'a, Arena<'a, A>, A> {
    match label {
        ShapeExprLabel::BNode { value } => pp_bnode(value, doc),
        ShapeExprLabel::IriRef { value } => pp_iri_ref(value, doc, prefixmap),
        ShapeExprLabel::Start => keyword("START", doc, keyword_color),
    }
}

pub(crate) fn pp_bnode<'a, A>(value: &BNode, doc: &'a Arena<'a, A>) -> DocBuilder<'a, Arena<'a, A>, A> {
    doc.text(format!("{value}"))
}

fn pp_numeric_literal<'a, A>(value: &NumericLiteral, doc: &'a Arena<'a, A>) -> DocBuilder<'a, Arena<'a, A>, A> {
    match value {
        NumericLiteral::Integer(n) => doc.text(n.to_string()),
        NumericLiteral::Decimal(decimal) => doc.text(decimal.to_string()),
        NumericLiteral::Double(d) => doc.text(d.to_string()),
        NumericLiteral::Long(l) => doc.text(l.to_string()),
        NumericLiteral::Float(n) => doc.text(n.to_string()),
        NumericLiteral::Byte(n) => doc.text(n.to_string()),
        NumericLiteral::Short(n) => doc.text(n.to_string()),
        NumericLiteral::NonNegativeInteger(n) => doc.text(n.to_string()),
        NumericLiteral::UnsignedLong(n) => doc.text(n.to_string()),
        NumericLiteral::UnsignedInt(n) => doc.text(n.to_string()),
        NumericLiteral::UnsignedShort(n) => doc.text(n.to_string()),
        NumericLiteral::UnsignedByte(n) => doc.text(n.to_string()),
        NumericLiteral::PositiveInteger(n) => doc.text(n.to_string()),
        NumericLiteral::NegativeInteger(n) => doc.text(n.to_string()),
        NumericLiteral::NonPositiveInteger(n) => doc.text(n.to_string()),
    }
}

fn pp_iri_ref<'a, A>(value: &IriRef, doc: &'a Arena<'a, A>, prefixmap: &PrefixMap) -> DocBuilder<'a, Arena<'a, A>, A> {
    match value {
        IriRef::Iri(iri) => pp_iri(iri, doc, prefixmap),
        IriRef::Prefixed { prefix, local } => doc
            .text(prefix.clone())
            .append(doc.text(":"))
            .append(doc.text(local.clone())),
    }
}

pub(crate) fn keyword<'a, U, A>(s: U, doc: &'a Arena<'a, A>, color: Option<Color>) -> DocBuilder<'a, Arena<'a, A>, A>
where
    U: Into<Cow<'a, str>>,
{
    if let Some(color) = color {
        // use std::borrow::Borrow;
        let data: Cow<str> = s.into();
        let s: String = match data {
            Cow::Owned(t) => t,
            Cow::Borrowed(t) => t.into(),
        };
        doc.text(s.as_str().color(color).to_string())
    } else {
        doc.text(s)
    }
}

fn pp_iri<'a, A>(iri: &IriS, doc: &'a Arena<'a, A>, prefixmap: &PrefixMap) -> DocBuilder<'a, Arena<'a, A>, A> {
    doc.text(prefixmap.qualify(iri))
}
