use crate::tap_error::Result;
use csv::StringRecord;
use tracing::debug;

#[derive(Debug, Default)]
pub(crate) struct TapHeaders {
    shape_id: Option<usize>,
    property_id: Option<usize>,
    property_label: Option<usize>,
}

impl TapHeaders {
    pub(crate) fn new() -> TapHeaders {
        TapHeaders::default()
    }

    pub(crate) fn from_record(record: &StringRecord) -> Result<TapHeaders> {
        let mut shape_id = None;
        let mut property_id = None;
        let mut property_label = None;

        for (idx, field) in record.iter().enumerate() {
            match clean(field).as_str() {
                "SHAPEID" => shape_id = Some(idx),
                "PROPERTYID" => property_id = Some(idx),
                "PROPERTYLABEL" => property_label = Some(idx),
                _ => {
                    debug!("Unknown field reading headers: {field}")
                }
            }
        }
        Ok(TapHeaders {
            shape_id,
            property_id,
            property_label,
        })
    }

    pub fn shape_id(&self, rcd: &StringRecord) -> Option<String> {
        self.shape_id.and_then(|idx| get_str_from_rcd(rcd, idx))
    }

    pub fn property_id(&self, rcd: &StringRecord) -> Option<String> {
        self.property_id.and_then(|idx| get_str_from_rcd(rcd, idx))
    }

    pub fn property_label(&self, rcd: &StringRecord) -> Option<String> {
        self.property_label
            .and_then(|idx| get_str_from_rcd(rcd, idx))
    }
}

fn clean(str: &str) -> String {
    str.to_uppercase()
}

fn get_str_from_rcd(rcd: &StringRecord, idx: usize) -> Option<String> {
    if let Some(str) = rcd.get(idx) {
        Some(str.to_string())
    } else {
        // Should be an internal error?
        None
    }
}
