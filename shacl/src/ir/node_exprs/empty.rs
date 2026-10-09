use crate::error::IRError;
use crate::ir::IRSchema;
use crate::validator::engine::Engine;
use rudof_rdf::rdf_core::FocusRDF;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Clone)]
pub struct Empty;

impl Empty {
    pub fn new() -> Self {
        Self
    }
}

impl Empty {
    pub fn evaluate<Rdf: FocusRDF + Debug>(
        &self,
        _: &mut Rdf,
        _: &HashMap<String, Rdf::Term>,
        _: &mut dyn Engine<Rdf>,
        _: &IRSchema,
    ) -> Result<Vec<Rdf::Term>, IRError> {
        Ok(Vec::new())
    }
}

impl Display for Empty {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Empty")
    }
}
