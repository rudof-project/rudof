pub mod adapters;
pub mod config;
pub mod engine;
pub mod errors;
pub mod ir;
pub mod metrics;
pub mod sinks;
pub mod values;
pub mod violations;

pub use config::{GeneratorConfig, SchemaFormat};
use errors::{DataGeneratorError, Result};
pub use metrics::{ConformanceMetrics, TranslationMetrics};

use crate::{engine::ParallelGenerator, ir::shape_info::ShapeProcessor};
#[cfg(not(target_family = "wasm"))]
use sinks::OutputWriter;
use std::{path::Path, str::FromStr};

/// Main data generator interface
pub struct DataGenerator {
    config: GeneratorConfig,
    processor: ShapeProcessor,
    generator: ParallelGenerator,
    #[cfg(not(target_family = "wasm"))]
    writer: OutputWriter,
    target_report: Option<TargetReport>,
}

/// What a `[targets]` search reached.
#[derive(Debug, Clone, PartialEq)]
pub struct TargetReport {
    /// The metric searched for, e.g. `"coherence"`.
    pub metric: &'static str,
    pub target: f64,
    /// The value of the metric on the generated graph.
    pub reached: f64,
    /// Whether `reached` is within the tolerance of `target`.
    pub within_tolerance: bool,
    /// The default fill of optional properties that produced the graph.
    pub fill: f64,
}

/// Lowest default fill a `[targets]` search tries: at fill 0 an optional property
/// disappears from its type, which raises coherence instead of lowering it.
const MIN_TARGET_FILL: f64 = 0.05;

/// Graphs a `[targets]` search generates at most.
const MAX_TARGET_STEPS: usize = 12;

/// What an in-memory run produced (see [`DataGenerator::generate_in_memory`]).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct GeneratedData {
    /// The graph as RDF (Turtle); from a PG-Schema, the RDF encoding of the property graph.
    pub data: String,
    /// A ShEx ShapeMap (compact syntax) with the shape of every generated node.
    pub shapemap: String,
    /// From a PG-Schema: the graph in rudof's property graph format.
    pub property_graph: Option<String>,
    /// From a PG-Schema: the type of every node and edge.
    pub type_map: Option<String>,
    /// With `[violations]`: what a validator of the schema's language must report (JSON Lines).
    pub labels: Option<String>,
    /// With `[violations]` and a PG-Schema: the same for the graph read as RDF (JSON Lines).
    pub rdf_labels: Option<String>,
}

impl DataGenerator {
    /// Create a new data generator with the given configuration
    pub fn new(config: GeneratorConfig) -> Result<Self> {
        let processor = ShapeProcessor::new();
        let generator = ParallelGenerator::new(&config)?;
        #[cfg(not(target_family = "wasm"))]
        let writer = OutputWriter::new(&config.output)?;

        Ok(Self {
            config,
            processor,
            generator,
            #[cfg(not(target_family = "wasm"))]
            writer,
            target_report: None,
        })
    }

    /// What the `[targets]` search of the last [`Self::generate`] reached, if the
    /// configuration sets targets.
    pub fn target_report(&self) -> Option<&TargetReport> {
        self.target_report.as_ref()
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

    /// Load and process a PG-Schema file
    pub async fn load_pgschema_schema<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        self.processor.load_pgschema_schema(path).await?;
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
    #[cfg(not(target_family = "wasm"))]
    pub async fn generate(&mut self) -> Result<()> {
        self.create_output_dirs()?;
        let start_time = std::time::Instant::now();
        let mut graph = match self.config.generation.targets.clone() {
            Some(targets) => self.generate_for_targets(&targets).await?,
            None => self.generator.generate_data(&self.config.generation).await?,
        };
        if let Some(violations) = &self.config.generation.violations {
            let injected;
            (graph, injected) = self.break_graph(graph, violations).await?;
            if let Some(path) = &violations.labels {
                violations::write_labels(path, &injected.labels)?;
            }
            if let Some(reading) = &violations.rdf {
                let rdf_labels = injected.rdf_labels.as_ref().ok_or_else(|| {
                    DataGeneratorError::Config("[violations.rdf] needs a PG-Schema input".to_string())
                })?;
                violations::write_labels(&reading.labels, rdf_labels)?;
            }
        }
        let generation_time = start_time.elapsed();

        let conformance_metrics = if let (Some(unified_model), Some(translation_metrics)) = (
            self.processor.get_unified_model(),
            self.processor.get_translation_metrics(),
        ) {
            Some(crate::metrics::conformance::ConformanceMetrics::from_graph_and_model(
                &graph,
                unified_model,
                translation_metrics.clone(),
            )?)
        } else {
            None
        };

        if self.config.output.format == config::OutputFormat::Pg {
            return self.write_property_graph(&graph).await;
        }
        self.writer
            .write_graph_with_timing(&graph, Some(generation_time), conformance_metrics.as_ref())
            .await?;
        if let Some(path) = &self.config.output.shapemap {
            std::fs::write(path, self.shape_map().await)?;
        }
        Ok(())
    }

    /// Create the folder of every file the run writes (`[output] dir` need not exist).
    #[cfg(not(target_family = "wasm"))]
    fn create_output_dirs(&self) -> Result<()> {
        let output = &self.config.output;
        let violations = self.config.generation.violations.as_ref();
        let paths = [
            Some(&output.path),
            output.shapemap.as_ref(),
            output.rdf.as_ref(),
            output.rdf_shapemap.as_ref(),
        ]
        .into_iter()
        .chain([violations.and_then(|v| v.labels.as_ref())])
        .chain([violations.and_then(|v| v.rdf.as_ref()).map(|r| &r.labels)])
        .flatten();
        for path in paths {
            if let Some(dir) = path.parent().filter(|d| !d.as_os_str().is_empty()) {
                std::fs::create_dir_all(dir)?;
            }
        }
        Ok(())
    }

    /// Search the default fill of optional properties until the graph reaches the
    /// target coherence, by bisection, since coherence grows with the fill. The default
    /// cardinality is `max` during the search, so that the fill alone decides whether
    /// an optional property appears. The
    /// generator of the closest graph replaces `self.generator`, so the ShapeMap and
    /// the violations describe that graph. A target outside the reachable range
    /// yields the closest graph and a warning.
    async fn generate_for_targets(
        &mut self,
        targets: &config::TargetsConfig,
    ) -> Result<rudof_rdf::rdf_impl::OxigraphInMemory> {
        use rudof_rdf::rdf_core::NeighsRDF;
        let target = targets
            .coherence
            .ok_or_else(|| DataGeneratorError::Config("[targets] sets no metric".to_string()))?;
        let model = self
            .processor
            .get_unified_model()
            .ok_or_else(|| DataGeneratorError::Config("[targets] needs a loaded schema".to_string()))?;
        let shapes = self.convert_unified_to_shape_infos(model);

        let mut best: Option<(TargetReport, ParallelGenerator, rudof_rdf::rdf_impl::OxigraphInMemory)> = None;
        let (mut low, mut high) = (MIN_TARGET_FILL, 1.0);
        for step in 0..MAX_TARGET_STEPS {
            let fill = match step {
                0 => high,
                1 => low,
                _ => (low + high) / 2.0,
            };
            let mut config = self.config.clone();
            config.generation.property_fill_probability = fill;
            config.generation.cardinality_strategy = config::CardinalityStrategy::Maximum;
            let mut generator = ParallelGenerator::new(&config)?;
            generator.set_shapes(shapes.clone()).await?;
            let graph = generator.generate_data(&config.generation).await?;
            let triples: Vec<oxrdf::Triple> = graph
                .triples()
                .map_err(|e| DataGeneratorError::GraphGeneration(e.to_string()))?
                .collect();
            let reached = metrics::coherence(&triples, &generator.entity_shapes().await);
            tracing::info!("[targets] fill {fill:.4}: coherence {reached:.4} (target {target})");

            let report = TargetReport {
                metric: "coherence",
                target,
                reached,
                within_tolerance: (reached - target).abs() <= targets.tolerance,
                fill,
            };
            let done = report.within_tolerance;
            let closer = best
                .as_ref()
                .is_none_or(|(b, _, _)| (reached - target).abs() < (b.reached - target).abs());
            if closer {
                best = Some((report, generator, graph));
            }
            // Out of range: the highest fill cannot reach a higher target, nor the
            // lowest a lower one.
            let out_of_range = (step == 0 && reached < target) || (step == 1 && reached > target);
            if done || out_of_range {
                break;
            }
            if step >= 1 {
                if reached < target {
                    low = fill;
                } else {
                    high = fill;
                }
            }
        }

        let (report, generator, graph) =
            best.ok_or_else(|| DataGeneratorError::GraphGeneration("[targets] generated no graph".to_string()))?;
        if !report.within_tolerance {
            tracing::warn!(
                "[targets] coherence {} not reached: the closest graph has {:.4} (fill {:.4})",
                report.target,
                report.reached,
                report.fill
            );
        }
        self.generator = generator;
        self.target_report = Some(report);
        Ok(graph)
    }

    /// Write the graph as a property graph (`.pg`) and the type map in place of
    /// the ShapeMap.
    #[cfg(not(target_family = "wasm"))]
    async fn write_property_graph(&self, graph: &rudof_rdf::rdf_impl::OxigraphInMemory) -> Result<()> {
        use rudof_rdf::rdf_core::{BuildRDF, NeighsRDF};
        let model = self
            .processor
            .get_unified_model()
            .ok_or_else(|| DataGeneratorError::Config("property graph output needs a loaded schema".to_string()))?;
        let triples: Vec<oxrdf::Triple> = graph
            .triples()
            .map_err(|e| DataGeneratorError::GraphGeneration(e.to_string()))?
            .collect();
        let entities = self.generator.entity_shapes().await;
        let (pg, type_map) = sinks::pg::property_graph(&triples, model, &entities);
        std::fs::write(&self.config.output.path, pg)?;
        if let Some(path) = &self.config.output.shapemap {
            std::fs::write(path, type_map)?;
        }
        // The same graph as RDF, for validation against RDF translations of the schema.
        if let Some(path) = &self.config.output.rdf {
            let mut turtle = Vec::new();
            graph
                .serialize(&rudof_rdf::rdf_core::RDFFormat::Turtle, &mut turtle)
                .map_err(|e| DataGeneratorError::OutputWriting(e.to_string()))?;
            std::fs::write(path, turtle)?;
        }
        if let Some(path) = &self.config.output.rdf_shapemap {
            std::fs::write(path, self.shape_map().await)?;
        }
        Ok(())
    }

    /// Break the configured share of entities; the labels say what must fail.
    async fn break_graph(
        &self,
        graph: rudof_rdf::rdf_impl::OxigraphInMemory,
        violations: &config::ViolationsConfig,
    ) -> Result<(rudof_rdf::rdf_impl::OxigraphInMemory, violations::Injected)> {
        use rudof_rdf::rdf_core::{BuildRDF, NeighsRDF};
        let model = self
            .processor
            .get_unified_model()
            .ok_or_else(|| DataGeneratorError::Config("violations need a loaded schema".to_string()))?;
        let mut triples: Vec<oxrdf::Triple> = graph
            .triples()
            .map_err(|e| DataGeneratorError::GraphGeneration(e.to_string()))?
            .collect();
        let entities = self.generator.entity_shapes().await;
        let injected = violations::inject(&mut triples, model, &entities, violations, self.config.generation.seed)?;
        tracing::info!("injected violations: {:?}", violations::summary(&injected.labels));

        let mut mutated = rudof_rdf::rdf_impl::OxigraphInMemory::default();
        for t in triples {
            mutated
                .add_triple(t.subject, t.predicate, t.object)
                .map_err(|e| DataGeneratorError::GraphGeneration(e.to_string()))?;
        }
        Ok((mutated, injected))
    }

    /// Load a schema given as text, in the language `format`.
    pub async fn load_schema_str(&mut self, text: &str, format: SchemaFormat) -> Result<()> {
        self.processor.load_schema_text(text, format).await?;
        if let Some(unified_model) = self.processor.get_unified_model() {
            let shape_infos = self.convert_unified_to_shape_infos(unified_model);
            self.generator.set_shapes(shape_infos).await?;
        }
        Ok(())
    }

    /// Generate into strings instead of files: the data, its ShapeMap and, from a
    /// PG-Schema, the property graph and its type map; with `[violations]`, the reports.
    /// The `[output]` settings are not used. Works in WebAssembly.
    pub async fn generate_in_memory(&mut self) -> Result<GeneratedData> {
        use rudof_rdf::rdf_core::{BuildRDF, NeighsRDF};
        let mut graph = match self.config.generation.targets.clone() {
            Some(targets) => self.generate_for_targets(&targets).await?,
            None => self.generator.generate_data(&self.config.generation).await?,
        };
        let mut out = GeneratedData::default();
        if let Some(violations) = self.config.generation.violations.clone() {
            let injected;
            (graph, injected) = self.break_graph(graph, &violations).await?;
            out.labels = Some(violations::labels_jsonl(&injected.labels)?);
            out.rdf_labels = injected
                .rdf_labels
                .as_deref()
                .map(violations::labels_jsonl)
                .transpose()?;
        }
        let mut turtle = Vec::new();
        graph
            .serialize(&rudof_rdf::rdf_core::RDFFormat::Turtle, &mut turtle)
            .map_err(|e| DataGeneratorError::OutputWriting(e.to_string()))?;
        out.data = String::from_utf8_lossy(&turtle).into_owned();
        out.shapemap = self.shape_map().await;
        let model = self
            .processor
            .get_unified_model()
            .ok_or_else(|| DataGeneratorError::Config("generation needs a loaded schema".to_string()))?;
        if adapters::pgschema::is_property_graph(model) {
            let triples: Vec<oxrdf::Triple> = graph
                .triples()
                .map_err(|e| DataGeneratorError::GraphGeneration(e.to_string()))?
                .collect();
            let entities = self.generator.entity_shapes().await;
            let (pg, type_map) = sinks::pg::property_graph(&triples, model, &entities);
            out.property_graph = Some(pg);
            out.type_map = Some(type_map);
        }
        Ok(out)
    }

    /// A ShEx ShapeMap (compact syntax) associating every generated node with
    /// the shape that generated it.
    pub async fn shape_map(&self) -> String {
        self.generator
            .entity_shapes()
            .await
            .into_iter()
            .map(|(node, shape)| format!("<{node}>@<{shape}>"))
            .collect::<Vec<_>>()
            .join(
                ",
",
            )
            + "
"
    }

    /// Run the complete generation pipeline (legacy ShEx support)
    #[cfg(not(target_family = "wasm"))]
    pub async fn run<P: AsRef<Path>>(&mut self, shex_path: P) -> Result<()> {
        tracing::trace!("Loading ShEx schema from: {}", shex_path.as_ref().display());
        self.load_schema(shex_path).await?;

        tracing::trace!("Generating {} entities", self.config.generation.entity_count);
        self.generate().await?;

        tracing::trace!("Data generation completed successfully");
        Ok(())
    }

    /// Run the complete generation pipeline with schema format detection
    #[cfg(not(target_family = "wasm"))]
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
            },
            Some(SchemaFormat::Shacl) => {
                self.load_shacl_schema(schema_path).await?;
            },
            Some(SchemaFormat::PgSchema) => {
                self.load_pgschema_schema(schema_path).await?;
            },
            None => {
                // Auto-detect based on file extension
                self.load_schema_auto(schema_path).await?;
            },
        }

        tracing::info!("Generating {} entities", self.config.generation.entity_count);
        self.generate().await?;

        tracing::info!("Data generation completed successfully");
        Ok(())
    }

    /// Run the complete generation pipeline with automatic schema format detection
    #[cfg(not(target_family = "wasm"))]
    pub async fn run_auto<P: AsRef<Path>>(&mut self, schema_path: P) -> Result<()> {
        self.run_with_format(schema_path, None).await
    }

    // Convert unified model to legacy shape infos for backward compatibility
    fn convert_unified_to_shape_infos(
        &self,
        unified_model: &crate::ir::UnifiedConstraintModel,
    ) -> Vec<crate::ir::shape_info::ShapeInfo> {
        use crate::ir::shape_info::{PropertyInfo, ShapeDependency, ShapeInfo};

        let mut shape_infos = Vec::new();

        for (shape_id, unified_shape) in &unified_model.shapes {
            let mut dependencies = Vec::new();
            let mut properties = Vec::new();

            // Convert properties
            for prop in &unified_shape.properties {
                // A shape reference, or for `sh:class` the shape whose instances
                // carry the class, is generated as a link to one of its instances.
                let target_of = |c: &crate::ir::UnifiedConstraint| match c {
                    crate::ir::UnifiedConstraint::ShapeReference(sr) => Some(sr.clone()),
                    crate::ir::UnifiedConstraint::Class(class) => {
                        unified_model.shape_for_class(class).map(String::from)
                    },
                    _ => None,
                };

                // Extract dependencies from shape references
                for constraint in &prop.constraints {
                    if let Some(target_shape) = target_of(constraint) {
                        dependencies.push(ShapeDependency {
                            target_shape,
                            property: prop.property_iri.clone(),
                            min_cardinality: prop.min_cardinality.map(|c| c as i32),
                            max_cardinality: prop.max_cardinality.map(|c| c as i32),
                        });
                    }
                }

                let datatype = prop.generated_datatype();

                // Extract shape reference
                let shape_ref = prop.constraints.iter().find_map(target_of);

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
            let shape_iri = match rudof_iri::IriS::from_str(shape_id) {
                Ok(iri) => prefixmap::IriRef::Iri(iri),
                Err(_) => {
                    // Fallback to a simple IRI if parsing fails
                    prefixmap::IriRef::Iri(rudof_iri::IriS::new_unchecked("http://example.org/shape"))
                },
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
                target_class: unified_shape.target_class.clone(),
            });
        }

        shape_infos
    }
}
