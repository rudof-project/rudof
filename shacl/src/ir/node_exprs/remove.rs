use crate::error::IRError;
use crate::ir::IRSchema;
use crate::ir::node_expr::IRNodeExpr;
use crate::validator::engine::Engine;
use rudof_rdf::rdf_core::FocusRDF;
use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Clone)]
pub struct Remove {
    remove: Box<IRNodeExpr>,
    nodes: Box<IRNodeExpr>,
}

impl Remove {
    pub fn new(remove: IRNodeExpr, nodes: IRNodeExpr) -> Self {
        Self {
            remove: remove.into(),
            nodes: nodes.into(),
        }
    }

    pub fn remove(&self) -> &IRNodeExpr {
        &self.remove
    }

    pub fn nodes(&self) -> &IRNodeExpr {
        &self.nodes
    }
}

impl Remove {
    pub fn evaluate<Rdf: FocusRDF + Debug>(
        &self,
        focus_graph: &mut Rdf,
        scope: &HashMap<String, Rdf::Term>,
        runner: &mut dyn Engine<Rdf>,
        ir: &IRSchema,
    ) -> Result<Vec<Rdf::Term>, IRError> {
        let nodes = self.nodes.evaluate(focus_graph, scope, runner, ir)?;
        let remove = self
            .remove
            .evaluate(focus_graph, scope, runner, ir)?
            .into_iter()
            .collect::<HashSet<_>>();

        Ok(nodes.into_iter().filter(|t| !remove.contains(t)).collect())
    }
}

impl Display for Remove {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Remove({} from {})", self.remove(), self.nodes())
    }
}
