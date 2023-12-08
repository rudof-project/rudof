use colored::*;
use iri_s::IriS;
use prefixmap::{IriRef, PrefixMap};
use pretty::{Arena, DocAllocator, DocBuilder, RefDoc};
use rust_decimal::Decimal;
use shex_ast::{object_value::ObjectValue, BNode, ShapeExprLabel};
use srdf::literal::Literal;
use std::borrow::Cow;

pub(crate) fn space<'a, A>(doc: &'a Arena<'a, A>) -> DocBuilder<'a, Arena<'a, A>, A> {
    doc.space()
}

pub(crate) fn pp_object_value<'a, A>(
    v: &ObjectValue,
    doc: &'a Arena<'a, A>,
    prefixmap: &PrefixMap,
) -> DocBuilder<'a, Arena<'a, A>, A> {
    match v {
        ObjectValue::IriRef(i) => pp_iri_ref(i, doc, prefixmap),
        ObjectValue::Literal(Literal::BooleanLiteral(value)) => {
            todo!()
        }
        ObjectValue::Literal(Literal::NumericLiteral(_)) => todo!(),
        ObjectValue::Literal(Literal::DatatypeLiteral {
            lexical_form,
            datatype,
        }) => todo!(),
        ObjectValue::Literal(Literal::StringLiteral { lexical_form, lang }) => todo!(),
    }
}

pub(crate) fn pp_label<'a, A>(
    label: &ShapeExprLabel,
    doc: &'a Arena<'a, A>,
    prefixmap: &PrefixMap,
    keyword_color: Option<Color>
) -> DocBuilder<'a, Arena<'a, A>, A> {
    match label {
        ShapeExprLabel::BNode { value } => pp_bnode(value, doc),
        ShapeExprLabel::IriRef { value } => pp_iri_ref(value, doc, prefixmap),
        ShapeExprLabel::Start => keyword("START", doc, keyword_color)
    }
}

pub(crate) fn pp_bnode<'a, A>(
    value: &BNode,
    doc: &'a Arena<'a, A>,
) -> DocBuilder<'a, Arena<'a, A>, A> {
    doc.text(format!("{value}"))
}

fn pp_isize<'a, A>(value: &isize, doc: &'a Arena<'a, A>) -> DocBuilder<'a, Arena<'a, A>, A> {
    doc.text(value.to_string())
}

fn pp_decimal<'a, A>(value: &Decimal, doc: &'a Arena<'a, A>) -> DocBuilder<'a, Arena<'a, A>, A> {
    doc.text(value.to_string())
}

fn pp_double<'a, A>(value: &f64, doc: &'a Arena<'a, A>) -> DocBuilder<'a, Arena<'a, A>, A> {
    doc.text(value.to_string())
}

fn pp_usize<'a, A>(value: &usize, doc: &'a Arena<'a, A>) -> DocBuilder<'a, Arena<'a, A>, A> {
    doc.text(value.to_string())
}

fn pp_iri_ref<'a, A>(
    value: &IriRef,
    doc: &'a Arena<'a, A>,
    prefixmap: &PrefixMap,
) -> DocBuilder<'a, Arena<'a, A>, A> {
    match value {
        IriRef::Iri(iri) => pp_iri(iri, doc, prefixmap),
        IriRef::Prefixed { prefix, local } => doc
            .text(prefix.clone())
            .append(doc.text(":"))
            .append(doc.text(local.clone())),
    }
}

pub(crate) fn keyword<'a, U, A>(
    s: U,
    doc: &'a Arena<'a, A>,
    color: Option<Color>,
) -> DocBuilder<'a, Arena<'a, A>, A>
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

fn pp_iri_unqualified<'a, A>(iri: &IriS, doc: &'a Arena<'a, A>) -> DocBuilder<'a, Arena<'a, A>, A> {
    let str = format!("<{}>", iri.to_string());
    doc.text(str)
}

fn pp_iri<'a, A>(
    iri: &IriS,
    doc: &'a Arena<'a, A>,
    prefixmap: &PrefixMap,
) -> DocBuilder<'a, Arena<'a, A>, A> {
    doc.text(prefixmap.qualify(iri))
}

fn is_empty<'a, A>(d: &DocBuilder<'a, Arena<'a, A>, A>) -> bool {
    use pretty::Doc::*;
    match &**d {
        Nil => true,
        FlatAlt(t1, t2) => is_empty_ref(t1) && is_empty_ref(t2),
        Group(t) => is_empty_ref(t),
        Nest(_, t) => is_empty_ref(t),
        Union(t1, t2) => is_empty_ref(t1) && is_empty_ref(t2),
        Annotated(_, t) => is_empty_ref(t),
        _ => false,
    }
}

fn is_empty_ref<'a, A>(rd: &RefDoc<'a, A>) -> bool {
    use pretty::Doc::*;

    match &**rd {
        Nil => true,
        FlatAlt(t1, t2) => is_empty_ref(t1) && is_empty_ref(t2),
        Group(t) => is_empty_ref(t),
        Nest(_, t) => is_empty_ref(t),
        Union(t1, t2) => is_empty_ref(t1) && is_empty_ref(t2),
        Annotated(_, t) => is_empty_ref(t),
        _ => false,
    }
}

pub(crate) fn enclose<'a, A>(
    left: &'a str,
    doc: DocBuilder<'a, Arena<'a, A>, A>,
    right: &'a str,
    arena: &'a Arena<'a, A>,
    indent: isize,
) -> DocBuilder<'a, Arena<'a, A>, A> {
    if is_empty(&doc) {
        arena.text(left).append(right)
    } else {
        arena
            .text(left)
            .append(arena.line_())
            .append(doc)
            .nest(indent)
            .append(arena.line_())
            .append(right)
            .group()
    }
}

pub(crate) fn enclose_space<'a, A>(
    left: &'a str,
    middle: DocBuilder<'a, Arena<'a, A>, A>,
    right: &'a str,
    arena: &'a Arena<'a, A>,
    indent: isize,
) -> DocBuilder<'a, Arena<'a, A>, A> {
    if is_empty(&middle) {
        arena.text(left).append(right)
    } else {
        arena
            .text(left)
            .append(arena.line())
            .append(middle)
            .nest(indent)
            .append(arena.line())
            .append(right)
            .group()
    }
}
