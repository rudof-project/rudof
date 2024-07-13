use std::fmt::Display;

use serde_derive::{Deserialize, Serialize};

use crate::{DatatypeId, NodeType, PropertyId, ShapeId};

#[derive(Deserialize, Serialize, Debug, PartialEq, Default, Clone)]
pub struct TapStatement {
    #[serde(rename = "propertyID")]
    property_id: PropertyId,

    #[serde(rename = "propertyLabel", skip_serializing_if = "Option::is_none")]
    property_label: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    mandatory: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    repeatable: Option<bool>,

    #[serde(rename = "valueNodeType", skip_serializing_if = "Option::is_none")]
    value_nodetype: Option<NodeType>,

    #[serde(rename = "valueDataType", skip_serializing_if = "Option::is_none")]
    value_datatype: Option<DatatypeId>,

    #[serde(rename = "valueConstraint", skip_serializing_if = "Option::is_none")]
    value_constraint: Option<String>,

    #[serde(
        rename = "valueConstraintType",
        skip_serializing_if = "Option::is_none"
    )]
    value_constraint_type: Option<String>,

    #[serde(rename = "valueShape", skip_serializing_if = "Option::is_none")]
    value_shape: Option<ShapeId>,

    #[serde(rename = "note", skip_serializing_if = "Option::is_none")]
    note: Option<String>,
    // state_warns: dict = field(default_factory=dict)
    // state_extras: dict = field(default_factory=dict)
}

impl TapStatement {
    pub fn new(property_id: PropertyId) -> TapStatement {
        TapStatement::default().with_property_id(property_id)
    }

    pub fn with_property_id(mut self, property_id: PropertyId) -> Self {
        self.property_id = property_id;
        self
    }

    pub fn set_repeatable(&mut self, repeatable: bool) {
        self.repeatable = Some(repeatable);
    }

    pub fn set_mandatory(&mut self, mandatory: bool) {
        self.mandatory = Some(mandatory);
    }

    pub fn set_value_datatype(&mut self, datatype: &DatatypeId) {
        self.value_datatype = Some(datatype.clone());
    }

    pub fn set_value_nodetype(&mut self, nodetype: &NodeType) {
        self.value_nodetype = Some(nodetype.clone());
    }

    pub fn set_value_shape(&mut self, value_shape: &ShapeId) {
        self.value_shape = Some(value_shape.clone());
    }

    pub fn set_property_label(&mut self, property_label: &str) {
        self.property_label = Some(property_label.to_string());
    }

    pub fn property_id(&self) -> PropertyId {
        self.property_id.clone()
    }

    pub fn mandatory(&self) -> Option<bool> {
        self.mandatory
    }
    pub fn repeatable(&self) -> Option<bool> {
        self.repeatable
    }
    pub fn value_datatype(&self) -> Option<DatatypeId> {
        self.value_datatype.clone()
    }
    pub fn value_shape(&self) -> Option<ShapeId> {
        self.value_shape.clone()
    }
}

impl Display for TapStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {} {}",
            show_property(&self.property_id, &self.property_label),
            show_node_constraints(
                &self.value_nodetype,
                &self.value_datatype,
                &self.value_constraint,
                &self.value_constraint_type,
                &self.value_shape
            ),
            show_cardinality(self.mandatory, self.repeatable),
            show_note(&self.note)
        )?;
        Ok(())
    }
}

fn show_property(property_id: &PropertyId, property_label: &Option<String>) -> String {
    let mut result = String::new();
    if let Some(label) = property_label {
        result.push_str(format!("{label} ({property_id})").as_str());
    } else {
        result.push_str(format!("{property_id}").as_str());
    }
    result
}

fn show_node_constraints(
    value_node_type: &Option<NodeType>,
    datatype: &Option<DatatypeId>,
    value_constraint: &Option<String>,
    value_constraint_type: &Option<String>,
    value_shape: &Option<ShapeId>,
) -> String {
    let mut result = String::new();
    if let Some(node_type) = value_node_type {
        result.push_str(format!("{node_type}").as_str());
    }
    if let Some(datatype) = datatype {
        result.push_str(format!("{datatype}").as_str());
    }
    if let Some(value_constraint) = value_constraint {
        result.push_str(value_constraint);
    }
    if let Some(value_constraint_type) = value_constraint_type {
        result.push_str(value_constraint_type);
    }
    if let Some(value_shape) = value_shape {
        result.push_str(format!("@{value_shape}").as_str());
    }
    if result.is_empty() {
        result.push('.')
    }
    result
}

fn show_cardinality(mandatory: Option<bool>, repeatable: Option<bool>) -> String {
    let mandatory = mandatory.unwrap_or(false);
    let repeatable = repeatable.unwrap_or(false);
    match (mandatory, repeatable) {
        (false, false) => "?".to_string(),
        (false, true) => "*".to_string(),
        (true, false) => "".to_string(),
        (true, true) => "+".to_string(),
    }
}

fn show_note(note: &Option<String>) -> String {
    if let Some(note) = note {
        format!("// {note}")
    } else {
        "".to_string()
    }
}
