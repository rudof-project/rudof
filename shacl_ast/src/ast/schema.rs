use std::collections::HashMap;

use prefixmap::PrefixMap;
use srdf::model::rdf::Rdf;
use srdf::model::rdf::TObject;
use srdf::model::rdf::TPredicate;

use super::shape::Shape;

#[derive(Default, Debug)]
pub struct Schema<R: Rdf> {
    // imports: Vec<IriS>,
    // entailments: Vec<IriS>,
    shapes: HashMap<TObject<R>, Shape<R>>,
    prefixmap: PrefixMap,
    base: Option<TPredicate<R>>,
}

impl<R: Rdf> Schema<R> {
    pub fn with_shapes(mut self, shapes: HashMap<TObject<R>, Shape<R>>) -> Self {
        self.shapes = shapes;
        self
    }

    pub fn with_prefixmap(mut self, prefixmap: PrefixMap) -> Self {
        self.prefixmap = prefixmap;
        self
    }

    pub fn with_base(mut self, base: Option<TPredicate<R>>) -> Self {
        self.base = base;
        self
    }

    pub fn prefix_map(&self) -> PrefixMap {
        self.prefixmap.clone()
    }

    pub fn base(&self) -> &Option<TPredicate<R>> {
        &self.base
    }

    pub fn iter(&self) -> impl Iterator<Item = (&TObject<R>, &Shape<R>)> {
        self.shapes.iter()
    }

    pub fn get_shape(&self, sref: &TObject<R>) -> Option<&Shape<R>> {
        self.shapes.get(sref)
    }
}
