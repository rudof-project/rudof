use crate::conformance_metrics::TranslationMetrics;
use crate::unified_constraints::{
    NodeKind as UnifiedNodeKind, UnifiedConstraint, UnifiedConstraintModel, UnifiedPropertyConstraint, UnifiedShape,
    Value as UnifiedValue,
};
use crate::{DataGeneratorError, Result};
use rudof_rdf::rdf_core::{RDFFormat, term::literal::ConcreteLiteral};
use rudof_rdf::rdf_impl::{InMemoryGraph, ReaderMode};
use shacl::ast::{ASTComponent, ASTNodeShape, ASTPropertyShape, ASTSchema, ASTShape};
use shacl::rdf::ShaclParser;
use shacl::types::{NodeKind, Target, Value};
use std::fs;
use std::path::Path;

pub struct ShaclToUnified;

impl Default for ShaclToUnified {
    fn default() -> Self {
        Self
    }
}

impl ShaclToUnified {
    pub async fn convert_file<P: AsRef<Path>>(
        &self,
        shacl_path: P,
    ) -> Result<(UnifiedConstraintModel, TranslationMetrics)> {
        let path = shacl_path.as_ref().to_path_buf();

        let schema_data = tokio::task::spawn_blocking(move || {
            fs::read_to_string(&path).map_err(|e| DataGeneratorError::Config(format!("Failed to read SHACL file: {e}")))
        })
        .await??;

        self.convert_schema(schema_data).await
    }

    pub async fn convert_schema(&self, schema_data: String) -> Result<(UnifiedConstraintModel, TranslationMetrics)> {
        let schema = tokio::task::spawn_blocking(move || {
            // Parse RDF data
            let graph = InMemoryGraph::from_str(&schema_data, &RDFFormat::Turtle, None, &ReaderMode::Strict)
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

    async fn convert_shacl_schema(
        &self,
        schema: &ASTSchema,
    ) -> Result<(UnifiedConstraintModel, TranslationMetrics)> {
        let mut model = UnifiedConstraintModel::new();
        let mut metrics = TranslationMetrics::default();

        // Get all shapes from the schema
        for (_shape_ref, shape) in schema.iter() {
            if let ASTShape::NodeShape(node_shape) = shape {
                let unified_shape = self.convert_node_shape(node_shape.as_ref(), schema, &mut metrics);
                model.add_shape(unified_shape);
            }
        }

        Ok((model, metrics))
    }

    fn convert_node_shape(
        &self,
        node_shape: &ASTNodeShape,
        schema: &ASTSchema,
        metrics: &mut TranslationMetrics,
    ) -> UnifiedShape {
        let shape_id = node_shape.id().to_string();
        let mut properties = Vec::new();

        // Extract target class if available
        let target_class = node_shape.targets().first().and_then(|target| match target {
            Target::Class(tc) => Some(tc.to_string()),
            _ => None,
        });

        // Process property shapes
        for prop_ref in node_shape.property_shapes() {
            if let Some(ASTShape::PropertyShape(prop_shape)) = schema.get_shape(prop_ref)
                && let Some(unified_prop) = self.convert_property_shape(prop_shape, metrics)
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

    fn convert_property_shape(
        &self,
        prop_shape: &ASTPropertyShape,
        metrics: &mut TranslationMetrics,
    ) -> Option<UnifiedPropertyConstraint> {
        let property_iri = prop_shape.path().to_string();
        let mut constraints = Vec::new();

        metrics.original_schema_constraints += prop_shape.components().len();

        // Extract cardinality from components
        let (min_cardinality, max_cardinality) = self.extract_cardinality(prop_shape.components());
        if min_cardinality.is_some() {
            metrics.represented_constraints_in_unified += 1;
        }
        if max_cardinality.is_some() {
            metrics.represented_constraints_in_unified += 1;
        }

        // Process constraints from components
        for component in prop_shape.components() {
            if self.convert_component(component, &mut constraints) {
                metrics.represented_constraints_in_unified += 1;
            }
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

    fn extract_cardinality(&self, components: &[ASTComponent]) -> (Option<u32>, Option<u32>) {
        let mut min_cardinality = None;
        let mut max_cardinality = None;

        for component in components {
            match component {
                ASTComponent::MinCount(min) if *min >= 0 => {
                    min_cardinality = Some(*min as u32);
                },
                ASTComponent::MaxCount(max) if *max >= 0 => {
                    max_cardinality = Some(*max as u32);
                },
                _ => {},
            }
        }

        (min_cardinality, max_cardinality)
    }

    fn convert_component(&self, component: &ASTComponent, constraints: &mut Vec<UnifiedConstraint>) -> bool {
        match component {
            ASTComponent::Datatype(datatype) => {
                constraints.push(UnifiedConstraint::Datatype(datatype.to_string()));
                true
            },
            ASTComponent::NodeKind(node_kind) => {
                let unified_nk = match *node_kind {
                    NodeKind::Iri => UnifiedNodeKind::Iri,
                    NodeKind::BNode => UnifiedNodeKind::BlankNode,
                    NodeKind::Lit => UnifiedNodeKind::Literal,
                    NodeKind::BNodeOrIri => UnifiedNodeKind::BlankNodeOrIri,
                    NodeKind::BNodeOrLit => UnifiedNodeKind::BlankNodeOrLiteral,
                    NodeKind::IriOrLit => UnifiedNodeKind::IriOrLiteral,
                };
                constraints.push(UnifiedConstraint::NodeKind(unified_nk));
                true
            },
            ASTComponent::Pattern { pattern, .. } => {
                constraints.push(UnifiedConstraint::Pattern(pattern.clone()));
                true
            },
            ASTComponent::MinLength(min_len) => {
                constraints.push(UnifiedConstraint::MinLength(*min_len as u32));
                true
            },
            ASTComponent::MaxLength(max_len) => {
                constraints.push(UnifiedConstraint::MaxLength(*max_len as u32));
                true
            },
            ASTComponent::MinInclusive(val) => {
                let unified_val = self.convert_literal_to_value(val);
                constraints.push(UnifiedConstraint::MinInclusive(unified_val));
                true
            },
            ASTComponent::MaxInclusive(val) => {
                let unified_val = self.convert_literal_to_value(val);
                constraints.push(UnifiedConstraint::MaxInclusive(unified_val));
                true
            },
            ASTComponent::MinExclusive(val) => {
                let unified_val = self.convert_literal_to_value(val);
                constraints.push(UnifiedConstraint::MinExclusive(unified_val));
                true
            },
            ASTComponent::MaxExclusive(val) => {
                let unified_val = self.convert_literal_to_value(val);
                constraints.push(UnifiedConstraint::MaxExclusive(unified_val));
                true
            },
            ASTComponent::HasValue(value) => {
                let unified_val = self.convert_value_to_unified_value(value);
                constraints.push(UnifiedConstraint::HasValue(unified_val));
                true
            },
            ASTComponent::In(values) => {
                let unified_vals: Vec<UnifiedValue> =
                    values.iter().map(|v| self.convert_value_to_unified_value(v)).collect();
                constraints.push(UnifiedConstraint::In(unified_vals));
                true
            },
            ASTComponent::Node(shape) => {
                constraints.push(UnifiedConstraint::ShapeReference(shape.to_string()));
                true
            },
            ASTComponent::MinCount(_) | ASTComponent::MaxCount(_) => false,
            _ => {
                // For other components, we might add basic string datatype
                // or handle them based on specific needs
                false
            },
        }
    }

    fn convert_literal_to_value(&self, literal: &ConcreteLiteral) -> UnifiedValue {
        // Simple conversion - in practice you'd want more sophisticated handling
        UnifiedValue::Literal(literal.lexical_form().to_string(), Some(literal.datatype().to_string()))
    }

    fn convert_value_to_unified_value(&self, value: &Value) -> UnifiedValue {
        match value {
            Value::Iri(iri) => UnifiedValue::Iri(iri.to_string()),
            Value::Literal(lit) => {
                UnifiedValue::Literal(lit.lexical_form().to_string(), Some(lit.datatype().to_string()))
            },
        }
    }
}
