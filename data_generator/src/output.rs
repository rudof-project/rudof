use crate::config::OutputConfig;
use crate::{Result, DataGeneratorError};
use srdf::srdf_graph::SRDFGraph;
use srdf::{RDFFormat, SRDFBuilder, Query, Triple};
use std::fs::File;
use std::path::PathBuf;

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
        self.write_graph_with_timing(graph, None).await
    }

    /// Write the generated graph to the configured output with timing information
    pub async fn write_graph_with_timing(&self, graph: &SRDFGraph, generation_time: Option<std::time::Duration>) -> Result<()> {
        if self.config.parallel_writing {
            self.write_graph_parallel(graph, generation_time).await
        } else {
            self.write_graph_sequential(graph, generation_time).await
        }
    }

    /// Write the graph using sequential (traditional) method
    async fn write_graph_sequential(&self, graph: &SRDFGraph, generation_time: Option<std::time::Duration>) -> Result<()> {
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
            self.write_statistics(graph, generation_time).await?;
        }

        // Compress output if requested
        if self.config.compress {
            self.compress_output().await?;
        }

        Ok(())
    }

    /// Write the graph using parallel method - splits data across multiple files
    async fn write_graph_parallel(&self, graph: &SRDFGraph, generation_time: Option<std::time::Duration>) -> Result<()> {
        let start_time = std::time::Instant::now();
        
        // Create output directory if it doesn't exist
        if let Some(parent) = self.config.path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        // Collect all triples first
        let all_triples = graph.triples()
            .map_err(|e| DataGeneratorError::OutputWriting(format!("Failed to collect triples: {e}")))?
            .collect::<Vec<_>>();

        let total_triples = all_triples.len();
        
        // Get optimal file count (either user-specified or auto-detected)
        let optimal_file_count = self.config.get_optimal_file_count(total_triples);
        let chunk_size = (total_triples + optimal_file_count - 1) / optimal_file_count;

        if chunk_size == 0 {
            // No triples to write
            tracing::warn!("No triples to write");
            return Ok(());
        }

        // Split triples into chunks for parallel writing
        let triple_chunks: Vec<_> = all_triples.chunks(chunk_size).collect();

        tracing::info!("Writing {} triples in {} parallel files ({} triples per file)", 
                      total_triples, triple_chunks.len(), chunk_size);

        // Write chunks in parallel
        let file_tasks: Vec<_> = triple_chunks.into_iter().enumerate().map(|(index, chunk)| {
            let format = self.get_rdf_format();
            let output_path = self.get_parallel_file_path(index);
            let chunk_triples = chunk.to_vec();
            
            tokio::spawn(async move {
                Self::write_triple_chunk(chunk_triples, format, output_path).await
            })
        }).collect();

        // Wait for all file writing tasks to complete
        let write_results = futures::future::try_join_all(file_tasks).await
            .map_err(|e| DataGeneratorError::OutputWriting(format!("Parallel write task failed: {e}")))?;

        // Check all writes succeeded
        for result in write_results {
            result?;
        }

        let write_time = start_time.elapsed();
        tracing::info!("Parallel writing completed in {:?}", write_time);

        // Write statistics if requested
        if self.config.write_stats {
            self.write_statistics(graph, generation_time).await?;
        }

        // Compress output files if requested
        if self.config.compress {
            self.compress_parallel_output().await?;
        }

        // Create a manifest file listing all the parallel files
        self.create_parallel_manifest(optimal_file_count).await?;

        Ok(())
    }

    /// Write a chunk of triples to a file
    async fn write_triple_chunk(triples: Vec<oxrdf::Triple>, format: RDFFormat, output_path: PathBuf) -> Result<()> {
        // Create a temporary graph for this chunk
        let mut chunk_graph = SRDFGraph::default();
        
        for triple in triples {
            chunk_graph.add_triple(
                triple.subject,
                triple.predicate,
                triple.object,
            ).map_err(|e| DataGeneratorError::OutputWriting(format!("Failed to add triple to chunk: {e}")))?;
        }

        // Write the chunk to file
        let mut file = File::create(&output_path)?;
        chunk_graph.serialize(&format, &mut file)
            .map_err(|e| DataGeneratorError::OutputWriting(format!("Failed to serialize chunk: {e}")))?;

        tracing::debug!("Chunk written to: {}", output_path.display());
        Ok(())
    }

    /// Generate file path for parallel file with given index
    fn get_parallel_file_path(&self, index: usize) -> PathBuf {
        let stem = self.config.path.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("output");
        let extension = self.config.path.extension()
            .and_then(|s| s.to_str())
            .unwrap_or("ttl");
        
        let parent = self.config.path.parent().unwrap_or_else(|| std::path::Path::new("."));
        parent.join(format!("{}_part_{:03}.{}", stem, index + 1, extension))
    }

    /// Create a manifest file listing all parallel output files
    async fn create_parallel_manifest(&self, actual_file_count: usize) -> Result<()> {
        let manifest_path = self.config.path.with_extension("manifest.txt");
        let mut manifest_content = String::new();
        
        manifest_content.push_str(&format!("# Data Generator Parallel Output Manifest\n"));
        manifest_content.push_str(&format!("# Generated on: {}\n", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")));
        manifest_content.push_str(&format!("# Total parallel files: {}\n\n", actual_file_count));

        for i in 0..actual_file_count {
            let file_path = self.get_parallel_file_path(i);
            if file_path.exists() {
                manifest_content.push_str(&format!("{}\n", file_path.display()));
            }
        }

        tokio::fs::write(manifest_path, manifest_content).await
            .map_err(|e| DataGeneratorError::OutputWriting(format!("Failed to write manifest: {e}")))?;

        Ok(())
    }

    /// Compress parallel output files
    async fn compress_parallel_output(&self) -> Result<()> {
        // TODO: Implement compression for parallel files
        tracing::warn!("Parallel output compression not yet implemented");
        Ok(())
    }

    /// Write generation statistics
    async fn write_statistics(&self, graph: &SRDFGraph, generation_time: Option<std::time::Duration>) -> Result<()> {
        let stats_path = self.config.path.with_extension("stats.json");
        let mut stats = GenerationStatistics::from_graph(graph);
        
        // Add timing information if provided
        if let Some(duration) = generation_time {
            stats = stats.with_timing(duration);
        }
        
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
        use srdf::Query;
        use std::collections::HashSet;
        
        let total_triples = graph.len();
        
        // Calculate unique subjects, predicates, and objects
        let mut subjects = HashSet::new();
        let mut predicates = HashSet::new();
        let mut objects = HashSet::new();
        let mut shape_counts = std::collections::HashMap::new();
        
        // Iterate through all triples to collect statistics
        if let Ok(triples) = graph.triples() {
            for triple in triples {
                subjects.insert(triple.subj().to_string());
                let pred_str = triple.pred().to_string();
                predicates.insert(pred_str.clone());
                objects.insert(triple.obj().to_string());
                
                // Count shape types (look for rdf:type triples)
                // The predicate comes with angle brackets around it
                if pred_str == "<http://www.w3.org/1999/02/22-rdf-syntax-ns#type>" {
                    let shape_type = triple.obj().to_string();
                    // Remove angle brackets if present
                    let shape_type = shape_type.trim_start_matches('<').trim_end_matches('>');
                    *shape_counts.entry(shape_type.to_string()).or_insert(0) += 1;
                }
            }
        }

        Self {
            total_triples,
            total_subjects: subjects.len(),
            total_predicates: predicates.len(),
            total_objects: objects.len(),
            generation_time: None,
            shape_counts,
        }
    }
    
    pub fn with_timing(mut self, duration: std::time::Duration) -> Self {
        self.generation_time = Some(format!("{}ms", duration.as_millis()));
        self
    }
}
