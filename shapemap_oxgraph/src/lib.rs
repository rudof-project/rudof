use oxrdf::*;
use shapemap::ShapeMap;
use shapemap::ShapeMapState;
use std::collections::HashMap;
use std::collections::HashSet;
use std::str::FromStr;
use thiserror::Error;

pub mod shapelabel_oxgraph;
use crate::shapelabel_oxgraph::*;

pub struct ShapeMapOxGraph<'a> {
    node_shape_map: HashMap<&'a oxrdf::Term, HashMap<&'a ShapeLabelOxGraph, ShapeMapState>>,
}

impl<'a> ShapeMapOxGraph<'a> {
    pub fn new() -> ShapeMapOxGraph<'a> {
        ShapeMapOxGraph {
            node_shape_map: HashMap::new(),
        }
    }
}

impl<'a> ShapeMap<'a> for ShapeMapOxGraph<'a> {
    type NodeIdx = oxrdf::Term;
    type ShapeIdx = ShapeLabelOxGraph;

    fn next_pending_pair(&self) -> Option<(&Self::NodeIdx, &Self::ShapeIdx)> {
        for (node, shape_map) in &self.node_shape_map {
            for (shape, state) in shape_map {
                match state {
                    ShapeMapState::Pending => return Some((&node, &shape)),
                    _ => (),
                }
            }
        }
        None
    }

    fn nodes_conform(&self, shape: &Self::ShapeIdx) -> HashSet<&Self::NodeIdx> {
        let mut result: HashSet<&Self::NodeIdx> = HashSet::new();
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

    fn add_pending(
        &mut self,
        node: &'a Self::NodeIdx,
        shape: &'a Self::ShapeIdx,
        pairs: Vec<(&'a Self::NodeIdx, &'a Self::ShapeIdx)>,
    ) {
        match self.node_shape_map.get(&node) {
            None => {
                let mut shape_map: HashMap<&ShapeLabelOxGraph, ShapeMapState> = HashMap::new();
                shape_map.insert(shape, ShapeMapState::Pending);
                self.node_shape_map.insert(node, shape_map);
            }
            Some(shape_map) => todo!(),
        }
    }

    fn add_conforms(&mut self, node: &'a Self::NodeIdx, shape: &'a Self::ShapeIdx) {
        match self.node_shape_map.get_mut(node) {
            None => {
                let mut sm: HashMap<&ShapeLabelOxGraph, ShapeMapState> = HashMap::new();
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
                    ShapeMapState::Pending => {
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

    fn add_fails(&mut self, node: &'a Self::NodeIdx, shape: &'a Self::ShapeIdx) {
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
        sm.add_pending(&alice, &s, Vec::new());
        assert_eq!(sm.next_pending_pair(), Some((&alice, &s)));
    }

    #[test]
    fn shapemap_adding_conforms() {
        let mut sm = ShapeMapOxGraph::new();
        let alice = Term::NamedNode(NamedNode::new("http://example.org/alice").unwrap());
        let s = ShapeLabelOxGraph::Iri(NamedNode::new("http://example.org/S").unwrap());
        sm.add_pending(&alice, &s, Vec::new());
        sm.add_conforms(&alice, &s);
        assert_eq!(sm.state(&alice, &s), &ShapeMapState::Conforms);
    }
}
