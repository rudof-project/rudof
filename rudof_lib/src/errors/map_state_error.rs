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
}
