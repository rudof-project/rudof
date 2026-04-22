//! Struct that converts DCTAP to ShEx schemas
//!
//!
use dctap::{DCTap, DatatypeId, ExtendsId, PropertyId, ShapeId, TapShape, TapStatement, Value, ValueConstraint};
use rudof_iri::IriS;
use rudof_iri::iri;
use prefixmap::IriRef;
use shex_ast::{
    Annotation, NodeConstraint, ObjectValue, Schema, Shape, ShapeDecl, ShapeExpr, ShapeExprLabel, TripleExpr,
    ValueSetValue,
};
use tracing::trace;

use crate::{Tap2ShExConfig, Tap2ShExError};
pub struct Tap2ShEx {
    config: Tap2ShExConfig,
}

impl Tap2ShEx {
    pub fn new(config: &Tap2ShExConfig) -> Self {
        Tap2ShEx { config: config.clone() }
    }

    // TODO: Added the following to make clippy happy...should we refactor Tap2ShExError ?
    #[allow(clippy::result_large_err)]
    pub fn convert(&self, tap: &DCTap) -> Result<Schema, Tap2ShExError> {
        let mut schema = Schema::new(&iri!("http://default/")).with_prefixmap(Some(self.config.prefixmap()));
        for tap_shape in tap.shapes() {
            let shape_decl = tapshape_to_shape(tap_shape, &self.config)?;
            schema.add_shape_decl(&shape_decl)
        }
        Ok(schema)
    }
}

// TODO: Added the following to make clippy happy...should we refactor Tap2ShExError ?
#[allow(clippy::result_large_err)]
fn tapshape_to_shape(tap_shape: &TapShape, config: &Tap2ShExConfig) -> Result<ShapeDecl, Tap2ShExError> {
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
fn _shape_id2shape_expr<'a>(shape_id: &'a ShapeId, config: &'a Tap2ShExConfig) -> Result<IriS, Tap2ShExError> {
    let iri = config.resolve_iri(shape_id.str(), shape_id.line())?;
    Ok(iri)
}

// TODO: Added the following to make clippy happy...should we refactor Tap2ShExError ?
#[allow(clippy::result_large_err)]
fn shape_id2iri<'a>(shape_id: &'a ShapeId, config: &'a Tap2ShExConfig) -> Result<IriS, Tap2ShExError> {
    let iri = config.resolve_iri(shape_id.str(), shape_id.line())?;
    Ok(iri)
}

// TODO: Added the following to make clippy happy...should we refactor Tap2ShExError ?
#[allow(clippy::result_large_err)]
fn extends_id2iri<'a>(extends_id: &'a ExtendsId, config: &'a Tap2ShExConfig) -> Result<IriS, Tap2ShExError> {
    let iri = config.resolve_iri(extends_id.str(), extends_id.line())?;
    Ok(iri)
}

// TODO: Added the following to make clippy happy...should we refactor Tap2ShExError ?
#[allow(clippy::result_large_err)]
fn tapshape_to_shape_expr(tap_shape: &TapShape, config: &Tap2ShExConfig) -> Result<ShapeExpr, Tap2ShExError> {
    let mut tes = Vec::new();
    for statement in tap_shape.statements() {
        let te = statement_to_triple_expr(statement, config)?;
        tes.push(te)
    }
    let mut shape = if tes.is_empty() {
        Shape::new(None, None, None)
    } else {
        let te = TripleExpr::each_of(tes);
        Shape::new(None, None, Some(te))
    };
    if tap_shape.has_extends() {
        for e in tap_shape.extends() {
            let iri = extends_id2iri(e, config)?;
            shape.add_extend(ShapeExprLabel::iri(iri))
        }
    }
    Ok(ShapeExpr::shape(shape))
}

// TODO: Added the following to make clippy happy...should we refactor Tap2ShExError ?
#[allow(clippy::result_large_err)]
fn statement_to_triple_expr(statement: &TapStatement, config: &Tap2ShExConfig) -> Result<TripleExpr, Tap2ShExError> {
    let pred = property_id2iri(&statement.property_id(), config)?;
    let min = get_min(statement.mandatory());
    let max = get_max(statement.repeatable());

    let value_expr = statement_to_value_expr(statement, config)?;
    let mut te = TripleExpr::triple_constraint(None, None, IriRef::Iri(pred), value_expr, min, max);
    if let Some(label) = statement.property_label() {
        te.add_annotation(Annotation::rdfs_label(label))
    }
    Ok(te)
}

#[allow(clippy::result_large_err)]
fn statement_to_value_expr(
    statement: &TapStatement,
    config: &Tap2ShExConfig,
) -> Result<Option<ShapeExpr>, Tap2ShExError> {
    let mut se: Option<ShapeExpr> = None;
    parse_node_constraint(statement, &mut se, config)?;
    parse_shape_ref(statement, &mut se, config)?;
    Ok(se)
}

#[allow(clippy::result_large_err)]
fn parse_shape_ref(
    statement: &TapStatement,
    shape_expr: &mut Option<ShapeExpr>,
    config: &Tap2ShExConfig,
) -> Result<(), Tap2ShExError> {
    if let Some(shape_refs) = statement.value_shape() {
        let line_no = shape_refs.line();
        let mut shapes = Vec::new();
        for s in shape_refs.str().split_whitespace() {
            if s == config.tap_config().value_shape_delimiter() {
                continue;
            }
            let sid = ShapeId::new(s, line_no);
            let iri = parse_single_shape_ref(&sid, config)?;
            shapes.push(ShapeExpr::iri_ref(iri));
        }
        match &shapes[..] {
            [] => return Ok(()),
            [shape] => {
                *shape_expr = match shape_expr {
                    Some(se) => Some(ShapeExpr::and(vec![se.clone(), shape.clone()])),
                    None => Some(shape.clone()),
                };
            },
            _ => {
                let shape_ref_expr = ShapeExpr::or(shapes);
                *shape_expr = match shape_expr {
                    Some(se) => Some(ShapeExpr::and(vec![se.clone(), shape_ref_expr])),
                    None => Some(shape_ref_expr),
                };
            },
        }
    }
    Ok(())
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
fn datatype_id2iri<'a>(datatype_id: &'a DatatypeId, config: &'a Tap2ShExConfig) -> Result<IriS, Tap2ShExError> {
    let iri = config.resolve_iri(datatype_id.str(), datatype_id.line())?;
    Ok(iri)
}

// TODO: Added the following to make clippy happy...should we refactor Tap2ShExError ?
#[allow(clippy::result_large_err)]
fn property_id2iri<'a>(property_id: &'a PropertyId, config: &'a Tap2ShExConfig) -> Result<IriS, Tap2ShExError> {
    let iri = config.resolve_iri(property_id.str(), property_id.line())?;
    Ok(iri)
}

#[allow(clippy::result_large_err)]
fn parse_node_constraint(
    statement: &TapStatement,
    shape_expr: &mut Option<ShapeExpr>,
    config: &Tap2ShExConfig,
) -> Result<(), Tap2ShExError> {
    match statement.value_datatype() {
        [] => {
            if let Some(constraint) = statement.value_constraint() {
                let mut nc = NodeConstraint::new();
                parse_constraint(constraint, config, &mut nc, statement.source_line_number())?;
                *shape_expr = Some(ShapeExpr::node_constraint(nc));
            }
        },
        [datatype] => {
            let mut nc = NodeConstraint::new();
            let iri = datatype_id2iri(datatype, config)?;
            nc.add_datatype(IriRef::iri(iri));
            if let Some(constraint) = statement.value_constraint() {
                parse_constraint(constraint, config, &mut nc, statement.source_line_number())?;
            }
        },
        datatypes => {
            let mut shape_exprs = Vec::new();
            for datatype in datatypes {
                let mut nc = NodeConstraint::new();
                let iri = datatype_id2iri(datatype, config)?;
                nc.add_datatype(IriRef::iri(iri));
                if let Some(constraint) = statement.value_constraint() {
                    parse_constraint(constraint, config, &mut nc, statement.source_line_number())?;
                }
                shape_exprs.push(ShapeExpr::node_constraint(nc));
            }
            *shape_expr = Some(ShapeExpr::or(shape_exprs));
        },
    }
    Ok(())
}

#[allow(clippy::result_large_err)]
fn parse_constraint(
    constraint: &ValueConstraint,
    config: &Tap2ShExConfig,
    node_constraint: &mut NodeConstraint,
    line: u64,
) -> Result<(), Tap2ShExError> {
    match constraint {
        ValueConstraint::PickList(values) => {
            let mut value_set_values: Vec<ValueSetValue> = Vec::new();
            for v in values {
                trace!("Parsing value set value: {v}");
                let value_set_value = parse_value_set_value(v, config, line)?;
                value_set_values.push(value_set_value)
            }
            node_constraint.add_values(value_set_values);
            Ok(())
        },
        _ => Err(Tap2ShExError::NotImplemented {
            msg: format!("ValueConstraint: {constraint:?}"),
        }),
    }
}

#[allow(clippy::result_large_err)]
fn parse_value_set_value(value: &Value, config: &Tap2ShExConfig, line: u64) -> Result<ValueSetValue, Tap2ShExError> {
    match value {
        Value::Str(str) => {
            let iri = config.resolve_iri(str, line)?;
            Ok(ValueSetValue::ObjectValue(ObjectValue::IriRef(IriRef::iri(iri))))
        },
        Value::Iri(iri) => Ok(ValueSetValue::ObjectValue(ObjectValue::IriRef(IriRef::iri(
            iri.clone(),
        )))),
    }
}

#[allow(clippy::result_large_err)]
fn parse_single_shape_ref(s: &ShapeId, config: &Tap2ShExConfig) -> Result<IriRef, Tap2ShExError> {
    let iri = shape_id2iri(s, config).map_err(|e| Tap2ShExError::ParsingValueShape {
        line: 0,
        error: Box::new(e),
    })?;
    Ok(IriRef::iri(iri))
}

/*fn parse_shape_ref(statement: &TapStatement, config: &Tap2ShExConfig) -> Result<Option<Vec<IriRef>>, Tap2ShExError> {
    if let Some(shape_ids) = statement.value_shape() {
        let line_no = shape_ids.line();
        let mut shapes = Vec::new();
        for s in shape_ids.str().split_whitespace() {
            if s == config.tap_config().value_shape_delimiter() {
                continue;
            }
            let sid = ShapeId::new(s, line_no);
            let iri = parse_single_shape_ref(&sid, config)?;
            shapes.push(iri);
        }
        Ok(Some(shapes))
    } else {
        Ok(None)
    }
}*/
