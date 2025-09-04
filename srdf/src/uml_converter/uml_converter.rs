use std::{
    fs::{self, File},
    io::{self, Write},
    path::{self, Path},
    process::Command,
};

use tempfile::TempDir;
use tracing::{Level, debug};

use crate::UmlConverterError;

pub trait UmlConverter {
    fn as_plantuml<W: Write>(
        &self,
        writer: &mut W,
        mode: &UmlGenerationMode,
    ) -> Result<(), UmlConverterError>;

    fn as_image<W: Write, P: AsRef<Path>>(
        &self,
        writer: &mut W,
        image_format: ImageFormat,
        mode: &UmlGenerationMode,
        plantuml_path: P,
    ) -> Result<(), UmlConverterError> {
        if let Err(e) = plantuml_path.as_ref().try_exists() {
            return Err(UmlConverterError::NoPlantUMLFile {
                path: plantuml_path.as_ref().display().to_string(),
                error: e.to_string(),
            });
        }
        let tempdir = TempDir::new().map_err(|e| UmlConverterError::TempFileError {
            error: e.to_string(),
        })?;

        let tempdir_path = tempdir.path();
        let tempfile_path = tempdir_path.join("temp.uml");
        let tempfile_name = tempfile_path.display().to_string();
        self.save_uml_to_tempfile(&tempfile_path, &tempfile_name, mode)?;
        debug!("ShEx contents stored in temporary file:{}", tempfile_name);
        if tracing::enabled!(Level::DEBUG) {
            show_contents(&tempfile_path).unwrap();
        }

        let (out_param, out_file_name) = match image_format {
            ImageFormat::PNG => ("-png", tempdir_path.join("temp.png")),
            ImageFormat::SVG => ("-svg", tempdir_path.join("temp.svg")),
        };

        // show_contents(&tempfile_path).unwrap();
        let mut command = Command::new("java");
        let output = command
            .arg("-jar")
            .arg(plantuml_path.as_ref().display().to_string())
            .arg("-o")
            .arg(tempdir_path.to_string_lossy().to_string())
            .arg(out_param)
            .arg("--verbose")
            .arg(tempfile_name)
            .output()
            .expect("Error executing PlantUML command");
        let stdout = String::from_utf8_lossy(&output.stdout);
        debug!("stdout:\n{}", stdout);

        let stderr = String::from_utf8_lossy(&output.stderr);
        debug!("stderr:\n{}", stderr);
        let command_name = format!("{:?}", &command);
        debug!("PLANTUML COMMAND:\n{command_name}");
        let result = command.output();
        match result {
            Ok(_) => {
                let mut temp_file = File::open(out_file_name.as_path()).map_err(|e| {
                    UmlConverterError::CantOpenGeneratedTempFile {
                        generated_name: out_file_name.display().to_string(),
                        error: e,
                    }
                })?;
                copy(&mut temp_file, writer).map_err(|e| UmlConverterError::CopyingTempFile {
                    temp_name: out_file_name.display().to_string(),
                    error: e,
                })?;
                Ok(())
            }
            Err(e) => Err(UmlConverterError::PlantUMLCommandError {
                command: command_name,
                error: e.to_string(),
            }),
        }
    }

    fn save_uml_to_tempfile(
        &self,
        tempfile_path: &std::path::Path,
        tempfile_name: &str,
        mode: &UmlGenerationMode,
    ) -> Result<(), UmlConverterError> {
        let mut file =
            File::create(tempfile_path).map_err(|e| UmlConverterError::CreatingTempUMLFile {
                tempfile_name: tempfile_name.to_string(),
                error: e.to_string(),
            })?;
        self.as_plantuml(&mut file, mode)
            .map_err(|e| UmlConverterError::UmlError {
                error: e.to_string(),
            })?;
        file.flush()
            .map_err(|e| UmlConverterError::FlushingTempUMLFile {
                tempfile_name: tempfile_name.to_string(),
                error: e.to_string(),
            })?;
        Ok(())
    }
}

/*fn generate_uml_output(
    &self,
    maybe_shape: &Option<String>,
    writer: &mut Box<dyn Write>,
    mode: &UmlGenerationMode,
    result_format: &OutputConvertFormat,
) -> Result<()> {
    match result_format {
        OutputConvertFormat::PlantUML => {
            self.as_plant_uml(writer)?;
            Ok(())
        }
        OutputConvertFormat::SVG => {
            self.as_image(writer, ImageFormat::SVG, mode)?;
            Ok(())
        }
        OutputConvertFormat::PNG => {
            self.as_image(writer, ImageFormat::PNG, mode)?;
            Ok(())
        }
        OutputConvertFormat::Default => {
            self.as_plant_uml(writer)?;
            Ok(())
        }
        _ => Err(anyhow!(
            "Conversion to UML does not support output format {result_format}"
        )),
    }
}*/

pub enum ImageFormat {
    SVG,
    PNG,
}

#[derive(Debug, Clone, Default)]
pub enum UmlGenerationMode {
    /// Show all nodes
    #[default]
    AllNodes,

    /// Show only the neighbours of a node
    Neighs(String),
}

impl UmlGenerationMode {
    pub fn all() -> UmlGenerationMode {
        UmlGenerationMode::AllNodes
    }

    pub fn neighs(node: &str) -> UmlGenerationMode {
        UmlGenerationMode::Neighs(node.to_string())
    }
}

fn show_contents(path: &path::Path) -> Result<(), io::Error> {
    let contents = fs::read_to_string(path)?;
    debug!("Contents of {}:\n{}", path.display(), contents);
    Ok(())
}

/*fn show_dir(path: &path::Path) -> Result<(), io::Error> {
    let entries = fs::read_dir(path)?;
    for entry in entries {
        let entry = entry?;
        debug!("Entry: {}", entry.path().display());
    }
    Ok(())
}*/

fn copy<W: Write>(file: &mut File, writer: &mut W) -> Result<(), io::Error> {
    io::copy(file, writer)?;
    Ok(())
}
