
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Unified constraint model that abstracts over ShEx and SHACL
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedConstraintModel {
    pub shapes: HashMap<String, UnifiedShape>,
    pub dependencies: HashMap<String, Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedShape {
    pub id: String,
    pub target_class: Option<String>,
    pub properties: Vec<UnifiedPropertyConstraint>,
    pub closed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedPropertyConstraint {
    pub property_iri: String,
    pub constraints: Vec<UnifiedConstraint>,
    pub min_cardinality: Option<u32>,
    pub max_cardinality: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UnifiedConstraint {
    Datatype(String),
    ShapeReference(String),
    NodeKind(NodeKind),
    Pattern(String),
    MinInclusive(Value),
    MaxInclusive(Value),
    MinExclusive(Value),
    MaxExclusive(Value),
    MinLength(u32),
    MaxLength(u32),
    In(Vec<Value>),
    HasValue(Value),
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum NodeKind {
    IRI,
    BlankNode,
    Literal,
    BlankNodeOrIRI,
    BlankNodeOrLiteral,
    IRIOrLiteral,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Value {
    IRI(String),
    Literal(String, Option<String>), // value, datatype
    BlankNode(String),
}

impl UnifiedConstraintModel {
    pub fn new() -> Self {
        Self {
            shapes: HashMap::new(),
            dependencies: HashMap::new(),
        }
    }

    pub fn add_shape(&mut self, shape: UnifiedShape) {
        let shape_id = shape.id.clone();
        
        // Extract dependencies from shape properties
        let mut deps = Vec::new();
        for prop in &shape.properties {
            for constraint in &prop.constraints {
                if let UnifiedConstraint::ShapeReference(ref target_shape) = constraint {
                    deps.push(target_shape.clone());
                }
            }
        }
        
        self.dependencies.insert(shape_id.clone(), deps);
        self.shapes.insert(shape_id, shape);
    }

    pub fn get_shape(&self, shape_id: &str) -> Option<&UnifiedShape> {
        self.shapes.get(shape_id)
    }

    pub fn get_dependencies(&self, shape_id: &str) -> Option<&Vec<String>> {
        self.dependencies.get(shape_id)
    }
}

impl Default for UnifiedConstraintModel {
    fn default() -> Self {
        Self::new()
    }
}
