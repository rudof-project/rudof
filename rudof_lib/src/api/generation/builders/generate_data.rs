use crate::{
    Result, Rudof,
    api::generation::GenerationOperations,
    api::generation::implementations::{GenerationValidation, validate_generated},
    errors::GenerationError,
    formats::{DataFormat, GenerationSchemaFormat, InputSpec},
};
use std::path::PathBuf;

/// Builder for `generate_data` operation.
///
/// Provides a fluent interface for configuring optional parameters for data
/// generation (result format, seed, parallelism) and executing the operation.
pub struct GenerateDataBuilder<'a> {
    rudof: &'a Rudof,
    schema: &'a InputSpec,
    schema_format: &'a GenerationSchemaFormat,
    output: Option<&'a PathBuf>,
    config_file: Option<&'a PathBuf>,
    result_generation_format: Option<&'a DataFormat>,
    number_entities: Option<usize>,
    seed: Option<u64>,
    parallel: Option<usize>,
    validate: bool,
}

impl<'a> GenerateDataBuilder<'a> {
    /// Create a new builder.
    ///
    /// Internal helper called by `Rudof::generate_data()`; not intended for
    /// direct public construction.
    pub(crate) fn new(
        rudof: &'a Rudof,
        schema: &'a InputSpec,
        schema_format: &'a GenerationSchemaFormat,
        number_entities: Option<usize>,
    ) -> Self {
        Self {
            rudof,
            schema,
            schema_format,
            result_generation_format: None,
            number_entities,
            seed: None,
            parallel: None,
            output: None,
            config_file: None,
            validate: false,
        }
    }

    /// Set the desired output data format for the generated RDF.
    pub fn with_result_generation_format(mut self, result_generation_format: &'a DataFormat) -> Self {
        self.result_generation_format = Some(result_generation_format);
        self
    }

    /// Set an explicit random seed for reproducible generation.
    pub fn with_seed(mut self, seed: u64) -> Self {
        self.seed = Some(seed);
        self
    }

    /// Set number of parallel worker threads to use.
    pub fn with_parallel(mut self, parallel: usize) -> Self {
        self.parallel = Some(parallel);
        self
    }

    /// Set an optional output file path to write the generated RDF data.
    pub fn with_output(mut self, output: &'a PathBuf) -> Self {
        self.output = Some(output);
        self
    }

    /// Set a `GeneratorConfig` file (TOML or JSON) controlling generation
    /// details (entity distribution, cardinality strategy, per-field
    /// generators, ...) beyond what the other builder methods expose.
    ///
    /// CLI-style overrides set through this builder (`with_result_generation_format`,
    /// `with_output`, `with_seed`, `with_parallel`, and a `Some` passed as
    /// `number_entities` to [`Rudof::generate_data`]) take precedence over the
    /// same settings in the config file; a `None` `number_entities` leaves the
    /// config file's `entity_count` (or the generator's own default) in effect.
    pub fn with_config_file(mut self, config_file: &'a PathBuf) -> Self {
        self.config_file = Some(config_file);
        self
    }

    /// Validate the generated data against the source schema after generating
    /// it. Requires [`with_output`](Self::with_output). A ShapeMap associating
    /// each node with the shape that generated it is written next to the
    /// output (`<output>.shapemap`) and used as the ShEx validation targets.
    pub fn with_validation(mut self, validate: bool) -> Self {
        self.validate = validate;
        self
    }

    /// Execute data generation with the configured parameters.
    pub async fn execute(self) -> Result<()> {
        self.execute_with_report().await.map(|_| ())
    }

    /// Execute data generation and, when [`with_validation`](Self::with_validation)
    /// is set, return the outcome of validating the output against the schema.
    pub async fn execute_with_report(self) -> Result<Option<GenerationValidation>> {
        let shapemap = if self.validate {
            let output = self.output.ok_or(GenerationError::ValidationNeedsOutput)?;
            Some(output.with_extension("shapemap"))
        } else {
            None
        };
        <Rudof as GenerationOperations>::generate_data(
            self.rudof,
            self.schema,
            self.schema_format,
            self.result_generation_format,
            self.output,
            self.config_file,
            self.number_entities,
            self.seed,
            self.parallel,
            shapemap.as_ref(),
        )
        .await?;

        let (Some(shapemap), Some(output)) = (shapemap, self.output) else {
            return Ok(None);
        };
        let InputSpec::Path(schema) = self.schema else {
            return Err(GenerationError::UnsupportedGenerationSchemaInput {
                input: "non-file schema".to_string(),
            })?;
        };
        let data_format = self.result_generation_format.copied().unwrap_or_default();
        validate_generated(
            self.rudof.config().execute(),
            schema,
            self.schema_format,
            output,
            &data_format,
            &shapemap,
        )
        .map(Some)
    }
}
