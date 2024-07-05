use crate::tap_error::Result;
use csv::StringRecord;
use tracing::debug;

#[derive(Debug, Default)]
pub(crate) struct TapHeaders {
    shape_id: Option<usize>,
    shape_label: Option<usize>,
    property_id: Option<usize>,
    property_label: Option<usize>,
    repeatable: Option<usize>,
    mandatory: Option<usize>,
    value_datatype: Option<usize>,
    value_shape: Option<usize>,
}

impl TapHeaders {
    pub(crate) fn new() -> TapHeaders {
        TapHeaders::default()
    }

    pub(crate) fn from_record(record: &StringRecord) -> Result<TapHeaders> {
        let mut shape_id = None;
        let mut shape_label = None;
        let mut property_id = None;
        let mut property_label = None;
        let mut repeatable = None;
        let mut mandatory = None;
        let mut value_datatype = None;
        let mut value_shape = None;

        for (idx, field) in record.iter().enumerate() {
            match clean(field).as_str() {
                "SHAPEID" => shape_id = Some(idx),
                "PROPERTYID" => property_id = Some(idx),
                "PROPERTYLABEL" => property_label = Some(idx),
                "SHAPELABEL" => shape_label = Some(idx),
                "REPEATABLE" => repeatable = Some(idx),
                "MANDATORY" => mandatory = Some(idx),
                "VALUEDATATYPE" => value_datatype = Some(idx),
                "VALUESHAPE" => value_shape = Some(idx),
                _ => {
                    debug!("Unknown field reading headers: {field}")
                }
            }
        }
        Ok(TapHeaders {
            shape_id,
            shape_label,
            property_id,
            property_label,
            repeatable,
            mandatory,
            value_datatype,
            value_shape,
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
    pub fn shape_label(&self, rcd: &StringRecord) -> Option<String> {
        self.shape_label.and_then(|idx| get_str_from_rcd(rcd, idx))
    }
    pub fn repeatable(&self, rcd: &StringRecord) -> Option<String> {
        self.repeatable.and_then(|idx| get_str_from_rcd(rcd, idx))
    }
    pub fn mandatory(&self, rcd: &StringRecord) -> Option<String> {
        self.mandatory.and_then(|idx| get_str_from_rcd(rcd, idx))
    }
    pub fn value_datatype(&self, rcd: &StringRecord) -> Option<String> {
        self.value_datatype
            .and_then(|idx| get_str_from_rcd(rcd, idx))
    }
    pub fn value_shape(&self, rcd: &StringRecord) -> Option<String> {
        self.value_shape.and_then(|idx| get_str_from_rcd(rcd, idx))
    }
}

fn clean(str: &str) -> String {
    str.to_uppercase()
}

fn get_str_from_rcd(rcd: &StringRecord, idx: usize) -> Option<String> {
    rcd.get(idx).map(|str| str.to_string())
}
