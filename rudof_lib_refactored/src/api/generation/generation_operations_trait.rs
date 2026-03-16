use crate::{
    Result, Rudof, 
    formats::{InputSpec, GenerationSchemaFormat, DataFormat},
    api::generation::implementations::generate_data
};

/// Operations for generating RDF data.
pub trait GenerationOperations {
    /// Generates RDF data based on a ShEx or SHACL schema.
    ///
    /// # Arguments
    ///
    /// * `schema` - Input specification defining the schema source
    /// * `schema_format` - Format of the input schema (ShEx or SHACL)
    /// * `result_format` - Optional output format for the generated RDF data (uses default if None)
    /// * `number_entities` - Number of entities to generate
    /// * `seed` - Optional random seed for reproducible generation (uses random seed if None)
    /// * `parallel` - Optional number of parallel threads (uses 2 by default)
    ///
    /// # Errors
    ///
    /// Returns an error if the schema cannot be parsed, loaded, or if data generation fails.
    fn generate_data(
        &self,
        schema: &InputSpec,
        schema_format: &GenerationSchemaFormat,
        result_format: Option<&DataFormat>,
        number_entities: usize,
        seed: Option<u64>,
        parallel: Option<usize>,
    ) -> Result<()>;
}

impl GenerationOperations for Rudof {
    fn generate_data(
        &self,
        schema: &InputSpec,
        schema_format: &GenerationSchemaFormat,
        result_format: Option<&DataFormat>,
        number_entities: usize,
        seed: Option<u64>,
        parallel: Option<usize>,
    ) -> Result<()> {
        generate_data(self, schema, schema_format, result_format, number_entities, seed, parallel)
    }
}
