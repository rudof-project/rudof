use std::io;

use prefixmap::IriRef;
use shex_ast::{Schema, SchemaJsonError, ShapeExprLabel};
use thiserror::Error;

use super::{HtmlShape, Name, NodeId};

#[derive(Error, Debug)]
pub enum ShEx2HtmlError {
    #[error("Shape {iri} not found in schema {schema:?}")]
    ShapeNotFound { iri: IriRef, schema: Schema },

    #[error("No local ref for shape name: {name:?}")]
    NoLocalRefName { name: Name },

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

    #[error(transparent)]
    MiniNinjaError {
        #[from]
        err: minijinja::Error,
    },

    #[error(transparent)]
    IOError {
        #[from]
        err: std::io::Error,
    },

    #[error("Error creating landing page at: {name}, error: {error}")]
    ErrorCreatingLandingPage { name: String, error: io::Error },

    #[error("Error creating shapes file at: {name}, error: {error}")]
    ErrorCreatingShapesFile { name: String, error: io::Error },

    #[error("Wrong cardinality: ({min},{max})")]
    WrongCardinality { min: i32, max: i32 },

    #[error("NodeId {node_id} already contains shape: {shape:?}")]
    NodeIdHasShape {
        node_id: NodeId,
        shape: Box<HtmlShape>,
    },

    #[error("ShEx2Uml error: Feature not implemented: {msg}")]
    NotImplemented { msg: String },
}

impl ShEx2HtmlError {
    pub fn not_implemented(msg: &str) -> ShEx2HtmlError {
        ShEx2HtmlError::NotImplemented {
            msg: msg.to_string(),
        }
    }
}
