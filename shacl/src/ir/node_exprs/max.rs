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
pub struct Max {
    max: Box<IRNodeExpr>,
}

impl Max {
    pub fn new(max: IRNodeExpr) -> Self {
        Self { max: max.into() }
    }

    pub fn max(&self) -> &IRNodeExpr {
        &self.max
    }
}

impl Max {
    pub fn evaluate<Rdf: FocusRDF + Debug>(
        &self,
        focus_graph: &mut Rdf,
        scope: &HashMap<String, Rdf::Term>,
        runner: &mut dyn Engine<Rdf>,
        ir: &IRSchema,
    ) -> Result<Vec<Rdf::Term>, IRError> {
        let out = self
            .max
            .evaluate(focus_graph, scope, runner, ir)?
            .into_iter()
            .map(|term| {
                let key: Option<Object> = term.clone().try_into().ok();
                (key, term)
            })
            .max_by(|(a, _), (b, _)| Self::compare(a, b))
            .map(|(_, term)| term);

        Ok(match out {
            None => Vec::new(),
            Some(out) => vec![out],
        })
    }

    fn compare(a: &Option<Object>, b: &Option<Object>) -> Ordering {
        match (a, b) {
            (Some(a), Some(b)) => a.cmp(b),
            _ => Ordering::Equal,
        }
    }
}

impl Display for Max {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Max({})", self.max())
    }
}
