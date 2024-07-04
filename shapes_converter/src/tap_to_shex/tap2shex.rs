//! Struct that converts DCTAP to ShEx schemas
//!
//!

use dctap::{DCTap, PropertyId, ShapeId, TapShape, TapStatement};
use iri_s::IriS;
use prefixmap::IriRef;
use shex_ast::{Schema, Shape, ShapeDecl, ShapeExpr, ShapeExprLabel, TripleExpr};

use crate::{Tap2ShExConfig, Tap2ShExError};
pub struct Tap2ShEx {
    config: Tap2ShExConfig,
}

impl Tap2ShEx {
    pub fn new(config: Tap2ShExConfig) -> Tap2ShEx {
        Tap2ShEx { config }
    }

    pub fn convert(&self, tap: &DCTap) -> Result<Schema, Tap2ShExError> {
        let mut schema = Schema::new();
        for tap_shape in tap.shapes() {
            let shape_decl = tapshape_to_shape(tap_shape, &self.config)?;
            schema.add_shape_decl(&shape_decl)
        }
        Ok(schema)
    }
}

fn tapshape_to_shape(
    tap_shape: &TapShape,
    config: &Tap2ShExConfig,
) -> Result<ShapeDecl, Tap2ShExError> {
    if let Some(shape_id) = tap_shape.shape_id() {
        let id = shape_id2iri(&shape_id, config)?;
        let label = ShapeExprLabel::iri(id);
        let shape_expr = tapshape_to_shape_expr(tap_shape, config)?;
        let shape = ShapeDecl::new(label, shape_expr, false);
        Ok(shape)
    } else {
        Err(Tap2ShExError::NoShapeId {
            tap_shape: tap_shape.clone(),
        })
    }
}

fn shape_id2iri<'a>(
    shape_id: &'a ShapeId,
    config: &'a Tap2ShExConfig,
) -> Result<IriS, Tap2ShExError> {
    let iri = match &config.base_iri {
        None => {
            todo!()
        }
        Some(base_iri) => base_iri.extend(shape_id.as_local_name().as_str())?,
    };
    Ok(iri.clone())
}

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

fn statement_to_triple_expr(
    statement: &TapStatement,
    config: &Tap2ShExConfig,
) -> Result<TripleExpr, Tap2ShExError> {
    let pred = property_id2iri(&statement.property_id(), config)?;
    let min = get_min(statement.mandatory(), statement.repeatable());
    let max = get_max(statement.mandatory(), statement.repeatable());
    Ok(TripleExpr::triple_constraint(
        None,
        None,
        IriRef::Iri(pred),
        None,
        min,
        max,
    ))
}

fn get_min(mandatory: Option<bool>, repeatable: Option<bool>) -> Option<i32> {
    match (mandatory, repeatable) {
        (Some(true), _) => Some(1),
        (Some(false), _) => Some(0),
        _ => None,
    }
}

fn get_max(mandatory: Option<bool>, repeatable: Option<bool>) -> Option<i32> {
    match (mandatory, repeatable) {
        (_, Some(false)) => Some(1),
        (_, Some(true)) => Some(-1),
        _ => None,
    }
}

fn property_id2iri<'a>(
    property_id: &'a PropertyId,
    config: &'a Tap2ShExConfig,
) -> Result<IriS, Tap2ShExError> {
    let iri = match &config.base_iri {
        None => {
            todo!()
        }
        Some(base_iri) => base_iri.extend(property_id.as_local_name().as_str())?,
    };
    Ok(iri.clone())
}
