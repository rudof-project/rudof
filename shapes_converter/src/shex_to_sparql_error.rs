use iri_s::IriS;
use shex_ast::{Schema, SchemaJsonError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ShEx2SparqlError {
    #[error("Shape {iri} not found in schema {schema:?}")]
    ShapeNotFound { iri: IriS, schema: Schema },

    #[error(transparent)]
    SchemaError {
        #[from]
        err: SchemaJsonError,
    },
}
