//! SHACL IR (Internal Representation)
//! Represents SHACL Internal representation which is used for validation

use crate::ir::error::IRError;
use crate::types::Value;
use iri_s::IriS;
use prefixmap::IriRef;
use rudof_rdf::rdf_core::term::Object;

mod component;
pub mod components;
pub(crate) mod error;
mod node_shape;
mod property_shape;
mod reifier_info;
mod schema;
mod shape;
mod shape_label_idx;
mod test;
pub mod dg;

pub use component::IRComponent;
pub use node_shape::IRNodeShape;
pub use property_shape::IRPropertyShape;
pub use reifier_info::ReifierInfo;
pub use schema::IRSchema;
pub use shape::IRShape;
pub use shape_label_idx::ShapeLabelIdx;

fn convert_iri_ref(iri_ref: IriRef) -> Result<IriS, IRError> {
    let iri = iri_ref.get_iri().map_err(|err| IRError::IriRefConversion {
        iri_ref: iri_ref.to_string(),
        err: err.to_string(),
    })?;

    Ok(iri.clone())
}

fn convert_value(value: Value) -> Result<Object, IRError> {
    let out = match value {
        Value::Iri(iri_ref) => Object::Iri(convert_iri_ref(iri_ref)?),
        Value::Literal(lit) => Object::literal(lit),
    };

    Ok(out)
}
