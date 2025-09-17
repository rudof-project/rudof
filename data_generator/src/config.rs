use crate::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Main configuration structure for the data generator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratorConfig {
    pub generation: GenerationConfig,
    pub field_generators: FieldGeneratorConfig,
    pub output: OutputConfig,
    pub parallel: ParallelConfig,
}

/// Configuration for data generation parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationConfig {
    /// Number of entities to generate
    pub entity_count: usize,
    /// Random seed for reproducible generation
    pub seed: Option<u64>,
    /// Distribution strategy for entities across shapes
    pub entity_distribution: EntityDistribution,
    /// Cardinality generation strategy
    pub cardinality_strategy: CardinalityStrategy,
    /// Schema format specification
    pub schema_format: Option<SchemaFormat>,
}

/// Schema format for the generator
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum SchemaFormat {
    ShEx,
    SHACL,
}

/// How to distribute entities across different shapes
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EntityDistribution {
    /// Equal distribution across all shapes
    Equal,
    /// Weighted distribution based on shape importance
    Weighted(HashMap<String, f64>),
    /// Custom distribution with explicit counts per shape
    Custom(HashMap<String, usize>),
}

/// Strategy for handling cardinalities in relationships
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum CardinalityStrategy {
    /// Use minimum cardinalities
    Minimum,
    /// Use maximum cardinalities (with reasonable bounds)
    Maximum,
    /// Random within cardinality bounds
    Random,
    /// Balanced approach favoring realistic distributions
    Balanced,
}

/// Configuration for field value generators
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldGeneratorConfig {
    /// Default generator settings
    pub default: DefaultFieldConfig,
    /// Per-datatype specific configurations
    pub datatypes: HashMap<String, DatatypeConfig>,
    /// Per-property specific configurations
    pub properties: HashMap<String, PropertyConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultFieldConfig {
    /// Locale for text generation (e.g., "en", "es", "fr")
    pub locale: String,
    /// Quality level for generated data (low, medium, high)
    pub quality: DataQuality,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum DataQuality {
    Low,    // Simple random data
    Medium, // Realistic patterns
    High,   // Complex realistic data with correlations
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatatypeConfig {
    /// Generator type to use for this datatype
    pub generator: String,
    /// Additional parameters for the generator
    pub parameters: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyConfig {
    /// Generator type to use for this property
    pub generator: String,
    /// Additional parameters for the generator
    pub parameters: HashMap<String, serde_json::Value>,
    /// Value templates or patterns
    pub templates: Option<Vec<String>>,
}

/// Output configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputConfig {
    /// Output file path
    pub path: PathBuf,
    /// Output format (turtle, ntriples, jsonld, etc.)
    pub format: OutputFormat,
    /// Whether to compress output
    pub compress: bool,
    /// Write statistics file
    pub write_stats: bool,
    /// Enable parallel writing to multiple files
    pub parallel_writing: bool,
    /// Number of parallel output files (when parallel_writing is true)
    /// If set to 0, the system will automatically determine the optimal count
    pub parallel_file_count: usize,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum OutputFormat {
    Turtle,
    NTriples,
    // NOTE: Only Turtle and NTriples are supported. 
    // JsonLd and RdfXml removed to avoid serialization issues.
}

/// Parallelization configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParallelConfig {
    /// Number of worker threads (None = auto-detect)
    pub worker_threads: Option<usize>,
    /// Batch size for parallel processing
    pub batch_size: usize,
    /// Enable parallel shape processing
    pub parallel_shapes: bool,
    /// Enable parallel field generation
    pub parallel_fields: bool,
}

impl Default for GeneratorConfig {
    fn default() -> Self {
        Self {
            generation: GenerationConfig {
                entity_count: 1000,
                seed: None,
                entity_distribution: EntityDistribution::Equal,
                cardinality_strategy: CardinalityStrategy::Balanced,
                schema_format: None, // Auto-detect
            },
            field_generators: FieldGeneratorConfig {
                default: DefaultFieldConfig {
                    locale: "en".to_string(),
                    quality: DataQuality::Medium,
                },
                datatypes: HashMap::new(),
                properties: HashMap::new(),
            },
            output: OutputConfig {
                path: PathBuf::from("output.ttl"),
                format: OutputFormat::Turtle,
                compress: false,
                write_stats: true,
                parallel_writing: false,
                parallel_file_count: 0, // 0 means auto-detect optimal count
            },
            parallel: ParallelConfig {
                worker_threads: None,
                batch_size: 100,
                parallel_shapes: true,
                parallel_fields: true,
            },
        }
    }
}

impl OutputConfig {
    /// Calculate optimal parallel file count based on dataset size and system capabilities
    pub fn get_optimal_file_count(&self, total_triples: usize) -> usize {
        // If user explicitly set a count, use it
        if self.parallel_file_count > 0 {
            return self.parallel_file_count;
        }

        // If parallel writing is disabled, always use 1 file
        if !self.parallel_writing {
            return 1;
        }

        // Detect CPU cores (with fallback to 4)
        let cpu_count = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(4);

        // Calculate optimal file count based on dataset size
        let optimal_count = match total_triples {
            0..=1000 => 1,                           // Small datasets: single file
            1001..=5000 => cpu_count.min(4),         // Small-medium: up to 4 files
            5001..=50000 => (cpu_count * 2).min(8),  // Medium: up to 2x CPU cores, max 8
            _ => (cpu_count * 2).min(16),            // Large: up to 2x CPU cores, max 16
        };

        tracing::info!(
            "Auto-detected optimal parallel file count: {} (CPU cores: {}, triples: {})",
            optimal_count, cpu_count, total_triples
        );

        optimal_count
    }
}

impl GeneratorConfig {
    /// Load configuration from a TOML file
    pub fn from_toml_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: Self = toml::from_str(&content)?;
        Ok(config)
    }

    /// Load configuration from a JSON file
    pub fn from_json_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: Self = serde_json::from_str(&content)?;
        Ok(config)
    }

    /// Save configuration to a TOML file
    pub fn to_toml_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let content = toml::to_string_pretty(self)
            .map_err(|e| crate::DataGeneratorError::Config(e.to_string()))?;
        std::fs::write(path, content)?;
        Ok(())
    }

    /// Merge with command-line overrides
    pub fn merge_cli_overrides(&mut self, entity_count: Option<usize>, output_path: Option<PathBuf>, seed: Option<u64>) {
        if let Some(count) = entity_count {
            self.generation.entity_count = count;
        }
        if let Some(path) = output_path {
            self.output.path = path;
        }
        if let Some(seed_val) = seed {
            self.generation.seed = Some(seed_val);
        }
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<()> {
        if self.generation.entity_count == 0 {
            return Err(crate::DataGeneratorError::Config(
                "entity_count must be greater than 0".to_string()
            ));
        }

        if self.parallel.batch_size == 0 {
            return Err(crate::DataGeneratorError::Config(
                "batch_size must be greater than 0".to_string()
            ));
        }

        // Validate entity distribution weights sum to reasonable values
        if let EntityDistribution::Weighted(ref weights) = self.generation.entity_distribution {
            let total: f64 = weights.values().sum();
            if total <= 0.0 {
                return Err(crate::DataGeneratorError::Config(
                    "Weighted distribution weights must sum to a positive value".to_string()
                ));
            }
        }

        Ok(())
    }
}
