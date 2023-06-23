use oxrdf::*;
use shapemap::ShapeMap;
use shapemap::ShapeMapState;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum NodeIndex {
    NamedNode(NamedNode),
    BNode(BlankNode),
    Literal(Literal),
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum ShapeLabel {
    NamedNode(NamedNode),
    BNode(BlankNode),
}

pub struct ShapeMapOxGraph<'a> {
    node_shape_map:
        HashMap<&'a NodeIndex, HashMap<&'a ShapeLabel, ShapeMapState<'a, NodeIndex, ShapeLabel>>>,
}

impl<'a> ShapeMapOxGraph<'a> {
    pub fn new() -> ShapeMapOxGraph<'a> {
        ShapeMapOxGraph {
            node_shape_map: HashMap::new(),
        }
    }
}

impl<'a> ShapeMap<'a> for ShapeMapOxGraph<'a> {
    type NodeIdx = NodeIndex;
    type ShapeIdx = ShapeLabel;

    fn next_pending_pair(&self) -> Option<(&Self::NodeIdx, &Self::ShapeIdx)> {
        for (node, shape_map) in &self.node_shape_map {
            for (shape, state) in shape_map {
                match state {
                    ShapeMapState::Pending { pairs: _ } => return Some((&node, &shape)),
                    _ => (),
                }
            }
        }
        None
    }

    fn nodes_conform(&self, shape: &Self::ShapeIdx) -> HashSet<&Self::NodeIdx> {
        let mut result: HashSet<&NodeIndex> = HashSet::new();
        for (node, shape_map) in &self.node_shape_map {
            for (shape, state) in shape_map {
                match state {
                    ShapeMapState::Conforms => {
                        result.insert(node);
                    }
                    _ => (),
                }
            }
        }
        result
    }

    fn state(
        &self,
        node: &Self::NodeIdx,
        shape: &Self::ShapeIdx,
    ) -> &ShapeMapState<'a, Self::NodeIdx, Self::ShapeIdx> {
        if let Some(shape_map) = self.node_shape_map.get(node) {
            if let Some(state) = shape_map.get(shape) {
                state
            } else {
                &ShapeMapState::Unknown
            }
        } else {
            &ShapeMapState::Unknown
        }
    }

    fn nodes(&self) -> HashSet<&Self::NodeIdx> {
        let mut result: HashSet<&NodeIndex> = HashSet::new();
        for (node, _) in &self.node_shape_map {
            result.insert(node);
        }
        result
    }

    fn shapes(&self) -> HashSet<&Self::ShapeIdx> {
        let mut result: HashSet<&ShapeLabel> = HashSet::new();
        for (_, shape_map) in &self.node_shape_map {
            for (shape, _) in shape_map {
                result.insert(shape);
            }
        }
        result
    }

    fn add_pending(
        &mut self,
        node: &'a NodeIndex,
        shape: &'a ShapeLabel,
        pairs: Vec<(&'a NodeIndex, &'a ShapeLabel)>,
    ) {
        match self.node_shape_map.get(&node) {
            None => {
                let mut shape_map = HashMap::new();
                shape_map.insert(shape, ShapeMapState::Pending { pairs: pairs });
                self.node_shape_map.insert(node, shape_map);
            }
            Some(shape_map) => todo!(),
        }
    }

    fn add_conforms(&mut self, node: &'a NodeIndex, shape: &'a ShapeLabel) {
        match self.node_shape_map.get_mut(node) {
            None => {
                let mut sm = HashMap::new();
                sm.insert(shape, ShapeMapState::Conforms);
                self.node_shape_map.insert(node, sm);
            }
            Some(sm) => match sm.get_mut(shape) {
                None => {
                    sm.insert(shape, ShapeMapState::Conforms);
                }
                Some(state) => match state {
                    ShapeMapState::Conforms => (),
                    ShapeMapState::Fails => {
                        sm.insert(shape, ShapeMapState::Inconsistent);
                    }
                    ShapeMapState::Pending { pairs: _ } => {
                        // TODO: Notify the pending pairs!
                        sm.insert(shape, ShapeMapState::Conforms);
                    }
                    ShapeMapState::Unknown => {
                        sm.insert(shape, ShapeMapState::Conforms);
                    }
                    ShapeMapState::Inconsistent => (),
                },
            },
        }
    }

    fn add_fails(&mut self, node: &'a NodeIndex, shape: &'a ShapeLabel) {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use crate::{NodeIndex, ShapeLabel, ShapeMapOxGraph};
    use oxrdf::NamedNode;
    use shapemap::{ShapeMap, ShapeMapState};

    #[test]
    fn shapemap_test() {
        let mut sm = ShapeMapOxGraph::new();
        let alice = NodeIndex::NamedNode(NamedNode::new("http://example.org/alice").unwrap());
        let s = ShapeLabel::NamedNode(NamedNode::new("http://example.org/S").unwrap());
        sm.add_pending(&alice, &s, Vec::new());
        assert_eq!(sm.next_pending_pair(), Some((&alice, &s)));
    }

    #[test]
    fn shapemap_adding_conforms() {
        let mut sm = ShapeMapOxGraph::new();
        let alice = NodeIndex::NamedNode(NamedNode::new("http://example.org/alice").unwrap());
        let s = ShapeLabel::NamedNode(NamedNode::new("http://example.org/S").unwrap());
        sm.add_pending(&alice, &s, Vec::new());
        sm.add_conforms(&alice, &s);
        assert_eq!(sm.state(&alice, &s), &ShapeMapState::Conforms);
    }
}
