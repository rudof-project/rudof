use crate::error::IRError;
use crate::ir::node_expr::IRNodeExpr;
use crate::ir::{IRSchema, ShapeLabelIdx};
use crate::validator::engine::{Engine, Validate};
use crate::validator::nodes::FocusNodes;
use rudof_rdf::rdf_core::FocusRDF;
use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Clone)]
pub struct Filter {
    filter_shape: ShapeLabelIdx,
    nodes: Box<IRNodeExpr>,
}

impl Filter {
    pub fn new(filter_shape: ShapeLabelIdx, nodes: IRNodeExpr) -> Self {
        Self {
            filter_shape,
            nodes: nodes.into(),
        }
    }

    pub fn filter_shape(&self) -> &ShapeLabelIdx {
        &self.filter_shape
    }

    pub fn nodes(&self) -> &IRNodeExpr {
        &self.nodes
    }
}

impl Filter {
    pub fn evaluate<Rdf: FocusRDF + Debug>(
        &self,
        focus_graph: &mut Rdf,
        scope: &HashMap<String, Rdf::Term>,
        runner: &mut dyn Engine<Rdf>,
        ir: &IRSchema,
    ) -> Result<Vec<Rdf::Term>, IRError> {
        let nodes = self.nodes.evaluate(focus_graph, scope, runner, ir)?;

        let targets = FocusNodes::from_iter(nodes.iter().cloned());
        let filter_shape = ir.get_shape_from_idx_e(&self.filter_shape)?;
        let outcome = filter_shape.validate(focus_graph, runner, Some(&targets), None, ir)?;

        let non_conforming = outcome
            .violations()
            .iter()
            .map(|v| Rdf::Term::from(v.focus_node().clone()))
            .collect::<HashSet<_>>();

        Ok(nodes.into_iter().filter(|n| !non_conforming.contains(n)).collect())
    }
}

impl Display for Filter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Filter({} for {})", self.filter_shape(), self.nodes())
    }
}
