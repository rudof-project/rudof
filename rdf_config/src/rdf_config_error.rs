use thiserror::Error;

#[derive(Error, Debug)]
pub enum RdfConfigError {
    #[error("Error reading file {source_name}")]
    ErrorReadingFile { source_name: String },

    #[error("Error parsing YAML from {source_name}: {error}")]
    ErrorParsingYaml { source_name: String, error: String },

    #[error("Error parsing YAML from {source_name}: empty document?")]
    ErrorParsingYamlEmpty { source_name: String },

    #[error("Error writing RDF config: {error}")]
    WritingRdfConfig { error: String },
}
