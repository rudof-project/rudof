use pyo3::{PyErr, PyResult, pyclass, pymethods};
use std::path::Path;

/// Python wrapper for GeneratorConfig
#[pyclass(name = "GeneratorConfig")]
pub struct PyGeneratorConfig {
    inner: rudof_generate::GeneratorConfig,
}

#[pymethods]
impl PyGeneratorConfig {
    /// Create a new GeneratorConfig with default values
    #[new]
    pub fn __init__() -> Self {
        Self {
            inner: rudof_generate::GeneratorConfig::default(),
        }
    }

    /// Load configuration from a TOML file
    #[staticmethod]
    #[pyo3(signature = (path))]
    pub fn from_toml_file(path: &str) -> PyResult<Self> {
        let config = rudof_generate::GeneratorConfig::from_toml_file(path)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", e)))?;
        Ok(Self { inner: config })
    }

    /// Load configuration from a JSON file
    #[staticmethod]
    #[pyo3(signature = (path))]
    pub fn from_json_file(path: &str) -> PyResult<Self> {
        let config = rudof_generate::GeneratorConfig::from_json_file(path)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", e)))?;
        Ok(Self { inner: config })
    }

    /// Save configuration to a TOML file
    #[pyo3(signature = (path))]
    pub fn to_toml_file(&self, path: &str) -> PyResult<()> {
        self.inner
            .to_toml_file(path)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", e)))?;
        Ok(())
    }

    /// Set the number of entities to generate
    #[pyo3(signature = (count))]
    pub fn set_entity_count(&mut self, count: usize) {
        self.inner.generation.entity_count = count;
    }

    /// Get the number of entities to generate
    #[pyo3(signature = ())]
    pub fn get_entity_count(&self) -> usize {
        self.inner.generation.entity_count
    }

    /// Set the random seed for reproducible generation
    #[pyo3(signature = (seed))]
    pub fn set_seed(&mut self, seed: Option<u64>) {
        self.inner.generation.seed = seed;
    }

    /// Get the random seed
    #[pyo3(signature = ())]
    pub fn get_seed(&self) -> Option<u64> {
        self.inner.generation.seed
    }

    /// Set the output file path
    #[pyo3(signature = (path))]
    pub fn set_output_path(&mut self, path: &str) {
        self.inner.output.path = Path::new(path).to_path_buf();
    }

    /// Get the output file path as a string
    #[pyo3(signature = ())]
    pub fn get_output_path(&self) -> String {
        self.inner.output.path.display().to_string()
    }

    /// Set the output format
    #[pyo3(signature = (format))]
    pub fn set_output_format(&mut self, format: PyOutputFormat) -> PyResult<()> {
        self.inner.output.format = format.into();
        Ok(())
    }

    /// Set the schema format
    #[pyo3(signature = (format))]
    pub fn set_schema_format(&mut self, format: Option<PySchemaFormat>) {
        self.inner.generation.schema_format = format.map(|f| f.into());
    }

    /// Set the cardinality strategy
    #[pyo3(signature = (strategy))]
    pub fn set_cardinality_strategy(&mut self, strategy: PyCardinalityStrategy) {
        self.inner.generation.cardinality_strategy = strategy.into();
    }

    /// Set whether to compress output
    #[pyo3(signature = (compress))]
    pub fn set_compress(&mut self, compress: bool) {
        self.inner.output.compress = compress;
    }

    /// Set whether to write statistics
    #[pyo3(signature = (write_stats))]
    pub fn set_write_stats(&mut self, write_stats: bool) {
        self.inner.output.write_stats = write_stats;
    }

    /// Set whether to use parallel writing
    #[pyo3(signature = (parallel_writing))]
    pub fn set_parallel_writing(&mut self, parallel_writing: bool) {
        self.inner.output.parallel_writing = parallel_writing;
    }

    /// Set the number of parallel output files
    #[pyo3(signature = (count))]
    pub fn set_parallel_file_count(&mut self, count: usize) {
        self.inner.output.parallel_file_count = count;
    }

    /// Set the number of worker threads
    #[pyo3(signature = (threads))]
    pub fn set_worker_threads(&mut self, threads: Option<usize>) {
        self.inner.parallel.worker_threads = threads;
    }

    /// Set the batch size for parallel processing
    #[pyo3(signature = (batch_size))]
    pub fn set_batch_size(&mut self, batch_size: usize) {
        self.inner.parallel.batch_size = batch_size;
    }

    /// Set whether to enable parallel shape processing
    #[pyo3(signature = (enabled))]
    pub fn set_parallel_shapes(&mut self, enabled: bool) {
        self.inner.parallel.parallel_shapes = enabled;
    }

    /// Get whether parallel shape processing is enabled
    #[pyo3(signature = ())]
    pub fn get_parallel_shapes(&self) -> bool {
        self.inner.parallel.parallel_shapes
    }

    /// Set whether to enable parallel field generation
    #[pyo3(signature = (enabled))]
    pub fn set_parallel_fields(&mut self, enabled: bool) {
        self.inner.parallel.parallel_fields = enabled;
    }

    /// Get whether parallel field generation is enabled
    #[pyo3(signature = ())]
    pub fn get_parallel_fields(&self) -> bool {
        self.inner.parallel.parallel_fields
    }

    /// Set the entity distribution strategy
    #[pyo3(signature = (distribution))]
    pub fn set_entity_distribution(&mut self, distribution: PyEntityDistribution) {
        self.inner.generation.entity_distribution = distribution.into();
    }

    /// Set the locale for field generation (e.g., "en", "es", "fr")
    #[pyo3(signature = (locale))]
    pub fn set_locale(&mut self, locale: &str) {
        self.inner.field_generators.default.locale = locale.to_string();
    }

    /// Get the locale for field generation
    #[pyo3(signature = ())]
    pub fn get_locale(&self) -> String {
        self.inner.field_generators.default.locale.clone()
    }

    /// Set the data quality level for generated data
    #[pyo3(signature = (quality))]
    pub fn set_data_quality(&mut self, quality: PyDataQuality) {
        self.inner.field_generators.default.quality = quality.into();
    }

    /// Get whether output will be compressed
    #[pyo3(signature = ())]
    pub fn get_compress(&self) -> bool {
        self.inner.output.compress
    }

    /// Get whether statistics will be written
    #[pyo3(signature = ())]
    pub fn get_write_stats(&self) -> bool {
        self.inner.output.write_stats
    }

    /// Get whether parallel writing is enabled
    #[pyo3(signature = ())]
    pub fn get_parallel_writing(&self) -> bool {
        self.inner.output.parallel_writing
    }

    /// Get the number of parallel output files
    #[pyo3(signature = ())]
    pub fn get_parallel_file_count(&self) -> usize {
        self.inner.output.parallel_file_count
    }

    /// Get the number of worker threads
    #[pyo3(signature = ())]
    pub fn get_worker_threads(&self) -> Option<usize> {
        self.inner.parallel.worker_threads
    }

    /// Get the batch size for parallel processing
    #[pyo3(signature = ())]
    pub fn get_batch_size(&self) -> usize {
        self.inner.parallel.batch_size
    }

    /// Validate the configuration
    #[pyo3(signature = ())]
    pub fn validate(&self) -> PyResult<()> {
        self.inner
            .validate()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", e)))?;
        Ok(())
    }

    /// Convert configuration to a String representation
    pub fn show(&self) -> String {
        format!("{:?}", self.inner)
    }
}

/// Schema format for the generator
#[pyclass(eq, eq_int, name = "SchemaFormat")]
#[derive(PartialEq, Clone, Copy)]
pub enum PySchemaFormat {
    ShEx,
    SHACL,
}

impl From<PySchemaFormat> for rudof_generate::SchemaFormat {
    fn from(val: PySchemaFormat) -> Self {
        match val {
            PySchemaFormat::ShEx => rudof_generate::SchemaFormat::ShEx,
            PySchemaFormat::SHACL => rudof_generate::SchemaFormat::SHACL,
        }
    }
}

/// Output format for generated data
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

/// Strategy for handling cardinalities in relationships
#[pyclass(eq, eq_int, name = "CardinalityStrategy")]
#[derive(PartialEq, Clone, Copy)]
pub enum PyCardinalityStrategy {
    Minimum,
    Maximum,
    Random,
    Balanced,
}

impl From<PyCardinalityStrategy> for rudof_generate::config::CardinalityStrategy {
    fn from(val: PyCardinalityStrategy) -> Self {
        match val {
            PyCardinalityStrategy::Minimum => rudof_generate::config::CardinalityStrategy::Minimum,
            PyCardinalityStrategy::Maximum => rudof_generate::config::CardinalityStrategy::Maximum,
            PyCardinalityStrategy::Random => rudof_generate::config::CardinalityStrategy::Random,
            PyCardinalityStrategy::Balanced => {
                rudof_generate::config::CardinalityStrategy::Balanced
            }
        }
    }
}

/// Entity distribution strategy
#[pyclass(eq, eq_int, name = "EntityDistribution")]
#[derive(PartialEq, Clone, Copy)]
pub enum PyEntityDistribution {
    /// Equal distribution across all shapes
    Equal,
}

impl From<PyEntityDistribution> for rudof_generate::config::EntityDistribution {
    fn from(val: PyEntityDistribution) -> Self {
        match val {
            PyEntityDistribution::Equal => rudof_generate::config::EntityDistribution::Equal,
        }
    }
}

/// Data quality level for generated data
#[pyclass(eq, eq_int, name = "DataQuality")]
#[derive(PartialEq, Clone, Copy)]
pub enum PyDataQuality {
    /// Simple random data
    Low,
    /// Realistic patterns
    Medium,
    /// Complex realistic data with correlations
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

/// Main data generator class
#[pyclass(name = "DataGenerator")]
pub struct PyDataGenerator {
    inner: Option<rudof_generate::DataGenerator>,
    runtime: tokio::runtime::Runtime,
}

#[pymethods]
impl PyDataGenerator {
    /// Create a new DataGenerator with the given configuration
    ///
    /// Args:
    ///   config: GeneratorConfig object containing the configuration
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

    /// Load and process a ShEx schema file
    ///
    /// Args:
    ///   path: Path to the ShEx schema file
    #[pyo3(signature = (path))]
    pub fn load_shex_schema(&mut self, path: &str) -> PyResult<()> {
        let generator = self.inner.as_mut().ok_or_else(|| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Generator not initialized")
        })?;

        self.runtime
            .block_on(generator.load_shex_schema(path))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", e)))?;

        Ok(())
    }

    /// Load and process a SHACL schema file
    ///
    /// Args:
    ///   path: Path to the SHACL schema file
    #[pyo3(signature = (path))]
    pub fn load_shacl_schema(&mut self, path: &str) -> PyResult<()> {
        let generator = self.inner.as_mut().ok_or_else(|| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Generator not initialized")
        })?;

        self.runtime
            .block_on(generator.load_shacl_schema(path))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", e)))?;

        Ok(())
    }

    /// Auto-detect schema format and load
    ///
    /// Args:
    ///   path: Path to the schema file
    #[pyo3(signature = (path))]
    pub fn load_schema_auto(&mut self, path: &str) -> PyResult<()> {
        let generator = self.inner.as_mut().ok_or_else(|| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Generator not initialized")
        })?;

        self.runtime
            .block_on(generator.load_schema_auto(path))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", e)))?;

        Ok(())
    }

    /// Generate synthetic data and write to output
    #[pyo3(signature = ())]
    pub fn generate(&mut self) -> PyResult<()> {
        let generator = self.inner.as_mut().ok_or_else(|| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Generator not initialized")
        })?;

        self.runtime
            .block_on(generator.generate())
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", e)))?;

        Ok(())
    }

    /// Run the complete generation pipeline with schema format detection
    ///
    /// Args:
    ///   schema_path: Path to the schema file
    ///   format: Optional schema format (ShEx or SHACL). If None, auto-detect
    #[pyo3(signature = (schema_path, format = None))]
    pub fn run_with_format(
        &mut self,
        schema_path: &str,
        format: Option<PySchemaFormat>,
    ) -> PyResult<()> {
        let generator = self.inner.as_mut().ok_or_else(|| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Generator not initialized")
        })?;

        let rust_format = format.map(|f| f.into());

        self.runtime
            .block_on(generator.run_with_format(schema_path, rust_format))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", e)))?;

        Ok(())
    }

    /// Run the complete generation pipeline with automatic schema format detection
    ///
    /// Args:
    ///   schema_path: Path to the schema file
    #[pyo3(signature = (schema_path))]
    pub fn run(&mut self, schema_path: &str) -> PyResult<()> {
        self.run_with_format(schema_path, None)
    }
}
