use oxiri::IriParseError;
use srdf::SRDFGraphError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SPARQLError {
    #[error("Query could not be performed")]
    Query,
}

#[derive(Error, Debug)]
pub enum SRDFError {
    #[error("Error during the SRDF operation")]
    Srdf,
    #[error("Error parsing the IRI")]
    IriParse(#[from] IriParseError),
    #[error("Error during the creation of the SRDFGraph")]
    SRDFGraph(#[from] SRDFGraphError),
}
