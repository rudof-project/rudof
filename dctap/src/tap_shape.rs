use crate::tap_statement::TapStatement;
use crate::TapError;
use crate::{tap_error::Result, tap_headers::TapHeaders};
use csv::StringRecord;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default, PartialEq, Clone)]
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

    pub(crate) fn parse_from_record(
        &mut self,
        record: StringRecord,
        tap_headers: &TapHeaders,
    ) -> Result<()> {
        println!("Trying to get a shape from record: {record:?} with tap_headers {tap_headers:?}");
        if let Some(shape_id) = tap_headers.shape_id() {
            if let Some(str) = record.get(shape_id) {
                self.shape_id = Some(str.to_string());
                println!("shape_id updated {self:?}");
            } else {
                return Err(TapError::NoShapeId { shape_id, record });
            }
        }
        Ok(())
    }

    pub fn with_shape_id(mut self, shape_id: &str) -> Self {
        self.shape_id = Some(shape_id.to_string());
        self
    }
}
