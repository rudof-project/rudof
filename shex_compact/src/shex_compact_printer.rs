use crate::pp_object_value;
use colored::*;
use iri_s::IriS;
use prefixmap::{IriRef, PrefixMap};
use pretty::{Arena, DocAllocator, DocBuilder, RefDoc};
use rust_decimal::Decimal;
/// This file converts ShEx AST to ShEx compact syntax
use shex_ast::{
    value_set_value::ValueSetValue, Annotation, BNode, IriOrStr, NodeConstraint, NodeKind,
    NumericFacet, ObjectValue, Pattern, Schema, SemAct, Shape, ShapeDecl, ShapeExpr,
    ShapeExprLabel, StringFacet, TripleExpr, XsFacet,
};
use srdf::graph::lang::Lang;
use srdf::graph::literal::Literal;
use srdf::graph::numeric_literal::NumericLiteral;
use std::{borrow::Cow, io, marker::PhantomData};

/// Struct that can be used to pretty print ShEx schemas
///
/// Example:
/// ```
/// use shex_compact::ShExFormatter;
/// use shex_ast::{Schema, ShapeExprLabel, ShapeExpr};
/// use iri_s::IriS;
///
/// let mut schema = Schema::new();
/// schema.add_prefix("ex", &IriS::new_unchecked("http://example.org/"));
/// schema.add_shape(ShapeExprLabel::iri_unchecked("http://example.org/S"), ShapeExpr::empty_shape(), false);
///
/// let expected = r#"prefix ex: <http://example.org/>
/// ex:S {  }"#;
///
/// assert_eq!(ShExFormatter::default().format_schema(&schema), expected);
/// ```
#[derive(Debug, Clone)]
pub struct ShExFormatter {
    keyword_color: Option<Color>,
    string_color: Option<Color>,
    prefix_color: Option<Color>,
    semicolon_color: Option<Color>,
    localname_color: Option<Color>,
}

impl ShExFormatter {
    pub fn keyword_color(&self) -> Option<Color> {
        self.keyword_color
    }

    pub fn prefix_color(&self) -> Option<Color> {
        self.prefix_color
    }

    pub fn semicolon_color(&self) -> Option<Color> {
        self.semicolon_color
    }

    pub fn localname_color(&self) -> Option<Color> {
        self.localname_color
    }

    pub fn with_keyword_color(mut self, color: Option<Color>) -> ShExFormatter {
        self.keyword_color = color;
        self
    }
    pub fn with_prefix_color(mut self, color: Option<Color>) -> ShExFormatter {
        self.prefix_color = color;
        self
    }

    pub fn with_semicolon_color(mut self, color: Option<Color>) -> ShExFormatter {
        self.semicolon_color = color;
        self
    }

    pub fn with_string_color(mut self, color: Option<Color>) -> ShExFormatter {
        self.string_color = color;
        self
    }

    pub fn with_localname_color(mut self, color: Option<Color>) -> ShExFormatter {
        self.localname_color = color;
        self
    }

    /// Changes the formatter to avoid showing colors
    pub fn without_colors(self) -> ShExFormatter {
        self.with_keyword_color(None)
            .with_localname_color(None)
            .with_prefix_color(None)
            .with_string_color(None)
            .with_semicolon_color(None)
    }

    pub fn format_schema(&self, schema: &Schema) -> String {
        let arena = Arena::<()>::new();
        let mut printer = ShExCompactPrinter::new(schema, &arena);
        printer = printer.with_keyword_color(self.keyword_color);
        printer = printer.with_string_color(self.string_color);
        printer = printer.with_qualify_localname_color(self.localname_color);
        printer = printer.with_qualify_prefix_color(self.prefix_color);
        printer = printer.with_qualify_semicolon_color(self.semicolon_color);
        printer.pretty_print()
    }

    pub fn write_schema<W: std::io::Write>(
        &self,
        schema: &Schema,
        writer: &mut W,
    ) -> Result<(), std::io::Error> {
        let arena = Arena::<()>::new();
        let mut printer = ShExCompactPrinter::new(schema, &arena);
        printer = printer.with_keyword_color(self.keyword_color);
        printer = printer.with_string_color(self.string_color);
        printer = printer.with_qualify_localname_color(self.localname_color);
        printer = printer.with_qualify_prefix_color(self.prefix_color);
        printer = printer.with_qualify_semicolon_color(self.semicolon_color);
        printer.pretty_print_write(writer)
    }
}

impl Default for ShExFormatter {
    fn default() -> Self {
        Self {
            keyword_color: DEFAULT_KEYWORD_COLOR,
            prefix_color: DEFAULT_QUALIFY_ALIAS_COLOR,
            semicolon_color: DEFAULT_QUALIFY_SEMICOLON_COLOR,
            string_color: DEFAULT_STRING_COLOR,
            localname_color: DEFAULT_QUALIFY_LOCALNAME_COLOR,
        }
    }
}

struct ShExCompactPrinter<'a, A>
where
    A: Clone,
{
    width: usize,
    indent: isize,
    keyword_color: Option<Color>,
    string_color: Option<Color>,
    schema: &'a Schema,
    doc: &'a Arena<'a, A>,
    marker: PhantomData<A>,
    prefixmap: PrefixMap,
}

const DEFAULT_WIDTH: usize = 100;
const DEFAULT_INDENT: isize = 4;
const DEFAULT_QUALIFY_ALIAS_COLOR: Option<Color> = Some(Color::Blue);
const DEFAULT_QUALIFY_SEMICOLON_COLOR: Option<Color> = Some(Color::BrightGreen);
const DEFAULT_QUALIFY_LOCALNAME_COLOR: Option<Color> = Some(Color::Black);
const DEFAULT_KEYWORD_COLOR: Option<Color> = Some(Color::BrightBlue);
const DEFAULT_STRING_COLOR: Option<Color> = Some(Color::Red);

impl<'a, A> ShExCompactPrinter<'a, A>
where
    A: Clone,
{
    pub fn new(schema: &'a Schema, doc: &'a Arena<'a, A>) -> ShExCompactPrinter<'a, A> {
        ShExCompactPrinter {
            width: DEFAULT_WIDTH,
            indent: DEFAULT_INDENT,
            keyword_color: DEFAULT_KEYWORD_COLOR,
            string_color: DEFAULT_STRING_COLOR,
            schema,
            doc,
            marker: PhantomData,
            prefixmap: schema
                .prefixmap()
                .unwrap_or_default()
                .with_qualify_localname_color(DEFAULT_QUALIFY_LOCALNAME_COLOR)
                .with_qualify_prefix_color(DEFAULT_QUALIFY_ALIAS_COLOR)
                .with_qualify_semicolon_color(DEFAULT_QUALIFY_SEMICOLON_COLOR),
        }
    }

    pub fn with_keyword_color(mut self, color: Option<Color>) -> Self {
        self.keyword_color = color;
        self
    }

    pub fn with_string_color(mut self, color: Option<Color>) -> Self {
        self.string_color = color;
        self
    }

    pub fn with_qualify_prefix_color(mut self, color: Option<Color>) -> Self {
        self.prefixmap = self.prefixmap.with_qualify_prefix_color(color);
        self
    }

    pub fn with_qualify_semicolon_color(mut self, color: Option<Color>) -> Self {
        self.prefixmap = self.prefixmap.with_qualify_semicolon_color(color);
        self
    }

    pub fn with_qualify_localname_color(mut self, color: Option<Color>) -> Self {
        self.prefixmap = self.prefixmap.with_qualify_localname_color(color);
        self
    }

    /// Pretty print to a String
    pub fn pretty_print(&self) -> String {
        let doc = self.pp_schema();
        doc.pretty(self.width).to_string()
    }

    /// Writes a ShEx schema to a `std::io::Write` object
    pub fn pretty_print_write<W: io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
        let doc = self.pp_schema();
        doc.render(self.width, writer)
    }

    fn pp_schema(&self) -> DocBuilder<'a, Arena<'a, A>, A> {
        self.opt_pp(self.schema.prefixmap(), self.pp_prefix_map())
            .append(self.opt_pp(self.schema.base(), self.pp_base()))
            .append(self.pp_imports(self.schema.imports()))
            .append(self.opt_pp(self.schema.start_actions(), self.pp_actions()))
            .append(self.opt_pp(self.schema.start(), self.pp_start()))
            .append(self.opt_pp(self.schema.shapes(), self.pp_shape_decls()))
    }

    fn pp_imports(&self, imports: Vec<IriOrStr>) -> DocBuilder<'a, Arena<'a, A>, A> {
        if imports.is_empty() {
            self.doc.nil()
        } else {
            let mut docs = Vec::new();
            for import in imports {
                docs.push(
                    self.keyword("import")
                        .append(self.space())
                        .append(self.pp_iri_or_str(import)),
                )
            }
            self.doc
                .intersperse(docs, self.doc.hardline())
                .append(self.doc.hardline())
        }
    }

    fn pp_iri_or_str(&self, iri_or_str: IriOrStr) -> DocBuilder<'a, Arena<'a, A>, A> {
        match iri_or_str {
            IriOrStr::IriS(iri) => self.pp_iri(&iri),
            IriOrStr::String(str) => self.pp_str(format!("<{}>", str.as_str()).as_str()),
        }
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
        self.pp_label(&sd.id)
            .append(self.space())
            .append(self.pp_shape_expr(&sd.shape_expr))
    }

    fn pp_start(
        &self,
    ) -> impl Fn(&ShapeExpr, &ShExCompactPrinter<'a, A>) -> DocBuilder<'a, Arena<'a, A>, A> {
        move |se, printer| {
            printer
                .keyword("start")
                .append(printer.space())
                .append("=")
                .append(printer.space())
                .append(printer.pp_shape_expr(se))
                .append(printer.doc.hardline())
        }
    }

    fn pp_shape_expr(&self, se: &ShapeExpr) -> DocBuilder<'a, Arena<'a, A>, A> {
        match se {
            ShapeExpr::Ref(ref_) => self.doc.text("@").append(self.pp_label(ref_)),
            ShapeExpr::Shape(s) => self.pp_shape(s),
            ShapeExpr::NodeConstraint(nc) => self.pp_node_constraint(nc),
            ShapeExpr::External => self.pp_external(),
            ShapeExpr::ShapeAnd { shape_exprs } => {
                let mut docs = Vec::new();
                for sew in shape_exprs {
                    docs.push(self.pp_shape_expr(&sew.se))
                }
                self.doc
                    .intersperse(docs, self.keyword(" AND "))
                    .group()
                    .nest(self.indent)
            }
            ShapeExpr::ShapeOr { shape_exprs } => {
                let mut docs = Vec::new();
                for sew in shape_exprs {
                    docs.push(self.pp_shape_expr(&sew.se))
                }
                self.doc
                    .intersperse(docs, self.keyword(" OR "))
                    .group()
                    .nest(self.indent)
            }
            ShapeExpr::ShapeNot { shape_expr } => self
                .doc
                .nil()
                .append(self.keyword("NOT "))
                .append(self.pp_shape_expr(&shape_expr.se)),
        }
    }

    fn space(&self) -> DocBuilder<'a, Arena<'a, A>, A> {
        self.doc.space()
    }

    fn pp_shape(&self, s: &Shape) -> DocBuilder<'a, Arena<'a, A>, A> {
        let closed = if s.is_closed() {
            self.keyword("CLOSED ")
        } else {
            self.doc.nil()
        };
        let extra = self.opt_pp1(&s.extra, self.pp_extra());
        let extends = self.opt_pp1(&s.extends, self.pp_extends());
        let annotations = self.opt_pp1(&s.annotations, self.pp_annotations());
        closed
            .append(extra)
            .append(extends)
            .append(self.doc.text("{"))
            .append(self.doc.line())
            .append(self.opt_pp(s.triple_expr(), self.pp_triple_expr()))
            .nest(self.indent)
            .append(self.doc.line())
            .append(self.doc.text("}"))
            .append(annotations)
            .group()
    }

    fn pp_extends(
        &self,
    ) -> impl Fn(&Vec<ShapeExprLabel>, &ShExCompactPrinter<'a, A>) -> DocBuilder<'a, Arena<'a, A>, A>
    {
        move |vs, printer| {
            let mut docs = Vec::new();
            for v in vs {
                docs.push(printer.pp_reference(v))
            }
            printer
                .doc
                .nil()
                .append(printer.keyword("EXTENDS"))
                .append(printer.space())
                .append(printer.doc.intersperse(docs, printer.doc.space()))
                .append(printer.space())
        }
    }

    fn pp_annotations(
        &self,
    ) -> impl Fn(&Vec<Annotation>, &ShExCompactPrinter<'a, A>) -> DocBuilder<'a, Arena<'a, A>, A>
    {
        move |vs, printer| {
            let mut docs = Vec::new();
            for a in vs {
                docs.push(printer.pp_annotation(a))
            }
            printer
                .doc
                .nil()
                .append(printer.space())
                .append(printer.doc.intersperse(docs, printer.doc.softline()))
        }
    }

    fn pp_triple_expr(
        &self,
    ) -> impl Fn(&TripleExpr, &ShExCompactPrinter<'a, A>) -> DocBuilder<'a, Arena<'a, A>, A> + '_
    {
        move |te, printer| match te {
            TripleExpr::EachOf { expressions, .. } => {
                let mut docs = Vec::new();
                for e in expressions {
                    let pp_te = printer.pp_triple_expr()(&e.te, printer);
                    docs.push(pp_te)
                }
                printer
                    .doc
                    .intersperse(docs, printer.doc.text(";").append(printer.doc.line()))
            }
            TripleExpr::OneOf { .. } => todo!(),
            TripleExpr::TripleConstraint {
                negated,
                inverse,
                predicate,
                value_expr,
                min,
                max,
                sem_acts,
                annotations,
                ..
            } => {
                let doc_expr = match value_expr {
                    Some(se) => printer.pp_shape_expr(se),
                    None => printer.doc.text("."),
                };
                printer
                    .doc
                    .nil()
                    .append(self.pp_negated(negated))
                    .append(self.pp_inverse(inverse))
                    .append(self.pp_iri_ref(predicate))
                    .append(self.doc.space())
                    .append(doc_expr)
                    .append(self.pp_cardinality(min, max))
                    .append(self.opt_pp1(sem_acts, self.pp_actions()))
                    .append(self.opt_pp1(annotations, self.pp_annotations()))
            }
            TripleExpr::TripleExprRef(_) => todo!(),
        }
    }

    // type DB<'a, A> = DocBuilder<'a, Arena<'a, A>, A>;

    fn pp_negated(&self, negated: &Option<bool>) -> DocBuilder<'a, Arena<'a, A>, A> {
        match negated {
            Some(true) => self.doc.text("!"),
            _ => self.doc.nil(),
        }
    }

    fn pp_inverse(&self, inverse: &Option<bool>) -> DocBuilder<'a, Arena<'a, A>, A> {
        match inverse {
            Some(true) => self.doc.text("^"),
            _ => self.doc.nil(),
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
            (Some(1), None) => self.doc.space().append(self.doc.text("+")),
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
                .append(self.doc.text(m.to_string()))
                .append(self.doc.text(",}")),
            (None, Some(-1)) => self.doc.space().append(self.doc.text("*")),
            (None, Some(n)) => self
                .doc
                .space()
                .append(self.doc.text("{,"))
                .append(self.doc.text(n.to_string()))
                .append(self.doc.text("}")),
            (None, None) => self.doc.nil(),
        }
    }

    fn pp_extra(
        &self,
    ) -> impl Fn(&Vec<IriRef>, &ShExCompactPrinter<'a, A>) -> DocBuilder<'a, Arena<'a, A>, A> {
        move |es, printer| {
            let mut docs = Vec::new();
            for e in es {
                docs.push(printer.pp_iri_ref(e))
            }
            printer
                .doc
                .nil()
                .append(printer.keyword("EXTRA "))
                .append(printer.doc.intersperse(docs, printer.doc.space()))
        }
    }

    fn pp_annotation(&self, annotation: &Annotation) -> DocBuilder<'a, Arena<'a, A>, A> {
        let predicate = self.pp_iri_ref(&annotation.predicate());
        let object = self.pp_object_value(&annotation.object());
        self.keyword("//")
            .append(self.space())
            .append(predicate)
            .append(self.space())
            .append(object)
    }

    fn pp_object_value(&self, object_value: &ObjectValue) -> DocBuilder<'a, Arena<'a, A>, A> {
        match object_value {
            ObjectValue::IriRef(iri_ref) => self.pp_iri_ref(iri_ref),
            ObjectValue::Literal(lit) => self.pp_literal(lit),
        }
    }

    fn pp_literal(&self, literal: &Literal) -> DocBuilder<'a, Arena<'a, A>, A> {
        match literal {
            Literal::StringLiteral { lexical_form, lang } => {
                self.pp_string_literal(lexical_form, lang)
            }
            Literal::DatatypeLiteral {
                lexical_form: _,
                datatype: _,
            } => todo!(),
            Literal::NumericLiteral(lit) => self.pp_numeric_literal(lit),
            Literal::BooleanLiteral(_) => todo!(),
        }
    }

    fn pp_string_literal(
        &self,
        lexical_form: &str,
        lang: &Option<Lang>,
    ) -> DocBuilder<'a, Arena<'a, A>, A> {
        match lang {
            Some(_) => todo!(),
            None => self.pp_string(lexical_form),
        }
    }

    fn pp_string(&self, str: &str) -> DocBuilder<'a, Arena<'a, A>, A> {
        let s = format!("\"{str}\"");
        if let Some(color) = self.string_color {
            self.doc.text(s.as_str().color(color).to_string())
        } else {
            self.doc.text(s)
        }
    }

    fn pp_node_constraint(&self, nc: &NodeConstraint) -> DocBuilder<'a, Arena<'a, A>, A> {
        self.opt_pp(nc.node_kind(), self.pp_node_kind())
            .append(self.opt_pp(nc.datatype(), self.pp_datatype()))
            .append(self.opt_pp(nc.values(), self.pp_value_set()))
            .append(self.opt_pp(nc.xs_facet(), self.pp_xsfacets()))
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

    fn pp_xsfacets(
        &self,
    ) -> impl Fn(&Vec<XsFacet>, &ShExCompactPrinter<'a, A>) -> DocBuilder<'a, Arena<'a, A>, A> {
        move |xsfacets, printer| {
            let mut docs = Vec::new();
            for v in xsfacets {
                docs.push(printer.pp_xsfacet(v))
            }
            printer
                .doc
                .space()
                .append(printer.doc.intersperse(docs, printer.doc.space()))
        }
    }

    fn pp_xsfacet(&self, xsfacet: &XsFacet) -> DocBuilder<'a, Arena<'a, A>, A> {
        match xsfacet {
            XsFacet::NumericFacet(nf) => self.pp_numericfacet(nf),
            XsFacet::StringFacet(sf) => self.pp_stringfacet(sf),
        }
    }

    fn pp_numericfacet(&self, nf: &NumericFacet) -> DocBuilder<'a, Arena<'a, A>, A> {
        match nf {
            NumericFacet::FractionDigits(fd) => self
                .keyword("FractionDigits")
                .append(self.space())
                .append(self.pp_usize(fd)),
            NumericFacet::TotalDigits(td) => self
                .keyword("TotalDigits")
                .append(self.space())
                .append(self.pp_usize(td)),
            NumericFacet::MinInclusive(m) => self
                .keyword("MinInclusive")
                .append(self.space())
                .append(self.pp_numeric_literal(m)),
            NumericFacet::MaxInclusive(m) => self
                .keyword("MaxInclusive")
                .append(self.space())
                .append(self.pp_numeric_literal(m)),
            NumericFacet::MinExclusive(m) => self
                .keyword("MinExclusive")
                .append(self.space())
                .append(self.pp_numeric_literal(m)),
            NumericFacet::MaxExclusive(m) => self
                .keyword("MaxExclusive")
                .append(self.space())
                .append(self.pp_numeric_literal(m)),
        }
    }

    fn pp_stringfacet(&self, sf: &StringFacet) -> DocBuilder<'a, Arena<'a, A>, A> {
        match sf {
            StringFacet::Length(l) => self
                .keyword("Length")
                .append(self.space())
                .append(self.pp_usize(l)),
            StringFacet::MinLength(l) => self
                .keyword("MinLength")
                .append(self.space())
                .append(self.pp_usize(l)),
            StringFacet::MaxLength(l) => self
                .keyword("MaxLength")
                .append(self.space())
                .append(self.pp_usize(l)),
            StringFacet::Pattern(pat) => self.pp_pattern(pat),
        }
    }

    fn pp_pattern(&self, pattern: &Pattern) -> DocBuilder<'a, Arena<'a, A>, A> {
        let flags = match &pattern.flags {
            Some(flags) => flags.clone(),
            None => "".to_string(),
        };
        let str = format!("/{}/{}", pattern.str, flags);
        self.doc.text(str)
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
            ValueSetValue::LanguageStem { .. } => todo!(),
            ValueSetValue::LanguageStemRange { .. } => todo!(),
            ValueSetValue::ObjectValue(ov) => pp_object_value(ov, self.doc, &self.prefixmap),
            ValueSetValue::IriStem { .. } => todo!(),
            ValueSetValue::IriStemRange { .. } => todo!(),
            ValueSetValue::LiteralStem { .. } => todo!(),
            ValueSetValue::LiteralStemRange { .. } => todo!(),
            ValueSetValue::Language { .. } => todo!(),
        }
    }

    fn pp_label(&self, ref_: &ShapeExprLabel) -> DocBuilder<'a, Arena<'a, A>, A> {
        match ref_ {
            ShapeExprLabel::BNode { value } => self.pp_bnode(value),
            ShapeExprLabel::IriRef { value } => self.pp_iri_ref(value),
            ShapeExprLabel::Start => self.keyword("START"),
        }
    }

    fn pp_reference(&self, ref_: &ShapeExprLabel) -> DocBuilder<'a, Arena<'a, A>, A> {
        self.doc.text("@").append(self.pp_label(ref_))
    }

    fn pp_numeric_literal(&self, value: &NumericLiteral) -> DocBuilder<'a, Arena<'a, A>, A> {
        match value {
            NumericLiteral::Integer(n) => self.pp_isize(n),
            NumericLiteral::Decimal(d) => self.pp_decimal(d),
            NumericLiteral::Double(d) => self.pp_double(d), // TODO: Review
        }
    }

    fn pp_bnode(&self, value: &BNode) -> DocBuilder<'a, Arena<'a, A>, A> {
        self.doc.text(format!("{value}"))
    }

    fn pp_isize(&self, value: &isize) -> DocBuilder<'a, Arena<'a, A>, A> {
        self.doc.text(value.to_string())
    }

    fn pp_decimal(&self, value: &Decimal) -> DocBuilder<'a, Arena<'a, A>, A> {
        self.doc.text(value.to_string())
    }

    fn pp_double(&self, value: &f64) -> DocBuilder<'a, Arena<'a, A>, A> {
        self.doc.text(value.to_string())
    }

    fn pp_usize(&self, value: &usize) -> DocBuilder<'a, Arena<'a, A>, A> {
        self.doc.text(value.to_string())
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
        if let Some(color) = self.keyword_color {
            let data: Cow<str> = s.into();
            let s: String = match data {
                Cow::Owned(t) => t,
                Cow::Borrowed(t) => t.into(),
            };
            self.doc.text(s.as_str().color(color).to_string())
        } else {
            let s: String = s.into().into();
            self.doc.text(s)
        }
    }

    fn pp_actions(
        &self,
    ) -> impl Fn(&Vec<SemAct>, &ShExCompactPrinter<'a, A>) -> DocBuilder<'a, Arena<'a, A>, A> {
        move |actions, printer| {
            let mut docs = Vec::new();
            for a in actions {
                docs.push(printer.pp_action(a))
            }
            printer
                .doc
                .intersperse(docs, printer.doc.hardline())
                .append(printer.doc.hardline())
        }
    }

    fn pp_action(&self, a: &SemAct) -> DocBuilder<'a, Arena<'a, A>, A> {
        self.doc
            .text("%")
            .append(self.pp_iri_ref(&a.name()))
            .append(match a.code() {
                None => self.doc.text("%"),
                Some(str) => self
                    .doc
                    .text("{")
                    .append(self.doc.text(str))
                    .append(self.doc.text("%}")),
            })
    }

    fn pp_base(
        &self,
    ) -> impl Fn(&IriS, &ShExCompactPrinter<'a, A>) -> DocBuilder<'a, Arena<'a, A>, A> {
        move |base, printer| {
            printer
                .keyword("base")
                .append(printer.doc.space())
                .append(printer.pp_iri_unqualified(base))
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
                        .nil()
                        .append(printer.keyword("prefix"))
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
        let str = format!("<{iri}>");
        self.doc.text(str)
    }

    fn pp_iri(&self, iri: &IriS) -> DocBuilder<'a, Arena<'a, A>, A> {
        self.doc.text(self.prefixmap.qualify(iri))
    }

    fn pp_str(&self, str: &str) -> DocBuilder<'a, Arena<'a, A>, A> {
        self.doc.text(str.to_string())
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

    fn opt_pp1<V>(
        &self,
        maybe: &Option<V>,
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
            FlatAlt(t1, t2) => Self::is_empty_ref(t1) && Self::is_empty_ref(t2),
            Group(t) => Self::is_empty_ref(t),
            Nest(_, t) => Self::is_empty_ref(t),
            Union(t1, t2) => Self::is_empty_ref(t1) && Self::is_empty_ref(t2),
            Annotated(_, t) => Self::is_empty_ref(t),
            _ => false,
        }
    }

    fn is_empty_ref(rd: &RefDoc<'a, A>) -> bool {
        use pretty::Doc::*;
        match &**rd {
            Nil => true,
            FlatAlt(t1, t2) => Self::is_empty_ref(t1) && Self::is_empty_ref(t2),
            Group(t) => Self::is_empty_ref(t),
            Nest(_, t) => Self::is_empty_ref(t),
            Union(t1, t2) => Self::is_empty_ref(t1) && Self::is_empty_ref(t2),
            Annotated(_, t) => Self::is_empty_ref(t),
            _ => false,
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
        pm.insert("", &IriS::new_unchecked("http://example.org/"))
            .unwrap();
        pm.insert("schema", &IriS::new_unchecked("https://schema.org/"))
            .unwrap();
        let schema = Schema::new().with_prefixmap(Some(pm));
        let s = ShExFormatter::default()
            .without_colors()
            .format_schema(&schema);
        assert_eq!(
            s,
            "prefix : <http://example.org/>\nprefix schema: <https://schema.org/>\n"
        );
    }
}
