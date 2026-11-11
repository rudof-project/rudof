//! SHACL IR (Internal Representation)
//! Represents SHACL Internal representation which is used for validation

use crate::ir::error::IRError;
use crate::types::{Target, Value};
use prefixmap::IriRef;
use rudof_iri::IriS;
use rudof_rdf::rdf_core::term::Object;

mod component;
pub mod components;
pub mod dg;
pub(crate) mod error;
mod node_expr;
mod node_expr_register;
pub mod node_exprs;
mod node_shape;
mod order_value;
mod property_shape;
mod recursion;
mod reifier_info;
mod schema;
mod shape;
mod shape_label_idx;
mod target;
mod test;

pub use component::IRComponent;
pub use node_expr::IRNodeExpr;
pub use node_shape::IRNodeShape;
pub use order_value::OrderValue;
pub use property_shape::IRPropertyShape;
pub use recursion::RecursionSemantics;
pub use reifier_info::ReifierInfo;
use rudof_rdf::rdf_core::NeighsRDF;
use rudof_rdf::rdf_core::term::literal::Literal;
pub use schema::IRSchema;
pub use shape::IRShape;
pub use shape_label_idx::ShapeLabelIdx;

/// A target declaration in the IR. Related [`Target`](Target)
pub type IRTarget = Target<IRNodeExpr>;

/// Records that the shape at `from` refers to the one at `to`, and walks the latter's own
/// references the first time it is reached.
pub(crate) fn add_shape_edge(
    from: ShapeLabelIdx,
    to: ShapeLabelIdx,
    dg: &mut dg::DependencyGraph,
    posneg: dg::PosNeg,
    ir: &IRSchema,
    cache: &mut std::collections::HashSet<ShapeLabelIdx>,
) {
    let Some(shape) = ir.get_shape_from_idx(&to) else {
        return;
    };

    dg.add_edge(from, to, posneg);
    if cache.insert(to) {
        shape.add_edges(to, dg, posneg, ir, cache);
    }
}

fn convert_iri_ref(iri_ref: IriRef) -> Result<IriS, IRError> {
    let iri = iri_ref.get_iri()?;

    Ok(iri.clone())
}

fn convert_value(value: Value) -> Result<Object, IRError> {
    let out = match value {
        Value::Iri(iri_ref) => Object::Iri(convert_iri_ref(iri_ref)?),
        Value::Literal(lit) => Object::literal(lit),
    };

    Ok(out)
}

/// Whether a node expression's output nodes are exactly `( true )`
pub(crate) fn node_expr_is_true<Rdf: NeighsRDF>(values: &[Rdf::Term]) -> bool {
    match values {
        [value] => Rdf::term_as_literal(value)
            .ok()
            .and_then(|lit| lit.to_bool())
            .unwrap_or(false),
        _ => false,
    }
}
