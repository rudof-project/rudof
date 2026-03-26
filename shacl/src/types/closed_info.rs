use crate::ast::error::ASTError;
use crate::ast::{ASTSchema, ASTShape};
use iri_s::IriS;
use rudof_rdf::rdf_core::term::Object;
use std::collections::HashSet;

#[derive(Debug, Clone, Default)]
pub(crate) enum ClosedInfo {
    Yes {
        /// Properties that have been declared as ignored
        ignored_properties: HashSet<IriS>,
        /// Properties that appear in the definition
        defined_properties: HashSet<IriS>,
    },

    #[default]
    No,
}

impl ClosedInfo {
    pub fn is_closed(&self) -> bool {
        matches!(self, ClosedInfo::Yes { .. })
    }

    pub fn ignored_properties(&self) -> Option<&HashSet<IriS>> {
        match self {
            ClosedInfo::Yes { ignored_properties, .. } => Some(ignored_properties),
            ClosedInfo::No => None,
        }
    }

    pub fn defined_properties(&self) -> Option<&HashSet<IriS>> {
        match self {
            ClosedInfo::Yes { defined_properties, .. } => Some(defined_properties),
            ClosedInfo::No => None,
        }
    }

    /// Allowed properties are the union of ignored properties and the properties that are defined in a shape
    pub fn allowed_properties(&self) -> Option<HashSet<IriS>> {
        match self {
            ClosedInfo::Yes { defined_properties, ignored_properties } => {
                let result = defined_properties.union(ignored_properties).cloned().collect();
                Some(result)
            }
            ClosedInfo::No => None,
        }
    }
}

pub(crate) fn defined_properties_for(properties: &[Object], ast: &ASTSchema) -> Result<HashSet<IriS>, ASTError> {
    let mut defined_properties = HashSet::new();
    for prop_shape_ref in properties {
        let prop_shape = ast.get_shape(prop_shape_ref)
            .ok_or_else(|| ASTError::ShapeNotFound {
                shape: Box::new(prop_shape_ref.clone()),
            })?;
        match prop_shape {
            ASTShape::PropertyShape(ps) => {
                // Better to ignore complex paths then make Rudof crash
                // Also, paths like [ sh:inversePath ex:path ] should be ignored, as that does not
                // add an expected property
                if let Some(pred) = ps.path().pred() {
                    defined_properties.insert(pred.clone());
                }
            },
            _ => return Err(ASTError::ShapeNotFound {
                shape: Box::new(prop_shape_ref.clone()),
            }),
        }
    }

    Ok(defined_properties)
}