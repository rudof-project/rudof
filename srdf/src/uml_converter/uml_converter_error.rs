use thiserror::Error;

#[derive(Debug, Error)]
pub enum UmlConverterError {
    #[error(
        "No PlantUML jar file found\nThe environment variable `PLANTUML` should point to a plantuml.jar file\nSearching jar file in: {path}: {error}"
    )]
    NoPlantUMLFile { path: String, error: String },

    #[error("Java is not installed or not found in PATH: {error}")]
    JavaNotInstalled { error: String },
    #[error("Error creating temporary UML file: {tempfile_name}: {error}")]
    CreatingTempUMLFile {
        tempfile_name: String,
        error: String,
    },
    #[error("Error flushing temporary UML file: {tempfile_name}: {error}")]
    FlushingTempUMLFile {
        tempfile_name: String,
        error: String,
    },

    #[error("Error creating temportary file: {error}")]
    TempFileError { error: String },

    #[error("Error launching PlantUML command: {command}: {error}")]
    PlantUMLCommandError { command: String, error: String },

    #[error("Error generating temporary file {generated_name} to store UML content: {error}")]
    CantOpenGeneratedTempFile {
        generated_name: String,
        error: std::io::Error,
    },
    #[error("Error copying temporary output file to writer: {temp_name}: {error}")]
    CopyingTempFile {
        temp_name: String,
        error: std::io::Error,
    },

    #[error("Label not found: {name}")]
    NotFoundLabel { name: String },

    #[error("UML error: {error}")]
    UmlError { error: String },
}
