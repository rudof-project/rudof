use std::{borrow::Cow, marker::PhantomData};

use iri_s::IriS;
use prefixmap::PrefixMap;
use pretty::{Arena, DocAllocator, DocBuilder, RefDoc};
/// This file converts ShEx AST to ShEx compact syntax
use shex_ast::{
    object_value::ObjectValue, value_set_value::ValueSetValue, IriRef, NodeConstraint, NodeKind,
    Ref, Schema, Shape, ShapeDecl, ShapeExpr, TripleExpr,
};

#[derive(Default, Debug, Clone)]
pub struct ShExFormatter {}

impl ShExFormatter {
    pub fn format_schema(&self, schema: &Schema) -> String {
        let arena = Arena::<()>::new();
        let printer = ShExCompactPrinter::new(schema, &arena);
        printer.pretty_print()
    }
}

struct ShExCompactPrinter<'a, A>
where
    A: Clone,
{
    width: usize,
    indent: isize,
    with_color: bool,
    keyword_color: Color,
    schema: &'a Schema,
    doc: &'a Arena<'a, A>,
    marker: PhantomData<A>,
}

enum Color {
    Blue,
    Black,
}

const DEFAULT_WIDTH: usize = 100;
const DEFAULT_INDENT: isize = 4;

impl<'a, A> ShExCompactPrinter<'a, A>
where
    A: Clone,
{
    pub fn new(schema: &'a Schema, doc: &'a Arena<'a, A>) -> ShExCompactPrinter<'a, A> {
        ShExCompactPrinter {
            width: DEFAULT_WIDTH,
            indent: DEFAULT_INDENT,
            with_color: false,
            keyword_color: Color::Blue,
            schema,
            doc,
            marker: PhantomData,
        }
    }

    pub fn with_width(mut self, width: usize) -> Self {
        self.width = width;
        self
    }

    pub fn pretty_print(&self) -> String {
        // let arena = pretty::Arena::<()>::new();
        let doc = self.pp_schema();
        doc.pretty(self.width as usize).to_string()
    }

    fn pp_schema(&self) -> DocBuilder<'a, Arena<'a, A>, A> {
        self.opt_pp(self.schema.prefixmap(), self.pp_prefix_map())
            .append(self.opt_pp(self.schema.base(), self.pp_base()))
            .append(self.opt_pp(self.schema.start(), self.pp_start()))
            .append(self.opt_pp(self.schema.shapes(), self.pp_shape_decls()))
    }

    fn pp_shape_decls(
        &self,
    ) -> impl Fn(&Vec<ShapeDecl>, &ShExCompactPrinter<'a, A>) -> DocBuilder<'a, Arena<'a, A>, A>
    {
        move |shape_decls, printer| {
            let mut docs = Vec::new();
            for sd in shape_decls {
                docs.push(printer.pp_shape_decl(sd))
            }
            printer.doc.intersperse(docs, printer.doc.hardline())
        }
    }

    fn pp_shape_decl(&self, sd: &ShapeDecl) -> DocBuilder<'a, Arena<'a, A>, A> {
        self.pp_ref(&sd.id)
            .append(self.doc.space())
            .append(self.pp_shape_expr(&sd.shape_expr))
    }

    fn pp_start(
        &self,
    ) -> impl Fn(&ShapeExpr, &ShExCompactPrinter<'a, A>) -> DocBuilder<'a, Arena<'a, A>, A> {
        move |se, printer| {
            printer
                .keyword("base")
                .append(printer.doc.space())
                .append("=")
                .append(printer.doc.space())
                .append(printer.pp_shape_expr(se))
        }
    }

    fn pp_shape_expr(&self, se: &ShapeExpr) -> DocBuilder<'a, Arena<'a, A>, A> {
        match se {
            ShapeExpr::Ref(ref_) => self.doc.text("@").append(self.pp_ref(ref_)),
            ShapeExpr::Shape(s) => self.pp_shape(s),
            ShapeExpr::NodeConstraint(nc) => self.pp_node_constraint(nc),
            ShapeExpr::External => self.pp_external(),
            ShapeExpr::ShapeAnd { shape_exprs } => todo!(),
            ShapeExpr::ShapeOr { shape_exprs } => todo!(),
            ShapeExpr::ShapeNot { shape_expr } => todo!(),
        }
    }

    fn pp_shape(&self, s: &Shape) -> DocBuilder<'a, Arena<'a, A>, A> {
        let closed = if s.is_closed() {
            self.keyword("CLOSED")
        } else {
            self.doc.nil()
        };
        let extra = self.opt_pp(s.extra.clone(), self.pp_extra());
        closed
            .append(extra)
            .append(self.doc.space())
            .append(self.doc.text("{"))
            .append(self.doc.line())
            .append(self.opt_pp(s.triple_expr(), self.pp_triple_expr()))
            .nest(self.indent)
            .append(self.doc.line())
            .append(self.doc.text("}"))
            .group()
    }

    fn pp_triple_expr(
        &self,
    ) -> impl Fn(&TripleExpr, &ShExCompactPrinter<'a, A>) -> DocBuilder<'a, Arena<'a, A>, A> + '_
    {
        move |te, printer| match te {
            TripleExpr::EachOf {
                id,
                expressions,
                min,
                max,
                sem_acts,
                annotations,
            } => {
                let mut docs = Vec::new();
                for e in expressions {
                    let pp_te = printer.pp_triple_expr()(&e.te, printer);
                    docs.push(pp_te)
                }
                printer
                    .doc
                    .intersperse(docs, printer.doc.text(";").append(printer.doc.line()))
            }
            TripleExpr::OneOf {
                id,
                expressions,
                min,
                max,
                sem_acts,
                annotations,
            } => todo!(),
            TripleExpr::TripleConstraint {
                id,
                inverse,
                predicate,
                value_expr,
                min,
                max,
                sem_acts,
                annotations,
            } => {
                let doc_expr = match value_expr {
                    Some(se) => printer.pp_shape_expr(se),
                    None => printer.doc.nil(),
                };
                printer
                    .pp_iri_ref(predicate)
                    .append(self.doc.space())
                    .append(doc_expr)
                    .append(self.pp_cardinality(min, max))
            }
            TripleExpr::TripleExprRef(_) => todo!(),
        }
    }

    fn pp_cardinality(
        &self,
        min: &Option<i32>,
        max: &Option<i32>,
    ) -> DocBuilder<'a, Arena<'a, A>, A> {
        match (min, max) {
            (Some(1), Some(1)) => self.doc.nil(),
            (Some(0), Some(1)) => self.doc.space().append(self.doc.text("?")),
            (Some(0), Some(-1)) => self.doc.space().append(self.doc.text("*")),
            (Some(1), Some(-1)) => self.doc.space().append(self.doc.text("+")),
            (Some(m), Some(n)) => self.doc.space().append(
                self.enclose_space(
                    "{",
                    self.doc
                        .text(m.to_string())
                        .append(self.doc.text(","))
                        .append(self.doc.text(n.to_string())),
                    "}",
                ),
            ),
            (Some(m), None) => self
                .doc
                .space()
                .append(self.doc.text("{"))
                .append(self.doc.space())
                .append(self.doc.text(m.to_string()))
                .append(self.doc.text(","))
                .append(self.doc.text("}")),
            (None, Some(-1)) => self
                .doc
                .space()
                .append(self.doc.text("{"))
                .append(self.doc.space())
                .append(self.doc.text(","))
                .append(self.doc.text("*"))
                .append(self.doc.text("}")),
            (None, Some(n)) => self
                .doc
                .space()
                .append(self.doc.text("{"))
                .append(self.doc.space())
                .append(self.doc.text(","))
                .append(self.doc.text(n.to_string()))
                .append(self.doc.text("}")),
            (None, None) => self.doc.nil(),
        }
    }

    fn pp_extra(
        &self,
    ) -> impl Fn(&Vec<IriRef>, &ShExCompactPrinter<'a, A>) -> DocBuilder<'a, Arena<'a, A>, A> {
        move |es, printer| printer.doc.text("EXTRA...todo!")
    }

    fn pp_node_constraint(&self, nc: &NodeConstraint) -> DocBuilder<'a, Arena<'a, A>, A> {
        self.opt_pp(nc.node_kind(), self.pp_node_kind())
            .append(self.opt_pp(nc.datatype(), self.pp_datatype()))
            .append(self.opt_pp(nc.values(), self.pp_value_set()))
    }

    fn pp_node_kind(
        &self,
    ) -> impl Fn(&NodeKind, &ShExCompactPrinter<'a, A>) -> DocBuilder<'a, Arena<'a, A>, A> {
        move |nk, printer| match nk {
            NodeKind::Iri => printer.keyword("IRI"),
            NodeKind::BNode => printer.keyword("BNODE"),
            NodeKind::NonLiteral => printer.keyword("NONLITERAL"),
            NodeKind::Literal => printer.keyword("LITERAL"),
        }
    }

    fn pp_value_set(
        &self,
    ) -> impl Fn(&Vec<ValueSetValue>, &ShExCompactPrinter<'a, A>) -> DocBuilder<'a, Arena<'a, A>, A>
    {
        move |values, printer| {
            let mut docs = Vec::new();
            for v in values {
                docs.push(printer.pp_value_set_value(v))
            }
            printer.doc.space().append(printer.enclose_space(
                "[",
                printer.doc.intersperse(docs, printer.doc.space()),
                "]",
            ))
        }
    }

    fn pp_datatype(
        &self,
    ) -> impl Fn(&IriRef, &ShExCompactPrinter<'a, A>) -> DocBuilder<'a, Arena<'a, A>, A> {
        move |dt, printer| printer.pp_iri_ref(dt)
    }

    fn pp_external(&self) -> DocBuilder<'a, Arena<'a, A>, A> {
        self.keyword("EXTERNAL")
    }

    fn pp_value_set_value(&self, v: &ValueSetValue) -> DocBuilder<'a, Arena<'a, A>, A> {
        match v {
            ValueSetValue::LanguageStem => todo!(),
            ValueSetValue::LanguageStemRange => todo!(),
            ValueSetValue::ObjectValue(ov) => self.pp_object_value(&ov.ov),
            ValueSetValue::IriStem { type_, stem } => todo!(),
            ValueSetValue::IriStemRange {
                type_,
                stem,
                exclusions,
            } => todo!(),
            ValueSetValue::LiteralStem { type_, stem } => todo!(),
            ValueSetValue::LiteralStemRange {
                type_,
                stem,
                exclusions,
            } => todo!(),
            ValueSetValue::Language {
                type_,
                language_tag,
            } => todo!(),
        }
    }

    fn pp_object_value(&self, v: &ObjectValue) -> DocBuilder<'a, Arena<'a, A>, A> {
        match v {
            ObjectValue::IriRef(i) => self.pp_iri_ref(i),
            ObjectValue::ObjectLiteral {
                type_,
                value,
                language,
            } => todo!(),
            ObjectValue::NumericLiteral(_) => todo!(),
        }
    }

    fn pp_ref(&self, ref_: &Ref) -> DocBuilder<'a, Arena<'a, A>, A> {
        match ref_ {
            Ref::BNode { value } => self.pp_bnode(value),
            Ref::IriRef { value } => self.pp_iri_ref(value),
        }
    }

    fn pp_bnode(&self, value: &String) -> DocBuilder<'a, Arena<'a, A>, A> {
        self.doc.text("_:").append(self.doc.text(value.clone()))
    }

    fn pp_iri_ref(&self, value: &IriRef) -> DocBuilder<'a, Arena<'a, A>, A> {
        match value {
            IriRef::Iri(iri) => self.pp_iri(iri),
            IriRef::Prefixed { prefix, local } => self
                .doc
                .text(prefix.clone())
                .append(self.doc.text(":"))
                .append(self.doc.text(local.clone())),
        }
    }

    fn keyword<U>(&self, s: U) -> DocBuilder<'a, Arena<'a, A>, A>
    where
        U: Into<Cow<'a, str>>,
    {
        if self.with_color {
            // TODO...add colors..
            self.doc.text(s)
        } else {
            self.doc.text(s)
        }
    }

    fn pp_base(
        &self,
    ) -> impl Fn(&IriS, &ShExCompactPrinter<'a, A>) -> DocBuilder<'a, Arena<'a, A>, A> {
        move |base, printer| {
            printer
                .keyword("base")
                .append(printer.doc.space())
                .append(printer.pp_iri_unqualified(&base))
                .append(printer.doc.hardline())
        }
    }

    fn pp_prefix_map(
        &self,
    ) -> impl Fn(&PrefixMap, &ShExCompactPrinter<'a, A>) -> DocBuilder<'a, Arena<'a, A>, A> {
        move |pm, printer| {
            let mut pms: Vec<DocBuilder<'a, Arena<'a, A>, A>> = Vec::new();
            for (alias, iri) in pm.map.clone().into_iter() {
                pms.push(
                    printer
                        .doc
                        .text("prefix")
                        .append(printer.doc.space())
                        .append(printer.doc.text(alias))
                        .append(printer.doc.text(":"))
                        .append(printer.doc.space())
                        .append(printer.pp_iri_unqualified(&iri)),
                )
            }
            printer
                .doc
                .intersperse(pms, printer.doc.hardline())
                .append(printer.doc.hardline())
        }
    }

    fn pp_iri_unqualified(&self, iri: &IriS) -> DocBuilder<'a, Arena<'a, A>, A> {
        self.doc
            .text("<")
            .append(self.doc.text(iri.to_string()))
            .append(self.doc.text(">"))
    }

    fn pp_iri(&self, iri: &IriS) -> DocBuilder<'a, Arena<'a, A>, A> {
        match self.schema.prefixmap() {
            Some(pm) => self.doc.text(pm.qualify(iri)),
            None => self
                .doc
                .text("<")
                .append(self.doc.text(iri.to_string()))
                .append(self.doc.text(">")),
        }
    }

    fn opt_pp<V>(
        &self,
        maybe: Option<V>,
        pp: impl Fn(&V, &ShExCompactPrinter<'a, A>) -> DocBuilder<'a, Arena<'a, A>, A>,
    ) -> DocBuilder<'a, Arena<'a, A>, A> {
        match maybe {
            None => self.doc.nil(),
            Some(ref v) => pp(v, self),
        }
    }

    fn is_empty(&self, d: &DocBuilder<'a, Arena<'a, A>, A>) -> bool {
        use pretty::Doc::*;
        match &**d {
            Nil => true,
            FlatAlt(t1, t2) => self.is_empty_ref(t1) && self.is_empty_ref(t2),
            Group(t) => self.is_empty_ref(t),
            Nest(_, t) => self.is_empty_ref(t),
            Union(t1, t2) => self.is_empty_ref(t1) && self.is_empty_ref(t2),
            Annotated(_, t) => self.is_empty_ref(t),
            _ => false,
        }
    }

    fn is_empty_ref(&self, rd: &RefDoc<'a, A>) -> bool {
        use pretty::Doc::*;
        match &**rd {
            Nil => true,
            FlatAlt(t1, t2) => self.is_empty_ref(t1) && self.is_empty_ref(t2),
            Group(t) => self.is_empty_ref(t),
            Nest(_, t) => self.is_empty_ref(t),
            Union(t1, t2) => self.is_empty_ref(t1) && self.is_empty_ref(t2),
            Annotated(_, t) => self.is_empty_ref(t),
            _ => false,
        }
    }

    pub fn enclose(
        &self,
        left: &'a str,
        doc: DocBuilder<'a, Arena<'a, A>, A>,
        right: &'a str,
    ) -> DocBuilder<'a, Arena<'a, A>, A> {
        if self.is_empty(&doc) {
            self.doc.text(left).append(right)
        } else {
            self.doc
                .text(left)
                .append(self.doc.line_())
                .append(doc)
                .nest(self.indent)
                .append(self.doc.line_())
                .append(right)
                .group()
        }
    }

    pub fn enclose_space(
        &self,
        left: &'a str,
        middle: DocBuilder<'a, Arena<'a, A>, A>,
        right: &'a str,
    ) -> DocBuilder<'a, Arena<'a, A>, A> {
        if self.is_empty(&middle) {
            self.doc.text(left).append(right)
        } else {
            self.doc
                .text(left)
                .append(self.doc.line())
                .append(middle)
                .nest(self.indent)
                .append(self.doc.line())
                .append(right)
                .group()
        }
    }
}

#[cfg(test)]
mod tests {
    use iri_s::IriS;
    use prefixmap::PrefixMap;

    use super::*;

    #[test]
    fn empty_schema() {
        let mut pm = PrefixMap::new();
        pm.insert("", &IriS::new_unchecked("http://example.org/"));
        pm.insert("schema", &IriS::new_unchecked("https://schema.org/"));
        let schema = Schema::new().with_prefixmap(Some(pm));
        let s = ShExFormatter::default().format_schema(&schema);
        assert_eq!(
            s,
            "prefix : <http://example.org/>\nprefix schema: <https://schema.org/>"
        );
    }
}
