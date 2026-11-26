use std::io;

use super::NodeId;
use rudof_viz::RenderError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum UmlError {
    #[error("NodeId {node_id} has already a component")]
    NodeIdHasComponent { node_id: NodeId },

    #[error(transparent)]
    IOError {
        #[from]
        err: io::Error,
    },

    #[error(transparent)]
    RenderError {
        #[from]
        err: RenderError,
    },

    #[error("UmlError: Feature not implemented: {msg}")]
    NotImplemented { msg: String },
}

impl UmlError {
    pub fn not_implemented(msg: &str) -> UmlError {
        UmlError::NotImplemented { msg: msg.to_string() }
    }
}
