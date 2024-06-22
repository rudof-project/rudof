use serde_derive::{Deserialize, Serialize};

use crate::PropertyId;

#[derive(Deserialize, Serialize, Debug, PartialEq, Default, Clone)]
pub struct TapStatement {
    #[serde(rename = "propertyID")]
    property_id: PropertyId,

    #[serde(rename = "propertyLabel", skip_serializing_if = "Option::is_none")]
    property_label: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    mandatory: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    repeatable: Option<String>,

    #[serde(rename = "valueNodeType", skip_serializing_if = "Option::is_none")]
    value_node_type: Option<String>,

    #[serde(rename = "valueDataType", skip_serializing_if = "Option::is_none")]
    value_data_type: Option<String>,

    #[serde(rename = "valueConstraint", skip_serializing_if = "Option::is_none")]
    value_constraint: Option<String>,

    #[serde(
        rename = "valueConstraintType",
        skip_serializing_if = "Option::is_none"
    )]
    value_constraint_type: Option<String>,

    #[serde(rename = "valueShape", skip_serializing_if = "Option::is_none")]
    valueshape: Option<String>,

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

    pub fn set_property_label(&mut self, property_label: &str) {
        self.property_label = Some(property_label.to_string());
    }
}
