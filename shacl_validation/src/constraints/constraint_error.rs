use thiserror::Error;

use crate::helper::helper_error::SPARQLError;

#[allow(clippy::upper_case_acronyms)]
#[derive(Error, Debug)]
pub enum ConstraintError {
    #[error("Error during the SPARQL operation")]
    SPARQL(#[from] SPARQLError),
    #[error("Not yet implemented Constraint")]
    NotImplemented,
}
