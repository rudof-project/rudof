use crate::rdf_core::visualizer::uml_converter::errors::UmlConverterError;
use std::{fs::File, io::{self, Write}, path::Path, process::Command};
use tempfile::TempDir;

/// Trait for converting data structures to PlantUML format.
///
/// Implementors of this trait can generate UML diagrams in PlantUML syntax
/// and convert them to various image formats using PlantUML.
pub trait UmlConverter {
    /// Converts the implementing type to PlantUML syntax and writes it to the given writer.
    ///
    /// # Arguments
    /// * `writer` - The writer to output the PlantUML code to
    /// * `mode` - The generation mode specifying what content to include in the diagram
    ///
    /// # Returns
    /// * `Result<(), UmlConverterError>` - Ok if successful, Err with details on failure
    fn as_plantuml<W: Write>(
        &self,
        writer: &mut W,
        mode: &UmlGenerationMode,
    ) -> Result<(), UmlConverterError>;

    /// Converts the implementing type to an image format using PlantUML.
    ///
    /// This method generates a PlantUML diagram and uses the PlantUML jar to convert
    /// it to the specified image format (PNG or SVG).
    ///
    /// # Arguments
    /// * `writer` - The writer to output the generated image data to
    /// * `image_format` - The desired output format (PNG or SVG)
    /// * `mode` - The generation mode specifying what content to include
    /// * `plantuml_path` - Path to the PlantUML jar file
    ///
    /// # Returns
    /// * `Result<(), UmlConverterError>` - Ok if successful, Err with details on failure
    fn as_image<W: Write, P: AsRef<Path>>(
        &self,
        writer: &mut W,
        image_format: ImageFormat,
        mode: &UmlGenerationMode,
        plantuml_path: P,
    ) -> Result<(), UmlConverterError> {
        // Check if the PlantUML jar file exists
        if let Err(e) = plantuml_path.as_ref().try_exists() {
            return Err(UmlConverterError::NoPlantUMLFile {
                path: plantuml_path.as_ref().display().to_string(),
                error: e.to_string(),
            });
        }

        // Create a temporary directory for intermediate files
        let tempdir = TempDir::new().map_err(|e| UmlConverterError::TempFileError {
            error: e.to_string(),
        })?;
        let tempdir_path = tempdir.path();

        // Define paths for temporary UML file
        let tempfile_path = tempdir_path.join("temp.uml");
        let tempfile_name = tempfile_path.display().to_string();

        // Generate and save the UML content to a temporary file
        self.save_uml_to_tempfile(&tempfile_path, &tempfile_name, mode)?;

        // Determine output parameters based on image format
        let (out_param, out_file_name) = match image_format {
            ImageFormat::PNG => ("-png", tempdir_path.join("temp.png")),
            ImageFormat::SVG => ("-svg", tempdir_path.join("temp.svg")),
        };

        // Verify Java is installed
        check_java_installed().map_err(|e| UmlConverterError::JavaNotInstalled {
            error: e.to_string(),
        })?;

        // Verify PlantUML jar is valid
        check_plantuml_jar(plantuml_path.as_ref()).map_err(|e| {
            UmlConverterError::NoPlantUMLFile {
                path: plantuml_path.as_ref().display().to_string(),
                error: e.to_string(),
            }
        })?;

        // Build the PlantUML command
        let mut command = Command::new("java");
        command.args([
            "-jar",
            &plantuml_path.as_ref().display().to_string(),
            "-o",
            &tempdir_path.to_string_lossy().to_string(),
            out_param,
            "--verbose",
            &tempfile_name,
        ]);

        // Execute the PlantUML command
        let output = command.output();
        match output {
            Ok(_) => {
                // Open the generated image file
                let mut temp_file = File::open(out_file_name.as_path()).map_err(|e| {
                    UmlConverterError::CantOpenGeneratedTempFile {
                        generated_name: out_file_name.display().to_string(),
                        error: e,
                    }
                })?;

                // Copy the image data to the output writer
                io::copy(&mut temp_file, writer).map_err(|e| {
                    UmlConverterError::CopyingTempFile {
                        temp_name: out_file_name.display().to_string(),
                        error: e,
                    }
                })?;
                Ok(())
            }
            Err(e) => Err(UmlConverterError::PlantUMLCommandError {
                command: format!("{:?}", command),
                error: e.to_string(),
            }),
        }
    }

    /// Saves the UML representation to a temporary file.
    ///
    /// # Arguments
    /// * `tempfile_path` - Path where the temporary file should be created
    /// * `tempfile_name` - Name of the temporary file for error reporting
    /// * `mode` - The generation mode specifying what content to include
    ///
    /// # Returns
    /// * `Result<(), UmlConverterError>` - Ok if successful, Err with details on failure
    fn save_uml_to_tempfile(
        &self,
        tempfile_path: &std::path::Path,
        tempfile_name: &str,
        mode: &UmlGenerationMode,
    ) -> Result<(), UmlConverterError> {
        // Create the temporary file
        let mut file =
            File::create(tempfile_path).map_err(|e| UmlConverterError::CreatingTempUMLFile {
                tempfile_name: tempfile_name.to_string(),
                error: e.to_string(),
            })?;

        // Write the PlantUML content to the file
        self.as_plantuml(&mut file, mode)
            .map_err(|e| UmlConverterError::UmlError {
                error: e.to_string(),
            })?;

        // Ensure all data is written to disk
        file.flush()
            .map_err(|e| UmlConverterError::FlushingTempUMLFile {
                tempfile_name: tempfile_name.to_string(),
                error: e.to_string(),
            })?;
        Ok(())
    }
}

/// Checks if Java is installed and accessible via PATH.
///
/// # Returns
/// * `Result<(), io::Error>` - Ok if Java is available, Err otherwise
fn check_java_installed() -> Result<(), io::Error> {
    let output = Command::new("java").arg("-version").output()?;
    if output.status.success() {
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Java is not installed or not found in PATH",
        ))
    }
}

/// Checks if the PlantUML jar file exists and is functional.
///
/// # Arguments
/// * `plantuml_path` - Path to the PlantUML jar file
///
/// # Returns
/// * `Result<(), io::Error>` - Ok if the jar is valid, Err otherwise
fn check_plantuml_jar<P: AsRef<Path>>(plantuml_path: P) -> Result<(), io::Error> {
    if plantuml_path.as_ref().exists() {
        // Test the jar by running PlantUML with -version flag
        let mut command = Command::new("java");
        command.args([
            "-jar",
            &plantuml_path.as_ref().display().to_string(),
            "-version",
        ]);
        let output = command.output()?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(io::Error::other(format!(
                "PlantUML jar file check failed with status: {}\nstderr: {}",
                output.status, stderr
            )))
        } else {
            Ok(())
        }
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!(
                "PlantUML jar file not found at path: {}",
                plantuml_path.as_ref().display()
            ),
        ))
    }
}

/// Supported image output formats for PlantUML conversion.
pub enum ImageFormat {
    /// Scalable Vector Graphics format
    SVG,
    /// Portable Network Graphics format
    PNG,
}

/// Modes for controlling what content is included in the generated UML diagram.
#[derive(Debug, Clone, Default)]
pub enum UmlGenerationMode {
    /// Include all nodes in the diagram
    #[default]
    AllNodes,

    /// Include only the neighbors of a specific node
    Neighs(String),
}

impl UmlGenerationMode {
    /// Creates a mode that includes all nodes.
    ///
    /// # Returns
    /// * `UmlGenerationMode::AllNodes`
    pub fn all() -> UmlGenerationMode {
        UmlGenerationMode::AllNodes
    }

    /// Creates a mode that includes only neighbors of the specified node.
    ///
    /// # Arguments
    /// * `node` - The name of the node whose neighbors should be included
    ///
    /// # Returns
    /// * `UmlGenerationMode::Neighs` containing the node name
    pub fn neighs(node: &str) -> UmlGenerationMode {
        UmlGenerationMode::Neighs(node.to_string())
    }
}
