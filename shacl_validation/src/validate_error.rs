use oxiri::IriParseError;
use prefixmap::Underef;
use srdf::SRDFGraphError;
use thiserror::Error;

use crate::constraints::constraint_error::ConstraintError;
use crate::helper::helper_error::{SPARQLError, SRDFError};

#[derive(Error, Debug)]
pub enum ValidateError {
    #[error("Error during the SPARQL operation")]
    SRDF,
    #[error("TargetNode cannot be a Blank Node")]
    TargetNodeBlankNode,
    #[error("TargetClass should be an IRI")]
    TargetClassNotIri,
    #[error("Error during the creation of the SRDFGraph")]
    SRDFGraph(#[from] SRDFGraphError),
    #[error("Error during the creation of the Shacl shapes")]
    ShaclParser,
    #[error("Error during the constraint evaluation")]
    Constraint(#[from] ConstraintError),
    #[error("Error parsing the IRI")]
    IriParse(#[from] IriParseError),
    #[error("Error during some I/O operation")]
    IO(#[from] std::io::Error),
    #[error("Error loading the Shapes")]
    Shapes(#[from] SRDFError),
    #[error("Error creating the SPARQL endpoint")]
    SPARQLCreation,
    #[error("Error creating the Graph in-memory")]
    GraphCreation,
    #[error("Error obtaining the underlying IRI")]
    Underef(#[from] Underef),
    #[error("The provided mode is not supported for the data structure")]
    UnsupportedMode,
    #[error("Error during the SPARQL operation")]
    Sparql(#[from] SPARQLError),
}
