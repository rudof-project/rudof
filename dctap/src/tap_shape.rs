use crate::tap_error::Result;
use crate::tap_statement::TapStatement;
use csv::StringRecord;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct TapShape {
    #[serde(rename = "shapeID")]
    shape_id: Option<String>,

    statements: Vec<TapStatement>,
}

impl TapShape {
    pub fn new() -> TapShape {
        TapShape {
            shape_id: Option::None,
            statements: Vec::new(),
        }
    }

    pub(crate) fn from_record(&mut self, _record: StringRecord) -> Result<()> {
        Ok(())
    }
}
