use std::fmt::Display;

use iri_s::IriS;
use rbe::{rbe::Rbe, Component, RbeError};
use shex_ast::{CompiledSchemaError, ShapeLabel, ShapeLabelIdx};
use srdf::Object;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ValidatorError {
    #[error("SRDF Error: {error}")]
    SRDFError { error: String },

    #[error("Not found shape label {shape}")]
    NotFoundShapeLabel { shape: ShapeLabel },

    #[error("Error converting object to iri: {object}")]
    ConversionObjectIri { object: Object },

    #[error(transparent)]
    CompiledSchemaError(#[from] CompiledSchemaError),


    #[error("Failed regular expression")]
    RbeFailed(),

    #[error(transparent)]
    RbeError(#[from] RbeError<IriS, Object, ShapeLabelIdx>),

}

