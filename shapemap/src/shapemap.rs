use std::collections::HashSet;

pub enum ShapeMapState {
    Conforms,
    Fails,
    Pending,
    Unknown,
}

pub trait ShapeMap {
    type NodeIdx;
    type ShapeIdx;

    fn next_pending_pair(&self) -> Option<(&Self::NodeIdx, &Self::ShapeIdx)>;
    fn nodes_conform(&self, shape: &Self::ShapeIdx) -> HashSet<&Self::NodeIdx>;
    fn state(&self, node: &Self::NodeIdx, shape: &Self::ShapeIdx) -> &ShapeMapState;
    fn nodes(&self) -> HashSet<&Self::NodeIdx>;
    fn shapes(&self) -> HashSet<&Self::ShapeIdx>;
}
