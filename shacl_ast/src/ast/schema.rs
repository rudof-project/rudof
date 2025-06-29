use std::{collections::HashMap, fmt::Display};

use crate::shape::Shape;
use iri_s::IriS;
use prefixmap::PrefixMap;
use srdf::{RDFNode, Rdf};

#[derive(Debug, Clone, Default)]
pub struct Schema<RDF: Rdf>
where
    RDF::Term: Clone,
{
    // imports: Vec<IriS>,
    // entailments: Vec<IriS>,
    shapes: HashMap<RDFNode, Shape<RDF>>,
    prefixmap: PrefixMap,
    base: Option<IriS>,
}

impl<RDF: Rdf> Schema<RDF> {
    pub fn new() -> Schema<RDF> {
        Schema {
            shapes: HashMap::new(),
            prefixmap: PrefixMap::new(),
            base: None,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.shapes.is_empty()
    }

    pub fn with_prefixmap(mut self, prefixmap: PrefixMap) -> Self {
        self.prefixmap = prefixmap;
        self
    }

    pub fn with_shapes(mut self, shapes: HashMap<RDFNode, Shape<RDF>>) -> Self {
        self.shapes = shapes;
        self
    }

    pub fn prefix_map(&self) -> PrefixMap {
        self.prefixmap.clone()
    }

    pub fn base(&self) -> Option<IriS> {
        self.base.clone()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&RDFNode, &Shape<RDF>)> {
        self.shapes.iter()
    }

    pub fn get_shape(&self, sref: &RDFNode) -> Option<&Shape<RDF>> {
        self.shapes.get(sref)
    }
}

impl<RDF: Rdf> Display for Schema<RDF> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (id, shape) in self.shapes.iter() {
            writeln!(f, "{id} -> {shape}")?;
        }
        Ok(())
    }
}
