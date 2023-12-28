use srdf::RDFParseError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ShaclParserError {
    #[error("RDF parse error: {err}")]
    RDFParseError {
        #[from]
        err: RDFParseError,
    },

    #[error("Expected RDFNode parsing node shape, found: {term}")]
    ExpectedRDFNodeNodeShape{ term: String }

}
