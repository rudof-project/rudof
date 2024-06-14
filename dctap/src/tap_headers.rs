use crate::tap_error::Result;
use csv::StringRecord;

#[derive(Debug, Default)]
pub(crate) struct TapHeaders {
    shape_id: Option<usize>,
}

impl TapHeaders {
    pub(crate) fn new() -> TapHeaders {
        TapHeaders::default()
    }

    pub(crate) fn from_record(record: &StringRecord) -> Result<TapHeaders> {
        let mut shape_id = None;

        for (idx, field) in record.iter().enumerate() {
            if clean(field) == "SHAPEID" {
                shape_id = Some(idx);
            }
        }
        Ok(TapHeaders { shape_id })
    }

    pub fn shape_id(&self) -> Option<usize> {
        self.shape_id
    }
}

fn clean(str: &str) -> String {
    str.to_uppercase()
}
