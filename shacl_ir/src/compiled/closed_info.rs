use std::collections::HashSet;

use iri_s::IriS;
use rdf::rdf_core::Rdf;
use shacl_ast::{ShaclError, ShaclSchema, node_shape::NodeShape, property_shape::PropertyShape, shape::Shape};

#[derive(Debug, Clone, Default)]
pub enum ClosedInfo {
    Yes {
        // Properties that have been declared as ignored
        ignored_properties: HashSet<IriS>,

        // Properties that appear in the definition
        defined_properties: HashSet<IriS>,

        // Union of ignored and defined properties: union of ignored and defined
        allowed_properties: HashSet<IriS>,
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

    /// Allowed properties are the union of ignored properties and the properties that are defined in a shape
    pub fn allowed_properties(&self) -> Option<&HashSet<IriS>> {
        match self {
            ClosedInfo::Yes { allowed_properties, .. } => Some(allowed_properties),
            ClosedInfo::No => None,
        }
    }

    pub fn get_closed_info_node_shape<R: Rdf>(
        shape: &NodeShape<R>,
        schema: &ShaclSchema<R>,
    ) -> Result<Self, ShaclError> {
        let (is_closed, ignored_properties) = shape.closed_component();
        if is_closed {
            let ignored_properties: HashSet<IriS> = ignored_properties.into_iter().collect();
            let defined_properties = defined_properties(shape, schema)?;
            let all_properties = defined_properties
                .union(&ignored_properties)
                .cloned()
                .collect::<HashSet<IriS>>();
            Ok(ClosedInfo::Yes {
                ignored_properties,
                defined_properties,
                allowed_properties: all_properties,
            })
        } else {
            Ok(ClosedInfo::No)
        }
    }

    pub fn get_closed_info_property_shape<R: Rdf>(
        shape: &PropertyShape<R>,
        schema: &ShaclSchema<R>,
    ) -> Result<Self, ShaclError> {
        let (is_closed, ignored_properties) = shape.closed_component();
        if is_closed {
            let ignored_properties: HashSet<IriS> = ignored_properties.into_iter().collect();
            let defined_properties = defined_properties_property_shape(shape, schema)?;
            let all_properties = defined_properties
                .union(&ignored_properties)
                .cloned()
                .collect::<HashSet<IriS>>();
            Ok(ClosedInfo::Yes {
                ignored_properties,
                defined_properties,
                allowed_properties: all_properties,
            })
        } else {
            Ok(ClosedInfo::No)
        }
    }
}

// TODO: Refactor to avoid code duplication between this method and the next one
fn defined_properties<R: Rdf>(shape: &NodeShape<R>, schema: &ShaclSchema<R>) -> Result<HashSet<IriS>, ShaclError> {
    let mut defined_properties: HashSet<IriS> = HashSet::new();
    for property_shape_ref in shape.property_shapes() {
        let property_shape = schema
            .get_shape(property_shape_ref)
            .ok_or_else(|| ShaclError::ShapeNotFound {
                shape: Box::new(property_shape_ref.clone()),
            })?;
        match property_shape {
            Shape::PropertyShape(ps) => {
                let pred = ps.path().pred().unwrap();
                defined_properties.insert(pred.clone());
            },
            _ => {
                return Err(ShaclError::ShapeNotFound {
                    shape: Box::new(property_shape_ref.clone()),
                });
            },
        }
    }
    Ok(defined_properties)
}

fn defined_properties_property_shape<R: Rdf>(
    shape: &PropertyShape<R>,
    schema: &ShaclSchema<R>,
) -> Result<HashSet<IriS>, ShaclError> {
    let mut defined_properties: HashSet<IriS> = HashSet::new();
    for property_shape_ref in shape.property_shapes() {
        let property_shape = schema
            .get_shape(property_shape_ref)
            .ok_or_else(|| ShaclError::ShapeNotFound {
                shape: Box::new(property_shape_ref.clone()),
            })?;
        match property_shape {
            Shape::PropertyShape(ps) => {
                let pred = ps.path().pred().unwrap();
                defined_properties.insert(pred.clone());
            },
            _ => {
                return Err(ShaclError::ShapeNotFound {
                    shape: Box::new(property_shape_ref.clone()),
                });
            },
        }
    }
    Ok(defined_properties)
}
