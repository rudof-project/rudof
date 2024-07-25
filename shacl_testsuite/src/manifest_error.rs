use oxigraph::store::{LoaderError, StorageError};
use oxiri::IriParseError;
use shacl_validation::validation_report::validation_report_error::ValidationReportError;
use srdf::SRDFGraphError;
use std::io::Error;
use thiserror::Error;

use crate::helper::helper_error::SPARQLError;

#[derive(Error, Debug)]
pub enum ManifestError {
    #[error("Error during the creation of the IRI")]
    Iri(#[from] IriParseError),
    #[error("Error during the creation of the graph")]
    Graph(#[from] SRDFGraphError),
    #[error("Error parsing the Validation Report")]
    ParsingValidationReport(#[from] ValidationReportError),
    #[error("Error parsing the SHACL Shape")]
    ShaclParser,
    #[error("Error performing the SPARQL operation")]
    SPARQL(#[from] SPARQLError),
    #[error("Error creating the SPARQL Store")]
    Storage(#[from] StorageError),
    #[error("Error during the I/O operations")]
    IO(#[from] Error),
    #[error("Error loading the triples")]
    Loader(#[from] LoaderError),
}
