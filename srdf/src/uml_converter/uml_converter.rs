use std::io::Write;

use crate::rdf_visualizer::rdf_visualizer_error::RdfVisualizerError;

pub trait UmlConverter {
    fn as_plant_uml(
        &self,
        writer: &mut Box<dyn Write>,
        mode: &UmlGenerationMode,
    ) -> Result<(), RdfVisualizerError>;

    fn as_image(
        &self,
        writer: &mut Box<dyn Write>,
        image_format: ImageFormat,
        mode: &UmlGenerationMode,
    ) -> Result<(), RdfVisualizerError>;

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
}

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
