use crate::error::IRError;
use crate::ir::IRSchema;
use crate::ir::node_expr::IRNodeExpr;
use crate::validator::engine::Engine;
use rudof_rdf::rdf_core::NeighsRDF;
use rudof_rdf::rdf_core::term::Object;
use rudof_rdf::rdf_core::term::literal::{ConcreteLiteral, NumericLiteral};
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Clone)]
pub struct Count {
    count: Box<IRNodeExpr>,
}

impl Count {
    pub fn new(count: IRNodeExpr) -> Self {
        Self { count: count.into() }
    }

    pub fn count(&self) -> &IRNodeExpr {
        &self.count
    }
}

impl Count {
    pub fn evaluate<Rdf: NeighsRDF + Debug>(
        &self,
        store: &Rdf,
        focus: &Rdf::Term,
        scope: &HashMap<String, Rdf::Term>,
        runner: &mut dyn Engine<Rdf>,
        ir: &IRSchema,
    ) -> Result<Vec<Rdf::Term>, IRError> {
        let l = self.count.evaluate(store, focus, scope, runner, ir)?.len();

        let out = Object::Literal(ConcreteLiteral::NumericLiteral(NumericLiteral::Integer(l as i128)));
        Ok(vec![Rdf::object_as_term(&out)])
    }
}

impl Display for Count {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Count({})", self.count())
    }
}
