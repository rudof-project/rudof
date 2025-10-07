pub mod config;
pub mod converters;
pub mod errors;
pub mod field_generators;
pub mod output;
pub mod parallel_generation;
pub mod shape_processing;
pub mod unified_constraints;

pub use config::{GeneratorConfig, SchemaFormat};
pub use errors::{DataGeneratorError, Result};

use crate::output::OutputWriter;
use crate::parallel_generation::ParallelGenerator;
use crate::shape_processing::ShapeProcessor;
use std::path::Path;
use std::str::FromStr;

/// Main data generator interface
pub struct DataGenerator {
    config: GeneratorConfig,
    processor: ShapeProcessor,
    generator: ParallelGenerator,
    writer: OutputWriter,
}

impl DataGenerator {
    /// Create a new data generator with the given configuration
    pub fn new(config: GeneratorConfig) -> Result<Self> {
        let processor = ShapeProcessor::new();
        let generator = ParallelGenerator::new(&config)?;
        let writer = OutputWriter::new(&config.output)?;

        Ok(Self {
            config,
            processor,
            generator,
            writer,
        })
    }

    /// Load and process a ShEx schema file
    pub async fn load_schema<P: AsRef<Path>>(&mut self, shex_path: P) -> Result<()> {
        let _shapes = self.processor.extract_shapes(shex_path).await?;

        // Get the processed shape infos from the processor
        let shape_infos: Vec<_> = self.processor.get_shapes().values().cloned().collect();

        self.generator.set_shapes(shape_infos).await?;
        Ok(())
    }

    /// Load and process a ShEx schema file
    pub async fn load_shex_schema<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        self.processor.load_shex_schema(path).await?;

        // For backward compatibility, also populate the legacy shapes
        if let Some(unified_model) = self.processor.get_unified_model() {
            // Convert unified model back to shape infos for the generator
            let shape_infos = self.convert_unified_to_shape_infos(unified_model);
            self.generator.set_shapes(shape_infos).await?;
        }

        Ok(())
    }

    /// Load and process a SHACL schema file
    pub async fn load_shacl_schema<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        self.processor.load_shacl_schema(path).await?;

        // Convert unified model to shape infos for the generator
        if let Some(unified_model) = self.processor.get_unified_model() {
            let shape_infos = self.convert_unified_to_shape_infos(unified_model);
            self.generator.set_shapes(shape_infos).await?;
        }

        Ok(())
    }

    /// Auto-detect schema format and load
    pub async fn load_schema_auto<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        self.processor.load_schema_auto(path).await?;

        // Convert unified model to shape infos for the generator
        if let Some(unified_model) = self.processor.get_unified_model() {
            let shape_infos = self.convert_unified_to_shape_infos(unified_model);
            self.generator.set_shapes(shape_infos).await?;
        }

        Ok(())
    }

    /// Generate synthetic data and write to output
    pub async fn generate(&mut self) -> Result<()> {
        let start_time = std::time::Instant::now();
        let graph = self
            .generator
            .generate_data(&self.config.generation)
            .await?;
        let generation_time = start_time.elapsed();

        self.writer
            .write_graph_with_timing(&graph, Some(generation_time))
            .await?;
        Ok(())
    }

    /// Run the complete generation pipeline (legacy ShEx support)
    pub async fn run<P: AsRef<Path>>(&mut self, shex_path: P) -> Result<()> {
        tracing::info!("Loading ShEx schema from: {}", shex_path.as_ref().display());
        self.load_schema(shex_path).await?;

        tracing::info!(
            "Generating {} entities",
            self.config.generation.entity_count
        );
        self.generate().await?;

        tracing::info!("Data generation completed successfully");
        Ok(())
    }

    /// Run the complete generation pipeline with schema format detection
    pub async fn run_with_format<P: AsRef<Path>>(
        &mut self,
        schema_path: P,
        format: Option<SchemaFormat>,
    ) -> Result<()> {
        let schema_path_ref = schema_path.as_ref();
        tracing::info!("Loading schema from: {}", schema_path_ref.display());

        match format.or(self.config.generation.schema_format) {
            Some(SchemaFormat::ShEx) => {
                self.load_shex_schema(schema_path).await?;
            }
            Some(SchemaFormat::SHACL) => {
                self.load_shacl_schema(schema_path).await?;
            }
            None => {
                // Auto-detect based on file extension
                self.load_schema_auto(schema_path).await?;
            }
        }

        tracing::info!(
            "Generating {} entities",
            self.config.generation.entity_count
        );
        self.generate().await?;

        tracing::info!("Data generation completed successfully");
        Ok(())
    }

    /// Run the complete generation pipeline with automatic schema format detection
    pub async fn run_auto<P: AsRef<Path>>(&mut self, schema_path: P) -> Result<()> {
        self.run_with_format(schema_path, None).await
    }

    // Convert unified model to legacy shape infos for backward compatibility
    fn convert_unified_to_shape_infos(
        &self,
        unified_model: &crate::unified_constraints::UnifiedConstraintModel,
    ) -> Vec<crate::shape_processing::ShapeInfo> {
        use crate::shape_processing::{PropertyInfo, ShapeDependency, ShapeInfo};

        let mut shape_infos = Vec::new();

        for (shape_id, unified_shape) in &unified_model.shapes {
            let mut dependencies = Vec::new();
            let mut properties = Vec::new();

            // Convert properties
            for prop in &unified_shape.properties {
                // Extract dependencies from shape references
                for constraint in &prop.constraints {
                    if let crate::unified_constraints::UnifiedConstraint::ShapeReference(
                        target_shape,
                    ) = constraint
                    {
                        dependencies.push(ShapeDependency {
                            target_shape: target_shape.clone(),
                            property: prop.property_iri.clone(),
                            min_cardinality: prop.min_cardinality.map(|c| c as i32),
                            max_cardinality: prop.max_cardinality.map(|c| c as i32),
                        });
                    }
                }

                // Extract datatype from constraints
                let datatype = prop.constraints.iter().find_map(|c| match c {
                    crate::unified_constraints::UnifiedConstraint::Datatype(dt) => Some(dt.clone()),
                    _ => None,
                });

                // Extract shape reference
                let shape_ref = prop.constraints.iter().find_map(|c| match c {
                    crate::unified_constraints::UnifiedConstraint::ShapeReference(sr) => {
                        Some(sr.clone())
                    }
                    _ => None,
                });

                properties.push(PropertyInfo {
                    property_iri: prop.property_iri.clone(),
                    datatype,
                    shape_ref,
                    min_cardinality: prop.min_cardinality.map(|c| c as i32),
                    max_cardinality: prop.max_cardinality.map(|c| c as i32),
                    constraints: prop.constraints.clone(),
                });
            }

            // Create a minimal ShapeDecl for backward compatibility
            let shape_iri = match iri_s::IriS::from_str(shape_id) {
                Ok(iri) => prefixmap::IriRef::Iri(iri),
                Err(_) => {
                    // Fallback to a simple IRI if parsing fails
                    prefixmap::IriRef::Iri(iri_s::IriS::new_unchecked("http://example.org/shape"))
                }
            };

            let dummy_decl = shex_ast::ast::ShapeDecl {
                id: shex_ast::ast::ShapeExprLabel::IriRef { value: shape_iri },
                shape_expr: shex_ast::ast::ShapeExpr::Shape(shex_ast::ast::Shape {
                    expression: None,
                    extra: None,
                    closed: Some(unified_shape.closed),
                    sem_acts: None,
                    annotations: None,
                    extends: None,
                }),
                type_: "".to_string(), // Empty string for type
                is_abstract: false,
            };

            shape_infos.push(ShapeInfo {
                declaration: dummy_decl,
                dependencies,
                properties,
            });
        }

        shape_infos
    }
}
