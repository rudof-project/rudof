use std::result;

use thiserror::Error;

pub type Result<T> = result::Result<T, TapError>;

#[derive(Error, Debug)]
pub enum TapError {
    #[error("CSV Error: {err}")]
    RDFParseError {
        #[from]
        err: csv::Error,
    },
}
