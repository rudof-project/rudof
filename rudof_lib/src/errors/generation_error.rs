use thiserror::Error;

/// Errors that can occur when generating synthetic RDF data from schemas.
#[derive(Error, Debug)]
pub enum GenerationError {
    /// The schema format for data generation is not supported.
    #[error("Unsupported schema format for data generation: '{format}'. Valid formats are: 'auto', 'shex', 'shacl'")]
    UnsupportedGenerationSchemaFormat { format: String },

    /// The input type for schema is not supported for data generation.
    #[error("Unsupported schema input type for data generation: '{input}'. Only file paths are supported.")]
    UnsupportedGenerationSchemaInput { input: String },

    /// Failed to load or parse the generator configuration file.
    #[error("Failed to load generator configuration: {error}")]
    WrongGeneratorConfig { error: String },

    /// Failed to create the data generator instance.
    #[error("Failed to create data generator: {error}")]
    FailedCreatingDataGenerator { error: String },

    /// Failed to load the schema into the generator.
    #[error("Failed to load schema for data generation: {error}")]
    FailedLoadingSchema { error: String },

    /// Failed to generate the synthetic data.
    #[error("Failed to generate data: {error}")]
    FailedGeneratingData { error: String },
}
