use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConstraintError {
    #[error("Error performing a query")]
    Srdf,

    #[error("Constraint not yet implemented: {}", ._0)]
    NotImplemented(String),

    #[error("Query error: {}", ._0)]
    Query(String),

    #[error("Expected IRI but found {term}")]
    ExpectedIri { term: String },
}
