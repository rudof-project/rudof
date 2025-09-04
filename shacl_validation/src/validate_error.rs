// use oxiri::IriParseError;
use shacl_ir::compiled::compiled_shacl_error::CompiledShaclError;
use shacl_rdf::rdf_to_shacl::shacl_parser_error::ShaclParserError;
use sparql_service::RdfDataError;
use srdf::RDFParseError;
use srdf::SRDFGraphError;
use thiserror::Error;

use crate::constraints::constraint_error::ConstraintError;
use crate::helpers::helper_error::SPARQLError;
use crate::helpers::helper_error::SRDFError;

#[derive(Error, Debug)]
pub enum ValidateError {
    #[error("Error during the SPARQL operation")]
    SRDF,
    #[error("TargetNode cannot be a Blank Node")]
    TargetNodeBlankNode,
    #[error("TargetClass should be an IRI")]
    TargetClassNotIri,
    #[error("Error when working with the SRDFGraph, {}", ._0)] // TODO: move to store
    Graph(#[from] SRDFGraphError),
    #[error("Error when parsing the SHACL Graph, {}", ._0)] // TODO: move to store
    ShaclParser(#[from] ShaclParserError),
    #[error("Error during the constraint evaluation")]
    Constraint(#[from] ConstraintError),
    // #[error("Error parsing the IRI")]
    //IriParse(#[from] IriParseError),
    #[error("Error during some I/O operation")]
    IO(#[from] std::io::Error),
    #[error("Error loading the Shapes")]
    Shapes(#[from] RDFParseError),
    #[error("Error creating the SPARQL endpoint")]
    SPARQLCreation,
    #[error("Error during the SPARQL operation")]
    Sparql(#[from] SPARQLError),
    #[error("Implicit class not found")]
    ImplicitClassNotFound,
    #[error("The provided mode is not supported for the {} structure", ._0)]
    UnsupportedMode(String),
    #[error(transparent)]
    SrdfHelper(#[from] SRDFError),
    #[error("TargetClass error: {msg}")]
    TargetClassError { msg: String },
    #[error("Error during the compilation of the Schema, {}", ._0)] // TODO: move to store
    CompiledShacl(#[from] CompiledShaclError),
    #[error("Not yet implemented: {msg}")]
    NotImplemented { msg: String },
    #[error(transparent)]
    RdfDataError(#[from] RdfDataError),
    #[error(
        "Error obtaining triples with subject {subject} during validation: {error}, checking CLOSED"
    )]
    TriplesWithSubject { subject: String, error: String },
}
