use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConstraintError {
    #[error("{} constraint not yet implemented", ._0)]
    NotImplemented(String),
    #[error("{}", ._0)]
    Query(String),

    #[error("Expected IRI but found {term}")]
    ExpectedIri { term: String },
}
