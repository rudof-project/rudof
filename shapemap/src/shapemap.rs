use crate::shapemap_state::*;
use std::collections::HashSet;

/// A ShapeMap is based on [shape maps specification](https://shexspec.github.io/shape-map/)
/// This trait is the more abstract representation of a shape map which is an association of nodes with shapes, whose values are a [ShapeMapState]
pub trait ShapeMap {
    type NodeIdx;
    type ShapeIdx;

    fn nodes_conform(&self, shape: &Self::ShapeIdx) -> HashSet<&Self::NodeIdx>;

    fn state(&self, node: &Self::NodeIdx, shape: &Self::ShapeIdx) -> &ShapeMapState;

    fn nodes(&self) -> HashSet<&Self::NodeIdx>;

    fn shapes(&self) -> HashSet<&Self::ShapeIdx>;

    fn add_pending(&mut self, node: &Self::NodeIdx, shape: &Self::ShapeIdx);

    fn add_conforms(&mut self, node: &Self::NodeIdx, shape: &Self::ShapeIdx);

    fn add_fails(&mut self, node: &Self::NodeIdx, shape: &Self::ShapeIdx);
}
