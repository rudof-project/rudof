use oxiri::IriParseError;
use srdf::SRDFGraphError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SPARQLError {}

#[derive(Error, Debug)]
pub enum SRDFError {
    #[error("Error during the SRDF operation")]
    SRDF,
    #[error("Error parsing the IRI")]
    IriParse(#[from] IriParseError),
    #[error("Error during the creation of the SRDFGraph")]
    SRDFGraph(#[from] SRDFGraphError),
}
