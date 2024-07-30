use thiserror::Error;

#[derive(Error, Debug)]
pub enum SPARQLError {}

#[derive(Error, Debug)]
pub enum SRDFError {
    #[error("Error during the SRDF operation")]
    SRDF,
}
