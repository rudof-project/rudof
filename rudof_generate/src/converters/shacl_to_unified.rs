use crate::unified_constraints::{
    NodeKind, UnifiedConstraint, UnifiedConstraintModel, UnifiedPropertyConstraint, UnifiedShape, Value,
};
use crate::{DataGeneratorError, Result};
use shacl_ast::{
    ShaclSchema, component::Component, node_shape::NodeShape, property_shape::PropertyShape, shape::Shape as ShaclShape,
};
use shacl_rdf::ShaclParser;
use srdf::{RDFFormat, ReaderMode, SRDFGraph};
use std::fs;
use std::path::Path;

pub struct ShaclToUnified;

impl Default for ShaclToUnified {
    fn default() -> Self {
        Self
    }
}

impl ShaclToUnified {
    pub async fn convert_file<P: AsRef<Path>>(&self, shacl_path: P) -> Result<UnifiedConstraintModel> {
        let path = shacl_path.as_ref().to_path_buf();

        let schema_data = tokio::task::spawn_blocking(move || {
            fs::read_to_string(&path).map_err(|e| DataGeneratorError::Config(format!("Failed to read SHACL file: {e}")))
        })
        .await??;

        self.convert_schema(schema_data).await
    }

    pub async fn convert_schema(&self, schema_data: String) -> Result<UnifiedConstraintModel> {
        let schema = tokio::task::spawn_blocking(move || {
            // Parse RDF data
            let graph = SRDFGraph::from_str(&schema_data, &RDFFormat::Turtle, None, &ReaderMode::Strict)
                .map_err(|e| DataGeneratorError::Config(format!("Failed to parse RDF: {e}")))?;

            // Parse SHACL schema from RDF
            let mut parser = ShaclParser::new(graph);
            parser
                .parse()
                .map_err(|e| DataGeneratorError::Config(format!("Failed to parse SHACL: {e}")))
        })
        .await??;

        self.convert_shacl_schema(&schema).await
    }

    async fn convert_shacl_schema(&self, schema: &ShaclSchema<SRDFGraph>) -> Result<UnifiedConstraintModel> {
        let mut model = UnifiedConstraintModel::new();

        // Get all shapes from the schema
        for (_shape_ref, shape) in schema.iter() {
            if let ShaclShape::NodeShape(node_shape) = shape {
                let unified_shape = self.convert_node_shape(node_shape.as_ref(), schema);
                model.add_shape(unified_shape);
            }
        }

        Ok(model)
    }

    fn convert_node_shape(&self, node_shape: &NodeShape<SRDFGraph>, schema: &ShaclSchema<SRDFGraph>) -> UnifiedShape {
        let shape_id = node_shape.id().to_string();
        let mut properties = Vec::new();

        // Extract target class if available
        let target_class = node_shape.targets().first().and_then(|target| match target {
            shacl_ast::target::Target::Class(tc) => Some(tc.to_string()),
            _ => None,
        });

        // Process property shapes
        for prop_ref in node_shape.property_shapes() {
            if let Some(ShaclShape::PropertyShape(prop_shape)) = schema.get_shape(prop_ref)
                && let Some(unified_prop) = self.convert_property_shape(prop_shape)
            {
                properties.push(unified_prop);
            }
        }

        // Get closed information from components (currently not available in AST)
        let closed = false; // TODO: Extract from components when available
        let _ignored_properties: Vec<String> = Vec::new(); // TODO: Handle ignored properties

        UnifiedShape {
            id: shape_id,
            target_class,
            properties,
            closed,
        }
    }

    fn convert_property_shape(&self, prop_shape: &PropertyShape<SRDFGraph>) -> Option<UnifiedPropertyConstraint> {
        let property_iri = prop_shape.path().to_string();
        let mut constraints = Vec::new();

        // Extract cardinality from components
        let (min_cardinality, max_cardinality) = self.extract_cardinality(prop_shape.components());

        // Process constraints from components
        for component in prop_shape.components() {
            self.convert_component(component, &mut constraints);
        }

        // If no datatype constraint found, default to string
        if constraints.is_empty() {
            constraints.push(UnifiedConstraint::Datatype(
                "http://www.w3.org/2001/XMLSchema#string".to_string(),
            ));
        }

        Some(UnifiedPropertyConstraint {
            property_iri,
            constraints,
            min_cardinality,
            max_cardinality,
        })
    }

    fn extract_cardinality(&self, components: &[Component]) -> (Option<u32>, Option<u32>) {
        let mut min_cardinality = None;
        let mut max_cardinality = None;

        for component in components {
            match component {
                Component::MinCount(min) => {
                    if *min >= 0 {
                        min_cardinality = Some(*min as u32);
                    }
                },
                Component::MaxCount(max) => {
                    if *max >= 0 {
                        max_cardinality = Some(*max as u32);
                    }
                },
                _ => {},
            }
        }

        (min_cardinality, max_cardinality)
    }

    fn convert_component(&self, component: &Component, constraints: &mut Vec<UnifiedConstraint>) {
        match component {
            Component::Datatype(datatype) => {
                constraints.push(UnifiedConstraint::Datatype(datatype.to_string()));
            },
            Component::NodeKind(node_kind) => {
                let unified_nk = match *node_kind {
                    shacl_ast::node_kind::NodeKind::Iri => NodeKind::Iri,
                    shacl_ast::node_kind::NodeKind::BlankNode => NodeKind::BlankNode,
                    shacl_ast::node_kind::NodeKind::Literal => NodeKind::Literal,
                    shacl_ast::node_kind::NodeKind::BlankNodeOrIri => NodeKind::BlankNodeOrIri,
                    shacl_ast::node_kind::NodeKind::BlankNodeOrLiteral => NodeKind::BlankNodeOrLiteral,
                    shacl_ast::node_kind::NodeKind::IRIOrLiteral => NodeKind::IriOrLiteral,
                };
                constraints.push(UnifiedConstraint::NodeKind(unified_nk));
            },
            Component::Pattern { pattern, .. } => {
                constraints.push(UnifiedConstraint::Pattern(pattern.clone()));
            },
            Component::MinLength(min_len) => {
                constraints.push(UnifiedConstraint::MinLength(*min_len as u32));
            },
            Component::MaxLength(max_len) => {
                constraints.push(UnifiedConstraint::MaxLength(*max_len as u32));
            },
            Component::MinInclusive(val) => {
                let unified_val = self.convert_literal_to_value(val);
                constraints.push(UnifiedConstraint::MinInclusive(unified_val));
            },
            Component::MaxInclusive(val) => {
                let unified_val = self.convert_literal_to_value(val);
                constraints.push(UnifiedConstraint::MaxInclusive(unified_val));
            },
            Component::MinExclusive(val) => {
                let unified_val = self.convert_literal_to_value(val);
                constraints.push(UnifiedConstraint::MinExclusive(unified_val));
            },
            Component::MaxExclusive(val) => {
                let unified_val = self.convert_literal_to_value(val);
                constraints.push(UnifiedConstraint::MaxExclusive(unified_val));
            },
            Component::HasValue { value } => {
                let unified_val = self.convert_value_to_unified_value(value);
                constraints.push(UnifiedConstraint::HasValue(unified_val));
            },
            Component::In { values } => {
                let unified_vals: Vec<Value> = values.iter().map(|v| self.convert_value_to_unified_value(v)).collect();
                constraints.push(UnifiedConstraint::In(unified_vals));
            },
            Component::Node { shape } => {
                constraints.push(UnifiedConstraint::ShapeReference(shape.to_string()));
            },
            _ => {
                // For other components, we might add basic string datatype
                // or handle them based on specific needs
            },
        }
    }

    fn convert_literal_to_value(&self, literal: &srdf::SLiteral) -> Value {
        // Simple conversion - in practice you'd want more sophisticated handling
        Value::Literal(literal.lexical_form().to_string(), Some(literal.datatype().to_string()))
    }

    fn convert_value_to_unified_value(&self, value: &shacl_ast::value::Value) -> Value {
        match value {
            shacl_ast::value::Value::Iri(iri) => Value::Iri(iri.to_string()),
            shacl_ast::value::Value::Literal(lit) => {
                Value::Literal(lit.lexical_form().to_string(), Some(lit.datatype().to_string()))
            },
        }
    }
}
