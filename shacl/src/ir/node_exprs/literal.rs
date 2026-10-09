use crate::error::IRError;
use crate::ir::IRSchema;
use crate::validator::engine::Engine;
use rudof_rdf::rdf_core::FocusRDF;
use rudof_rdf::rdf_core::term::Object;
use rudof_rdf::rdf_core::term::literal::ConcreteLiteral;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct Literal {
    lit: ConcreteLiteral,
}

impl Literal {
    pub fn new(lit: ConcreteLiteral) -> Self {
        Self { lit }
    }

    pub fn literal(&self) -> &ConcreteLiteral {
        &self.lit
    }
}

impl Literal {
    pub fn evaluate<Rdf: FocusRDF>(
        &self,
        _: &mut Rdf,
        _: &HashMap<String, Rdf::Term>,
        _: &mut dyn Engine<Rdf>,
        _: &IRSchema,
    ) -> Result<Vec<Rdf::Term>, IRError> {
        Ok(vec![Rdf::object_as_term(&Object::Literal(self.lit.clone()))])
    }
}

impl Display for Literal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Literal({})", self.literal())
    }
}
