use srdf::oxgraph_error::GraphParseError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServiceDescriptionError {
    #[error(transparent)]
    GraphParse {
        #[from]
        err: GraphParseError,
    },

    #[error(transparent)]
    RdfParse {
        #[from]
        err: srdf::RdfParseError,
    },

    #[error("Expected IRI as value for property: {property} but got {term}")]
    ExpectedIRIAsValueForProperty { property: String, term: String },
}
