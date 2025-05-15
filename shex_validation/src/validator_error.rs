use std::fmt::Display;

use prefixmap::PrefixMapError;
use rbe::RbeError;
use serde::Serialize;
use shex_ast::ir::preds::Preds;
use shex_ast::ir::shape_expr::ShapeExpr;
use shex_ast::{ir::shape_label::ShapeLabel, Node, Pred, ShapeExprLabel, ShapeLabelIdx};
use srdf::Object;
use thiserror::Error;

use crate::Reasons;

#[derive(Error, Debug, Clone)]
pub enum ValidatorError {
    #[error("Negation cycle error: {neg_cycles:?}")]
    NegCycleError {
        neg_cycles: Vec<Vec<(String, String, Vec<String>)>>,
    },

    #[error("SRDF Error: {error}")]
    SRDFError { error: String },

    #[error("Not found shape label {shape}")]
    NotFoundShapeLabel { shape: ShapeLabel },

    #[error("Not found shape label with index {idx}")]
    NotFoundShapeLabelWithIndex { idx: ShapeLabelIdx },

    #[error("Error converting object to iri: {object}")]
    ConversionObjectIri { object: Object },

    #[error("Compiling schema: {error}")]
    SchemaIRError { error: String },

    #[error("Shapemap error: {error}")]
    ShapeMapError { error: String },

    #[error("Failed regular expression")]
    RbeFailed(),

    #[error("Closed shape but found properties {remainder:?} which are not part of shape declared properties: {declared:?}")]
    ClosedShapeWithRemainderPreds { remainder: Preds, declared: Preds },

    #[error(transparent)]
    RbeError(#[from] RbeError<Pred, Node, ShapeLabelIdx>),

    #[error(transparent)]
    PrefixMapError(#[from] PrefixMapError),

    #[error("ShapeLabel not found {shape_label:?}: {error}")]
    ShapeLabelNotFoundError {
        shape_label: ShapeExprLabel,
        error: String,
    },

    #[error("And error: shape expression {shape_expr} failed for node {node}: {errors}")]
    ShapeAndError {
        shape_expr: ShapeExpr,
        node: Node,
        errors: ValidatorErrors,
    },

    #[error("OR error: shape expression {shape_expr} failed for node {node}: all branches failed")]
    ShapeOrError {
        shape_expr: ShapeExpr,
        node: Node,
        errors: Vec<(ShapeExpr, ValidatorErrors)>,
    },

    #[error(
        "Shape Not error: failed for node {node} because it passed {shape_expr} with {reasons}"
    )]
    ShapeNotError {
        shape_expr: ShapeExpr,
        node: Node,
        reasons: Reasons,
    },

    #[error("Error reading config file from path {path}: {error}")]
    ValidatorConfigFromPathError { path: String, error: String },

    #[error("Error reading config file from path {path}: {error}")]
    ValidatorConfigTomlError { path: String, error: String },

    #[error("Adding non conformant {node}@{label} error: {error}")]
    AddingNonConformantError {
        node: String,
        label: String,
        error: String,
    },

    #[error("Adding conformant {node}@{label} error: {error}")]
    AddingConformantError {
        node: String,
        label: String,
        error: String,
    },

    #[error("Adding pending {node}@{label} error: {error}")]
    AddingPendingError {
        node: String,
        label: String,
        error: String,
    },
}

#[derive(Debug, Clone)]
pub struct ValidatorErrors {
    errs: Vec<ValidatorError>,
}

impl ValidatorErrors {
    pub fn new(errs: Vec<ValidatorError>) -> ValidatorErrors {
        ValidatorErrors { errs }
    }
}

impl Display for ValidatorErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for err in self.errs.iter() {
            writeln!(f, "  {err}")?;
        }
        Ok(())
    }
}
