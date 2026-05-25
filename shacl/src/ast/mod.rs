//! SHACL AST
//! Represents [SHACL](https://www.w3.org/TR/shacl/) Abstract Syntax Tree.
//! This project started as a re-implementation in Rust of [SHACL-s](https://github.com/weso/shacl-s).

mod component;
pub(crate) mod error;
mod node_expr;
mod node_shape;
mod property_shape;
mod reifier_info;
mod schema;
mod shape;

use crate::error::ASTError;
pub use component::ASTComponent;
pub use node_shape::ASTNodeShape;
pub use property_shape::ASTPropertyShape;
pub use reifier_info::ReifierInfo;
use rudof_iri::IriS;
use rudof_rdf::rdf_core::term::Object;
pub use schema::ASTSchema;
pub use shape::ASTShape;
use std::collections::HashSet;

pub fn defined_properties_for(properties: &[Object], ast: &ASTSchema) -> Result<HashSet<IriS>, ASTError> {
    let mut defined_properties = HashSet::new();
    for prop_shape_ref in properties {
        let prop_shape = ast.get_shape(prop_shape_ref).ok_or_else(|| prop_shape_ref.clone())?;
        match prop_shape {
            ASTShape::PropertyShape(ps) => {
                // Better to ignore complex paths then make Rudof crash
                // Also, paths like [ sh:inversePath ex:path ] should be ignored, as that does not
                // add an expected property
                if let Some(pred) = ps.path().pred() {
                    defined_properties.insert(pred.clone());
                }
            },
            _ => {
                return Err(ASTError::UnexpectedShapeType {
                    expected: "PropertyShape".to_string(),
                    shape: Box::new(prop_shape_ref.clone()),
                });
            },
        }
    }

    Ok(defined_properties)
}
