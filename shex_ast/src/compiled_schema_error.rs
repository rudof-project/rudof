use iri_s::{IriSError, IriError};
use thiserror::Error;

use crate::schema_json;

#[derive(Error, Debug)]
pub enum CompiledSchemaError {
   
    #[error("Parsing {str:?} as IRI")]
    Str2IriError { str: String },

    #[error("Parsing as IRI: {err:?}")]
    IriParseError { #[from] err: IriSError },

    #[error("SchemaJson Error")]
    SchemaJsonError(#[from] schema_json::SchemaJsonError)
}
