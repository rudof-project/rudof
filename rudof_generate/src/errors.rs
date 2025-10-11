use thiserror::Error;

pub type Result<T> = std::result::Result<T, DataGeneratorError>;

#[derive(Error, Debug)]
pub enum DataGeneratorError {
    #[error("Configuration error: {0}")]
    Config(String),

    #[error("ShEx parsing error: {0}")]
    ShexParsing(String),

    #[error("Field generation error: {0}")]
    FieldGeneration(String),

    #[error("Graph generation error: {0}")]
    GraphGeneration(String),

    #[error("Output writing error: {0}")]
    OutputWriting(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("TOML parsing error: {0}")]
    Toml(#[from] toml::de::Error),

    #[error("JSON parsing error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Join error: {0}")]
    Join(#[from] tokio::task::JoinError),
}
