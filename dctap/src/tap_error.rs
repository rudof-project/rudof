use std::{io, result};

use calamine::{XlsError, XlsxError};
use csv::{Position, StringRecord};
use thiserror::Error;

pub type Result<T> = result::Result<T, TapError>;

#[derive(Error, Debug)]
pub enum TapError {
    #[error("Empty node type at line: {}, Record: {}", pos.line(), pos.record())]
    EmptyNodeType { pos: Position },

    #[error("Unexpected node type: {str}. Line: {}, Record: {}", pos.line(), pos.record())]
    UnexpectedNodeType { str: String, pos: Position },

    #[error("Unexpected value constraint type: {value}. Line: {}, Record: {}", pos.line(), pos.record())]
    UnexpectedValueConstraintType { value: String, pos: Position },

    #[error("CSV Error: {err}")]
    CSVError {
        #[from]
        err: csv::Error,
    },

    #[error("Cannot obtain shape id with index {shape_id} from record {record:?}")]
    NoShapeId {
        shape_id: usize,
        record: StringRecord,
    },

    #[error(
        "Value of field {field} is {value} and should be boolean. Line: {}. Record: {}",
        pos.line(),
        pos.record()
    )]
    ShouldBeBoolean {
        field: String,
        value: String,
        pos: Position,
    },

    #[error("Error reading config file from path {path}: {error}")]
    TapConfigFromPathError { path: String, error: io::Error },

    #[error("Error reading config file from path {path}: {error}")]
    TapConfigYamlError {
        path: String,
        error: serde_yml::Error,
    },

    #[error("Reading Excel file from {path}: {error}")]
    ReadingExcelError { path: String, error: io::Error },

    #[error(transparent)]
    XlsxError {
        #[from]
        error: XlsxError,
    },

    #[error("No headers found in Excel file: {path}")]
    NoHeadersExcel { path: String },
}
