use super::{
    annotation::Annotation,
    dependency_graph::{DependencyGraph, PosNeg},
    sem_act::SemAct,
};
use crate::{Node, Pred, ShapeLabelIdx};
use iri_s::IriS;
use itertools::Itertools;
use rbe::RbeTable;
use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Shape {
    closed: bool,
    extra: Vec<IriS>,
    rbe_table: RbeTable<Pred, Node, ShapeLabelIdx>,
    sem_acts: Vec<SemAct>,
    annotations: Vec<Annotation>,
    preds: Vec<IriS>,
    display: String,
}

impl Shape {
    pub fn new(
        closed: bool,
        extra: Vec<IriS>,
        rbe_table: RbeTable<Pred, Node, ShapeLabelIdx>,
        sem_acts: Vec<SemAct>,
        annotations: Vec<Annotation>,
        preds: Vec<IriS>,
        display: String,
    ) -> Self {
        Shape {
            closed,
            extra,
            rbe_table,
            sem_acts,
            annotations,
            preds,
            display,
        }
    }

    pub fn preds(&self) -> Vec<IriS> {
        self.preds.clone()
    }

    pub fn rbe_table(&self) -> &RbeTable<Pred, Node, ShapeLabelIdx> {
        &self.rbe_table
    }

    pub fn is_closed(&self) -> bool {
        self.closed
    }

    pub fn add_edges(&self, source: ShapeLabelIdx, graph: &mut DependencyGraph, pos_neg: PosNeg) {
        println!("Adding edges for shape: {}", self.rbe_table);
        // todo!()
    }
}

impl Display for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let closed = if self.closed { "CLOSED" } else { "" };
        let extra = if self.extra.is_empty() {
            "".to_string()
        } else {
            format!("EXTRA [{}]", self.extra.iter().join(" "))
        };
        let preds = if self.preds.is_empty() {
            "".to_string()
        } else {
            format!("{}", self.preds.iter().join(","))
        };
        write!(f, "Shape {closed}{extra} ")?;
        writeln!(f, "Preds: {}", preds)?;
        writeln!(f, "{}", self.rbe_table)?;
        Ok(())
    }
}
