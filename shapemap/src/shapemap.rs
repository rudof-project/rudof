use crate::shapemap_state::*;
use std::collections::HashSet;

pub trait ShapeMap {
    type NodeIdx;
    type ShapeIdx;

    //    fn next_pending_pair(&self) -> Option<(&Self::NodeIdx, &Self::ShapeIdx)>;

    fn nodes_conform(&self, shape: &Self::ShapeIdx) -> HashSet<&Self::NodeIdx>;

    fn state(&self, node: &Self::NodeIdx, shape: &Self::ShapeIdx) -> &ShapeMapState;

    fn nodes(&self) -> HashSet<&Self::NodeIdx>;

    fn shapes(&self) -> HashSet<&Self::ShapeIdx>;

    fn add_pending(&mut self, node: &Self::NodeIdx, shape: &Self::ShapeIdx);

    fn add_conforms(&mut self, node: &Self::NodeIdx, shape: &Self::ShapeIdx);

    fn add_fails(&mut self, node: &Self::NodeIdx, shape: &Self::ShapeIdx);
}
