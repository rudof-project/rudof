use crate::error::IRError;
use crate::ir::IRSchema;
use crate::ir::node_expr::IRNodeExpr;
use crate::validator::engine::Engine;
use itertools::Itertools;
use rudof_rdf::rdf_core::FocusRDF;
use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Clone)]
pub struct Intersection {
    intersection: Vec<IRNodeExpr>,
}

impl Intersection {
    pub fn new(intersection: Vec<IRNodeExpr>) -> Self {
        Self { intersection }
    }

    pub fn intersection(&self) -> &Vec<IRNodeExpr> {
        &self.intersection
    }
}

impl Intersection {
    pub fn evaluate<Rdf: FocusRDF + Debug>(
        &self,
        focus_graph: &mut Rdf,
        scope: &HashMap<String, Rdf::Term>,
        runner: &mut dyn Engine<Rdf>,
        ir: &IRSchema,
    ) -> Result<Vec<Rdf::Term>, IRError> {
        let mut exprs = self.intersection.iter();

        let Some(first) = exprs.next() else {
            return Ok(Vec::new());
        };

        let first = first.evaluate(focus_graph, scope, runner, ir)?;
        let rest = exprs
            .map(|expr| Ok(expr.evaluate(focus_graph, scope, runner, ir)?.into_iter().collect()))
            .collect::<Result<Vec<HashSet<Rdf::Term>>, IRError>>()?;

        let out = first
            .into_iter()
            .filter(|term| rest.iter().all(|values| values.contains(term)))
            .unique()
            .collect();

        Ok(out)
    }
}

impl Display for Intersection {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let items = self
            .intersection()
            .iter()
            .map(|ne| ne.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "Intersection[{items}]")
    }
}
