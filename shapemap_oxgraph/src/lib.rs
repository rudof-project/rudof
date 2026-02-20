use shapemap::ShapeMap;
use shapemap::ShapeMapState;
use std::collections::HashMap;
use std::collections::HashSet;

pub mod shapelabel_oxgraph;
use crate::shapelabel_oxgraph::*;

type NodeLabelPair = (oxrdf::Term, ShapeLabelOxGraph);
pub struct ShapeMapOxGraph<'a> {
    node_shape_map: HashMap<oxrdf::Term, HashMap<ShapeLabelOxGraph, &'a ShapeMapState>>,
    pending: Vec<NodeLabelPair>,
    associated_pendings: HashMap<NodeLabelPair, Vec<NodeLabelPair>>,
}

impl<'a> ShapeMapOxGraph<'a> {
    pub fn new() -> ShapeMapOxGraph<'a> {
        ShapeMapOxGraph {
            node_shape_map: HashMap::new(),
            pending: Vec::new(),
            associated_pendings: HashMap::new(),
        }
    }
}

impl<'a> ShapeMap for ShapeMapOxGraph<'a> {
    type NodeIdx = oxrdf::Term;
    type ShapeIdx = ShapeLabelOxGraph;

    /*     fn next_pending_pair(&self) -> Option<(&Self::NodeIdx, &Self::ShapeIdx)> {
        for (node, shape_map) in &self.node_shape_map {
            for (shape, state) in shape_map {
                match state {
                    ShapeMapState::Pending => return Some((&node, &shape)),
                    _ => (),
                }
            }
        }
        None
    } */

    fn nodes_conform(&self, shape: &Self::ShapeIdx) -> HashSet<&Self::NodeIdx> {
        let mut result: HashSet<&Self::NodeIdx> = HashSet::new();
        for (node, shape_map) in &self.node_shape_map {
            for (shape, state) in shape_map {
                match state {
                    ShapeMapState::Conforms => {
                        result.insert(node);
                    },
                    _ => (),
                }
            }
        }
        result
    }

    fn state(&self, node: &Self::NodeIdx, shape: &Self::ShapeIdx) -> &ShapeMapState {
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
        let mut result: HashSet<&Self::NodeIdx> = HashSet::new();
        for (node, _) in &self.node_shape_map {
            result.insert(node);
        }
        result
    }

    fn shapes(&self) -> HashSet<&Self::ShapeIdx> {
        let mut result: HashSet<&Self::ShapeIdx> = HashSet::new();
        for (_, shape_map) in &self.node_shape_map {
            for (shape, _) in shape_map {
                result.insert(shape);
            }
        }
        result
    }

    fn add_pending(&mut self, node: &Self::NodeIdx, shape: &Self::ShapeIdx) {
        match self.node_shape_map.get(&node) {
            None => {
                let mut shape_map = HashMap::new();
                shape_map.insert((*shape).clone(), &ShapeMapState::Pending);
                self.node_shape_map.insert((*node).clone(), shape_map);
            },
            Some(shape_map) => todo!(),
        }
    }

    fn add_conforms(&mut self, node: &Self::NodeIdx, shape: &Self::ShapeIdx) {
        match self.node_shape_map.get_mut(&node) {
            None => {
                let mut sm: HashMap<ShapeLabelOxGraph, &'a ShapeMapState> = HashMap::new();
                sm.insert((*shape).clone(), &ShapeMapState::Conforms);
                self.node_shape_map.insert(node.clone(), sm);
            },
            Some(sm) => match sm.get_mut(&shape) {
                None => {
                    sm.insert((*shape).clone(), &ShapeMapState::Conforms);
                },
                Some(state) => match state {
                    ShapeMapState::Conforms => (),
                    ShapeMapState::Fails => {
                        sm.insert((*shape).clone(), &ShapeMapState::Inconsistent);
                    },
                    ShapeMapState::Pending => {
                        // TODO: Notify the pending pairs!
                        sm.insert((*shape).clone(), &ShapeMapState::Conforms);
                    },
                    ShapeMapState::Unknown => {
                        sm.insert((*shape).clone(), &ShapeMapState::Conforms);
                    },
                    ShapeMapState::Inconsistent => (),
                },
            },
        }
    }

    fn add_fails(&mut self, node: &Self::NodeIdx, shape: &Self::ShapeIdx) {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use crate::{ShapeLabelOxGraph, ShapeMapOxGraph};
    use oxrdf::{NamedNode, Term};
    use shapemap::{ShapeMap, ShapeMapState};

    #[test]
    fn shapemap_test() {
        let mut sm = ShapeMapOxGraph::new();
        let alice = Term::NamedNode(NamedNode::new("http://example.org/alice").unwrap());
        let s = ShapeLabelOxGraph::Iri(NamedNode::new("http://example.org/S").unwrap());
        sm.add_pending(&alice, &s);
        // assert_eq!(sm.next_pending_pair(), Some((&alice, &s)));
    }

    #[test]
    fn shapemap_adding_conforms() {
        let mut sm = ShapeMapOxGraph::new();
        let alice = Term::NamedNode(NamedNode::new("http://example.org/alice").unwrap());
        let s = ShapeLabelOxGraph::Iri(NamedNode::new("http://example.org/S").unwrap());
        sm.add_pending(&alice, &s);
        sm.add_conforms(&alice, &s);
        assert_eq!(sm.state(&alice, &s), &ShapeMapState::Conforms);
    }
}
