use thiserror::Error;

use crate::helper::helper_error::SPARQLError;

#[derive(Error, Debug)]
pub enum ConstraintError {
    #[error("Error during the SPARQL operation")]
    SPARQL(#[from] SPARQLError),
}
