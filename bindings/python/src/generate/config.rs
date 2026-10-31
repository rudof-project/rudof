use crate::{
    error::Result,
    generate::enums::{PyCardinalityStrategy, PyDataQuality, PyEntityDistribution, PyOutputFormat, PySchemaFormat},
};
use pyo3::prelude::*;
use rudof_lib::errors::{GenerationError, RudofError as CoreError};
use std::path::PathBuf;

/// Routes a `rudof_generate` config failure through the crate's error boundary.
fn config_error(e: impl std::fmt::Display) -> CoreError {
    GenerationError::WrongGeneratorConfig { error: e.to_string() }.into()
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

    /// Loads configuration from a TOML string (v1, or v2 with `version = 2`).
    ///
    /// Args:
    ///     content (str): The TOML document.
    ///
    /// Returns:
    ///     GeneratorConfig: Loaded configuration object.
    ///
    /// Raises:
    ///     GenerateError: If the document cannot be parsed.
    #[staticmethod]
    fn from_toml_str(content: &str) -> Result<Self> {
        let inner = rudof_generate::GeneratorConfig::from_toml_str(content).map_err(config_error)?;
        Ok(Self { inner })
    }

    /// Sets the probability of generating an optional property (0.0 to 1.0).
    fn set_property_fill(&mut self, probability: f64) {
        self.inner.generation.property_fill_probability = probability;
    }

    /// Sets where to write the ShapeMap (RDF output) or the type map (property graph
    /// output) that associates every generated node with its shape or type.
    fn set_shapemap_path(&mut self, path: Option<PathBuf>) {
        self.inner.output.shapemap = path;
    }

    /// With property graph output: also write the same graph as RDF (Turtle), and its
    /// ShapeMap, so that it can be validated against ShEx or SHACL translations.
    ///
    /// Args:
    ///     path (str | os.PathLike): Where to write the RDF graph.
    ///     shapemap (str | os.PathLike, optional): Where to write its ShapeMap.
    #[pyo3(signature = (path, shapemap = None))]
    fn set_rdf_output(&mut self, path: PathBuf, shapemap: Option<PathBuf>) {
        self.inner.output.rdf = Some(path);
        self.inner.output.rdf_shapemap = shapemap;
    }

    /// Breaks a share of the generated entities on purpose and writes the
    /// ground-truth report: every node (and, for a property graph, edge) that a correct
    /// validator must report, with the operator, the property and, for failures induced
    /// through references, their cause.
    ///
    /// Args:
    ///     rate (float): Probability that an entity receives one violation (0.0 to 1.0).
    ///     labels (str | os.PathLike): Where to write the report (JSON Lines).
    ///     operators (list[str], optional): Operator names, e.g. `["drop-required",
    ///         "dangling-reference"]`; all of them when omitted.
    ///
    /// Raises:
    ///     GenerateError: If the rate is out of range or an operator is unknown.
    #[pyo3(signature = (rate, labels, operators = None))]
    fn set_violations(&mut self, rate: f64, labels: PathBuf, operators: Option<Vec<String>>) -> Result<()> {
        use rudof_generate::config::{ViolationOperator, ViolationsConfig};
        if !(0.0..=1.0).contains(&rate) {
            return Err(config_error(format!("violation rate must be between 0.0 and 1.0, got {rate}")).into());
        }
        let operators = match operators {
            None => ViolationOperator::ALL.to_vec(),
            Some(names) => names
                .iter()
                .map(|n| {
                    ViolationOperator::from_name(n)
                        .ok_or_else(|| config_error(format!("unknown violation operator `{n}`")))
                })
                .collect::<std::result::Result<Vec<_>, _>>()?,
        };
        let rdf = self.inner.generation.violations.take().and_then(|v| v.rdf);
        self.inner.generation.violations = Some(ViolationsConfig {
            rate,
            operators,
            shapes: Vec::new(),
            labels: Some(labels),
            rdf,
        });
        Ok(())
    }

    /// With a PG-Schema input: also write the report of the same broken graph read as
    /// RDF, where a reference failure fails the node holding the reference, transitively.
    /// `closed` and `keys_required` say whether the RDF translation keeps PG-Schema's
    /// closed records and required keys (a SHACL modeller would usually drop both).
    /// Call after `set_violations`.
    ///
    /// Raises:
    ///     GenerateError: If no violations are configured.
    #[pyo3(signature = (labels, closed = true, keys_required = true))]
    fn set_rdf_labels(&mut self, labels: PathBuf, closed: bool, keys_required: bool) -> Result<()> {
        let violations = self
            .inner
            .generation
            .violations
            .as_mut()
            .ok_or_else(|| config_error("call set_violations before set_rdf_labels"))?;
        violations.rdf = Some(rudof_generate::config::RdfReading {
            labels,
            closed,
            keys_required,
        });
        Ok(())
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
