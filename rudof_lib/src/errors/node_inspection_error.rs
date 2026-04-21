use thiserror::Error;

/// Errors that can occur during node inspection operations.
#[derive(Error, Debug)]
pub enum NodeInspectionError {
    /// The node inspection mode is unsupported by Rudof.
    #[error("Unsupported node inspection mode: '{mode}'. Valid values are: 'outgoing', 'incoming', 'both'")]
    UnsupportedNodeInspectionMode { mode: String },
}
