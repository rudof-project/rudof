use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum SchemaJsonError {
    #[error("Reading path {path_name:?} error: {error:?}")]
    ReadingPathError {
        path_name: String,
        error: String, // We need to clone so we use String instead of io::Error
    },

    #[error("Reading JSON from {path_name:?}. Error: {error:?}")]
    JsonError {
        path_name: String,
        error: String, // We need to clone errors so we use String instead of serde_json::Error,
    },

    #[error("Reading JSON from reader. Error: {error:?}")]
    JsonErrorFromReader {
        error: String, // We need to clone errors so we use String instead of serde_json::Error,
    },

    #[error("Shape Decl with prefixed shape {prefix:}:{local} but no prefix map declaration")]
    ShapeDeclPrefixNoPrefixMap { prefix: String, local: String },

    #[error(transparent)]
    PrefixMapError {
        #[from]
        err: prefixmap::PrefixMapError,
    },
}
