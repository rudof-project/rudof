use crate::error::IRError;
use crate::ir::node_expr::IRNodeExpr;
use crate::ir::{IRSchema, ShapeLabelIdx};
use crate::validator::engine::{Engine, Validate};
use crate::validator::nodes::FocusNodes;
use rudof_rdf::rdf_core::FocusRDF;
use rudof_rdf::rdf_core::term::Object;
use rudof_rdf::rdf_core::term::literal::ConcreteLiteral;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Clone)]
pub struct MatchAll {
    match_all: ShapeLabelIdx,
    nodes: Option<Box<IRNodeExpr>>,
}

impl MatchAll {
    pub fn new(match_all: ShapeLabelIdx) -> Self {
        Self { match_all, nodes: None }
    }

    pub fn with_nodes(mut self, nodes: Option<IRNodeExpr>) -> Self {
        self.nodes = nodes.map(Box::new);
        self
    }

    pub fn match_all(&self) -> &ShapeLabelIdx {
        &self.match_all
    }

    pub fn nodes(&self) -> Option<&IRNodeExpr> {
        self.nodes.as_deref()
    }
}

impl MatchAll {
    pub fn evaluate<Rdf: FocusRDF + Debug>(
        &self,
        focus_graph: &mut Rdf,
        scope: &HashMap<String, Rdf::Term>,
        runner: &mut dyn Engine<Rdf>,
        ir: &IRSchema,
    ) -> Result<Vec<Rdf::Term>, IRError> {
        let nodes = match &self.nodes {
            Some(nodes) => nodes.evaluate(focus_graph, scope, runner, ir)?,
            None => focus_graph.get_focus().cloned().map(|n| vec![n]).unwrap_or_default(),
        };

        let targets = FocusNodes::from_iter(nodes);
        let match_all = ir.get_shape_from_idx_e(&self.match_all)?;
        let outcome = match_all.validate(focus_graph, runner, Some(&targets), None, ir)?;

        Ok(vec![Rdf::object_as_term(&Object::Literal(
            ConcreteLiteral::BooleanLiteral(outcome.conforms()),
        ))])
    }
}

impl Display for MatchAll {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let nodes = if let Some(nodes) = &self.nodes {
            &format!("{nodes}")
        } else {
            "focusNode"
        };
        write!(f, "MatchAll({} in {})", self.match_all(), nodes)
    }
}
