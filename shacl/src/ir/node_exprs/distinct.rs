use crate::error::IRError;
use crate::ir::IRSchema;
use crate::ir::node_expr::IRNodeExpr;
use crate::validator::engine::Engine;
use rudof_rdf::rdf_core::FocusRDF;
use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Clone)]
pub struct Distinct {
    distinct: Box<IRNodeExpr>,
}

impl Distinct {
    pub fn new(distinct: IRNodeExpr) -> Self {
        Self {
            distinct: distinct.into(),
        }
    }

    pub fn distinct(&self) -> &IRNodeExpr {
        &self.distinct
    }
}

impl Distinct {
    pub fn evaluate<Rdf: FocusRDF + Debug>(
        &self,
        focus_graph: &mut Rdf,
        scope: &HashMap<String, Rdf::Term>,
        runner: &mut dyn Engine<Rdf>,
        ir: &IRSchema,
    ) -> Result<Vec<Rdf::Term>, IRError> {
        let mut seen = HashSet::new();
        let out = self
            .distinct
            .evaluate(focus_graph, scope, runner, ir)?
            .into_iter()
            .filter(|term| seen.insert(term.clone()))
            .collect();

        Ok(out)
    }
}

impl Display for Distinct {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Distinct({})", self.distinct())
    }
}
