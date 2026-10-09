use crate::error::IRError;
use crate::ir::IRSchema;
use crate::ir::node_expr::IRNodeExpr;
use crate::validator::engine::Engine;
use rudof_rdf::rdf_core::FocusRDF;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Clone)]
pub struct Concat {
    concat: Vec<IRNodeExpr>,
}

impl Concat {
    pub fn new(concat: Vec<IRNodeExpr>) -> Self {
        Self { concat }
    }

    pub fn concat(&self) -> &Vec<IRNodeExpr> {
        &self.concat
    }
}

impl Concat {
    pub fn evaluate<Rdf: FocusRDF + Debug>(
        &self,
        focus_graph: &mut Rdf,
        scope: &HashMap<String, Rdf::Term>,
        runner: &mut dyn Engine<Rdf>,
        ir: &IRSchema,
    ) -> Result<Vec<Rdf::Term>, IRError> {
        let mut out = Vec::new();
        for expr in &self.concat {
            out.extend(expr.evaluate(focus_graph, scope, runner, ir)?);
        }
        Ok(out)
    }
}

impl Display for Concat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let items = self
            .concat()
            .iter()
            .map(|ne| ne.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "Concat[{items}]")
    }
}
