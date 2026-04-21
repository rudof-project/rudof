use thiserror::Error;

/// Errors that can occur during map state operations in Rudof.
#[derive(Error, Debug)]
pub enum MapStateError {
    /// No MapState is available (ShEx validation with Map semantic actions has not been run).
    #[error("No MapState available. Run ShEx validation with Map semantic actions first.")]
    NoMapStateLoaded,

    /// Failed to serialize the MapState.
    #[error("Failed to serialize MapState: {error}")]
    FailedSerializingMapState { error: String },

    /// Failed to read the MapState file from disk.
    #[error("Failed to load MapState from '{path}': {error}")]
    FailedLoadingMapState { path: String, error: String },

    /// Failed to deserialize the MapState JSON.
    #[error("Failed to deserialize MapState from '{path}': {error}")]
    FailedDeserializingMapState { path: String, error: String },
}
