//! Struct that converts DCTAP to ShEx schemas
//!
//!

use dctap::{DCTap, ShapeId, TapShape};
use iri_s::IriS;
use shex_ast::{Schema, ShapeDecl, ShapeExpr, ShapeExprLabel};

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
        let shape = ShapeDecl::new(label, ShapeExpr::any(), false);
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
