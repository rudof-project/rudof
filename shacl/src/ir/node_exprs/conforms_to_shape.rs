use crate::ast::error::ASTError;
use crate::error::IRError;
use crate::ir::IRSchema;
use crate::ir::error::NodeExprEvalError;
use crate::ir::node_expr::IRNodeExpr;
use crate::validator::engine::{Engine, Validate};
use crate::validator::nodes::FocusNodes;
use rudof_rdf::rdf_core::NeighsRDF;
use rudof_rdf::rdf_core::term::Object;
use rudof_rdf::rdf_core::term::literal::ConcreteLiteral;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Clone)]
pub struct ConformsToShape {
    node: Box<IRNodeExpr>,
    shape: Box<IRNodeExpr>,
}

impl ConformsToShape {
    pub fn new(node: IRNodeExpr, shape: IRNodeExpr) -> Self {
        Self {
            node: node.into(),
            shape: shape.into(),
        }
    }

    pub fn node(&self) -> &IRNodeExpr {
        &self.node
    }

    pub fn shape(&self) -> &IRNodeExpr {
        &self.shape
    }
}

impl ConformsToShape {
    pub fn evaluate<Rdf: NeighsRDF + Debug>(
        &self,
        store: &Rdf,
        focus: &Rdf::Term,
        scope: &HashMap<String, Rdf::Term>,
        runner: &mut dyn Engine<Rdf>,
        ir: &IRSchema,
    ) -> Result<Vec<Rdf::Term>, IRError> {
        let Some(node) = self.single(self.node(), "node", store, focus, scope, runner, ir)? else {
            return Ok(Vec::new());
        };
        let Some(shape) = self.single(self.shape(), "shape", store, focus, scope, runner, ir)? else {
            return Ok(Vec::new());
        };

        let shape = Rdf::term_as_object(&shape)?;
        let idx = *ir.get_idx(&shape).ok_or_else(|| IRError::from(ASTError::from(shape)))?;
        let shape = ir.get_shape_from_idx_e(&idx)?;

        let target = FocusNodes::single(node);
        let outcome = shape.validate(store, runner, Some(&target), None, ir)?;

        Ok(vec![Rdf::object_as_term(&Object::Literal(
            ConcreteLiteral::BooleanLiteral(outcome.conforms()),
        ))])
    }

    /// The only output node of `expr`, which is an evaluation failure if it has more than one.
    #[allow(clippy::too_many_arguments)]
    fn single<Rdf: NeighsRDF + Debug>(
        &self,
        expr: &IRNodeExpr,
        arg: &str,
        store: &Rdf,
        focus: &Rdf::Term,
        scope: &HashMap<String, Rdf::Term>,
        runner: &mut dyn Engine<Rdf>,
        ir: &IRSchema,
    ) -> Result<Option<Rdf::Term>, IRError> {
        let nodes = expr.evaluate(store, focus, scope, runner, ir)?;

        match nodes.len() {
            0 => Ok(None),
            1 => Ok(nodes.into_iter().next()),
            found => Err(NodeExprEvalError::MultipleArgumentNodes {
                expr: self.to_string(),
                arg: arg.to_string(),
                found,
            }
            .into()),
        }
    }
}

impl Display for ConformsToShape {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ConformsToShape({}, {})", self.node(), self.shape())
    }
}
