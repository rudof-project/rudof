use crate::error::IRError;
use crate::ir::IRSchema;
use crate::validator::engine::Engine;
use rudof_iri::IriS;
use rudof_rdf::rdf_core::FocusRDF;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct Iri {
    iri: IriS,
}

impl Iri {
    pub fn new(iri: IriS) -> Self {
        Self { iri }
    }

    pub fn iri(&self) -> &IriS {
        &self.iri
    }
}

impl Iri {
    pub fn evaluate<Rdf: FocusRDF>(
        &self,
        _: &mut Rdf,
        _: &HashMap<String, Rdf::Term>,
        _: &mut dyn Engine<Rdf>,
        _: &IRSchema,
    ) -> Result<Vec<Rdf::Term>, IRError> {
        Ok(vec![Rdf::iris_as_term(&self.iri)])
    }
}

impl Display for Iri {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Iri({})", self.iri())
    }
}
