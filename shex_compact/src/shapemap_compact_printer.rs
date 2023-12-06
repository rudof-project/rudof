use crate::{keyword, pp_label, pp_object_value};
use colored::*;
use prefixmap::PrefixMap;
use pretty::{Arena, DocAllocator, DocBuilder};
use shapemap::{query_shape_map::QueryShapeMap, Association, NodeSelector, ShapeSelector};
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
    indent: isize,
    keyword_color: Option<Color>,
    shapemap: &'a QueryShapeMap,
    doc: &'a Arena<'a, A>,
    marker: PhantomData<A>,
    nodes_prefixmap: PrefixMap,
    shapes_prefixmap: PrefixMap,
}

const DEFAULT_WIDTH: usize = 100;
const DEFAULT_INDENT: isize = 4;
const DEFAULT_QUALIFY_ALIAS_COLOR: Option<Color> = Some(Color::Blue);
const DEFAULT_QUALIFY_SEMICOLON_COLOR: Option<Color> = Some(Color::BrightGreen);
const DEFAULT_QUALIFY_LOCALNAME_COLOR: Option<Color> = Some(Color::Black);
const DEFAULT_KEYWORD_COLOR: Option<Color> = Some(Color::BrightBlue);

impl<'a, A> ShapemapCompactPrinter<'a, A>
where
    A: Clone,
{
    pub fn new(
        shapemap: &'a QueryShapeMap,
        doc: &'a Arena<'a, A>,
    ) -> ShapemapCompactPrinter<'a, A> {
        ShapemapCompactPrinter {
            width: DEFAULT_WIDTH,
            indent: DEFAULT_INDENT,
            keyword_color: DEFAULT_KEYWORD_COLOR,
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

    pub fn with_width(mut self, width: usize) -> Self {
        self.width = width;
        self
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

    pub fn pretty_print(&self) -> String {
        let doc = self.pp_shapemap();
        doc.pretty(self.width as usize).to_string()
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
            NodeSelector::Node(v) => pp_object_value(&v, self.doc, &self.nodes_prefixmap),
            NodeSelector::TriplePattern {
                subject,
                pred,
                object,
            } => todo!(),
            NodeSelector::TriplePatternPath {
                subject,
                pred,
                object,
            } => todo!(),
            NodeSelector::Sparql { query } => todo!(),
            NodeSelector::Generic { iri, param } => todo!(),
        }
    }

    fn pp_shape_selector(&self, s: &ShapeSelector) -> DocBuilder<'a, Arena<'a, A>, A> {
        match s {
            ShapeSelector::Label(label) => pp_label(&label, self.doc, &self.shapes_prefixmap),
            ShapeSelector::Start => keyword("START", self.doc, self.keyword_color),
        }
    }
}

#[cfg(test)]
mod tests {}
