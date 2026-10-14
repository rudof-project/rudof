use crate::error::IRError;
use crate::ir::IRSchema;
use crate::ir::error::NodeExprEvalError;
use crate::validator::engine::Engine;
use rudof_rdf::rdf_core::NeighsRDF;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Clone)]
pub struct Var {
    var: String,
}

impl Var {
    pub fn new(var: String) -> Self {
        Self { var }
    }

    pub fn var(&self) -> &String {
        &self.var
    }
}

impl Var {
    pub fn evaluate<Rdf: NeighsRDF + Debug>(
        &self,
        _: &Rdf,
        focus: &Rdf::Term,
        scope: &HashMap<String, Rdf::Term>,
        _: &mut dyn Engine<Rdf>,
        _: &IRSchema,
    ) -> Result<Vec<Rdf::Term>, IRError> {
        if self.var.len() == 0 {
            return Err(NodeExprEvalError::EmptyVar(self.clone()).into());
        }
        if self.var == "focusNode" {
            return Ok(vec![focus.clone()]);
        }

        Ok(match scope.get(&self.var) {
            None => Vec::new(),
            Some(item) => vec![item.clone()],
        })
    }
}

impl Display for Var {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Var({})", self.var())
    }
}
