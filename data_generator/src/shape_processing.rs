use shex_ast::ast::{ShapeDecl, ShapeExpr, TripleExpr};
use crate::{Result, DataGeneratorError};
use std::collections::HashMap;
use std::path::Path;
use shex_compact::ShExParser;

/// Shape analysis and dependency information
#[derive(Debug, Clone)]
pub struct ShapeInfo {
    pub declaration: ShapeDecl,
    pub dependencies: Vec<ShapeDependency>,
    pub properties: Vec<PropertyInfo>,
}

/// Dependency between shapes
#[derive(Debug, Clone)]
pub struct ShapeDependency {
    pub target_shape: String,
    pub property: String,
    pub min_cardinality: Option<i32>,
    pub max_cardinality: Option<i32>,
}

/// Property information for field generation
#[derive(Debug, Clone)]
pub struct PropertyInfo {
    pub property_iri: String,
    pub datatype: Option<String>,
    pub shape_ref: Option<String>,
    pub min_cardinality: Option<i32>,
    pub max_cardinality: Option<i32>,
}

/// Processes ShEx schemas and extracts shape information
pub struct ShapeProcessor {
    shapes: HashMap<String, ShapeInfo>,
    dependency_graph: HashMap<String, Vec<String>>,
}

impl ShapeProcessor {
    pub fn new() -> Self {
        Self {
            shapes: HashMap::new(),
            dependency_graph: HashMap::new(),
        }
    }

    /// Extract shapes from a ShEx file asynchronously
    pub async fn extract_shapes<P: AsRef<Path>>(&mut self, shex_path: P) -> Result<Vec<ShapeDecl>> {
        let path = shex_path.as_ref().to_path_buf();
        
        // Parse ShEx file in a blocking task to avoid blocking the async runtime
        let shapes = tokio::task::spawn_blocking(move || {
            let schema = ShExParser::parse_buf(&path, None)
                .map_err(|e| DataGeneratorError::ShexParsing(format!("Failed to parse ShEx: {e}")))?;
            
            schema.shapes()
                .ok_or_else(|| DataGeneratorError::ShexParsing("No shapes found in schema".to_string()))
        }).await??;

        // Process shapes to extract dependencies and properties
        self.process_shapes(&shapes).await?;
        
        Ok(shapes)
    }

    /// Process shapes to extract metadata and dependencies
    async fn process_shapes(&mut self, shapes: &[ShapeDecl]) -> Result<()> {
        self.shapes.clear();
        self.dependency_graph.clear();

        // Process each shape in parallel
        let shape_futures: Vec<_> = shapes.iter()
            .map(|shape| self.process_single_shape(shape))
            .collect();

        let processed_shapes: Result<Vec<ShapeInfo>> = futures::future::try_join_all(shape_futures).await;
        let processed_shapes = processed_shapes?;

        // Build the dependency graph
        for shape_info in processed_shapes {
            let shape_id = shape_info.declaration.id.to_string();
            
            // Extract dependencies
            let dependencies: Vec<String> = shape_info.dependencies
                .iter()
                .map(|dep| dep.target_shape.clone())
                .collect();
            
            self.dependency_graph.insert(shape_id.clone(), dependencies);
            self.shapes.insert(shape_id, shape_info);
        }

        Ok(())
    }

    /// Process a single shape to extract its information
    async fn process_single_shape(&self, shape: &ShapeDecl) -> Result<ShapeInfo> {
        let mut dependencies = Vec::new();
        let mut properties = Vec::new();

        if let ShapeExpr::Shape(s) = &shape.shape_expr {
            if let Some(expr) = &s.expression {
                self.extract_dependencies_and_properties(&expr.te, &mut dependencies, &mut properties);
            }
        }

        Ok(ShapeInfo {
            declaration: shape.clone(),
            dependencies,
            properties,
        })
    }

    /// Recursively extract dependencies and properties from a triple expression
    fn extract_dependencies_and_properties(
        &self,
        expr: &TripleExpr,
        dependencies: &mut Vec<ShapeDependency>,
        properties: &mut Vec<PropertyInfo>,
    ) {
        match expr {
            TripleExpr::EachOf { expressions, .. } | TripleExpr::OneOf { expressions, .. } => {
                for e in expressions {
                    self.extract_dependencies_and_properties(&e.te, dependencies, properties);
                }
            }
            TripleExpr::TripleConstraint { predicate, value_expr, min, max, .. } => {
                let property_iri = predicate.to_string();
                let (min_card, max_card) = match (*min, *max) {
                    (None, None) => (Some(1), Some(1)), // Default cardinality is (1,1)
                    (min, max) => (min, max),
                };

                if let Some(val_expr) = value_expr {
                    match &**val_expr {
                        ShapeExpr::Ref(ref_to) => {
                            // This is a reference to another shape (object property)
                            dependencies.push(ShapeDependency {
                                target_shape: ref_to.to_string(),
                                property: property_iri.clone(),
                                min_cardinality: min_card,
                                max_cardinality: max_card,
                            });

                            properties.push(PropertyInfo {
                                property_iri,
                                datatype: None,
                                shape_ref: Some(ref_to.to_string()),
                                min_cardinality: min_card,
                                max_cardinality: max_card,
                            });
                        }
                        ShapeExpr::NodeConstraint(node_constraint) => {
                            // Extract datatype from node constraint (data property)
                            let datatype = if let Some(dt) = node_constraint.datatype() {
                                Some(dt.to_string())
                            } else {
                                // Default to string if no datatype specified
                                Some("http://www.w3.org/2001/XMLSchema#string".to_string())
                            };

                            properties.push(PropertyInfo {
                                property_iri,
                                datatype,
                                shape_ref: None,
                                min_cardinality: min_card,
                                max_cardinality: max_card,
                            });
                        }
                        _ => {
                            // Other shape expressions - treat as generic string property
                            properties.push(PropertyInfo {
                                property_iri,
                                datatype: Some("http://www.w3.org/2001/XMLSchema#string".to_string()),
                                shape_ref: None,
                                min_cardinality: min_card,
                                max_cardinality: max_card,
                            });
                        }
                    }
                } else {
                    // No value expression - treat as generic string property
                    properties.push(PropertyInfo {
                        property_iri,
                        datatype: Some("http://www.w3.org/2001/XMLSchema#string".to_string()),
                        shape_ref: None,
                        min_cardinality: min_card,
                        max_cardinality: max_card,
                    });
                }
            }
            TripleExpr::TripleExprRef(_) => {
                // Handle triple expression references if needed
            }
        }
    }

    /// Get processed shape information
    pub fn get_shapes(&self) -> &HashMap<String, ShapeInfo> {
        &self.shapes
    }

    /// Get dependency graph
    pub fn get_dependency_graph(&self) -> &HashMap<String, Vec<String>> {
        &self.dependency_graph
    }

    /// Get shapes in topological order for generation
    pub fn get_topological_order(&self) -> Result<Vec<String>> {
        topological_sort(&self.dependency_graph)
    }

    /// Get a specific shape by ID
    pub fn get_shape(&self, shape_id: &str) -> Option<&ShapeInfo> {
        self.shapes.get(shape_id)
    }
}

/// Perform topological sort on the dependency graph
fn topological_sort(graph: &HashMap<String, Vec<String>>) -> Result<Vec<String>> {
    let mut result = Vec::new();
    let mut visited = std::collections::HashSet::new();
    let mut temp_visited = std::collections::HashSet::new();

    fn visit(
        node: &str,
        graph: &HashMap<String, Vec<String>>,
        visited: &mut std::collections::HashSet<String>,
        temp_visited: &mut std::collections::HashSet<String>,
        result: &mut Vec<String>,
    ) -> Result<()> {
        if temp_visited.contains(node) {
            return Err(DataGeneratorError::GraphGeneration(
                format!("Circular dependency detected involving shape: {}", node)
            ));
        }
        
        if visited.contains(node) {
            return Ok(());
        }

        temp_visited.insert(node.to_string());

        if let Some(dependencies) = graph.get(node) {
            for dep in dependencies {
                visit(dep, graph, visited, temp_visited, result)?;
            }
        }

        temp_visited.remove(node);
        visited.insert(node.to_string());
        result.push(node.to_string());

        Ok(())
    }

    for node in graph.keys() {
        if !visited.contains(node) {
            visit(node, graph, &mut visited, &mut temp_visited, &mut result)?;
        }
    }

    // Reverse to get correct dependency order
    result.reverse();
    Ok(result)
}
