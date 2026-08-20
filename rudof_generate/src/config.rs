use crate::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Main configuration structure for the data generator
///
/// Every field, and every field of the structs it nests, has a sensible
/// default (see the respective `Default` impls), so a config file only
/// needs to set what it wants to override — down to an empty file, which
/// is equivalent to [`GeneratorConfig::default()`].
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct GeneratorConfig {
    pub generation: GenerationConfig,
    pub field_generators: FieldGeneratorConfig,
    pub output: OutputConfig,
    pub parallel: ParallelConfig,
}

/// Configuration for data generation parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
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

    // --- Coherence Control Parameters ---
    /// Probability (0.0 to 1.0) that a property will be included
    pub property_fill_probability: f64,

    /// Whether to ignore minimum cardinality constraints (treat minCount as 0)
    pub ignore_min_cardinality: bool,

    /// Maximum number of properties per instance (0 = unlimited)
    pub max_properties_per_instance: usize,

    /// Strategy for selecting properties when count is limited
    pub property_selection_strategy: PropertySelectionStrategy,

    /// Variance in property count (0.0 to 1.0)
    pub property_count_variance: f64,

    /// List of properties to explicitly exclude
    pub excluded_properties: Vec<String>,

    /// Per-type coherence settings overrides
    pub type_overrides: HashMap<String, TypeOverrideConfig>,
}

impl Default for GenerationConfig {
    fn default() -> Self {
        Self {
            entity_count: 1000,
            seed: None,
            entity_distribution: EntityDistribution::default(),
            cardinality_strategy: CardinalityStrategy::default(),
            schema_format: None,
            property_fill_probability: 1.0,
            ignore_min_cardinality: false,
            max_properties_per_instance: 0,
            property_selection_strategy: PropertySelectionStrategy::default(),
            property_count_variance: 0.0,
            excluded_properties: Vec::new(),
            type_overrides: HashMap::new(),
        }
    }
}

/// Strategy for selecting which properties to keep when limiting count
#[derive(Default, Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum PropertySelectionStrategy {
    #[default]
    All,
    Random,
    Weighted,
}

/// Overrides for coherence settings per type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeOverrideConfig {
    pub property_fill_probability: Option<f64>,
    pub ignore_min_cardinality: Option<bool>,
    pub max_properties_per_instance: Option<usize>,
    pub property_selection_strategy: Option<PropertySelectionStrategy>,
    pub property_count_variance: Option<f64>,
    pub excluded_properties: Option<Vec<String>>,
}

/// Schema format for the generator
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum SchemaFormat {
    ShEx,
    Shacl,
}

/// How to distribute entities across different shapes
///
/// In TOML/JSON, the payload-carrying variants are externally tagged, e.g.:
/// ```toml
/// [generation.entity_distribution]
/// Weighted = { "http://example.org/Person" = 0.5, "http://example.org/Course" = 0.5 }
/// ```
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub enum EntityDistribution {
    /// Equal distribution across all shapes
    #[default]
    Equal,
    /// Weighted distribution based on shape importance
    Weighted(HashMap<String, f64>),
    /// Custom distribution with explicit counts per shape
    Custom(HashMap<String, usize>),
}

/// Strategy for handling cardinalities in relationships
#[derive(Default, Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum CardinalityStrategy {
    /// Use minimum cardinalities
    Minimum,
    /// Use maximum cardinalities (with reasonable bounds)
    Maximum,
    /// Random within cardinality bounds
    Random,
    /// Balanced approach favoring realistic distributions
    #[default]
    Balanced,
}

/// Configuration for field value generators
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct FieldGeneratorConfig {
    /// Default generator settings
    pub default: DefaultFieldConfig,
    /// Per-datatype specific configurations
    pub datatypes: HashMap<String, DatatypeConfig>,
    /// Per-property specific configurations
    pub properties: HashMap<String, PropertyConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct DefaultFieldConfig {
    /// Locale for text generation (e.g., "en", "es", "fr")
    pub locale: String,
    /// Quality level for generated data (low, medium, high)
    pub quality: DataQuality,
}

impl Default for DefaultFieldConfig {
    fn default() -> Self {
        Self {
            locale: "en".to_string(),
            quality: DataQuality::default(),
        }
    }
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq)]
pub enum DataQuality {
    Low, // Simple random data
    #[default]
    Medium, // Realistic patterns
    High, // Complex realistic data with correlations
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
#[serde(default)]
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

impl Default for OutputConfig {
    fn default() -> Self {
        Self {
            path: PathBuf::from("output.ttl"),
            format: OutputFormat::default(),
            compress: false,
            write_stats: true,
            parallel_writing: false,
            parallel_file_count: 0,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq)]
pub enum OutputFormat {
    #[default]
    Turtle,
    NTriples,
    // NOTE: Only Turtle and NTriples are supported.
    // JsonLd and RdfXml removed to avoid serialization issues.
}

/// Parallelization configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
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

impl Default for ParallelConfig {
    fn default() -> Self {
        Self {
            worker_threads: None,
            batch_size: 100,
            parallel_shapes: true,
            parallel_fields: true,
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
        let cpu_count = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(4);

        // Calculate optimal file count based on dataset size
        let optimal_count = match total_triples {
            0..=1000 => 1,                          // Small datasets: single file
            1001..=5000 => cpu_count.min(4),        // Small-medium: up to 4 files
            5001..=50000 => (cpu_count * 2).min(8), // Medium: up to 2x CPU cores, max 8
            _ => (cpu_count * 2).min(16),           // Large: up to 2x CPU cores, max 16
        };

        tracing::info!(
            "Auto-detected optimal parallel file count: {} (CPU cores: {}, triples: {})",
            optimal_count,
            cpu_count,
            total_triples
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
        let content = toml::to_string_pretty(self).map_err(|e| crate::DataGeneratorError::Config(e.to_string()))?;
        std::fs::write(path, content)?;
        Ok(())
    }

    /// Merge with command-line overrides
    pub fn merge_cli_overrides(
        &mut self,
        entity_count: Option<usize>,
        output_path: Option<PathBuf>,
        seed: Option<u64>,
    ) {
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
                "entity_count must be greater than 0".to_string(),
            ));
        }

        if self.parallel.batch_size == 0 {
            return Err(crate::DataGeneratorError::Config(
                "batch_size must be greater than 0".to_string(),
            ));
        }

        // Validate entity distribution weights sum to reasonable values
        if let EntityDistribution::Weighted(ref weights) = self.generation.entity_distribution {
            let total: f64 = weights.values().sum();
            if total <= 0.0 {
                return Err(crate::DataGeneratorError::Config(
                    "Weighted distribution weights must sum to a positive value".to_string(),
                ));
            }
        }

        // Validate coherence parameters
        if self.generation.property_fill_probability < 0.0 || self.generation.property_fill_probability > 1.0 {
            return Err(crate::DataGeneratorError::Config(
                "property_fill_probability must be between 0.0 and 1.0".to_string(),
            ));
        }

        if self.generation.property_count_variance < 0.0 || self.generation.property_count_variance > 1.0 {
            return Err(crate::DataGeneratorError::Config(
                "property_count_variance must be between 0.0 and 1.0".to_string(),
            ));
        }

        Ok(())
    }
}
