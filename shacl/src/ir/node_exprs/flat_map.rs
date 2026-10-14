use crate::error::IRError;
use crate::ir::IRSchema;
use crate::ir::node_expr::IRNodeExpr;
use crate::validator::engine::Engine;
use rudof_rdf::rdf_core::NeighsRDF;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Clone)]
pub struct FlatMap {
    flat_map: Box<IRNodeExpr>,
    nodes: Box<Option<IRNodeExpr>>,
}

impl FlatMap {
    pub fn new(flat_map: IRNodeExpr) -> Self {
        Self {
            flat_map: flat_map.into(),
            nodes: None.into(),
        }
    }

    pub fn with_nodes(mut self, nodes: Option<IRNodeExpr>) -> Self {
        self.nodes = nodes.into();
        self
    }

    pub fn flat_map(&self) -> &IRNodeExpr {
        &self.flat_map
    }

    pub fn nodes(&self) -> &Option<IRNodeExpr> {
        &self.nodes
    }
}

impl FlatMap {
    pub fn evaluate<Rdf: NeighsRDF + Debug>(
        &self,
        store: &Rdf,
        focus: &Rdf::Term,
        scope: &HashMap<String, Rdf::Term>,
        runner: &mut dyn Engine<Rdf>,
        ir: &IRSchema,
    ) -> Result<Vec<Rdf::Term>, IRError> {
        let nodes = match self.nodes.as_ref() {
            Some(nodes) => nodes.evaluate(store, focus, scope, runner, ir)?,
            None => vec![focus.clone()],
        };

        let mut out = Vec::new();
        for node in nodes {
            out.extend(self.flat_map.evaluate(store, &node, scope, runner, ir)?);
        }

        Ok(out)
    }
}

impl Display for FlatMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let nodes = self
            .nodes()
            .as_ref()
            .map(|n| n.to_string())
            .unwrap_or_else(|| "focusNode".to_string());
        write!(f, "FlatMap({} for {})", self.flat_map(), nodes)
    }
}
