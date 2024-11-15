use std::collections::HashMap;

use prefixmap::PrefixMap;
use srdf::model::rdf::Predicate;
use srdf::model::rdf::Rdf;
use srdf::model::rdf::Subject;

use super::shape::Shape;

#[derive(Default, Debug)]
pub struct Schema<R: Rdf> {
    // imports: Vec<IriS>,
    // entailments: Vec<IriS>,
    shapes: HashMap<Subject<R>, Shape<R>>,
    prefixmap: PrefixMap,
    base: Option<Predicate<R>>,
}

impl<R: Rdf> Schema<R> {
    pub fn new(
        shapes: HashMap<Subject<R>, Shape<R>>,
        prefixmap: PrefixMap,
        base: Option<Predicate<R>>,
    ) -> Schema<R> {
        Schema {
            shapes,
            prefixmap,
            base,
        }
    }

    pub fn prefix_map(&self) -> PrefixMap {
        self.prefixmap.clone()
    }

    pub fn base(&self) -> &Option<Predicate<R>> {
        &self.base
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Subject<R>, &Shape<R>)> {
        self.shapes.iter()
    }

    pub fn get_shape(&self, sref: &Subject<R>) -> Option<&Shape<R>> {
        self.shapes.get(sref)
    }
}
