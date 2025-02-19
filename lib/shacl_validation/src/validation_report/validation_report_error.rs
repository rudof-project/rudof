use thiserror::Error;

#[derive(Error, Debug)]
pub enum ReportError {
    #[error("Error parsing the ValidationReport")]
    Srdf,

    #[error(transparent)]
    Result(#[from] ResultError),
}

#[derive(Error, Debug)]
pub enum ResultError {
    #[error("Error parsing the ValidationResult")]
    Srdf(),

    #[error("Error parsing the ValidationResult<R>, the {} field is missing", _0)]
    MissingRequiredField(&'static str),
}
