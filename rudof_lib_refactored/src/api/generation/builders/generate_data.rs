use crate::{Rudof, Result, api::generation::GenerationOperations, formats::{InputSpec, GenerationSchemaFormat, DataFormat}};

/// Builder for `generate_data` operation.
///
/// Provides a fluent interface for configuring optional parameters for data
/// generation (result format, seed, parallelism) and executing the operation.
pub struct GenerateDataBuilder<'a> {
	rudof: &'a Rudof,
	schema: &'a InputSpec,
	schema_format: &'a GenerationSchemaFormat,
	result_generation_format: Option<&'a DataFormat>,
	number_entities: usize,
	seed: Option<u64>,
	parallel: Option<usize>,
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
		number_entities: usize,
	) -> Self {
		Self {
			rudof,
			schema,
			schema_format,
			result_generation_format: None,
			number_entities,
			seed: None,
			parallel: None,
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

	/// Execute data generation with the configured options.
	///
	/// # Errors
	///
	/// Returns an error if the schema cannot be parsed/loaded or if
	/// generation fails for any reason.
	pub fn execute(self) -> Result<()> {
		<Rudof as GenerationOperations>::generate_data(
			self.rudof,
			self.schema,
			self.schema_format,
			self.result_generation_format,
			self.number_entities,
			self.seed,
			self.parallel,
		)
	}
}
