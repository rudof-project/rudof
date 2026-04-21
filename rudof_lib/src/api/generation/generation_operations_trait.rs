use crate::{
    Result, Rudof,
    api::generation::implementations::generate_data,
    formats::{DataFormat, GenerationSchemaFormat, InputSpec},
};
use std::path::PathBuf;

/// Operations for generating RDF data.
pub trait GenerationOperations {
    /// Generates RDF data based on a ShEx or SHACL schema.
    ///
    /// # Arguments
    ///
    /// * `schema` - Input specification defining the schema source
    /// * `schema_format` - Format of the input schema (ShEx or SHACL)
    /// * `result_generation_format` - Optional output format for the generated RDF data (uses default if None)
    /// * `output` - Optional file path to write the generated data (prints to console if None)
    /// * `config_file` - Optional path to a configuration file for generation settings
    /// * `number_entities` - Number of entities to generate
    /// * `seed` - Optional random seed for reproducible generation (uses random seed if None)
    /// * `parallel` - Optional number of parallel threads (uses 2 by default)
    ///
    /// # Errors
    ///
    /// Returns an error if the schema cannot be parsed, loaded, or if data generation fails.
    async fn generate_data(
        &self,
        schema: &InputSpec,
        schema_format: &GenerationSchemaFormat,
        result_generation_format: Option<&DataFormat>,
        output: Option<&PathBuf>,
        config_file: Option<&PathBuf>,
        number_entities: usize,
        seed: Option<u64>,
        parallel: Option<usize>,
    ) -> Result<()>;
}

impl GenerationOperations for Rudof {
    async fn generate_data(
        &self,
        schema: &InputSpec,
        schema_format: &GenerationSchemaFormat,
        result_generation_format: Option<&DataFormat>,
        output: Option<&PathBuf>,
        config_file: Option<&PathBuf>,
        number_entities: usize,
        seed: Option<u64>,
        parallel: Option<usize>,
    ) -> Result<()> {
        generate_data(
            self,
            schema,
            schema_format,
            result_generation_format,
            output,
            config_file,
            number_entities,
            seed,
            parallel,
        )
        .await
    }
}
