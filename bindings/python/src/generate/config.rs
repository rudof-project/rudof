use crate::{
    error::Result,
    generate::enums::{
        PyCardinalityStrategy, PyDataQuality, PyEntityDistribution, PyOutputFormat, PySchemaFormat,
    },
};
use pyo3::prelude::*;
use rudof_lib::errors::{GenerationError, RudofError as CoreError};
use std::path::PathBuf;

/// Routes a `rudof_generate` config failure through the crate's error boundary.
fn config_error(e: impl std::fmt::Display) -> CoreError {
    GenerationError::WrongGeneratorConfig {
        error: e.to_string(),
    }
    .into()
}

/// Configuration for synthetic data generation.
#[cfg_attr(feature = "stub-gen", pyo3_stub_gen_derive::gen_stub_pyclass)]
#[pyclass(name = "GeneratorConfig", module = "pyrudof._pyrudof")]
pub struct PyGeneratorConfig {
    pub(crate) inner: rudof_generate::GeneratorConfig,
}

#[cfg_attr(feature = "stub-gen", pyo3_stub_gen_derive::gen_stub_pymethods)]
#[pymethods]
impl PyGeneratorConfig {
    #[new]
    fn __init__() -> Self {
        Self {
            inner: rudof_generate::GeneratorConfig::default(),
        }
    }

    /// Loads configuration from a TOML file.
    ///
    /// Args:
    ///     path (str | os.PathLike): Path to the TOML configuration file.
    ///
    /// Returns:
    ///     GeneratorConfig: Loaded configuration object.
    ///
    /// Raises:
    ///     GenerateError: If the file cannot be read or parsed.
    #[staticmethod]
    fn from_toml_file(path: PathBuf) -> Result<Self> {
        let inner = rudof_generate::GeneratorConfig::from_toml_file(&path).map_err(config_error)?;
        Ok(Self { inner })
    }

    /// Loads configuration from a JSON file.
    ///
    /// Args:
    ///     path (str | os.PathLike): Path to the JSON configuration file.
    ///
    /// Returns:
    ///     GeneratorConfig: Loaded configuration object.
    ///
    /// Raises:
    ///     GenerateError: If the file cannot be read or parsed.
    #[staticmethod]
    fn from_json_file(path: PathBuf) -> Result<Self> {
        let inner = rudof_generate::GeneratorConfig::from_json_file(&path).map_err(config_error)?;
        Ok(Self { inner })
    }

    /// Saves this configuration to a TOML file.
    ///
    /// Args:
    ///     path (str | os.PathLike): Path where the TOML file will be written.
    ///
    /// Raises:
    ///     GenerateError: If writing to the file fails.
    fn to_toml_file(&self, path: PathBuf) -> Result<()> {
        self.inner.to_toml_file(&path).map_err(config_error)?;
        Ok(())
    }

    /// Sets the number of entities to generate.
    fn set_entity_count(&mut self, count: usize) {
        self.inner.generation.entity_count = count;
    }

    /// Returns the number of entities to generate.
    fn get_entity_count(&self) -> usize {
        self.inner.generation.entity_count
    }

    /// Sets the random seed for reproducible generation.
    fn set_seed(&mut self, seed: Option<u64>) {
        self.inner.generation.seed = seed;
    }

    /// Returns the random seed, if one is set.
    fn get_seed(&self) -> Option<u64> {
        self.inner.generation.seed
    }

    /// Sets the schema format. ``None`` means auto-detect.
    fn set_schema_format(&mut self, format: Option<PySchemaFormat>) {
        self.inner.generation.schema_format = format.map(Into::into);
    }

    /// Sets the strategy used to pick cardinalities.
    fn set_cardinality_strategy(&mut self, strategy: PyCardinalityStrategy) {
        self.inner.generation.cardinality_strategy = strategy.into();
    }

    /// Sets how entities are distributed across shapes.
    fn set_entity_distribution(&mut self, distribution: PyEntityDistribution) {
        self.inner.generation.entity_distribution = distribution.into();
    }

    /// Sets the output file path.
    fn set_output_path(&mut self, path: PathBuf) {
        self.inner.output.path = path;
    }

    /// Returns the output file path.
    fn get_output_path(&self) -> String {
        self.inner.output.path.display().to_string()
    }

    /// Sets the output serialization format.
    fn set_output_format(&mut self, format: PyOutputFormat) {
        self.inner.output.format = format.into();
    }

    /// Enables or disables compression of the output.
    fn set_compress(&mut self, compress: bool) {
        self.inner.output.compress = compress;
    }

    /// Returns whether the output is compressed.
    fn get_compress(&self) -> bool {
        self.inner.output.compress
    }

    /// Enables or disables writing a statistics file alongside the output.
    fn set_write_stats(&mut self, write_stats: bool) {
        self.inner.output.write_stats = write_stats;
    }

    /// Returns whether a statistics file is written.
    fn get_write_stats(&self) -> bool {
        self.inner.output.write_stats
    }

    /// Enables or disables writing the output from several threads.
    fn set_parallel_writing(&mut self, parallel_writing: bool) {
        self.inner.output.parallel_writing = parallel_writing;
    }

    /// Returns whether the output is written in parallel.
    fn get_parallel_writing(&self) -> bool {
        self.inner.output.parallel_writing
    }

    /// Sets how many files parallel writing splits the output into.
    fn set_parallel_file_count(&mut self, count: usize) {
        self.inner.output.parallel_file_count = count;
    }

    /// Returns how many files parallel writing splits the output into.
    fn get_parallel_file_count(&self) -> usize {
        self.inner.output.parallel_file_count
    }

    /// Sets the number of worker threads. ``None`` uses the runtime default.
    fn set_worker_threads(&mut self, threads: Option<usize>) {
        self.inner.parallel.worker_threads = threads;
    }

    /// Returns the configured number of worker threads.
    fn get_worker_threads(&self) -> Option<usize> {
        self.inner.parallel.worker_threads
    }

    /// Sets the batch size used by parallel processing.
    fn set_batch_size(&mut self, batch_size: usize) {
        self.inner.parallel.batch_size = batch_size;
    }

    /// Returns the batch size used by parallel processing.
    fn get_batch_size(&self) -> usize {
        self.inner.parallel.batch_size
    }

    /// Enables or disables processing shapes in parallel.
    fn set_parallel_shapes(&mut self, enabled: bool) {
        self.inner.parallel.parallel_shapes = enabled;
    }

    /// Returns whether shapes are processed in parallel.
    fn get_parallel_shapes(&self) -> bool {
        self.inner.parallel.parallel_shapes
    }

    /// Enables or disables processing fields in parallel.
    fn set_parallel_fields(&mut self, enabled: bool) {
        self.inner.parallel.parallel_fields = enabled;
    }

    /// Returns whether fields are processed in parallel.
    fn get_parallel_fields(&self) -> bool {
        self.inner.parallel.parallel_fields
    }

    /// Sets the locale used by the default field generators.
    fn set_locale(&mut self, locale: &str) {
        self.inner.field_generators.default.locale = locale.to_string();
    }

    /// Returns the locale used by the default field generators.
    fn get_locale(&self) -> String {
        self.inner.field_generators.default.locale.clone()
    }

    /// Sets how realistic the generated values should be.
    fn set_data_quality(&mut self, quality: PyDataQuality) {
        self.inner.field_generators.default.quality = quality.into();
    }

    /// Validates the configuration.
    ///
    /// Raises:
    ///     GenerateError: If the configuration is invalid.
    fn validate(&self) -> Result<()> {
        self.inner.validate().map_err(config_error)?;
        Ok(())
    }

    /// Returns a debug rendering of the whole configuration.
    fn show(&self) -> String {
        format!("{:?}", self.inner)
    }

    fn __repr__(&self) -> String {
        format!(
            "GeneratorConfig(entity_count={}, output_path='{}')",
            self.get_entity_count(),
            self.get_output_path()
        )
    }
}
