use std::collections::HashSet;

#[derive(PartialEq, Eq, Debug)]
pub enum ShapeMapState<'a, N, S> {
    Conforms,
    Fails,
    Pending { pairs: Vec<(&'a N, &'a S)> },
    Unknown,
    Inconsistent,
}

pub trait ShapeMap<'a> {
    type NodeIdx;
    type ShapeIdx;

    fn next_pending_pair(&self) -> Option<(&Self::NodeIdx, &Self::ShapeIdx)>;

    fn nodes_conform(&self, shape: &Self::ShapeIdx) -> HashSet<&Self::NodeIdx>;

    fn state(
        &self,
        node: &Self::NodeIdx,
        shape: &Self::ShapeIdx,
    ) -> &ShapeMapState<'a, Self::NodeIdx, Self::ShapeIdx>;

    fn nodes(&self) -> HashSet<&Self::NodeIdx>;

    fn shapes(&self) -> HashSet<&Self::ShapeIdx>;

    fn add_pending(
        &mut self,
        node: &'a Self::NodeIdx,
        shape: &'a Self::ShapeIdx,
        pairs: Vec<(&'a Self::NodeIdx, &'a Self::ShapeIdx)>,
    );

    fn add_conforms(&mut self, node: &'a Self::NodeIdx, shape: &'a Self::ShapeIdx);

    fn add_fails(&mut self, node: &'a Self::NodeIdx, shape: &'a Self::ShapeIdx);
}
