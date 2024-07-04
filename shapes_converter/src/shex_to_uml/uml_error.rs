use std::io;

use super::NodeId;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum UmlError {
    #[error("NodeId has already a component")]
    NodeIdHasComponent { node_id: NodeId },

    #[error(transparent)]
    IOError {
        #[from]
        err: io::Error,
    },
    #[error("UmlError: Feature not implemented: {msg}")]
    NotImplemented { msg: String },
}

impl UmlError {
    pub fn not_implemented(msg: &str) -> UmlError {
        UmlError::NotImplemented {
            msg: msg.to_string(),
        }
    }
}
