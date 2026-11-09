use crate::error::IRError;
use crate::ir::IRSchema;
use crate::validator::engine::Engine;
use rudof_rdf::rdf_core::NeighsRDF;
use rudof_rdf::rdf_core::term::Object;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Clone)]
pub struct TripleTerm {
    triple: Object,
}

impl TripleTerm {
    pub fn new(triple: Object) -> Self {
        Self { triple }
    }

    pub fn triple(&self) -> &Object {
        &self.triple
    }
}

impl TripleTerm {
    pub fn evaluate<Rdf: NeighsRDF + Debug>(
        &self,
        _: &Rdf,
        _: &Rdf::Term,
        _: &HashMap<String, Rdf::Term>,
        _: &mut dyn Engine<Rdf>,
        _: &IRSchema,
    ) -> Result<Vec<Rdf::Term>, IRError> {
        Ok(vec![Rdf::object_as_term(&self.triple)])
    }
}

impl Display for TripleTerm {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "TripleTerm({})", self.triple())
    }
}
