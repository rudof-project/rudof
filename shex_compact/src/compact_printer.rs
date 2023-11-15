use std::borrow::Cow;

use iri_s::IriS;
use prefixmap::PrefixMap;
use pretty::{Arena, DocAllocator, DocBuilder};
/// This file converts ShEx AST to ShEx compact syntax
use shex_ast::{
    object_value::ObjectValue, value_set_value::ValueSetValue, IriRef, NodeConstraint, NodeKind,
    Ref, Schema, Shape, ShapeDecl, ShapeExpr, TripleExpr,
};

pub struct ShExCompactPrinter<'a> {
    width: usize,
    indent: isize,
    with_color: bool,
    keyword_color: Color,
    schema: &'a Schema,
}

enum Color {
    Blue,
    Black,
}

const DEFAULT_WIDTH: usize = 100;
const DEFAULT_INDENT: isize = 4;

impl<'a> ShExCompactPrinter<'a> {
    pub fn new(schema: &'a Schema) -> ShExCompactPrinter<'a> {
        ShExCompactPrinter {
            width: DEFAULT_WIDTH,
            indent: DEFAULT_INDENT,
            with_color: false,
            keyword_color: Color::Blue,
            schema,
        }
    }

    pub fn with_width(mut self, width: usize) -> Self {
        self.width = width;
        self
    }

    pub fn pretty_print(&self) -> String {
        let arena = pretty::Arena::<()>::new();
        let doc = self.pp_schema(&arena);
        doc.pretty(self.width as usize).to_string()
    }

    fn pp_schema<D, A>(&self, doc: &'a D) -> DocBuilder<'a, D, A>
    where
        D: DocAllocator<'a, A>,
        D::Doc: Clone,
        A: Clone + 'a,
    {
        self.opt_pp(self.schema.prefixmap(), self.pp_prefix_map(), doc)
            .append(self.opt_pp(self.schema.base(), self.pp_base(), doc))
            .append(self.opt_pp(self.schema.start(), self.pp_start(), doc))
            .append(self.opt_pp(self.schema.shapes(), self.pp_shape_decls(), doc))
    }

    fn pp_shape_decls<D, A>(
        &self,
    ) -> impl Fn(&Vec<ShapeDecl>, &'a D, &ShExCompactPrinter<'a>) -> DocBuilder<'a, D, A>
    where
        D: DocAllocator<'a, A>,
        D::Doc: Clone,
        A: Clone + 'a,
    {
        move |shape_decls, doc, printer| {
            let mut docs = Vec::new();
            for sd in shape_decls {
                docs.push(printer.pp_shape_decl(sd, doc))
            }
            doc.intersperse(docs, doc.hardline())
        }
    }

    fn pp_shape_decl<D, A>(&self, sd: &ShapeDecl, doc: &'a D) -> DocBuilder<'a, D, A>
    where
        D: DocAllocator<'a, A>,
        D::Doc: Clone,
        A: Clone,
    {
        self.pp_ref(&sd.id, doc)
            .append(doc.space())
            .append(self.pp_shape_expr(&sd.shape_expr, doc))
    }

    fn pp_start<D, A>(
        &self,
    ) -> impl Fn(&ShapeExpr, &'a D, &ShExCompactPrinter<'a>) -> DocBuilder<'a, D, A>
    where
        D: DocAllocator<'a, A>,
        D::Doc: Clone,
        A: Clone + 'a,
    {
        move |se, doc, printer| {
            printer
                .keyword("base", doc)
                .append(doc.space())
                .append("=")
                .append(doc.space())
                .append(printer.pp_shape_expr(se, doc))
        }
    }

    fn pp_shape_expr<D, A>(&self, se: &ShapeExpr, doc: &'a D) -> DocBuilder<'a, D, A>
    where
        D: DocAllocator<'a, A>,
        D::Doc: Clone,
        A: Clone,
    {
        match se {
            ShapeExpr::Ref(ref_) => doc.text("@").append(self.pp_ref(ref_, doc)),
            ShapeExpr::Shape(s) => self.pp_shape(s, doc),
            ShapeExpr::NodeConstraint(nc) => self.pp_node_constraint(nc, doc),
            ShapeExpr::External => self.pp_external(doc),
            ShapeExpr::ShapeAnd { shape_exprs } => todo!(),
            ShapeExpr::ShapeOr { shape_exprs } => todo!(),
            ShapeExpr::ShapeNot { shape_expr } => todo!(),
        }
    }

    fn pp_shape<D, A>(&self, s: &Shape, doc: &'a D) -> DocBuilder<'a, D, A>
    where
        D: DocAllocator<'a, A>,
        D::Doc: Clone,
        A: Clone + 'a,
    {
        let closed = if s.is_closed() {
            self.keyword("CLOSED", doc)
        } else {
            doc.nil()
        };
        let extra = self.opt_pp(s.extra.clone(), self.pp_extra(), doc);
        closed
            .append(extra)
            .append(doc.space())
            .append(doc.text("{"))
            .append(doc.line())
            .append(self.opt_pp(s.triple_expr(), self.pp_triple_expr(), doc))
            .nest(self.indent)
            .append(doc.line())
            .append(doc.text("}"))
            .group()
    }

    fn pp_triple_expr<D, A>(
        &self,
    ) -> impl Fn(&TripleExpr, &'a D, &ShExCompactPrinter<'a>) -> DocBuilder<'a, D, A> + '_
    where
        D: DocAllocator<'a, A>,
        D::Doc: Clone,
        A: Clone + 'a,
    {
        move |te, doc, printer| match te {
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
                    let pp_te = printer.pp_triple_expr()(&e.te, doc, printer);
                    docs.push(pp_te)
                }
                doc.intersperse(docs, doc.text(";").append(doc.line()))
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
                    Some(se) => printer.pp_shape_expr(se, doc),
                    None => doc.nil(),
                };
                printer
                    .pp_iri_ref(predicate, doc)
                    .append(doc.space())
                    .append(doc_expr)
                    .append(self.pp_cardinality(min, max, doc))
            }
            TripleExpr::TripleExprRef(_) => todo!(),
        }
    }

    fn pp_cardinality<D, A>(
        &self,
        min: &Option<i32>,
        max: &Option<i32>,
        doc: &'a D,
    ) -> DocBuilder<'a, D, A>
    where
        D: DocAllocator<'a, A>,
        D::Doc: Clone,
        A: Clone,
    {
        match (min, max) {
            (Some(1), Some(1)) => doc.nil(),
            (Some(0), Some(1)) => doc.space().append(doc.text("?")),
            (Some(0), Some(-1)) => doc.space().append(doc.text("*")),
            (Some(1), Some(-1)) => doc.space().append(doc.text("+")),
            (Some(m), Some(n)) => doc
                .space()
                .append(doc.text("{"))
                .append(doc.space())
                .append(doc.text(m.to_string()))
                .append(doc.text(","))
                .append(doc.space())
                .append(doc.text(n.to_string()))
                .append(doc.space())
                .append(doc.text("}")),
            (Some(m), None) => doc
                .space()
                .append(doc.text("{"))
                .append(doc.space())
                .append(doc.text(m.to_string()))
                .append(doc.text(","))
                .append(doc.text("}")),
            (None, Some(-1)) => doc
                .space()
                .append(doc.text("{"))
                .append(doc.space())
                .append(doc.text(","))
                .append(doc.text("*"))
                .append(doc.text("}")),
            (None, Some(n)) => doc
                .space()
                .append(doc.text("{"))
                .append(doc.space())
                .append(doc.text(","))
                .append(doc.text(n.to_string()))
                .append(doc.text("}")),
            (None, None) => doc.nil(),
        }
    }

    fn pp_extra<D, A>(
        &self,
    ) -> impl Fn(&Vec<IriRef>, &'a D, &ShExCompactPrinter<'a>) -> DocBuilder<'a, D, A>
    where
        D: DocAllocator<'a, A>,
        D::Doc: Clone,
        A: Clone + 'a,
    {
        move |es, doc, printer| doc.text("EXTRA...todo!")
    }

    fn pp_node_constraint<D, A>(&self, nc: &NodeConstraint, doc: &'a D) -> DocBuilder<'a, D, A>
    where
        D: DocAllocator<'a, A>,
        D::Doc: Clone,
        A: Clone,
    {
        self.opt_pp(nc.node_kind(), self.pp_node_kind(), doc)
            .append(self.opt_pp(nc.datatype(), self.pp_datatype(), doc))
            .append(self.opt_pp(nc.values(), self.pp_value_set(), doc))
    }

    fn pp_node_kind<D, A>(
        &self,
    ) -> impl Fn(&NodeKind, &'a D, &ShExCompactPrinter<'a>) -> DocBuilder<'a, D, A>
    where
        D: DocAllocator<'a, A>,
        D::Doc: Clone,
        A: Clone + 'a,
    {
        move |nk, doc, printer| match nk {
            NodeKind::Iri => printer.keyword("IRI", doc),
            NodeKind::BNode => printer.keyword("BNODE", doc),
            NodeKind::NonLiteral => printer.keyword("NONLITERAL", doc),
            NodeKind::Literal => printer.keyword("LITERAL", doc),
        }
    }

    fn pp_value_set<D, A>(
        &self,
    ) -> impl Fn(&Vec<ValueSetValue>, &'a D, &ShExCompactPrinter<'a>) -> DocBuilder<'a, D, A>
    where
        D: DocAllocator<'a, A>,
        D::Doc: Clone,
        A: Clone + 'a,
    {
        move |values, doc, printer| {
            let mut docs = Vec::new();
            for v in values {
                docs.push(printer.pp_value_set_value(v, doc))
            }
            doc.space()
                .append(doc.text("["))
                .append(doc.intersperse(docs, doc.text(",").append(doc.space())))
                .append(doc.text("]"))
        }
    }

    fn pp_datatype<D, A>(
        &self,
    ) -> impl Fn(&IriRef, &'a D, &ShExCompactPrinter<'a>) -> DocBuilder<'a, D, A>
    where
        D: DocAllocator<'a, A>,
        D::Doc: Clone,
        A: Clone + 'a,
    {
        move |dt, doc, printer| printer.pp_iri_ref(dt, doc)
    }

    fn pp_external<D, A>(&self, doc: &'a D) -> DocBuilder<'a, D, A>
    where
        D: DocAllocator<'a, A>,
        D::Doc: Clone,
        A: Clone,
    {
        self.keyword("EXTERNAL", doc)
    }

    fn pp_value_set_value<D, A>(&self, v: &ValueSetValue, doc: &'a D) -> DocBuilder<'a, D, A>
    where
        D: DocAllocator<'a, A>,
        D::Doc: Clone,
        A: Clone,
    {
        match v {
            ValueSetValue::LanguageStem => todo!(),
            ValueSetValue::LanguageStemRange => todo!(),
            ValueSetValue::ObjectValue(ov) => self.pp_object_value(&ov.ov, doc),
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

    fn pp_object_value<D, A>(&self, v: &ObjectValue, doc: &'a D) -> DocBuilder<'a, D, A>
    where
        D: DocAllocator<'a, A>,
        D::Doc: Clone,
        A: Clone,
    {
        match v {
            ObjectValue::IriRef(i) => self.pp_iri_ref(i, doc),
            ObjectValue::ObjectLiteral {
                type_,
                value,
                language,
            } => todo!(),
            ObjectValue::NumericLiteral(_) => todo!(),
        }
    }

    fn pp_ref<D, A>(&self, ref_: &Ref, doc: &'a D) -> DocBuilder<'a, D, A>
    where
        D: DocAllocator<'a, A>,
        D::Doc: Clone,
        A: Clone,
    {
        match ref_ {
            Ref::BNode { value } => self.pp_bnode(value, doc),
            Ref::IriRef { value } => self.pp_iri_ref(value, doc),
        }
    }

    fn pp_bnode<D, A>(&self, value: &String, doc: &'a D) -> DocBuilder<'a, D, A>
    where
        D: DocAllocator<'a, A>,
        D::Doc: Clone,
        A: Clone,
    {
        doc.text("_:").append(doc.text(value.clone()))
    }

    fn pp_iri_ref<D, A>(&self, value: &IriRef, doc: &'a D) -> DocBuilder<'a, D, A>
    where
        D: DocAllocator<'a, A>,
        D::Doc: Clone,
        A: Clone,
    {
        match value {
            IriRef::Iri(iri) => self.pp_iri(iri, doc),
            IriRef::Prefixed { prefix, local } => doc
                .text(prefix.clone())
                .append(doc.text(":"))
                .append(doc.text(local.clone())),
        }
    }

    fn keyword<D, A, U>(&self, s: U, doc: &'a D) -> DocBuilder<'a, D, A>
    where
        D: DocAllocator<'a, A>,
        D::Doc: Clone,
        A: Clone,
        U: Into<Cow<'a, str>>,
    {
        doc.text(s)
    }

    fn pp_base<D, A>(
        &self,
    ) -> impl Fn(&IriS, &'a D, &ShExCompactPrinter<'a>) -> DocBuilder<'a, D, A>
    where
        D: DocAllocator<'a, A>,
        D::Doc: Clone,
        A: Clone + 'a,
    {
        move |base, doc, printer| {
            doc.text("base")
                .append(doc.space())
                .append(printer.pp_iri(&base, doc))
        }
    }

    fn pp_prefix_map<D, A>(
        &self,
    ) -> impl Fn(&PrefixMap, &'a D, &ShExCompactPrinter<'a>) -> DocBuilder<'a, D, A>
    where
        D: DocAllocator<'a, A>,
        D::Doc: Clone,
        A: Clone + 'a,
    {
        move |pm, doc, printer| {
            let mut pms: Vec<DocBuilder<'a, D, A>> = Vec::new();
            for (alias, iri) in pm.map.clone().into_iter() {
                pms.push(
                    doc.text("prefix")
                        .append(doc.space())
                        .append(doc.text(alias))
                        .append(doc.text(":"))
                        .append(doc.space())
                        .append(printer.pp_iri_unqualified(&iri, doc)),
                )
            }
            doc.intersperse(pms, doc.hardline()).append(doc.hardline())
        }
    }

    fn pp_iri_unqualified<D, A>(&self, iri: &IriS, doc: &'a D) -> DocBuilder<'a, D, A>
    where
        D: DocAllocator<'a, A>,
        D::Doc: Clone,
        A: Clone,
    {
        doc.text("<")
            .append(doc.text(iri.to_string()))
            .append(doc.text(">"))
    }

    fn pp_iri<'b, D, A>(&self, iri: &IriS, doc: &'b D) -> DocBuilder<'b, D, A>
    where
        D: DocAllocator<'b, A>,
        D::Doc: Clone,
        A: Clone + 'b,
    {
        match self.schema.prefixmap() {
            Some(pm) => doc.text(pm.qualify(iri)),
            None => doc
                .text("<")
                .append(doc.text(iri.to_string()))
                .append(doc.text(">")),
        }
    }

    fn opt_pp<D, A, V>(
        &self,
        maybe: Option<V>,
        pp: impl Fn(&V, &'a D, &ShExCompactPrinter<'a>) -> DocBuilder<'a, D, A>,
        allocator: &'a D,
    ) -> DocBuilder<'a, D, A>
    where
        D: DocAllocator<'a, A>,
        D::Doc: Clone,
        A: Clone + 'a,
    {
        match maybe {
            None => allocator.nil(),
            Some(ref v) => pp(v, allocator, self),
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
        let s = ShExCompactPrinter::new(&schema).pretty_print();
        assert_eq!(
            s,
            "prefix : <http://example.org/>\nprefix schema: <https://schema.org/>"
        );
    }
}
