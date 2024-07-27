use thiserror::Error;

#[derive(Error, Debug)]
pub enum SPARQLError {
    #[error("Error during the creation of the IRI")]
    NoTripleFound,
}
