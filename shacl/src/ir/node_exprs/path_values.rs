use crate::error::IRError;
use crate::ir::IRSchema;
use crate::ir::node_expr::IRNodeExpr;
use crate::ir::node_expr_eval_error::NodeExprEvalError;
use crate::validator::engine::Engine;
use rudof_rdf::rdf_core::{FocusRDF, SHACLPath};
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Clone)]
pub struct PathValues {
    path: SHACLPath,
    focus_nodes: Option<Box<IRNodeExpr>>,
}

impl PathValues {
    pub fn new(path: SHACLPath) -> Self {
        Self {
            path,
            focus_nodes: None,
        }
    }

    pub fn with_focus_node(mut self, fnode: Option<IRNodeExpr>) -> Self {
        self.focus_nodes = fnode.map(Box::new);
        self
    }

    pub fn path(&self) -> &SHACLPath {
        &self.path
    }

    pub fn focus_nodes(&self) -> Option<&IRNodeExpr> {
        self.focus_nodes.as_deref()
    }
}

impl PathValues {
    pub fn evaluate<Rdf: FocusRDF + Debug>(
        &self,
        focus_graph: &mut Rdf,
        scope: &HashMap<String, Rdf::Term>,
        runner: &mut dyn Engine<Rdf>,
        ir: &IRSchema,
    ) -> Result<Vec<Rdf::Term>, IRError> {
        let nodes = match &self.focus_nodes {
            None => focus_graph
                .get_focus()
                .cloned()
                .map(|fnode| vec![fnode])
                .unwrap_or_default(),
            Some(fnode) => fnode.evaluate(focus_graph, scope, runner, ir)?,
        };

        let focus_node = match nodes.len() {
            0 => return Ok(Vec::new()),
            1 => nodes.into_iter().next().unwrap(),
            n => return Err(NodeExprEvalError::MultipleFocusNodes(self.to_string(), n).into()),
        };

        let values = focus_graph.objects_for_shacl_path(&focus_node, &self.path)?;

        Ok(values.into_iter().collect())
    }
}

impl Display for PathValues {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let fnode = if let Some(fnode) = &self.focus_nodes {
            &format!(" for {fnode}")
        } else {
            ""
        };
        write!(f, "PathValues({}{fnode})", self.path())
    }
}
