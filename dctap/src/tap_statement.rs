use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct TapStatement {

    #[serde(rename = "propertyID")]
    property_id: String, 
    
    #[serde(rename = "propertyLabel")]
    property_label: String, 

    mandatory: String, 
    repeatable: String, 

    #[serde(rename = "valueNodeType")]
    value_node_type: String, 

    #[serde(rename = "valueDataType")]
    value_data_type: String, 

    #[serde(rename = "valueConstraint")]
    value_constraint: String, 

    #[serde(rename = "valueConstraintType")]
    value_constraint_type: String, 

    #[serde(rename = "valueShape")]
    valueshape: String, 

    note: String, 

    // state_warns: dict = field(default_factory=dict)
    // state_extras: dict = field(default_factory=dict)

}