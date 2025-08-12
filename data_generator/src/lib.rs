pub mod config;
pub mod field_generators;
pub mod shape_processing;
pub mod parallel_generation;
pub mod output;
pub mod errors;

pub use config::GeneratorConfig;
pub use errors::{DataGeneratorError, Result};

use crate::shape_processing::ShapeProcessor;
use crate::parallel_generation::ParallelGenerator;
use crate::output::OutputWriter;
use std::path::Path;

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

    /// Generate synthetic data and write to output
    pub async fn generate(&mut self) -> Result<()> {
        let start_time = std::time::Instant::now();
        let graph = self.generator.generate_data(&self.config.generation).await?;
        let generation_time = start_time.elapsed();
        
        self.writer.write_graph_with_timing(&graph, Some(generation_time)).await?;
        Ok(())
    }

    /// Run the complete generation pipeline
    pub async fn run<P: AsRef<Path>>(&mut self, shex_path: P) -> Result<()> {
        tracing::info!("Loading ShEx schema from: {}", shex_path.as_ref().display());
        self.load_schema(shex_path).await?;
        
        tracing::info!("Generating {} entities", self.config.generation.entity_count);
        self.generate().await?;
        
        tracing::info!("Data generation completed successfully");
        Ok(())
    }
}
