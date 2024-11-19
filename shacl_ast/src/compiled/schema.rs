use std::collections::HashMap;
use std::hash::Hash;

use prefixmap::PrefixMap;
use srdf::model::rdf::Rdf;
use srdf::model::rdf::TPredicateRef;
use srdf::model::rdf::TSubjectRef;

use crate::Schema;

use super::compiled_shacl_error::CompiledShaclError;
use super::shape::CompiledShape;

#[derive(Debug)]
pub struct CompiledSchema<R: Rdf> {
    // imports: Vec<IriS>,
    // entailments: Vec<IriS>,
    shapes: HashMap<TSubjectRef<R>, CompiledShape<R>>,
    prefixmap: PrefixMap,
    base: Option<TPredicateRef<R>>,
}

impl<R: Rdf> CompiledSchema<R> {
    pub fn new(
        shapes: HashMap<TSubjectRef<R>, CompiledShape<R>>,
        prefixmap: PrefixMap,
        base: Option<TPredicateRef<R>>,
    ) -> CompiledSchema<R> {
        CompiledSchema {
            shapes,
            prefixmap,
            base,
        }
    }

    pub fn prefix_map(&self) -> PrefixMap {
        self.prefixmap.clone()
    }

    pub fn base(&self) -> &Option<TPredicateRef<R>> {
        &self.base
    }

    pub fn iter(&self) -> impl Iterator<Item = (&TSubjectRef<R>, &CompiledShape<R>)> {
        self.shapes.iter()
    }

    pub fn get_shape(&self, sref: &TSubjectRef<R>) -> Option<&CompiledShape<R>> {
        self.shapes.get(sref)
    }
}

impl<R: Rdf + Eq + Clone + Hash> TryFrom<Schema<R>> for CompiledSchema<R> {
    type Error = CompiledShaclError;

    fn try_from(schema: Schema<R>) -> Result<Self, Self::Error> {
        let mut shapes: HashMap<TSubjectRef<R>, CompiledShape<R>> = HashMap::default();

        for (term, shape) in schema.iter() {
            let term = match term.clone().try_into() {
                Ok(term) => term,
                Err(_) => return Err(CompiledShaclError::ShapeIdIsNotValid),
            };
            let shape = CompiledShape::compile(shape.clone(), &schema)?;
            shapes.insert(term, shape);
        }

        Ok(CompiledSchema::new(
            shapes,
            schema.prefix_map(),
            schema.base().clone(),
        ))
    }
}
