use serde_derive::{Deserialize, Serialize};

use crate::PropertyId;

#[derive(Deserialize, Serialize, Debug, PartialEq, Default, Clone)]
pub struct TapStatement {
    #[serde(rename = "propertyID")]
    property_id: PropertyId,

    #[serde(rename = "propertyLabel")]
    property_label: Option<String>,

    mandatory: String,
    repeatable: Option<String>,

    #[serde(rename = "valueNodeType")]
    value_node_type: Option<String>,

    #[serde(rename = "valueDataType")]
    value_data_type: Option<String>,

    #[serde(rename = "valueConstraint")]
    value_constraint: Option<String>,

    #[serde(rename = "valueConstraintType")]
    value_constraint_type: Option<String>,

    #[serde(rename = "valueShape")]
    valueshape: Option<String>,

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
}
