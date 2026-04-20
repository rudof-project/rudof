use crate::conformance_metrics::TranslationMetrics;
use crate::unified_constraints::{
    NodeKind, UnifiedConstraint, UnifiedConstraintModel, UnifiedPropertyConstraint, UnifiedShape,
};
use crate::{DataGeneratorError, Result};
use iri_s::IriS;
use shex_ast::ast::{NodeConstraint, ShapeDecl, ShapeExpr, TripleExpr, XsFacet};
use shex_ast::compact::ShExParser;
use std::path::Path;

pub struct ShExToUnified;

impl Default for ShExToUnified {
    fn default() -> Self {
        Self
    }
}

impl ShExToUnified {
    pub async fn convert_file<P: AsRef<Path>>(
        &self,
        shex_path: P,
    ) -> Result<(UnifiedConstraintModel, TranslationMetrics)> {
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

    pub async fn convert_schema(&self, schema_data: String) -> Result<(UnifiedConstraintModel, TranslationMetrics)> {
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

    async fn convert_shapes(&self, shapes: &[ShapeDecl]) -> Result<(UnifiedConstraintModel, TranslationMetrics)> {
        let mut model = UnifiedConstraintModel::new();
        let mut metrics = TranslationMetrics::default();

        for shape_decl in shapes {
            let unified_shape = self.convert_shape_decl(shape_decl, &mut metrics);
            model.add_shape(unified_shape);
        }

        Ok((model, metrics))
    }

    fn convert_shape_decl(&self, shape_decl: &ShapeDecl, metrics: &mut TranslationMetrics) -> UnifiedShape {
        let shape_id = shape_decl.id.to_string();
        let mut properties = Vec::new();

        if let ShapeExpr::Shape(s) = &shape_decl.shape_expr
            && let Some(expr) = &s.expression
        {
            self.extract_properties(&expr.te, &mut properties, metrics);
        }

        UnifiedShape {
            id: shape_id,
            target_class: None, // ShEx doesn't have explicit target classes
            properties,
            closed: false, // TODO: extract from ShEx closed property if available
        }
    }

    fn extract_properties(
        &self,
        expr: &TripleExpr,
        properties: &mut Vec<UnifiedPropertyConstraint>,
        metrics: &mut TranslationMetrics,
    ) {
        match expr {
            TripleExpr::EachOf { expressions, .. } | TripleExpr::OneOf { expressions, .. } => {
                for e in expressions {
                    self.extract_properties(&e.te, properties, metrics);
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

                if min.is_some() {
                    metrics.original_schema_constraints += 1;
                    metrics.represented_constraints_in_unified += 1;
                }
                if max.is_some() {
                    metrics.original_schema_constraints += 1;
                    metrics.represented_constraints_in_unified += 1;
                }

                if let Some(val_expr) = value_expr {
                    match &**val_expr {
                        ShapeExpr::Ref(ref_to) => {
                            metrics.original_schema_constraints += 1;
                            metrics.represented_constraints_in_unified += 1;
                            constraints.push(UnifiedConstraint::ShapeReference(ref_to.to_string()));
                        },
                        ShapeExpr::NodeConstraint(node_constraint) => {
                            self.convert_node_constraint(node_constraint, &mut constraints, metrics);
                        },
                        _ => {
                            // Unsupported complex value expression contributes to original constraints.
                            metrics.original_schema_constraints += 1;
                        },
                    }
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

    fn convert_node_constraint(
        &self,
        node_constraint: &NodeConstraint,
        constraints: &mut Vec<UnifiedConstraint>,
        metrics: &mut TranslationMetrics,
    ) {
        // Convert datatype
        if let Some(dt) = node_constraint.datatype() {
            metrics.original_schema_constraints += 1;
            metrics.represented_constraints_in_unified += 1;
            constraints.push(UnifiedConstraint::Datatype(dt.to_string()));
        }

        // Convert node kind
        if let Some(nk) = node_constraint.node_kind() {
            metrics.original_schema_constraints += 1;
            metrics.represented_constraints_in_unified += 1;
            let unified_nk = match nk {
                shex_ast::ast::NodeKind::Iri => NodeKind::Iri,
                shex_ast::ast::NodeKind::BNode => NodeKind::BlankNode,
                shex_ast::ast::NodeKind::Literal => NodeKind::Literal,
                shex_ast::ast::NodeKind::NonLiteral => NodeKind::BlankNodeOrIri,
            };
            constraints.push(UnifiedConstraint::NodeKind(unified_nk));
        }

        if let Some(values) = node_constraint.values() {
            // Value set constraints are currently not translated in this converter.
            metrics.original_schema_constraints += values.len();
        }

        if let Some(facets) = node_constraint.facets() {
            for facet in facets {
                metrics.original_schema_constraints += 1;
                if self.try_convert_xs_facet(&facet, constraints) {
                    metrics.represented_constraints_in_unified += 1;
                }
            }
        }

        // Basic constraint extraction - simplified for now
        // TODO: Add proper facet and value extraction when needed
    }

    fn try_convert_xs_facet(&self, facet: &XsFacet, constraints: &mut Vec<UnifiedConstraint>) -> bool {
        match facet {
            XsFacet::StringFacet(sf) => match sf {
                shex_ast::ast::StringFacet::Pattern(pattern) => {
                    constraints.push(UnifiedConstraint::Pattern(pattern.regex().to_string()));
                    true
                },
                shex_ast::ast::StringFacet::MinLength(min) => {
                    constraints.push(UnifiedConstraint::MinLength(*min as u32));
                    true
                },
                shex_ast::ast::StringFacet::MaxLength(max) => {
                    constraints.push(UnifiedConstraint::MaxLength(*max as u32));
                    true
                },
                shex_ast::ast::StringFacet::Length(_) => false,
            },
            XsFacet::NumericFacet(nf) => match nf {
                shex_ast::ast::NumericFacet::MinInclusive(val) => {
                    constraints.push(UnifiedConstraint::MinInclusive(
                        crate::unified_constraints::Value::Literal(val.to_string(), None),
                    ));
                    true
                },
                shex_ast::ast::NumericFacet::MaxInclusive(val) => {
                    constraints.push(UnifiedConstraint::MaxInclusive(
                        crate::unified_constraints::Value::Literal(val.to_string(), None),
                    ));
                    true
                },
                shex_ast::ast::NumericFacet::MinExclusive(val) => {
                    constraints.push(UnifiedConstraint::MinExclusive(
                        crate::unified_constraints::Value::Literal(val.to_string(), None),
                    ));
                    true
                },
                shex_ast::ast::NumericFacet::MaxExclusive(val) => {
                    constraints.push(UnifiedConstraint::MaxExclusive(
                        crate::unified_constraints::Value::Literal(val.to_string(), None),
                    ));
                    true
                },
                shex_ast::ast::NumericFacet::TotalDigits(_) | shex_ast::ast::NumericFacet::FractionDigits(_) => false,
            },
        }
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
