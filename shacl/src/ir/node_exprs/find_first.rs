use crate::error::IRError;
use crate::ir::node_expr::IRNodeExpr;
use crate::ir::{IRSchema, ShapeLabelIdx};
use crate::validator::engine::{Engine, Validate};
use crate::validator::nodes::FocusNodes;
use rudof_rdf::rdf_core::FocusRDF;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Clone)]
pub struct FindFirst {
    find_first: ShapeLabelIdx,
    nodes: Box<IRNodeExpr>,
}

impl FindFirst {
    pub fn new(find_first: ShapeLabelIdx, nodes: IRNodeExpr) -> Self {
        Self {
            find_first,
            nodes: nodes.into(),
        }
    }

    pub fn find_first(&self) -> &ShapeLabelIdx {
        &self.find_first
    }

    pub fn nodes(&self) -> &IRNodeExpr {
        &self.nodes
    }
}

impl FindFirst {
    pub fn evaluate<Rdf: FocusRDF + Debug>(
        &self,
        focus_graph: &mut Rdf,
        scope: &HashMap<String, Rdf::Term>,
        runner: &mut dyn Engine<Rdf>,
        ir: &IRSchema,
    ) -> Result<Vec<Rdf::Term>, IRError> {
        let nodes = self.nodes.evaluate(focus_graph, scope, runner, ir)?;
        let find_first = ir.get_shape_from_idx_e(&self.find_first)?;

        for node in nodes {
            let target = FocusNodes::single(node.clone());
            let outcome = find_first.validate(focus_graph, runner, Some(&target), None, ir)?;

            if outcome.conforms() {
                return Ok(vec![node]);
            }
        }

        Ok(Vec::new())
    }
}

impl Display for FindFirst {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "FindFirst({} for {})", self.find_first(), self.nodes())
    }
}
