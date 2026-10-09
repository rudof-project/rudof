use crate::error::IRError;
use crate::ir::IRSchema;
use crate::ir::node_expr::IRNodeExpr;
use crate::validator::engine::Engine;
use rudof_rdf::rdf_core::FocusRDF;
use rudof_rdf::rdf_core::term::literal::Literal;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Clone)]
pub struct IfExpression {
    if_condition: Box<IRNodeExpr>,
    then: Option<Box<IRNodeExpr>>,
    else_expr: Option<Box<IRNodeExpr>>,
}

impl IfExpression {
    pub fn new(if_condition: IRNodeExpr) -> Self {
        Self {
            if_condition: if_condition.into(),
            then: None,
            else_expr: None,
        }
    }

    pub fn if_condition(&self) -> &IRNodeExpr {
        &self.if_condition
    }

    pub fn with_then(mut self, then: Option<IRNodeExpr>) -> Self {
        self.then = then.map(Box::new);
        self
    }

    pub fn with_else(mut self, else_expr: Option<IRNodeExpr>) -> Self {
        self.else_expr = else_expr.map(Box::new);
        self
    }

    pub fn then(&self) -> Option<&IRNodeExpr> {
        self.then.as_deref()
    }

    pub fn else_expression(&self) -> Option<&IRNodeExpr> {
        self.else_expr.as_deref()
    }
}

impl IfExpression {
    pub fn evaluate<Rdf: FocusRDF + Debug>(
        &self,
        focus_graph: &mut Rdf,
        scope: &HashMap<String, Rdf::Term>,
        runner: &mut dyn Engine<Rdf>,
        ir: &IRSchema,
    ) -> Result<Vec<Rdf::Term>, IRError> {
        if self.then.is_none() && self.else_expr.is_none() {
            return Err(IRError::IfNodeExprBodyErr(self.clone()));
        }

        let if_result = self.if_condition.evaluate(focus_graph, scope, runner, ir)?;

        let is_true = if_result.len() == 1
            && Rdf::term_as_literal(&if_result[0])
                .ok()
                .and_then(|lit| lit.to_bool())
                .unwrap_or(false);

        if is_true {
            match &self.then {
                Some(expr) => expr.evaluate(focus_graph, scope, runner, ir),
                None => Ok(Vec::new()),
            }
        } else {
            match &self.else_expr {
                Some(expr) => expr.evaluate(focus_graph, scope, runner, ir),
                None => Ok(Vec::new()),
            }
        }
    }
}

impl Display for IfExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let then = self.then().map(|t| t.to_string()).unwrap_or_default();
        let else_expr = self.else_expression().map(|e| e.to_string()).unwrap_or_default();
        write!(f, "If({}, then {}, else {})", self.if_condition(), then, else_expr)
    }
}
