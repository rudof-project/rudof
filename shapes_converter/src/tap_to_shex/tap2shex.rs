//! Struct that converts DCTAP to ShEx schemas
//!
//!

use dctap::{DCTap, DatatypeId, PropertyId, ShapeId, TapShape, TapStatement};
use iri_s::IriS;
use prefixmap::IriRef;
use shex_ast::{
    Annotation, NodeConstraint, Schema, Shape, ShapeDecl, ShapeExpr, ShapeExprLabel, TripleExpr,
};

use crate::{Tap2ShExConfig, Tap2ShExError};
pub struct Tap2ShEx {
    config: Tap2ShExConfig,
}

impl Tap2ShEx {
    pub fn new(config: &Tap2ShExConfig) -> Tap2ShEx {
        Tap2ShEx {
            config: config.clone(),
        }
    }

    // TODO: Added the following to make clippy happy...should we refactor Tap2ShExError ?
    #[allow(clippy::result_large_err)]
    pub fn convert(&self, tap: &DCTap) -> Result<Schema, Tap2ShExError> {
        let mut schema = Schema::new().with_prefixmap(Some(self.config.prefixmap()));
        for tap_shape in tap.shapes() {
            let shape_decl = tapshape_to_shape(tap_shape, &self.config)?;
            schema.add_shape_decl(&shape_decl)
        }
        Ok(schema)
    }
}

// TODO: Added the following to make clippy happy...should we refactor Tap2ShExError ?
#[allow(clippy::result_large_err)]
fn tapshape_to_shape(
    tap_shape: &TapShape,
    config: &Tap2ShExConfig,
) -> Result<ShapeDecl, Tap2ShExError> {
    if let Some(shape_id) = tap_shape.shape_id() {
        let id = shape_id2iri(&shape_id, config)?;
        let label = ShapeExprLabel::iri(id);
        let shape_expr = tapshape_to_shape_expr(tap_shape, config)?;
        let mut shape = ShapeDecl::new(label, shape_expr, false);
        if let Some(shape_label) = tap_shape.shape_label() {
            shape.add_annotation(Annotation::rdfs_label(shape_label.as_str()))
        }
        Ok(shape)
    } else {
        Err(Tap2ShExError::NoShapeId {
            tap_shape: tap_shape.clone(),
        })
    }
}

// TODO: Added the following to make clippy happy...should we refactor Tap2ShExError ?
#[allow(clippy::result_large_err)]
fn shape_id2iri<'a>(
    shape_id: &'a ShapeId,
    config: &'a Tap2ShExConfig,
) -> Result<IriS, Tap2ShExError> {
    let iri = config.resolve_iri(shape_id.str(), shape_id.line())?;
    Ok(iri)
}

// TODO: Added the following to make clippy happy...should we refactor Tap2ShExError ?
#[allow(clippy::result_large_err)]
fn tapshape_to_shape_expr(
    tap_shape: &TapShape,
    config: &Tap2ShExConfig,
) -> Result<ShapeExpr, Tap2ShExError> {
    let mut tes = Vec::new();
    for statement in tap_shape.statements() {
        let te = statement_to_triple_expr(statement, config)?;
        tes.push(te)
    }
    let shape = if tes.is_empty() {
        Shape::new(None, None, None)
    } else {
        let te = TripleExpr::each_of(tes);
        Shape::new(None, None, Some(te))
    };
    let se = ShapeExpr::shape(shape);
    Ok(se)
}

// TODO: Added the following to make clippy happy...should we refactor Tap2ShExError ?
#[allow(clippy::result_large_err)]
fn statement_to_triple_expr(
    statement: &TapStatement,
    config: &Tap2ShExConfig,
) -> Result<TripleExpr, Tap2ShExError> {
    let pred = property_id2iri(&statement.property_id(), config)?;
    let min = get_min(statement.mandatory());
    let max = get_max(statement.repeatable());
    let value_expr = match (statement.value_datatype(), statement.value_shape()) {
        (Some(datatype), None) => {
            let iri = datatype_id2iri(&datatype, config)?;
            Ok(Some(ShapeExpr::node_constraint(
                NodeConstraint::new().with_datatype(IriRef::iri(iri)),
            )))
        }
        (None, Some(shape_id)) => {
            let iri = shape_id2iri(&shape_id, config)?;
            Ok(Some(ShapeExpr::iri_ref(IriRef::iri(iri))))
        }
        (None, None) => Ok(None),
        (Some(datatype), Some(valueshape)) => Err(Tap2ShExError::MultipleValueExprInStatement {
            value_datatype: datatype.clone(),
            value_shape: valueshape.clone(),
        }),
    }?;
    let mut te = TripleExpr::triple_constraint(None, None, IriRef::Iri(pred), value_expr, min, max);
    if let Some(label) = statement.property_label() {
        te.add_annotation(Annotation::rdfs_label(label))
    }
    Ok(te)
}

fn get_min(mandatory: Option<bool>) -> Option<i32> {
    match mandatory {
        Some(true) => Some(1),
        Some(false) => Some(0),
        None => Some(1),
    }
}

fn get_max(repeatable: Option<bool>) -> Option<i32> {
    match repeatable {
        Some(false) => Some(1),
        Some(true) => Some(-1),
        None => None,
    }
}

// TODO: Added the following to make clippy happy...should we refactor Tap2ShExError ?
#[allow(clippy::result_large_err)]
fn datatype_id2iri<'a>(
    datatype_id: &'a DatatypeId,
    config: &'a Tap2ShExConfig,
) -> Result<IriS, Tap2ShExError> {
    let iri = config.resolve_iri(datatype_id.str(), datatype_id.line())?;
    Ok(iri)
}

// TODO: Added the following to make clippy happy...should we refactor Tap2ShExError ?
#[allow(clippy::result_large_err)]
fn property_id2iri<'a>(
    property_id: &'a PropertyId,
    config: &'a Tap2ShExConfig,
) -> Result<IriS, Tap2ShExError> {
    let iri = config.resolve_iri(property_id.str(), property_id.line())?;
    Ok(iri)
}
