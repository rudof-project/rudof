use crate::tap_error::Result;
use csv::StringRecord;
use tracing::debug;

#[derive(Debug, Default)]
pub(crate) struct TapHeaders {
    shape_id: Option<usize>,
    property_id: Option<usize>,
}

impl TapHeaders {
    pub(crate) fn new() -> TapHeaders {
        TapHeaders::default()
    }

    pub(crate) fn from_record(record: &StringRecord) -> Result<TapHeaders> {
        let mut shape_id = None;
        let mut property_id = None;

        for (idx, field) in record.iter().enumerate() {
            match clean(field).as_str() {
                "SHAPEID" => shape_id = Some(idx),
                "PROPERTYID" => property_id = Some(idx),
                _ => {
                    debug!("Unknown field reading headers: {field}")
                }
            }
        }
        Ok(TapHeaders {
            shape_id,
            property_id,
        })
    }

    pub fn shape_id(&self) -> Option<usize> {
        self.shape_id
    }

    pub fn property_id(&self) -> Option<usize> {
        self.property_id
    }
}

fn clean(str: &str) -> String {
    str.to_uppercase()
}
