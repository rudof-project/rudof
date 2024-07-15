use std::result;

use csv::StringRecord;
use thiserror::Error;

pub type Result<T> = result::Result<T, TapError>;

#[derive(Error, Debug)]
pub enum TapError {
    #[error("Empty node type")]
    EmptyNodeType,

    #[error("Unexpected node type: {str}")]
    UnexpectedNodeType { str: String },

    #[error("Unexpected value constraint type: {value}")]
    UnexpectedValueConstraintType { value: String },

    #[error("CSV Error: {err}")]
    RDFParseError {
        #[from]
        err: csv::Error,
    },

    #[error("Cannot obtain shape id with index {shape_id} from record {record:?}")]
    NoShapeId {
        shape_id: usize,
        record: StringRecord,
    },

    #[error("Value of field {field} is {value} and should be boolean")]
    ShouldBeBoolean { field: String, value: String },
}
