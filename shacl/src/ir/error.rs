use crate::ast::error::ASTError;
use crate::ir::ShapeLabelIdx;
use crate::rdf::error::ShaclParserError;
use prefixmap::IriRefError;
use rudof_rdf::rdf_core::term::Object;
use rudof_rdf::rdf_core::utils::RDFRegexError;
use rudof_rdf::rdf_core::{RDFError, Rdf, SHACLPath};
use rudof_rdf::rdf_impl::OxigraphInMemoryError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum IRError {
    #[error(transparent)]
    IriRef(#[from] Box<IriRefError>),

    #[error("Unable to find shape associated to idx {0}")]
    ShapeNotFound(ShapeLabelIdx),

    #[error("Failed to {operation}: {error}")]
    GraphError { operation: String, error: String },

    #[error(transparent)]
    ASTError(#[from] Box<ASTError>),

    #[error(transparent)]
    ShaclParserError(#[from] Box<ShaclParserError>),

    #[error(transparent)]
    OxigraphInMemoryError(#[from] Box<OxigraphInMemoryError>),

    #[error(
        "Invalid path for property shape with reifier shape {shape}, the path must be a single predicate, but got: {path}"
    )]
    InvalidReifierShapePath { shape: Box<Object>, path: Box<SHACLPath> },

    #[error(transparent)]
    RdfRegexError(#[from] Box<RDFRegexError>),

    #[error(transparent)]
    RDFError(#[from] Box<RDFError>),
}

impl IRError {
    pub fn from_rdf_err<RDF: Rdf>(op: &str, err: RDF::Err) -> Self {
        Self::GraphError {
            error: err.to_string(),
            operation: op.to_string(),
        }
    }

    pub fn add_triple<RDF: Rdf>(err: RDF::Err) -> Self {
        Self::from_rdf_err::<RDF>("add triple", err)
    }
}

impl From<IriRefError> for IRError {
    fn from(value: IriRefError) -> Self {
        Self::IriRef(Box::new(value))
    }
}

impl From<ASTError> for IRError {
    fn from(value: ASTError) -> Self {
        Self::ASTError(Box::new(value))
    }
}

impl From<ShaclParserError> for IRError {
    fn from(value: ShaclParserError) -> Self {
        Self::ShaclParserError(Box::new(value))
    }
}

impl From<OxigraphInMemoryError> for IRError {
    fn from(value: OxigraphInMemoryError) -> Self {
        Self::OxigraphInMemoryError(Box::new(value))
    }
}

impl From<RDFRegexError> for IRError {
    fn from(value: RDFRegexError) -> Self {
        Self::RdfRegexError(Box::new(value))
    }
}

impl From<RDFError> for IRError {
    fn from(value: RDFError) -> Self {
        Self::RDFError(Box::new(value))
    }
}
