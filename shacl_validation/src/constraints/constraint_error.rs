use thiserror::Error;

use crate::helper::helper_error::{SPARQLError, SRDFError};

#[allow(clippy::upper_case_acronyms)]
#[derive(Error, Debug)]
pub enum ConstraintError {
    #[error("Error during the SPARQL operation")]
    SPARQL(#[from] SPARQLError),
    #[error("Not yet implemented Constraint")]
    NotImplemented,
    #[error("Error creating the constriant")]
    Create,
    #[error("Error during some of the query operations")]
    Query,
    #[error("Error Shape not found")]
    ShapeNotFound,
    #[error("Error the class has not been defined")]
    ClassNotDefined,
    #[error("Error during some SRDF operation")]
    SRDF(#[from] SRDFError),
    #[error("Error the shape could not be found")]
    MissingShape,
}
