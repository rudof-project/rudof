use thiserror::Error;

/// Represents all possible errors that can occur during UML conversion operations.
#[derive(Debug, Error)]
pub enum UmlConverterError {
    /// Error when no PlantUML jar file is found.
    ///
    /// This occurs when the environment variable `PLANTUML` is not set or points
    /// to a non-existent file, and no jar file can be located in the search paths.
    ///
    /// # Fields
    /// - `path`: The paths that were searched for the PlantUML jar file
    /// - `error`: Detailed description of why the file could not be found
    #[error("No PlantUML jar file found\nThe environment variable `PLANTUML` should point 
    to a plantuml.jar file\nSearching jar file in: {path}: {error}")]
    NoPlantUMLFile { path: String, error: String },

    /// Error when Java is not installed or not found in the system PATH.
    ///
    /// # Fields
    /// - `error`: Detailed description of why Java could not be found
    #[error("Java is not installed or not found in PATH: {error}")]
    JavaNotInstalled { error: String },

    /// Error creating a temporary UML file for processing.
    ///
    /// # Fields
    /// - `tempfile_name`: The name of the temporary file that failed to be created
    /// - `error`: Detailed description of the file creation failure
    #[error("Error creating temporary UML file: {tempfile_name}: {error}")]
    CreatingTempUMLFile {
        tempfile_name: String,
        error: String,
    },

    /// Error flushing (writing) content to a temporary UML file.
    ///
    /// # Fields
    /// - `tempfile_name`: The name of the temporary file that failed to be flushed
    /// - `error`: Detailed description of the flush failure
    #[error("Error flushing temporary UML file: {tempfile_name}: {error}")]
    FlushingTempUMLFile {
        tempfile_name: String,
        error: String,
    },

    /// Generic error when creating a temporary file fails.
    ///
    /// # Fields
    /// - `error`: Detailed description of the temporary file creation failure
    #[error("Error creating temportary file: {error}")]
    TempFileError { error: String },

    /// Error launching the PlantUML command-line tool.
    ///
    /// # Fields
    /// - `command`: The full command that was attempted to be executed
    /// - `error`: Detailed description of why the command failed
    #[error("Error launching PlantUML command: {command}: {error}")]
    PlantUMLCommandError { command: String, error: String },

    /// Error generating or opening a temporary file to store generated UML content.
    ///
    /// # Fields
    /// - `generated_name`: The name of the generated temporary file
    /// - `error`: The underlying I/O error that occurred
    #[error("Error generating temporary file {generated_name} to store UML content: {error}")]
    CantOpenGeneratedTempFile {
        generated_name: String,
        error: std::io::Error,
    },

    /// Error copying content from a temporary output file to the final writer.
    ///
    /// # Fields
    /// - `temp_name`: The name of the temporary file being copied
    /// - `error`: The underlying I/O error that occurred during copying
    #[error("Error copying temporary output file to writer: {temp_name}: {error}")]
    CopyingTempFile {
        temp_name: String,
        error: std::io::Error,
    },

    /// Error when a requested label cannot be found in the UML diagram.
    ///
    /// # Fields
    /// - `name`: The name of the label that was not found
    #[error("Label not found: {name}")]
    NotFoundLabel { name: String },

    /// Generic UML processing error.
    ///
    /// # Fields
    /// - `error`: Detailed description of the UML-related error
    #[error("UML error: {error}")]
    UmlError { error: String },
}
