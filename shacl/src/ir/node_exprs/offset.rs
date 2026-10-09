use crate::error::IRError;
use crate::ir::IRSchema;
use crate::ir::node_expr::IRNodeExpr;
use crate::validator::engine::Engine;
use rudof_rdf::rdf_core::FocusRDF;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Clone)]
pub struct Offset {
    offset: usize,
    nodes: Box<IRNodeExpr>,
}

impl Offset {
    pub fn new(offset: usize, nodes: IRNodeExpr) -> Self {
        Self {
            offset,
            nodes: nodes.into(),
        }
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn nodes(&self) -> &IRNodeExpr {
        &self.nodes
    }
}

impl Offset {
    pub fn evaluate<Rdf: FocusRDF + Debug>(
        &self,
        focus_graph: &mut Rdf,
        scope: &HashMap<String, Rdf::Term>,
        runner: &mut dyn Engine<Rdf>,
        ir: &IRSchema,
    ) -> Result<Vec<Rdf::Term>, IRError> {
        Ok(self
            .nodes
            .evaluate(focus_graph, scope, runner, ir)?
            .into_iter()
            .skip(self.offset)
            .collect())
    }
}

impl Display for Offset {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Offset({} of {})", self.offset, self.nodes())
    }
}
