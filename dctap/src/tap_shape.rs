use serde_derive::{Deserialize, Serialize};

use crate::tap_statement::TapStatement;

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct TapShape {

    #[serde(rename = "shapeID")]
    shape_id: String, 

    statements: Vec<TapStatement>


    
}