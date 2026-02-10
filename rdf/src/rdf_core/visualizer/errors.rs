use crate::rdf_core::visualizer::{VisualRDFNode, uml_converter::errors::UmlConverterError};
use std::io;
use thiserror::Error;

/// Represents all possible errors that can occur during RDF visualization operations.
#[derive(Error, Debug)]
pub enum RdfVisualizerError {
    /// Error wrapping I/O operations failures.
    ///
    /// This variant is used for any standard I/O errors that occur during
    /// file operations, network access, or other I/O-related tasks in the
    /// visualization process.
    ///
    /// # Fields
    /// - `err`: The underlying `std::io::Error` that was encountered
    #[error(transparent)]
    IOError {#[from] err: io::Error},

    /// Error when a specific RDF node cannot be found in the visual graph.
    ///
    /// This occurs when trying to access or manipulate a node that does not
    /// exist in the current visualization graph.
    ///
    /// # Fields
    /// - `node`: The `VisualRDFNode` that was not found
    #[error("VisualRDFNode not found: {node} in Visual graph")]
    NodeNotFound { node: VisualRDFNode },

    /// Error wrapping UML converter operation failures.
    ///
    /// This variant encapsulates errors that originate from the UML conversion
    /// process, such as PlantUML execution failures or diagram generation issues.
    ///
    /// # Fields
    /// - `err`: The underlying `UmlConverterError` that was encountered
    #[error(transparent)]
    UmlConverterError {#[from] err: UmlConverterError},
}
