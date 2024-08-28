use prefixmap::IriRef;
use shex_ast::{Schema, SchemaJsonError, ShapeExprLabel};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ShEx2SparqlError {
    #[error("Shape {iri} not found in schema {schema:?}")]
    ShapeNotFound { iri: IriRef, schema: Schema },

    #[error("Shape reference {sref} not found in schema {schema:?}")]
    ShapeRefNotFound {
        sref: ShapeExprLabel,
        schema: Schema,
    },

    #[error("No shapes found in schema to convert to SPARQL. Schema\n{schema:?}")]
    NoShapes { schema: Schema },

    #[error(
        "No shape found to convert to SPARQL because list of shapes is empty. Schema\n{schema:?}"
    )]
    EmptyShapes { schema: Schema },

    #[error(transparent)]
    SchemaError {
        #[from]
        err: SchemaJsonError,
    },

    #[error("ShEx2Sparql: Feature not implemented: {msg}")]
    NotImplemented { msg: String },
}

impl ShEx2SparqlError {
    pub fn not_implemented(msg: &str) -> ShEx2SparqlError {
        ShEx2SparqlError::NotImplemented {
            msg: msg.to_string(),
        }
    }
}
