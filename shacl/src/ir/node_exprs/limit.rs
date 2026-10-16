use crate::error::IRError;
use crate::ir::IRSchema;
use crate::ir::node_expr::IRNodeExpr;
use crate::validator::engine::Engine;
use rudof_rdf::rdf_core::NeighsRDF;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Clone)]
pub struct Limit {
    limit: usize,
    nodes: Box<IRNodeExpr>,
}

impl Limit {
    pub fn new(limit: usize, nodes: IRNodeExpr) -> Self {
        Self {
            limit,
            nodes: nodes.into(),
        }
    }

    pub fn limit(&self) -> usize {
        self.limit
    }

    pub fn nodes(&self) -> &IRNodeExpr {
        &self.nodes
    }
}

impl Limit {
    pub fn evaluate<Rdf: NeighsRDF + Debug>(
        &self,
        store: &Rdf,
        focus: &Rdf::Term,
        scope: &HashMap<String, Rdf::Term>,
        runner: &mut dyn Engine<Rdf>,
        ir: &IRSchema,
    ) -> Result<Vec<Rdf::Term>, IRError> {
        let out = self
            .nodes
            .evaluate(store, focus, scope, runner, ir)?
            .into_iter()
            .take(self.limit)
            .collect();
        Ok(out)
    }
}

impl Display for Limit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Limit({} of {})", self.limit, self.nodes())
    }
}
