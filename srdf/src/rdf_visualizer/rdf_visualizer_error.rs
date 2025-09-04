use std::io;

use thiserror::Error;

use crate::{UmlConverterError, rdf_visualizer::visual_rdf_node::VisualRDFNode};

#[derive(Error, Debug)]
pub enum RdfVisualizerError {
    #[error(transparent)]
    IOError {
        #[from]
        err: io::Error,
    },
    #[error("UmlError: Feature not implemented: {msg}")]
    NotImplemented { msg: String },

    #[error("VisualRDFNode not found: {node} in Visual graph")]
    NodeNotFound { node: VisualRDFNode },

    #[error(transparent)]
    UmlConverterError {
        #[from]
        err: UmlConverterError,
    },
}

impl RdfVisualizerError {
    pub fn not_implemented(msg: &str) -> RdfVisualizerError {
        RdfVisualizerError::NotImplemented {
            msg: msg.to_string(),
        }
    }
}
