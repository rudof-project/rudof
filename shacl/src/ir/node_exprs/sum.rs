use crate::error::IRError;
use crate::ir::IRSchema;
use crate::ir::node_expr::IRNodeExpr;
use crate::ir::node_expr_eval_error::NodeExprEvalError;
use crate::validator::engine::Engine;
use rudof_rdf::rdf_core::FocusRDF;
use rudof_rdf::rdf_core::term::Object;
use rudof_rdf::rdf_core::term::literal::{ConcreteLiteral, NumericLiteral};
use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Clone)]
pub struct Sum {
    sum: Box<IRNodeExpr>,
}

impl Sum {
    pub fn new(sum: IRNodeExpr) -> Self {
        Self { sum: sum.into() }
    }

    pub fn sum(&self) -> &IRNodeExpr {
        &self.sum
    }
}

impl Sum {
    pub fn evaluate<Rdf: FocusRDF + Debug>(
        &self,
        focus_graph: &mut Rdf,
        scope: &HashMap<String, Rdf::Term>,
        runner: &mut dyn Engine<Rdf>,
        ir: &IRSchema,
    ) -> Result<Vec<Rdf::Term>, IRError> {
        let terms = self.sum.evaluate(focus_graph, scope, runner, ir)?;

        let mut rank = 0u8;
        let mut total = Decimal::ZERO;

        for term in &terms {
            let object: Object = term
                .clone()
                .try_into()
                .map_err(|_| NodeExprEvalError::NotNumeric(term.to_string()))?;
            let num = object
                .numeric_value()
                .ok_or_else(|| NodeExprEvalError::NotNumeric(term.to_string()))?;
            let decimal = num
                .to_decimal()
                .ok_or_else(|| NodeExprEvalError::NotNumeric(term.to_string()))?;

            rank = rank.max(Self::promote_rank(&num));
            total += decimal;
        }

        let result = match rank {
            3 => NumericLiteral::Double(total.to_f64().unwrap_or_default()),
            2 => NumericLiteral::Float(total.to_f32().unwrap_or_default()),
            1 => NumericLiteral::Decimal(total),
            _ => NumericLiteral::Integer(total.to_i128().unwrap_or_default()),
        };

        Ok(vec![Rdf::object_as_term(&Object::Literal(
            ConcreteLiteral::NumericLiteral(result),
        ))])
    }

    fn promote_rank(num: &NumericLiteral) -> u8 {
        match num {
            NumericLiteral::Double(_) => 3,
            NumericLiteral::Float(_) => 2,
            NumericLiteral::Decimal(_) => 1,
            _ => 0,
        }
    }
}

impl Display for Sum {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Sum({})", self.sum())
    }
}
