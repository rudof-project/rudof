//! Python bindings for the Rudof synthetic data generation library.
//!
//! This module provides Python wrappers for configuring and running the Rudof data generator

use pyo3::{PyErr, PyResult, pyclass, pymethods};
use std::path::Path;

/// Python wrapper for `GeneratorConfig` from `rudof_generate`.
///
/// Provides access to configuration options for synthetic data generation.
#[pyclass(name = "GeneratorConfig")]
pub struct PyGeneratorConfig {
    /// The internal Rust `GeneratorConfig` object.
    inner: rudof_generate::GeneratorConfig,
}

#[pymethods]
impl PyGeneratorConfig {
    #[new]
    pub fn __init__() -> Self {
        Self {
            inner: rudof_generate::GeneratorConfig::default(),
        }
    }

    /// Load configuration from a TOML file.
    ///
    /// Args:
    ///     path (str): Path to the TOML configuration file.
    ///
    /// Returns:
    ///     GeneratorConfig: Loaded configuration object.
    ///
    /// Raises:
    ///     ValueError: If the file cannot be read or parsed.
    #[staticmethod]
    pub fn from_toml_file(path: &str) -> PyResult<Self> {
        let config = rudof_generate::GeneratorConfig::from_toml_file(path)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", e)))?;
        Ok(Self { inner: config })
    }

    /// Load configuration from a JSON file.
    ///
    /// Args:
    ///     path (str): Path to the JSON configuration file.
    ///
    /// Returns:
    ///     GeneratorConfig: Loaded configuration object.
    ///
    /// Raises:
    ///     ValueError: If the file cannot be read or parsed.
    #[staticmethod]
    pub fn from_json_file(path: &str) -> PyResult<Self> {
        let config = rudof_generate::GeneratorConfig::from_json_file(path)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", e)))?;
        Ok(Self { inner: config })
    }

    /// Save configuration to a TOML file.
    ///
    /// Args:
    ///     path (str): Path where the TOML file will be saved.
    ///
    /// Raises:
    ///     ValueError: If writing to the file fails.
    pub fn to_toml_file(&self, path: &str) -> PyResult<()> {
        self.inner
            .to_toml_file(path)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", e)))?;
        Ok(())
    }

    /// Set the number of entities to generate.
    ///
    /// Args:
    ///     count (int): Number of entities to generate.
    pub fn set_entity_count(&mut self, count: usize) {
        self.inner.generation.entity_count = count;
    }

    /// Get the number of entities to generate.
    ///
    /// Returns:
    ///     int: Number of entities.
    pub fn get_entity_count(&self) -> usize {
        self.inner.generation.entity_count
    }

    /// Set the random seed for reproducible generation.
    ///
    /// Args:
    ///     seed (Optional[int]): Seed value.
    pub fn set_seed(&mut self, seed: Option<u64>) {
        self.inner.generation.seed = seed;
    }

    /// Get the random seed.
    ///
    /// Returns:
    ///     Optional[int]: Seed value.
    pub fn get_seed(&self) -> Option<u64> {
        self.inner.generation.seed
    }

    /// Set the output file path.
    ///
    /// Args:
    ///     path (str): Path to output file.
    pub fn set_output_path(&mut self, path: &str) {
        self.inner.output.path = Path::new(path).to_path_buf();
    }

    /// Get the output file path.
    ///
    /// Returns:
    ///     str: Output path.
    pub fn get_output_path(&self) -> String {
        self.inner.output.path.display().to_string()
    }

    /// Set the output format.
    ///
    /// Args:
    ///     format (OutputFormat): Desired output format.
    pub fn set_output_format(&mut self, format: PyOutputFormat) -> PyResult<()> {
        self.inner.output.format = format.into();
        Ok(())
    }

    /// Set the schema format.
    ///
    /// Args:
    ///     format (Optional[SchemaFormat]): Desired schema format.
    pub fn set_schema_format(&mut self, format: Option<PySchemaFormat>) {
        self.inner.generation.schema_format = format.map(|f| f.into());
    }

    /// Set the cardinality strategy.
    ///
    /// Args:
    ///     strategy (CardinalityStrategy): Strategy for cardinalities.
    pub fn set_cardinality_strategy(&mut self, strategy: PyCardinalityStrategy) {
        self.inner.generation.cardinality_strategy = strategy.into();
    }

    /// Enable or disable compression.
    ///
    /// Args:
    ///     compress (bool): Whether to compress the output.
    pub fn set_compress(&mut self, compress: bool) {
        self.inner.output.compress = compress;
    }

    /// Enable or disable writing statistics.
    ///
    /// Args:
    ///     write_stats (bool): Whether to write statistics.
    pub fn set_write_stats(&mut self, write_stats: bool) {
        self.inner.output.write_stats = write_stats;
    }

    /// Enable or disable parallel writing.
    ///
    /// Args:
    ///     parallel_writing (bool): Whether to write output in parallel.
    pub fn set_parallel_writing(&mut self, parallel_writing: bool) {
        self.inner.output.parallel_writing = parallel_writing;
    }

    /// Set the number of parallel output files.
    ///
    /// Args:
    ///     count (int): Number of files.
    pub fn set_parallel_file_count(&mut self, count: usize) {
        self.inner.output.parallel_file_count = count;
    }

    /// Set the number of worker threads.
    ///
    /// Args:
    ///     threads (Optional[int]): Number of threads.
    pub fn set_worker_threads(&mut self, threads: Option<usize>) {
        self.inner.parallel.worker_threads = threads;
    }

    /// Set the batch size for parallel processing.
    ///
    /// Args:
    ///     batch_size (int): Batch size.
    pub fn set_batch_size(&mut self, batch_size: usize) {
        self.inner.parallel.batch_size = batch_size;
    }

    /// Enable or disable parallel shape processing.
    ///
    /// Args:
    ///     enabled (bool): Whether parallel shape processing is enabled.
    pub fn set_parallel_shapes(&mut self, enabled: bool) {
        self.inner.parallel.parallel_shapes = enabled;
    }

    /// Get whether parallel shape processing is enabled.
    ///
    /// Returns:
    ///     bool: True if enabled.
    pub fn get_parallel_shapes(&self) -> bool {
        self.inner.parallel.parallel_shapes
    }

    /// Enable or disable parallel field generation.
    ///
    /// Args:
    ///     enabled (bool): Whether parallel field generation is enabled.
    pub fn set_parallel_fields(&mut self, enabled: bool) {
        self.inner.parallel.parallel_fields = enabled;
    }

    /// Get whether parallel field generation is enabled.
    ///
    /// Returns:
    ///     bool: True if enabled.
    pub fn get_parallel_fields(&self) -> bool {
        self.inner.parallel.parallel_fields
    }

    /// Set entity distribution strategy.
    ///
    /// Args:
    ///     distribution (EntityDistribution): Entity distribution strategy.
    pub fn set_entity_distribution(&mut self, distribution: PyEntityDistribution) {
        self.inner.generation.entity_distribution = distribution.into();
    }

    /// Set locale for field generation.
    ///
    /// Args:
    ///     locale (str): Locale string (e.g., "en", "es").
    pub fn set_locale(&mut self, locale: &str) {
        self.inner.field_generators.default.locale = locale.to_string();
    }

    /// Get locale for field generation.
    ///
    /// Returns:
    ///     str: Current locale.
    pub fn get_locale(&self) -> String {
        self.inner.field_generators.default.locale.clone()
    }

    /// Set data quality level.
    ///
    /// Args:
    ///     quality (DataQuality): Data quality (Low, Medium, High).
    pub fn set_data_quality(&mut self, quality: PyDataQuality) {
        self.inner.field_generators.default.quality = quality.into();
    }

    /// Get whether output is compressed.
    ///
    /// Returns:
    ///     bool: True if compression is enabled.
    pub fn get_compress(&self) -> bool {
        self.inner.output.compress
    }

    /// Get whether statistics will be written.
    ///
    /// Returns:
    ///     bool: True if statistics are written.
    pub fn get_write_stats(&self) -> bool {
        self.inner.output.write_stats
    }

    /// Get whether parallel writing is enabled.
    ///
    /// Returns:
    ///     bool: True if enabled.
    pub fn get_parallel_writing(&self) -> bool {
        self.inner.output.parallel_writing
    }

    /// Get number of parallel output files.
    ///
    /// Returns:
    ///     int: Number of files.
    pub fn get_parallel_file_count(&self) -> usize {
        self.inner.output.parallel_file_count
    }

    /// Get number of worker threads.
    ///
    /// Returns:
    ///     Optional[int]: Worker threads.
    pub fn get_worker_threads(&self) -> Option<usize> {
        self.inner.parallel.worker_threads
    }

    /// Get batch size for parallel processing.
    ///
    /// Returns:
    ///     int: Batch size.
    pub fn get_batch_size(&self) -> usize {
        self.inner.parallel.batch_size
    }

    /// Validate the configuration.
    ///
    /// Raises:
    ///     ValueError: If configuration is invalid.
    pub fn validate(&self) -> PyResult<()> {
        self.inner
            .validate()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", e)))?;
        Ok(())
    }

    /// Convert configuration to string.
    ///
    /// Returns:
    ///     str: Debug string of the configuration.
    pub fn show(&self) -> String {
        format!("{:?}", self.inner)
    }
}

/// Schema format for the generator.
///
/// Represents the supported schema formats that can be used
/// to drive the data generation process.
#[pyclass(eq, eq_int, name = "SchemaFormat")]
#[derive(PartialEq, Clone, Copy)]
pub enum PySchemaFormat {
    ShEx,
    Shacl,
}

impl From<PySchemaFormat> for rudof_generate::SchemaFormat {
    fn from(val: PySchemaFormat) -> Self {
        match val {
            PySchemaFormat::ShEx => rudof_generate::SchemaFormat::ShEx,
            PySchemaFormat::Shacl => rudof_generate::SchemaFormat::Shacl,
        }
    }
}

/// Output format for generated data.
///
/// Defines the RDF serialization format used for generated output.
#[pyclass(eq, eq_int, name = "OutputFormat")]
#[derive(PartialEq, Clone, Copy)]
pub enum PyOutputFormat {
    Turtle,
    NTriples,
}

impl From<PyOutputFormat> for rudof_generate::config::OutputFormat {
    fn from(val: PyOutputFormat) -> Self {
        match val {
            PyOutputFormat::Turtle => rudof_generate::config::OutputFormat::Turtle,
            PyOutputFormat::NTriples => rudof_generate::config::OutputFormat::NTriples,
        }
    }
}

/// Strategy for handling cardinalities in relationships.
///
/// Determines how many relationships are generated when constraints
/// define minimum and maximum cardinalities.
#[pyclass(eq, eq_int, name = "CardinalityStrategy")]
#[derive(PartialEq, Clone, Copy)]
pub enum PyCardinalityStrategy {
    /// Always use the minimum cardinality.
    Minimum,
    /// Always use the maximum cardinality.
    Maximum,
    /// Choose a random value within the allowed range.
    Random,
    /// Balance values across the allowed range.
    Balanced,
}

impl From<PyCardinalityStrategy> for rudof_generate::config::CardinalityStrategy {
    fn from(val: PyCardinalityStrategy) -> Self {
        match val {
            PyCardinalityStrategy::Minimum => rudof_generate::config::CardinalityStrategy::Minimum,
            PyCardinalityStrategy::Maximum => rudof_generate::config::CardinalityStrategy::Maximum,
            PyCardinalityStrategy::Random => rudof_generate::config::CardinalityStrategy::Random,
            PyCardinalityStrategy::Balanced => rudof_generate::config::CardinalityStrategy::Balanced,
        }
    }
}

/// Entity distribution strategy.
///
/// Defines how entities are distributed across shapes during generation.
#[pyclass(eq, eq_int, name = "EntityDistribution")]
#[derive(PartialEq, Clone, Copy)]
pub enum PyEntityDistribution {
    /// Equal distribution across all shapes.
    Equal,
}

impl From<PyEntityDistribution> for rudof_generate::config::EntityDistribution {
    fn from(val: PyEntityDistribution) -> Self {
        match val {
            PyEntityDistribution::Equal => rudof_generate::config::EntityDistribution::Equal,
        }
    }
}

/// Data quality level for generated data.
///
/// Controls how realistic and complex the generated data should be.
#[pyclass(eq, eq_int, name = "DataQuality")]
#[derive(PartialEq, Clone, Copy)]
pub enum PyDataQuality {
    /// Simple random data.
    Low,
    /// Realistic patterns.
    Medium,
    /// Complex realistic data with correlations.
    High,
}

impl From<PyDataQuality> for rudof_generate::config::DataQuality {
    fn from(val: PyDataQuality) -> Self {
        match val {
            PyDataQuality::Low => rudof_generate::config::DataQuality::Low,
            PyDataQuality::Medium => rudof_generate::config::DataQuality::Medium,
            PyDataQuality::High => rudof_generate::config::DataQuality::High,
        }
    }
}

/// Main data generator class.
///
/// Provides an interface to load schemas and generate synthetic RDF data.
#[pyclass(name = "DataGenerator")]
pub struct PyDataGenerator {
    /// Internal Rust data generator instance.
    inner: Option<rudof_generate::DataGenerator>,
    /// Tokio runtime used to execute async operations.
    runtime: tokio::runtime::Runtime,
}

#[pymethods]
impl PyDataGenerator {
    /// Create a new DataGenerator with the given configuration.
    ///
    /// Args:
    ///     config (GeneratorConfig): Configuration object.
    ///
    /// Returns:
    ///     DataGenerator: Initialized generator instance.
    ///
    /// Raises:
    ///     RuntimeError: If the async runtime cannot be created.
    ///     ValueError: If the generator cannot be initialized.
    #[new]
    pub fn __init__(config: &PyGeneratorConfig) -> PyResult<Self> {
        let runtime = tokio::runtime::Runtime::new()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{}", e)))?;

        let generator = rudof_generate::DataGenerator::new(config.inner.clone())
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", e)))?;

        Ok(Self {
            inner: Some(generator),
            runtime,
        })
    }

    /// Load and process a ShEx schema file.
    ///
    /// Args:
    ///     path (str): Path to the ShEx schema file.
    ///
    /// Raises:
    ///     RuntimeError: If the generator is not initialized.
    ///     ValueError: If the schema cannot be loaded or parsed.
    pub fn load_shex_schema(&mut self, path: &str) -> PyResult<()> {
        let generator = self
            .inner
            .as_mut()
            .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Generator not initialized"))?;

        self.runtime
            .block_on(generator.load_shex_schema(path))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", e)))?;

        Ok(())
    }

    /// Load and process a SHACL schema file.
    ///
    /// Args:
    ///     path (str): Path to the SHACL schema file.
    ///
    /// Raises:
    ///     RuntimeError: If the generator is not initialized.
    ///     ValueError: If the schema cannot be loaded or parsed.
    pub fn load_shacl_schema(&mut self, path: &str) -> PyResult<()> {
        let generator = self
            .inner
            .as_mut()
            .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Generator not initialized"))?;

        self.runtime
            .block_on(generator.load_shacl_schema(path))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", e)))?;

        Ok(())
    }

    /// Auto-detect schema format and load it.
    ///
    /// Args:
    ///     path (str): Path to the schema file.
    ///
    /// Raises:
    ///     RuntimeError: If the generator is not initialized.
    ///     ValueError: If the schema cannot be loaded or parsed.
    pub fn load_schema_auto(&mut self, path: &str) -> PyResult<()> {
        let generator = self
            .inner
            .as_mut()
            .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Generator not initialized"))?;

        self.runtime
            .block_on(generator.load_schema_auto(path))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", e)))?;

        Ok(())
    }

    /// Generate synthetic data and write it to the configured output.
    ///
    /// Raises:
    ///     RuntimeError: If the generator is not initialized.
    ///     ValueError: If data generation fails.
    pub fn generate(&mut self) -> PyResult<()> {
        let generator = self
            .inner
            .as_mut()
            .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Generator not initialized"))?;

        self.runtime
            .block_on(generator.generate())
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", e)))?;

        Ok(())
    }

    /// Run the complete generation pipeline with optional schema format.
    ///
    /// Args:
    ///     schema_path (str): Path to the schema file.
    ///     format (Optional[SchemaFormat]): Schema format. If None, auto-detect.
    ///
    /// Raises:
    ///     RuntimeError: If the generator is not initialized.
    ///     ValueError: If schema loading or generation fails.
    pub fn run_with_format(&mut self, schema_path: &str, format: Option<PySchemaFormat>) -> PyResult<()> {
        let generator = self
            .inner
            .as_mut()
            .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Generator not initialized"))?;

        let rust_format = format.map(|f| f.into());

        self.runtime
            .block_on(generator.run_with_format(schema_path, rust_format))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", e)))?;

        Ok(())
    }

    /// Run the complete generation pipeline with automatic schema format detection.
    ///
    /// Args:
    ///     schema_path (str): Path to the schema file.
    ///
    /// Raises:
    ///     RuntimeError: If the generator is not initialized.
    ///     ValueError: If schema loading or generation fails.
    pub fn run(&mut self, schema_path: &str) -> PyResult<()> {
        self.run_with_format(schema_path, None)
    }
}
