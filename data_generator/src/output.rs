use crate::config::OutputConfig;
use crate::{Result, DataGeneratorError};
use srdf::srdf_graph::SRDFGraph;
use srdf::{RDFFormat, SRDFBuilder};
use std::fs::File;

/// Handles writing generated data to various output formats
pub struct OutputWriter {
    config: OutputConfig,
}

impl OutputWriter {
    pub fn new(config: &OutputConfig) -> Result<Self> {
        Ok(Self {
            config: config.clone(),
        })
    }

    /// Write the generated graph to the configured output
    pub async fn write_graph(&self, graph: &SRDFGraph) -> Result<()> {
        let format = self.get_rdf_format();
        
        // Create output directory if it doesn't exist
        if let Some(parent) = self.config.path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        // Write the graph
        let mut file = File::create(&self.config.path)?;
        graph.serialize(&format, &mut file)
            .map_err(|e| DataGeneratorError::OutputWriting(format!("Failed to serialize graph: {e}")))?;

        tracing::info!("Graph written to: {}", self.config.path.display());

        // Write statistics if requested
        if self.config.write_stats {
            self.write_statistics(graph).await?;
        }

        // Compress output if requested
        if self.config.compress {
            self.compress_output().await?;
        }

        Ok(())
    }

    /// Write generation statistics
    async fn write_statistics(&self, graph: &SRDFGraph) -> Result<()> {
        let stats_path = self.config.path.with_extension("stats.json");
        let stats = GenerationStatistics::from_graph(graph);
        
        let stats_json = serde_json::to_string_pretty(&stats)?;
        std::fs::write(stats_path, stats_json)?;
        
        Ok(())
    }

    /// Compress the output file
    async fn compress_output(&self) -> Result<()> {
        // TODO: Implement compression (gzip, bzip2, etc.)
        tracing::warn!("Output compression not yet implemented");
        Ok(())
    }

    /// Convert config format to SRDF format
    fn get_rdf_format(&self) -> RDFFormat {
        match self.config.format {
            crate::config::OutputFormat::Turtle => RDFFormat::Turtle,
            crate::config::OutputFormat::NTriples => RDFFormat::NTriples,
            crate::config::OutputFormat::JsonLd => RDFFormat::Turtle, // Fallback to Turtle for now
            crate::config::OutputFormat::RdfXml => RDFFormat::RDFXML,
        }
    }
}

/// Statistics about the generated data
#[derive(Debug, serde::Serialize)]
pub struct GenerationStatistics {
    pub total_triples: usize,
    pub total_subjects: usize,
    pub total_predicates: usize,
    pub total_objects: usize,
    pub generation_time: Option<String>,
    pub shape_counts: std::collections::HashMap<String, usize>,
}

impl GenerationStatistics {
    pub fn from_graph(graph: &SRDFGraph) -> Self {
        let total_triples = graph.len();
        
        // For now, provide basic statistics
        // TODO: Implement proper statistics extraction once we have access to graph internals
        let shape_counts = std::collections::HashMap::new();

        Self {
            total_triples,
            total_subjects: 0, // Will be calculated properly later
            total_predicates: 0, // Will be calculated properly later  
            total_objects: 0, // Will be calculated properly later
            generation_time: None,
            shape_counts,
        }
    }
}
