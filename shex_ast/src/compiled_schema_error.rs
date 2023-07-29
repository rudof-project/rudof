use iri_s::{IriSError, IriError};
use thiserror::Error;

use crate::{schema_json};
use crate::schema_json::TripleExprLabel;

#[derive(Error, Debug)]
pub enum CompiledSchemaError {
   
    #[error("Parsing {str:?} as IRI")]
    Str2IriError { str: String },

    #[error("Parsing as IRI: {err:?}")]
    IriParseError { 
        #[from]
        err: IriSError 
    },

    #[error("Obtaining IRI: {err:?}")]
    IriError { 
        #[from]
        err: IriError 
    },


    #[error("SchemaJson Error")]
    SchemaJsonError(#[from] schema_json::SchemaJsonError),

    #[error("Duplicated triple expression label in schema: {label:?}")]
    DuplicatedTripleExprLabel {
        label: TripleExprLabel
    }
}
