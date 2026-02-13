use super::{
    annotation::Annotation,
    dependency_graph::{DependencyGraph, PosNeg},
    sem_act::SemAct,
};
use crate::{Expr, Pred, ShapeLabelIdx};
use itertools::Itertools;
use std::{collections::HashMap, fmt::Display};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Shape {
    closed: bool,
    extra: Vec<Pred>,
    expr: Expr,
    sem_acts: Vec<SemAct>,
    annotations: Vec<Annotation>,
    preds: Vec<Pred>,
    extends: Vec<ShapeLabelIdx>,
    // references: HashMap<Pred, Vec<ShapeLabelIdx>>,
}

impl Shape {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        closed: bool,
        extra: Vec<Pred>,
        expr: Expr,
        sem_acts: Vec<SemAct>,
        annotations: Vec<Annotation>,
        preds: Vec<Pred>,
        extends: Vec<ShapeLabelIdx>,
        // references: HashMap<Pred, Vec<ShapeLabelIdx>>,
    ) -> Self {
        Shape {
            closed,
            extra,
            expr,
            sem_acts,
            annotations,
            preds,
            extends,
        }
    }

    pub fn extends(&self) -> &Vec<ShapeLabelIdx> {
        &self.extends
    }

    pub fn preds(&self) -> Vec<Pred> {
        self.preds.clone()
    }

    pub fn extra(&self) -> &Vec<Pred> {
        &self.extra
    }

    pub fn references(&self) -> HashMap<Pred, Vec<ShapeLabelIdx>> {
        self.get_value_expr_references()
    }

    /// Regular Bag expression that corresponds to the triple expression of the shape
    /// Replace by expr which easier to remember?
    pub fn triple_expr(&self) -> &Expr {
        &self.expr
    }

    pub fn is_closed(&self) -> bool {
        self.closed
    }

    /*pub fn show_qualified(&self) -> String {
        let closed = if self.closed { "CLOSED" } else { "" };
        let extra = if self.extra.is_empty() {
            "".to_string()
        } else {
            format!("EXTRA [{}]", self.extra.iter().join(" "))
        };
        format!("Shape {closed}{extra}")
    }*/

    pub fn add_edges(&self, source: ShapeLabelIdx, graph: &mut DependencyGraph, pos_neg: PosNeg) {
        for (_component, _pred, cond) in self.expr.components() {
            match cond {
                rbe::MatchCond::Single(_single_cond) => {},
                rbe::MatchCond::And(_match_conds) => {},
                rbe::MatchCond::Ref(r) => graph.add_edge(source, r, pos_neg),
            }
        }
    }

    fn get_value_expr_references(&self) -> HashMap<Pred, Vec<ShapeLabelIdx>> {
        let mut result: HashMap<Pred, Vec<ShapeLabelIdx>> = HashMap::new();
        for (_component, pred, cond) in self.expr.components() {
            match cond {
                rbe::MatchCond::Single(_single_cond) => {},
                rbe::MatchCond::And(_match_conds) => {},
                rbe::MatchCond::Ref(r) => {
                    result.entry(pred.clone()).or_default().push(r);
                },
            }
        }
        result
    }
}

impl Display for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let extends = if self.extends.is_empty() {
            "".to_string()
        } else {
            format!("EXTENDS [{}]", self.extends.iter().map(|e| e.to_string()).join(" "))
        };
        let closed = if self.closed { "CLOSED" } else { "" };
        let extra = if self.extra.is_empty() {
            "".to_string()
        } else {
            format!("EXTRA [{}]", self.extra.iter().join(" "))
        };
        let preds = if self.preds.is_empty() {
            "".to_string()
        } else {
            self.preds.iter().join(",")
        };
        write!(f, "Shape {extends}{closed}{extra} ")?;
        write!(f, "Preds: {preds}")?;
        write!(f, ", TripleExpr: {}", self.expr)?;
        write!(
            f,
            ", References: [{}]",
            self.references()
                .iter()
                .map(|(p, ls)| format!("{}->{}", p, ls.iter().join(" ")))
                .join(", ")
        )?;
        Ok(())
    }
}
