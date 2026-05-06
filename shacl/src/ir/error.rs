use crate::ast::error::ASTError;
use crate::rdf::error::ShaclParserError;
use rudof_rdf::rdf_core::term::Object;
use rudof_rdf::rdf_core::utils::RDFRegexError;
use rudof_rdf::rdf_impl::InMemoryGraphError;
use thiserror::Error;
use prefixmap::IriRefError;
use rudof_rdf::rdf_core::{Rdf, SHACLPath};
use crate::ir::ShapeLabelIdx;

#[derive(Error, Debug)]
pub enum IRError {
    #[error(transparent)]
    IriRef(#[from] Box<IriRefError>),

    #[error("Unable to find shape associated to idx {0}")]
    ShapeNotFound(ShapeLabelIdx),

    #[error("Failed to {operation}: {error}")]
    GraphError {
        operation: String,
        error: String,
    },

    #[error(transparent)]
    ASTError(#[from] Box<ASTError>),

    #[error(transparent)]
    ShaclParserError(#[from] Box<ShaclParserError>),

    #[error(transparent)]
    InMemoryGraphError(#[from] Box<InMemoryGraphError>),

    #[error("Invalid path for property shape with reifier shape {shape}, the path must be a single predicate, but got: {path}")]
    InvalidReifierShapePath {
        shape: Object,
        path: SHACLPath,
    },

    #[error(transparent)]
    RdfRegexError(#[from] Box<RDFRegexError>),
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

impl From<InMemoryGraphError> for IRError {
    fn from(value: InMemoryGraphError) -> Self {
        Self::InMemoryGraphError(Box::new(value))
    }
}

impl From<RDFRegexError> for IRError {
    fn from(value: RDFRegexError) -> Self {
        Self::RdfRegexError(Box::new(value))
    }
}