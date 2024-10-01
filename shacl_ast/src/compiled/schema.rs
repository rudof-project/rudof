use std::collections::HashMap;

use prefixmap::PrefixMap;
use srdf::SRDFBasic;

use super::shape::Shape;

pub struct Schema<S: SRDFBasic> {
    // imports: Vec<IriS>,
    // entailments: Vec<IriS>,
    shapes: HashMap<S::Term, Shape<S>>,
    prefixmap: PrefixMap,
    base: S::IRI,
}

impl<S: SRDFBasic> Schema<S> {
    pub fn new(
        shapes: HashMap<S::Term, Shape<S>>,
        prefixmap: PrefixMap,
        base: S::IRI,
    ) -> Schema<S> {
        Schema {
            shapes,
            prefixmap,
            base,
        }
    }

    pub fn prefix_map(&self) -> PrefixMap {
        self.prefixmap.clone()
    }

    pub fn base(&self) -> &S::IRI {
        &self.base
    }

    pub fn iter(&self) -> impl Iterator<Item = (&S::Term, &Shape<S>)> {
        self.shapes.iter()
    }

    pub fn get_shape(&self, sref: &S::Term) -> Option<&Shape<S>> {
        self.shapes.get(sref)
    }
}
