use std::io;

use prefixmap::IriRef;
use shex_ast::{Schema, SchemaJsonError, ShapeExprLabel};
use thiserror::Error;

use super::UmlError;

#[derive(Error, Debug)]
pub enum ShEx2UmlError {
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

    #[error(transparent)]
    UmlError {
        #[from]
        err: UmlError,
    },

    #[error("Couldn't create temporary file to generate PlantUML content")]
    TempFileError { err: io::Error },

    #[error("Wrong cardinality: ({min},{max})")]
    WrongCardinality { min: i32, max: i32 },

    #[error("Not found environment variable: {env_name}, which should point to the folder where the external tool PlantUML is located")]
    NoPlantUMLPath { env_name: String },

    #[error("Error launching command: {command:?}\nError: {error} ")]
    PlantUMLCommandError { command: String, error: io::Error },

    #[error("Can't open generated temporary file used from PlantUML. Temporary file name: {generated_name}, error: {error:?}")]
    CantOpenGeneratedTempFile {
        generated_name: String,
        error: io::Error,
    },

    #[error("Can't create temporary file for UML content. Temporary file name: {tempfile_name}, error: {error:?}")]
    CreatingTempUMLFile {
        tempfile_name: String,
        error: io::Error,
    },

    #[error("Can't copy temporary output file to writer: {temp_name}, error: {error:?}")]
    CopyingTempFile { temp_name: String, error: io::Error },

    #[error("ShEx2Uml error: Feature not implemented: {msg}")]
    NotImplemented { msg: String },
}

impl ShEx2UmlError {
    pub fn not_implemented(msg: &str) -> ShEx2UmlError {
        ShEx2UmlError::NotImplemented {
            msg: msg.to_string(),
        }
    }
}
