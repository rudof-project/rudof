use std::collections::HashMap;

use api::model::rdf::NamedNode;
use api::model::rdf::Predicate;
use api::model::rdf::Rdf;
use api::model::Subject;
use prefixmap::PrefixMap;

use crate::Schema;

use super::compiled_shacl_error::CompiledShaclError;
use super::shape::CompiledShape;

#[derive(Debug)]
pub struct CompiledSchema<R: Rdf> {
    // imports: Vec<IriS>,
    // entailments: Vec<IriS>,
    shapes: HashMap<Subject<R>, CompiledShape<R>>,
    prefixmap: PrefixMap,
    base: Option<NamedNode<R>>,
}

impl<S: SRDFBasic> CompiledSchema<S> {
    pub fn new(
        shapes: HashMap<S::Term, CompiledShape<S>>,
        prefixmap: PrefixMap,
        base: Option<S::IRI>,
    ) -> CompiledSchema<S> {
        CompiledSchema {
            shapes,
            prefixmap,
            base,
        }
    }

    pub fn prefix_map(&self) -> PrefixMap {
        self.prefixmap.clone()
    }

    pub fn base(&self) -> &Option<S::IRI> {
        &self.base
    }

    pub fn iter(&self) -> impl Iterator<Item = (&S::Term, &CompiledShape<S>)> {
        self.shapes.iter()
    }

    pub fn get_shape(&self, sref: &S::Term) -> Option<&CompiledShape<S>> {
        self.shapes.get(sref)
    }
}

impl<S: SRDFBasic> TryFrom<Schema> for CompiledSchema<S> {
    type Error = CompiledShaclError;

    fn try_from(schema: Schema) -> Result<Self, Self::Error> {
        let mut shapes = HashMap::default();

        for (rdf_node, shape) in schema.iter() {
            let term = S::object_as_term(rdf_node);
            let shape = CompiledShape::compile(shape.to_owned(), &schema)?;
            shapes.insert(term, shape);
        }

        let prefixmap = schema.prefix_map();

        let base = schema.base().map(|base| S::iri_s2iri(&base));

        Ok(CompiledSchema::new(shapes, prefixmap, base))
    }
}
