use std::io;
use thiserror::Error;

/// Errors that can occur while rendering a [`crate::model::Diagram`] to text or to an image.
#[derive(Error, Debug)]
pub enum RenderError {
    /// Wraps any I/O failure encountered while writing rendered output.
    #[error(transparent)]
    IOError {
        #[from]
        err: io::Error,
    },

    /// A required external tool (e.g. `java`) is missing or failed a sanity check.
    #[error("Required external tool not available: {tool}: {error}")]
    ExternalToolUnavailable { tool: String, error: String },

    /// A required external resource (e.g. a `plantuml.jar` file) could not be found.
    #[error(
        "Required external resource not found at {path}\nError: {error}\nHint: set the {env_var} environment variable"
    )]
    ExternalResourceMissing {
        path: String,
        error: String,
        env_var: String,
    },

    /// A temporary file needed for rendering could not be created or written.
    #[error("Error creating temporary file: {error}")]
    TempFileError { error: String },

    /// Launching the external rendering command failed.
    #[error("Error launching command: {command}: {error}")]
    CommandError { command: String, error: String },

    /// The output file the external tool was expected to produce could not be opened.
    #[error("Error opening generated output file {path}: {error}")]
    CantOpenOutputFile { path: String, error: io::Error },

    /// Copying the generated output file to the caller's writer failed.
    #[error("Error copying output file {path} to writer: {error}")]
    CopyError { path: String, error: io::Error },

    /// The requested [`crate::model::DiagramScope::Neighs`] box could not be found.
    #[error("Box not found: {title}")]
    BoxNotFound { title: String },

    /// A generic rendering error, for backend-specific failures with no dedicated variant.
    #[error("Rendering error: {error}")]
    Other { error: String },
}
