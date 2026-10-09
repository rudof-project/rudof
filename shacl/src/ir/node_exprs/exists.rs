use crate::error::IRError;
use crate::ir::IRSchema;
use crate::ir::node_expr::IRNodeExpr;
use crate::validator::engine::Engine;
use rudof_rdf::rdf_core::FocusRDF;
use rudof_rdf::rdf_core::term::Object;
use rudof_rdf::rdf_core::term::literal::ConcreteLiteral;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Clone)]
pub struct Exists {
    exists: Box<IRNodeExpr>,
}

impl Exists {
    pub fn new(exists: IRNodeExpr) -> Self {
        Self { exists: exists.into() }
    }

    pub fn exists(&self) -> &IRNodeExpr {
        &self.exists
    }
}

impl Exists {
    pub fn evaluate<Rdf: FocusRDF + Debug>(
        &self,
        focus_graph: &mut Rdf,
        scope: &HashMap<String, Rdf::Term>,
        runner: &mut dyn Engine<Rdf>,
        ir: &IRSchema,
    ) -> Result<Vec<Rdf::Term>, IRError> {
        let out = !self.exists.evaluate(focus_graph, scope, runner, ir)?.is_empty();

        Ok(vec![Rdf::object_as_term(&Object::Literal(
            ConcreteLiteral::BooleanLiteral(out),
        ))])
    }
}

impl Display for Exists {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Exists({})", self.exists())
    }
}
