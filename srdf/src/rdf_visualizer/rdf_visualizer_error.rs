use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum RdfVisualizerError {
    #[error(transparent)]
    IOError {
        #[from]
        err: io::Error,
    },
    #[error("UmlError: Feature not implemented: {msg}")]
    NotImplemented { msg: String },
}

impl RdfVisualizerError {
    pub fn not_implemented(msg: &str) -> RdfVisualizerError {
        RdfVisualizerError::NotImplemented {
            msg: msg.to_string(),
        }
    }
}
