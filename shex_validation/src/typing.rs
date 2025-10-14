use std::collections::{HashMap, HashSet};

use shex_ast::{Node, ShapeLabelIdx, shapemap::ValidationStatus};

/// Typing represents a mapping from (Node, ShapeLabelIdx) to ValidationStatus
/// This will be used to collect errors and reasons during validation
#[derive(Debug, Clone)]
pub struct Typing {
    map: HashMap<(Node, ShapeLabelIdx), ValidationStatus>,
}

impl Typing {
    pub fn new() -> Self {
        Typing {
            map: HashMap::new(),
        }
    }

    pub fn insert(&mut self, node: Node, shape: ShapeLabelIdx, status: ValidationStatus) {
        self.map
            .entry((node, shape))
            .and_modify(|e| e.merge(status.clone()))
            .or_insert(status);
    }

    pub fn is_conformant(&self, node: &Node, shape: &ShapeLabelIdx) -> bool {
        self.map
            .get(&(node.clone(), shape.clone()))
            .map_or(false, |status| status.is_conformant())
    }

    pub fn union_hyp(&mut self, hyp: &HashSet<(Node, ShapeLabelIdx)>) -> Typing {
        for (node, shape) in hyp {
            self.map
                .entry((node.clone(), shape.clone()))
                .or_insert(ValidationStatus::Pending);
        }
        self.clone()
    }

    pub fn iter_conformant(&self) -> impl Iterator<Item = &(Node, ShapeLabelIdx)> {
        self.map.iter().filter_map(|(key, status)| {
            if status.is_conformant() {
                Some(key)
            } else {
                None
            }
        })
    }

    pub fn get_status(&self, node: &Node, shape: &ShapeLabelIdx) -> Option<&ValidationStatus> {
        self.map.get(&(node.clone(), shape.clone()))
    }
}

impl Default for Typing {
    fn default() -> Self {
        Self::new()
    }
}
