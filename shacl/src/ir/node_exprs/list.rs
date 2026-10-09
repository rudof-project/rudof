use crate::error::IRError;
use crate::ir::IRSchema;
use crate::validator::engine::Engine;
use rudof_rdf::rdf_core::FocusRDF;
use rudof_rdf::rdf_core::term::Object;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct List {
    list: Vec<Object>,
}

impl List {
    pub fn new(list: Vec<Object>) -> Self {
        Self { list }
    }

    pub fn list(&self) -> &Vec<Object> {
        &self.list
    }
}

impl List {
    pub fn evaluate<Rdf: FocusRDF>(
        &self,
        _: &mut Rdf,
        _: &HashMap<String, Rdf::Term>,
        _: &mut dyn Engine<Rdf>,
        _: &IRSchema,
    ) -> Result<Vec<Rdf::Term>, IRError> {
        Ok(self.list.iter().map(Rdf::object_as_term).collect())
    }
}

impl Display for List {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let items = self.list().iter().map(|o| o.to_string()).collect::<Vec<_>>().join(", ");

        write!(f, "List[{items}]")
    }
}
