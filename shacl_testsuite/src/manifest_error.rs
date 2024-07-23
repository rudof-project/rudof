use oxiri::IriParseError;
use shacl_validation::validation_report::validation_report_error::ValidationReportError;
use srdf::SRDFGraphError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ManifestError {
    #[error("Error during the creation of the IRI")]
    Iri(#[from] IriParseError),
    #[error("Error during the creation of the graph")]
    Graph(#[from] SRDFGraphError),
    #[error("Error parsing the Validation Report")]
    ParsingValidationReport(#[from] ValidationReportError),
}
