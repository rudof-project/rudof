use crate::unified_constraints::{
    NodeKind, UnifiedConstraint, UnifiedConstraintModel, UnifiedPropertyConstraint, UnifiedShape,
};
use crate::{DataGeneratorError, Result};
use iri_s::IriS;
use shex_ast::ast::{NodeConstraint, ShapeDecl, ShapeExpr, TripleExpr};
use shex_ast::compact::ShExParser;
use std::path::Path;

pub struct ShExToUnified;

impl Default for ShExToUnified {
    fn default() -> Self {
        Self
    }
}

impl ShExToUnified {
    pub async fn convert_file<P: AsRef<Path>>(&self, shex_path: P) -> Result<UnifiedConstraintModel> {
        let path = shex_path.as_ref().to_path_buf();

        let shapes = tokio::task::spawn_blocking(move || {
            let schema = ShExParser::parse_buf(&path, None)
                .map_err(|e| DataGeneratorError::ShexParsing(format!("Failed to parse ShEx: {e}")))?;

            schema
                .shapes()
                .ok_or_else(|| DataGeneratorError::ShexParsing("No shapes found in schema".to_string()))
        })
        .await??;

        self.convert_shapes(&shapes).await
    }

    pub async fn convert_schema(&self, schema_data: String) -> Result<UnifiedConstraintModel> {
        let shapes = tokio::task::spawn_blocking(move || {
            // Create a default base IRI for parsing
            let default_base = IriS::new_unchecked("http://example.org/");
            let schema = ShExParser::parse(&schema_data, None, &default_base)
                .map_err(|e| DataGeneratorError::ShexParsing(format!("Failed to parse ShEx: {e}")))?;

            schema
                .shapes()
                .ok_or_else(|| DataGeneratorError::ShexParsing("No shapes found in schema".to_string()))
        })
        .await??;

        self.convert_shapes(&shapes).await
    }

    async fn convert_shapes(&self, shapes: &[ShapeDecl]) -> Result<UnifiedConstraintModel> {
        let mut model = UnifiedConstraintModel::new();

        for shape_decl in shapes {
            let unified_shape = self.convert_shape_decl(shape_decl);
            model.add_shape(unified_shape);
        }

        Ok(model)
    }

    fn convert_shape_decl(&self, shape_decl: &ShapeDecl) -> UnifiedShape {
        let shape_id = shape_decl.id.to_string();
        let mut properties = Vec::new();

        if let ShapeExpr::Shape(s) = &shape_decl.shape_expr
            && let Some(expr) = &s.expression
        {
            self.extract_properties(&expr.te, &mut properties);
        }

        UnifiedShape {
            id: shape_id,
            target_class: None, // ShEx doesn't have explicit target classes
            properties,
            closed: false, // TODO: extract from ShEx closed property if available
        }
    }

    fn extract_properties(&self, expr: &TripleExpr, properties: &mut Vec<UnifiedPropertyConstraint>) {
        match expr {
            TripleExpr::EachOf { expressions, .. } | TripleExpr::OneOf { expressions, .. } => {
                for e in expressions {
                    self.extract_properties(&e.te, properties);
                }
            },
            TripleExpr::TripleConstraint {
                predicate,
                value_expr,
                min,
                max,
                ..
            } => {
                let property_iri = predicate.to_string();
                let (min_card, max_card) = self.convert_cardinality(*min, *max);
                let mut constraints = Vec::new();

                if let Some(val_expr) = value_expr {
                    match &**val_expr {
                        ShapeExpr::Ref(ref_to) => {
                            constraints.push(UnifiedConstraint::ShapeReference(ref_to.to_string()));
                        },
                        ShapeExpr::NodeConstraint(node_constraint) => {
                            self.convert_node_constraint(node_constraint, &mut constraints);
                        },
                        _ => {
                            // Default to string for other cases
                            constraints.push(UnifiedConstraint::Datatype(
                                "http://www.w3.org/2001/XMLSchema#string".to_string(),
                            ));
                        },
                    }
                } else {
                    // No value expression - default to string
                    constraints.push(UnifiedConstraint::Datatype(
                        "http://www.w3.org/2001/XMLSchema#string".to_string(),
                    ));
                }

                properties.push(UnifiedPropertyConstraint {
                    property_iri,
                    constraints,
                    min_cardinality: min_card,
                    max_cardinality: max_card,
                });
            },
            TripleExpr::Ref(_) => {
                // Handle triple expression references if needed
            },
        }
    }

    fn convert_node_constraint(&self, node_constraint: &NodeConstraint, constraints: &mut Vec<UnifiedConstraint>) {
        // Convert datatype
        if let Some(dt) = node_constraint.datatype() {
            constraints.push(UnifiedConstraint::Datatype(dt.to_string()));
        }

        // Convert node kind
        if let Some(nk) = node_constraint.node_kind() {
            let unified_nk = match nk {
                shex_ast::ast::NodeKind::Iri => NodeKind::Iri,
                shex_ast::ast::NodeKind::BNode => NodeKind::BlankNode,
                shex_ast::ast::NodeKind::Literal => NodeKind::Literal,
                shex_ast::ast::NodeKind::NonLiteral => NodeKind::BlankNodeOrIri,
            };
            constraints.push(UnifiedConstraint::NodeKind(unified_nk));
        }

        // Basic constraint extraction - simplified for now
        // TODO: Add proper facet and value extraction when needed
    }

    fn convert_cardinality(&self, min: Option<i32>, max: Option<i32>) -> (Option<u32>, Option<u32>) {
        let min_card = match min {
            None => Some(1), // Default min cardinality is 1
            Some(m) if m >= 0 => Some(m as u32),
            Some(_) => Some(0), // Negative values become 0
        };

        let max_card = max.and_then(|m| if m >= 0 { Some(m as u32) } else { None });

        (min_card, max_card)
    }
}
