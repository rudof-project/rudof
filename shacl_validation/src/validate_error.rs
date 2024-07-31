use oxiri::IriParseError;
use srdf::SRDFGraphError;
use thiserror::Error;

use crate::constraints::constraint_error::ConstraintError;
use crate::helper::helper_error::{SPARQLError, SRDFError};

#[derive(Error, Debug)]
pub enum ValidateError {
    #[error("Error during the SPARQL operation")]
    SPARQL(#[from] SPARQLError),
    #[error("TargetNode cannot be a Blank Node")]
    TargetNodeBlankNode,
    #[error("TargetClass cannot be a Blank Node")]
    TargetClassBlankNode,
    #[error("TargetClass cannot be a Literal")]
    TargetClassLiteral,
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
}
