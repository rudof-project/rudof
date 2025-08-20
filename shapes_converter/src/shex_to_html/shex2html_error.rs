use std::{
    io::{self, BufWriter, IntoInnerError},
    string::FromUtf8Error,
};

use prefixmap::{IriRef, PrefixMapError};
use shex_ast::{Schema, SchemaJsonError, ShapeExprLabel};
use srdf::UmlConverterError;
use thiserror::Error;

use crate::ShEx2UmlError;

use super::{HtmlShape, Name, NodeId};

#[derive(Error, Debug)]
pub enum ShEx2HtmlError {
    #[error("Shape {iri} not found in schema {schema:?}")]
    ShapeNotFound { iri: IriRef, schema: Box<Schema> },

    #[error(transparent)]
    UmlConverterError {
        #[from]
        err: UmlConverterError,
    },

    #[error("No local referece for shape name: {name:?}")]
    NoLocalRefName { name: Name },

    #[error("Shape reference {sref} not found in schema {schema:?}")]
    ShapeRefNotFound {
        sref: ShapeExprLabel,
        schema: Box<Schema>,
    },

    #[error("No shapes found in schema to convert to SPARQL. Schema\n{schema:?}")]
    NoShapes { schema: Box<Schema> },

    #[error(
        "No shape found to convert to SPARQL because list of shapes is empty. Schema\n{schema:?}"
    )]
    EmptyShapes { schema: Box<Schema> },

    #[error(transparent)]
    SchemaError {
        #[from]
        err: SchemaJsonError,
    },

    #[error(transparent)]
    PrefixMapError {
        #[from]
        err: PrefixMapError,
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

    #[error(transparent)]
    UTF8Error {
        #[from]
        err: FromUtf8Error,
    },

    #[error(transparent)]
    IntoInnerError {
        #[from]
        err: IntoInnerError<BufWriter<Vec<u8>>>,
    },

    #[error(transparent)]
    ShEx2UmlError {
        #[from]
        err: ShEx2UmlError,
    },

    #[error("Error creating landing page at: {name}, error: {error}")]
    ErrorCreatingLandingPage { name: String, error: io::Error },

    #[error("Error creating shapes file at: {name}, error: {error}")]
    ErrorCreatingShapesFile { name: String, error: io::Error },

    #[error("Wrong cardinality: ({min},{max})")]
    WrongCardinality { min: i32, max: i32 },

    #[error("Adding component: {component:?} to nodeId {node_id} fails because that node already contains shape: {shape:?}")]
    AddingComponentNodeIdHasShape {
        node_id: NodeId,
        shape: Box<HtmlShape>,
        component: Box<HtmlShape>,
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
