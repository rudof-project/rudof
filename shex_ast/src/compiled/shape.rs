use iri_s::IriS;
use rbe::RbeTable;
use std::fmt::Display;

use crate::{Node, Pred, ShapeLabelIdx};

use super::{annotation::Annotation, sem_act::SemAct};

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
}

impl Display for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Shape: {}", self.display)
    }
}
