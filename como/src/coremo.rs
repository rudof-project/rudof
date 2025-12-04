use std::{collections::HashMap, fmt::Display};

use crate::{NodeId, NodeLabel, ResultAssociation, ShapeId, ShapeLabel};

/// Common Result Model
#[derive(Debug)]
pub struct CoReMo {
    has_errors: bool,
    associations: HashMap<ShapeId, HashMap<NodeId, ResultAssociation>>,
    shape_labels_id: HashMap<ShapeLabel, ShapeId>,
    node_labels_id: HashMap<NodeLabel, NodeId>,
    nodes_counter: usize,
    shapes_counter: usize,
}

impl CoReMo {
    pub fn new() -> Self {
        CoReMo {
            has_errors: false,
            associations: HashMap::new(),
            shape_labels_id: HashMap::new(),
            node_labels_id: HashMap::new(),
            nodes_counter: 0,
            shapes_counter: 0,
        }
    }
}

impl CoReMo {
    pub fn add_association(
        &mut self,
        shape: ShapeLabel,
        node: NodeLabel,
        association: ResultAssociation,
    ) {
        if !association.conforms() {
            self.has_errors = true;
        }
        let shape_id = *self.shape_labels_id.entry(shape).or_insert_with(|| {
            let id = self.shapes_counter;
            self.shapes_counter += 1;
            id
        });
        let node_id = *self.node_labels_id.entry(node).or_insert_with(|| {
            let id = self.nodes_counter;
            self.nodes_counter += 1;
            id
        });
        let shape_assoc = self
            .associations
            .entry(shape_id)
            .or_insert_with(HashMap::new);
        shape_assoc.insert(node_id, association);
    }
}
