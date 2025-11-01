use std::{collections::HashMap, fmt::Display};

use either::Either;

use crate::{
    edge::Edge, edge_id::EdgeId, edge_type::EdgeType, evidence::Evidence,
    label_property_spec::LabelPropertySpec, node::Node, node_id::NodeId, pgs_error::PgsError,
    type_name::TypeName,
};

/// Simple representation of a property graph
#[derive(Debug, Clone)]
pub struct PropertyGraphSchema {
    node_types: HashMap<NodeId, LabelPropertySpec>,
    edge_types: HashMap<EdgeId, EdgeType>,
    node_names: HashMap<String, NodeId>,
    edge_names: HashMap<String, EdgeId>,
    node_types_id_counter: usize,
    edge_id_counter: usize,
}

impl PropertyGraphSchema {
    pub fn new() -> Self {
        PropertyGraphSchema {
            node_types: HashMap::new(),
            edge_types: HashMap::new(),
            node_names: HashMap::new(),
            edge_names: HashMap::new(),
            node_types_id_counter: 0,
            edge_id_counter: 0,
        }
    }

    pub fn get_node_semantics(&self, type_name: &str) -> Result<&LabelPropertySpec, PgsError> {
        let node_id = self
            .node_names
            .get(type_name)
            .ok_or(PgsError::MissingNodeLabel {
                label: type_name.to_string(),
            })?;
        self.node_types
            .get(&node_id)
            .ok_or(PgsError::MissingType(type_name.to_string()))
    }

    pub fn get_edge_semantics(&self, type_name: &str) -> Result<&EdgeType, PgsError> {
        let edge_id = self
            .edge_names
            .get(type_name)
            .ok_or(PgsError::MissingEdgeLabel {
                label: type_name.to_string(),
            })?;
        self.edge_types
            .get(&edge_id)
            .ok_or(PgsError::MissingType(type_name.to_string()))
    }

    /*pub fn get_semantics_by_label(
        &self,
        label: &str,
    ) -> Result<Either<&FormalGraphType, &FormalEdgeType>, PgsError> {
        if let Some(node_id) = self.node_names.get(label) {
            let node = self
                .node_types
                .get(node_id)
                .ok_or(PgsError::MissingNodeLabel {
                    label: label.to_string(),
                })?;
            Ok(Either::Left(node))
        } else if let Some(edge_id) = self.edge_names.get(label) {
            let edge = self
                .edge_types
                .get(edge_id)
                .ok_or(PgsError::MissingNodeEdgeLabel {
                    label: label.to_string(),
                })?;
            Ok(Either::Right(edge))
        } else {
            Err(PgsError::MissingNodeEdgeLabel {
                label: label.to_string(),
            })
        }
    }*/

    pub fn add_node_spec(
        &mut self,
        type_name: &str,
        spec: LabelPropertySpec,
    ) -> Result<NodeId, PgsError> {
        let node_id = NodeId::new(self.node_types_id_counter);
        self.node_types.insert(node_id.clone(), spec);
        self.node_names
            .insert(type_name.to_string(), node_id.clone());
        self.node_types_id_counter += 1;
        Ok(node_id)
    }

    pub fn add_blank_node_spec(&mut self, spec: LabelPropertySpec) -> Result<NodeId, PgsError> {
        let type_name = format!("{}", self.node_types_id_counter);
        self.add_node_spec(type_name.as_str(), spec)
    }

    pub fn add_blank_edge_spec(
        &mut self,
        source: LabelPropertySpec,
        edge: LabelPropertySpec,
        target: LabelPropertySpec,
    ) -> Result<EdgeId, PgsError> {
        let type_name = format!("{}", self.edge_id_counter);
        self.add_edge_spec(type_name.as_str(), source, edge, target)
    }

    pub fn add_edge_spec(
        &mut self,
        type_name: &str,
        source: LabelPropertySpec,
        edge: LabelPropertySpec,
        target: LabelPropertySpec,
    ) -> Result<EdgeId, PgsError> {
        let edge_id = EdgeId::new(self.edge_id_counter);
        if self.edge_names.contains_key(type_name) {
            return Err(PgsError::DuplicateEdgeTypeName {
                type_name: type_name.to_string(),
            });
        }
        self.edge_types
            .insert(edge_id.clone(), EdgeType::new(source, edge, target));
        self.edge_names
            .insert(type_name.to_string(), edge_id.clone());
        self.edge_id_counter += 1;
        Ok(edge_id)
    }

    pub fn conforms_node(
        &self,
        type_name: &TypeName,
        node: &Node,
    ) -> Either<Vec<PgsError>, Vec<Evidence>> {
        if let Some(node_id) = self.node_names.get(type_name) {
            if let Some(spec) = self.node_types.get(node_id) {
                match spec.semantics(&self) {
                    Ok(semantics) => semantics.conforms(node.labels(), node.content()),
                    Err(e) => Either::Left(vec![e]),
                }
            } else {
                Either::Left(vec![PgsError::MissingType(type_name.clone())])
            }
        } else {
            Either::Left(vec![PgsError::MissingNodeLabel {
                label: type_name.to_string(),
            }])
        }
    }

    pub fn conforms_edge(
        &self,
        type_name: &TypeName,
        edge: &Edge,
    ) -> Either<Vec<PgsError>, Vec<Evidence>> {
        if let Some(edge_id) = self.edge_names.get(type_name) {
            if let Some(spec) = self.edge_types.get(edge_id) {
                match spec.semantics(&self) {
                    Ok(semantics) => semantics.conforms_edge(type_name, edge),
                    Err(e) => Either::Left(vec![e]),
                }
            } else {
                Either::Left(vec![PgsError::MissingType(type_name.clone())])
            }
        } else {
            Either::Left(vec![PgsError::MissingEdgeLabel {
                label: type_name.to_string(),
            }])
        }
    }
}

impl Display for PropertyGraphSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Property Graph Schema:")?;
        writeln!(f, "Node Types:")?;
        for (id, node) in &self.node_types {
            writeln!(f, "  {}: {}", id, node)?;
        }
        writeln!(f, "Edge Types:")?;
        for (id, edge) in &self.edge_types {
            writeln!(f, "  {}: {}", id, edge)?;
        }
        Ok(())
    }
}
