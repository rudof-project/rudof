use oxrdf::*;
use shapemap::ShapeMap;
use shapemap::ShapeMapState;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash)]
pub enum NodeIndex {
    NamedNode(NamedNode),
    BNode(BlankNode),
    Literal(Literal),
}

#[derive(PartialEq, Eq, Hash)]
pub enum ShapeLabel {
    NamedNode(NamedNode),
    BNode(BlankNode),
}

pub struct ShapeMap_OxGraph {
    node_shape_map: HashMap<NodeIndex, HashMap<ShapeLabel, ShapeMapState>>,
}

impl ShapeMap_OxGraph {
    fn new() -> ShapeMap_OxGraph {
        ShapeMap_OxGraph {
            node_shape_map: HashMap::new(),
        }
    }

    fn add_pending(&mut self, node: NodeIndex, shape: ShapeLabel) {
        match self.node_shape_map.get(&node) {
            None => {
                let mut shape_map = HashMap::new();
                shape_map.insert(shape, ShapeMapState::Pending);
                self.node_shape_map.insert(node, shape_map);
            }
            Some(shape_map) => todo!(),
        }
    }
}
impl ShapeMap for ShapeMap_OxGraph {
    type NodeIdx = NodeIndex;
    type ShapeIdx = ShapeLabel;

    fn next_pending_pair(&self) -> Option<(&Self::NodeIdx, &Self::ShapeIdx)> {
        for (node, shape_map) in &self.node_shape_map {
            for (shape, state) in shape_map {
                match state {
                    ShapeMapState::Pending => return Some((&node, &shape)),
                    _ => ()
                }
            }
        }
        None
    }

    fn nodes_conform(&self, shape: &Self::ShapeIdx) -> HashSet<&Self::NodeIdx> {
        let mut result = HashSet::new();
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
        let mut result = HashSet::new();
        for (node, _) in &self.node_shape_map {
            result.insert(node);
        }
        result
    }

    fn shapes(&self) -> HashSet<&Self::ShapeIdx> {
        let mut result = HashSet::new();
        for (_, shape_map) in &self.node_shape_map {
            for (shape, _) in shape_map {
                result.insert(shape);
            }
        }
        result
    }
}
