use crate::error::IRError;
use crate::ir::IRSchema;
use crate::ir::node_expr::IRNodeExpr;
use crate::validator::engine::Engine;
use rudof_rdf::rdf_core::FocusRDF;
use rudof_rdf::rdf_core::term::Object;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Clone)]
pub struct OrderBy {
    desc: Option<bool>,
    order_by: Box<IRNodeExpr>,
    nodes: Box<IRNodeExpr>,
}

impl OrderBy {
    pub fn new(order_by: IRNodeExpr, nodes: IRNodeExpr) -> Self {
        Self {
            order_by: order_by.into(),
            nodes: nodes.into(),
            desc: None,
        }
    }

    pub fn with_desc(mut self, desc: Option<bool>) -> Self {
        self.desc = desc;
        self
    }

    pub fn desc(&self) -> Option<bool> {
        self.desc
    }

    pub fn order_by(&self) -> &IRNodeExpr {
        &self.order_by
    }

    pub fn nodes(&self) -> &IRNodeExpr {
        &self.nodes
    }
}

impl OrderBy {
    pub fn evaluate<Rdf: FocusRDF + Debug>(
        &self,
        focus_graph: &mut Rdf,
        scope: &HashMap<String, Rdf::Term>,
        runner: &mut dyn Engine<Rdf>,
        ir: &IRSchema,
    ) -> Result<Vec<Rdf::Term>, IRError> {
        let desc = self.desc.unwrap_or(false);

        let nodes = self.nodes.evaluate(focus_graph, scope, runner, ir)?;
        let original_focus = focus_graph.get_focus().cloned();

        let mut keyed: Vec<(Rdf::Term, Option<Object>)> = Vec::with_capacity(nodes.len());
        for node in nodes {
            focus_graph.set_focus(&node);
            let key = self
                .order_by
                .evaluate(focus_graph, scope, runner, ir)?
                .into_iter()
                .next()
                .and_then(|k| k.try_into().ok());
            keyed.push((node, key));
        }

        if let Some(original_focus) = &original_focus {
            focus_graph.set_focus(original_focus);
        }

        keyed.sort_by(|(_, a), (_, b)| Self::compare_keys(a, b));

        if desc {
            keyed.reverse();
        }

        Ok(keyed.into_iter().map(|(node, _)| node).collect())
    }

    fn compare_keys(a: &Option<Object>, b: &Option<Object>) -> Ordering {
        match (a, b) {
            (None, None) => Ordering::Equal,
            (None, Some(_)) => Ordering::Less,
            (Some(_), None) => Ordering::Greater,
            (Some(a), Some(b)) => a.cmp(b),
        }
    }
}

impl Display for OrderBy {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let desc = if let Some(flag) = &self.desc {
            format!(" desc={flag}")
        } else {
            "".to_string()
        };
        write!(f, "OrderBy({} for {}{desc})", self.order_by(), self.nodes())
    }
}
