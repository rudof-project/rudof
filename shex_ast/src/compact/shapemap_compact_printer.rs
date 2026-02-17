use crate::ObjectValue;
use crate::shapemap::SHACLPathRef;
use crate::shapemap::node_selector::Pattern;
use crate::shapemap::{Association, NodeSelector, ShapeSelector, query_shape_map::QueryShapeMap};
use crate::{keyword, pp_label, pp_object_value};
use colored::*;
use prefixmap::{IriRef, PrefixMap};
use pretty::{Arena, DocAllocator, DocBuilder};
use rust_decimal::Decimal;
use rdf::rdf_core::term::literal::{ConcreteLiteral, NumericLiteral, Lang};
use std::borrow::Cow;
use std::marker::PhantomData;

/// Struct that can be used to pretty print Shapemaps
///
#[derive(Debug, Clone)]
pub struct ShapemapFormatter {
    keyword_color: Option<Color>,
    qualify_prefix_color: Option<Color>,
    qualify_semicolon_color: Option<Color>,
    qualify_localname_color: Option<Color>,
}

impl ShapemapFormatter {
    pub fn with_keyword_color(mut self, color: Option<Color>) -> Self {
        self.keyword_color = color;
        self
    }
    pub fn with_qualify_prefix_color(mut self, color: Option<Color>) -> Self {
        self.qualify_prefix_color = color;
        self
    }
    pub fn with_semicolon_prefix_color(mut self, color: Option<Color>) -> Self {
        self.qualify_semicolon_color = color;
        self
    }

    pub fn with_qualify_localname_color(mut self, color: Option<Color>) -> Self {
        self.qualify_localname_color = color;
        self
    }

    pub fn without_colors(mut self) -> Self {
        self.keyword_color = None;
        self.qualify_localname_color = None;
        self.qualify_prefix_color = None;
        self.qualify_semicolon_color = None;
        self
    }

    pub fn format_shapemap(&self, shapemap: &QueryShapeMap) -> String {
        let arena = Arena::<()>::new();
        let mut printer = ShapemapCompactPrinter::new(shapemap, &arena);
        printer = printer.with_keyword_color(self.keyword_color);
        printer = printer.with_qualify_localname_color(self.qualify_localname_color);
        printer = printer.with_qualify_prefix_color(self.qualify_prefix_color);
        printer = printer.with_qualify_semicolon_color(self.qualify_semicolon_color);
        printer.pretty_print()
    }

    pub fn write_shapemap<W: std::io::Write>(
        &self,
        shapemap: &QueryShapeMap,
        writer: &mut W,
    ) -> Result<(), std::io::Error> {
        let arena = Arena::<()>::new();
        let mut printer = ShapemapCompactPrinter::new(shapemap, &arena);
        printer = printer.with_keyword_color(self.keyword_color);
        printer = printer.with_qualify_localname_color(self.qualify_localname_color);
        printer = printer.with_qualify_prefix_color(self.qualify_prefix_color);
        printer = printer.with_qualify_semicolon_color(self.qualify_semicolon_color);
        printer.pretty_print_write(writer)
    }
}

impl Default for ShapemapFormatter {
    fn default() -> Self {
        Self {
            keyword_color: DEFAULT_KEYWORD_COLOR,
            qualify_prefix_color: DEFAULT_QUALIFY_ALIAS_COLOR,
            qualify_semicolon_color: DEFAULT_QUALIFY_SEMICOLON_COLOR,
            qualify_localname_color: DEFAULT_QUALIFY_LOCALNAME_COLOR,
        }
    }
}

struct ShapemapCompactPrinter<'a, A>
where
    A: Clone,
{
    width: usize,
    keyword_color: Option<Color>,
    string_color: Option<Color>,
    shapemap: &'a QueryShapeMap,
    doc: &'a Arena<'a, A>,
    marker: PhantomData<A>,
    nodes_prefixmap: PrefixMap,
    shapes_prefixmap: PrefixMap,
}

const DEFAULT_WIDTH: usize = 100;
const DEFAULT_QUALIFY_ALIAS_COLOR: Option<Color> = Some(Color::Blue);
const DEFAULT_QUALIFY_SEMICOLON_COLOR: Option<Color> = Some(Color::BrightGreen);
const DEFAULT_QUALIFY_LOCALNAME_COLOR: Option<Color> = Some(Color::Black);
const DEFAULT_KEYWORD_COLOR: Option<Color> = Some(Color::BrightBlue);
const DEFAULT_STRING_COLOR: Option<Color> = Some(Color::Red);

impl<'a, A> ShapemapCompactPrinter<'a, A>
where
    A: Clone,
{
    pub fn new(shapemap: &'a QueryShapeMap, doc: &'a Arena<'a, A>) -> ShapemapCompactPrinter<'a, A> {
        ShapemapCompactPrinter {
            width: DEFAULT_WIDTH,
            keyword_color: DEFAULT_KEYWORD_COLOR,
            string_color: DEFAULT_STRING_COLOR,
            shapemap,
            doc,
            marker: PhantomData,
            nodes_prefixmap: shapemap
                .nodes_prefixmap()
                .with_qualify_localname_color(DEFAULT_QUALIFY_LOCALNAME_COLOR)
                .with_qualify_prefix_color(DEFAULT_QUALIFY_ALIAS_COLOR)
                .with_qualify_semicolon_color(DEFAULT_QUALIFY_SEMICOLON_COLOR),
            shapes_prefixmap: shapemap
                .shapes_prefixmap()
                .with_qualify_localname_color(DEFAULT_QUALIFY_LOCALNAME_COLOR)
                .with_qualify_prefix_color(DEFAULT_QUALIFY_ALIAS_COLOR)
                .with_qualify_semicolon_color(DEFAULT_QUALIFY_SEMICOLON_COLOR),
        }
    }

    pub fn with_keyword_color(mut self, color: Option<Color>) -> Self {
        self.keyword_color = color;
        self
    }

    pub fn with_qualify_prefix_color(mut self, color: Option<Color>) -> Self {
        self.nodes_prefixmap = self.nodes_prefixmap.with_qualify_prefix_color(color);
        self.shapes_prefixmap = self.shapes_prefixmap.with_qualify_prefix_color(color);
        self
    }

    pub fn with_qualify_semicolon_color(mut self, color: Option<Color>) -> Self {
        self.nodes_prefixmap = self.nodes_prefixmap.with_qualify_semicolon_color(color);
        self.shapes_prefixmap = self.shapes_prefixmap.with_qualify_semicolon_color(color);
        self
    }

    pub fn with_qualify_localname_color(mut self, color: Option<Color>) -> Self {
        self.nodes_prefixmap = self.nodes_prefixmap.with_qualify_localname_color(color);
        self.shapes_prefixmap = self.shapes_prefixmap.with_qualify_localname_color(color);
        self
    }

    pub fn pretty_print_write<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
        let doc = self.pp_shapemap();
        doc.render(self.width, writer)
    }

    pub fn pretty_print(&self) -> String {
        let doc = self.pp_shapemap();
        doc.pretty(self.width).to_string()
    }

    fn pp_shapemap(&self) -> DocBuilder<'a, Arena<'a, A>, A> {
        let mut docs = Vec::new();
        for a in self.shapemap.iter() {
            docs.push(self.pp_association(a))
        }
        self.doc.intersperse(docs, self.doc.hardline())
    }

    fn pp_association(&self, assoc: &Association) -> DocBuilder<'a, Arena<'a, A>, A> {
        self.pp_node_selector(&assoc.node_selector)
            .append(self.doc.text("@"))
            .append(self.pp_shape_selector(&assoc.shape_selector))
    }

    fn pp_node_selector(&self, ns: &NodeSelector) -> DocBuilder<'a, Arena<'a, A>, A> {
        match ns {
            NodeSelector::Node(v) => pp_object_value(v, self.doc, &self.nodes_prefixmap),
            NodeSelector::TriplePattern { subject, path, object } => self
                .doc
                .text("{")
                .append(self.space())
                .append(self.pp_pattern(subject))
                .append(self.space())
                .append(self.pp_shacl_path(path, &self.nodes_prefixmap))
                .append(self.space())
                .append(self.pp_pattern(object))
                .append(self.space())
                .append(self.doc.text("}")),
            NodeSelector::Sparql { query } => self
                .keyword("SPARQL")
                .append(self.space())
                .append(self.triple_quotes(query)),
            NodeSelector::Generic { .. } => todo!(),
        }
    }

    fn space(&self) -> DocBuilder<'a, Arena<'a, A>, A> {
        self.doc.space()
    }

    pub fn triple_quotes(&self, str: &str) -> DocBuilder<'a, Arena<'a, A>, A> {
        self.doc.text(format!("'''{str}'''"))
    }

    fn pp_shacl_path(&self, path: &SHACLPathRef, prefixmap: &PrefixMap) -> DocBuilder<'a, Arena<'a, A>, A> {
        match path {
            SHACLPathRef::Predicate { pred } => self.pp_iri_ref(pred, prefixmap),
            _ => todo!(),
        }
    }

    pub fn pp_pattern(&self, pat: &Pattern) -> DocBuilder<'a, Arena<'a, A>, A> {
        match pat {
            Pattern::Node(object_value) => self.pp_object_value(object_value),
            Pattern::Wildcard => self.doc.text("_"),
            Pattern::Focus => self.keyword("FOCUS"),
        }
    }

    pub fn pp_object_value(&self, ov: &ObjectValue) -> DocBuilder<'a, Arena<'a, A>, A> {
        match ov {
            ObjectValue::IriRef(iri_ref) => self.pp_iri_ref(iri_ref, &self.nodes_prefixmap),
            ObjectValue::Literal(sliteral) => self.pp_literal(sliteral),
        }
    }

    fn pp_literal(&self, literal: &ConcreteLiteral) -> DocBuilder<'a, Arena<'a, A>, A> {
        match literal {
            ConcreteLiteral::StringLiteral { lexical_form, lang } => {
                self.pp_string_literal(lexical_form, lang)
            }
            ConcreteLiteral::DatatypeLiteral {
                lexical_form: _,
                datatype: _,
            } => todo!(),
            ConcreteLiteral::WrongDatatypeLiteral {
                lexical_form: _,
                datatype: _,
                error: _,
            } => todo!(),
            ConcreteLiteral::NumericLiteral(lit) => self.pp_numeric_literal(lit),
            ConcreteLiteral::BooleanLiteral(_) => todo!(),
            ConcreteLiteral::DatetimeLiteral(_xsd_date_time) => todo!(),
        }
    }

    fn pp_numeric_literal(&self, value: &NumericLiteral) -> DocBuilder<'a, Arena<'a, A>, A> {
        match value {
            NumericLiteral::Integer(n) => self.pp_isize(&(*n as isize)),
            NumericLiteral::Decimal(d) => self.pp_decimal(d),
            NumericLiteral::Double(d) => self.pp_double(d),
            NumericLiteral::Long(l) => self.pp_isize(&(*l as isize)),
            NumericLiteral::Float(f) => self.pp_float(&(*f as f64)),
            NumericLiteral::Byte(b) => self.pp_byte(b),
            NumericLiteral::Short(s) => self.pp_short(s),
            NumericLiteral::NonNegativeInteger(n) => self.pp_non_negative_integer(n),
            NumericLiteral::UnsignedLong(u) => self.pp_unsigned_long(u),
            NumericLiteral::UnsignedInt(u) => self.pp_unsigned_int(u),
            NumericLiteral::UnsignedShort(u) => self.pp_unsigned_short(u),
            NumericLiteral::UnsignedByte(n) => self.pp_unsigned_byte(n),
            NumericLiteral::PositiveInteger(n) => self.pp_positive_integer(n),
            NumericLiteral::NegativeInteger(n) => self.pp_negative_integer(n),
            NumericLiteral::NonPositiveInteger(n) => self.pp_non_positive_integer(n),
        }
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

    fn pp_float(&self, value: &f64) -> DocBuilder<'a, Arena<'a, A>, A> {
        self.doc.text(value.to_string())
    }

    fn pp_byte(&self, value: &i8) -> DocBuilder<'a, Arena<'a, A>, A> {
        self.doc.text(value.to_string())
    }

    fn pp_short(&self, value: &i16) -> DocBuilder<'a, Arena<'a, A>, A> {
        self.doc.text(value.to_string())
    }

    fn pp_non_negative_integer(&self, value: &u128) -> DocBuilder<'a, Arena<'a, A>, A> {
        self.doc.text(value.to_string())
    }

    fn pp_unsigned_long(&self, value: &u64) -> DocBuilder<'a, Arena<'a, A>, A> {
        self.doc.text(value.to_string())
    }

    fn pp_unsigned_int(&self, value: &u32) -> DocBuilder<'a, Arena<'a, A>, A> {
        self.doc.text(value.to_string())
    }

    fn pp_unsigned_short(&self, value: &u16) -> DocBuilder<'a, Arena<'a, A>, A> {
        self.doc.text(value.to_string())
    }

    fn pp_unsigned_byte(&self, value: &u8) -> DocBuilder<'a, Arena<'a, A>, A> {
        self.doc.text(value.to_string())
    }

    fn pp_positive_integer(&self, value: &u128) -> DocBuilder<'a, Arena<'a, A>, A> {
        self.doc.text(value.to_string())
    }

    fn pp_negative_integer(&self, value: &i128) -> DocBuilder<'a, Arena<'a, A>, A> {
        self.doc.text(value.to_string())
    }

    fn pp_non_positive_integer(&self, value: &i128) -> DocBuilder<'a, Arena<'a, A>, A> {
        self.doc.text(value.to_string())
    }

    fn pp_string_literal(&self, lexical_form: &str, lang: &Option<Lang>) -> DocBuilder<'a, Arena<'a, A>, A> {
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

    fn pp_iri_ref(&self, value: &IriRef, prefixmap: &PrefixMap) -> DocBuilder<'a, Arena<'a, A>, A> {
        match value {
            IriRef::Iri(iri) => self.doc.text(prefixmap.qualify(iri)),
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

    fn pp_shape_selector(&self, s: &ShapeSelector) -> DocBuilder<'a, Arena<'a, A>, A> {
        match s {
            ShapeSelector::Label(label) => pp_label(label, self.doc, &self.shapes_prefixmap, self.keyword_color),
            ShapeSelector::Start => keyword("START", self.doc, self.keyword_color),
        }
    }
}

#[cfg(test)]
mod tests {}
