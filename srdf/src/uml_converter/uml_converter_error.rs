use thiserror::Error;

#[derive(Debug, Error)]

pub enum UmlConverterError {
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

    #[error("No PlantUML file path found at path: {path}: {error}")]
    NoPlantUMLFile { path: String, error: String },

    #[error("Label not found: {name}")]
    NotFoundLabel { name: String },

    #[error("UML error: {error}")]
    UmlError { error: String },
}
