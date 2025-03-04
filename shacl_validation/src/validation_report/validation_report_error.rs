use thiserror::Error;

#[derive(Error, Debug)]
pub enum ReportError {
    #[error("Error parsing the Subject, expected a Subject but found a Term")]
    ExpectedSubject,

    #[error("Error querying the store when parsing the ValidationReport")]
    Query,

    #[error(transparent)]
    Result(#[from] ResultError),
}

#[derive(Error, Debug)]
pub enum ResultError {
    #[error("Error parsing the Subject, expected a Subject but found a Term")]
    ExpectedSubject,

    #[error("Error querying the store when parsing the ValidationResult")]
    Query,

    #[error("Error parsing the ValidationResult, the required  field {_0} is missing")]
    MissingField(&'static str),
}
