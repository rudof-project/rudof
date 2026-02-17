// Shared core logic for data management
use iri_s::IriS;
use iri_s::MimeType;
use rdf::rdf_core::{
    RDFFormat, 
    visualizer::{VisualRDFGraph, uml_converter::{UmlConverter, UmlGenerationMode, ImageFormat}}
};
use rdf::rdf_impl::ReaderMode;
use std::str::FromStr;

use crate::{
    InputSpec, Rudof, RudofConfig, RudofError,
    data_format::{DataFormat, DataFormatError},
};

// Converts a rudof_lib DataFormat into a srdf RDFFormat.
pub fn data_format2rdf_format(data_format: &DataFormat) -> Result<RDFFormat, DataFormatError> {
    match data_format {
        DataFormat::N3 => Ok(RDFFormat::N3),
        DataFormat::NQuads => Ok(RDFFormat::NQuads),
        DataFormat::NTriples => Ok(RDFFormat::NTriples),
        DataFormat::RdfXml => Ok(RDFFormat::RdfXml),
        DataFormat::TriG => Ok(RDFFormat::TriG),
        DataFormat::Turtle => Ok(RDFFormat::Turtle),
        DataFormat::JsonLd => Ok(RDFFormat::JsonLd),
        DataFormat::Pg => Err(DataFormatError::NonRdfFormat {
            format: data_format.to_string(),
        }),
    }
}

// Helper function to determine the base IRI for reading data.
pub fn get_base(input: &InputSpec, config: &RudofConfig, base: &Option<IriS>) -> Result<Option<String>, RudofError> {
    if let Some(base) = base {
        Ok(Some(base.to_string()))
    } else {
        let base = match config.rdf_data_base() {
            Some(base) => Some(base.to_string()),
            None => {
                if config.automatic_base() {
                    let base = input.guess_base().map_err(|e| RudofError::BaseIriError {
                        str: "automatic base".to_string(),
                        error: e.to_string(),
                    })?;
                    Some(base)
                } else {
                    None
                }
            },
        };
        Ok(base)
    }
}

// Loads RDF data into the Rudof instance from a list of sources or a SPARQL endpoint.
#[allow(clippy::too_many_arguments)]
pub fn get_data_rudof(
    rudof: &mut Rudof,
    data: &Vec<InputSpec>,
    data_format: &DataFormat,
    base: &Option<IriS>,
    endpoint: &Option<String>,
    reader_mode: &ReaderMode,
    config: &RudofConfig,
    allow_no_data: bool,
) -> Result<(), RudofError> {
    match (data.is_empty(), endpoint) {
        (true, None) => {
            if allow_no_data {
                rudof.reset_data();
                Ok(())
            } else {
                Err(RudofError::MissingDataAndEndpoint)
            }
        },
        (false, None) => {
            let rdf_format =
                data_format2rdf_format(data_format).map_err(|e| RudofError::DataFormatError { error: e })?;
            for data_input in data {
                let mut data_reader = data_input
                    .open_read(Some(data_format.mime_type()), "RDF data")
                    .map_err(|e| RudofError::RDFDataReadError {
                        source_name: data_input.source_name(),
                        mime_type: data_format.mime_type().to_string(),
                        error: e.to_string(),
                    })?;
                let base = get_base(data_input, config, base)?;
                rudof.read_data(
                    &mut data_reader,
                    data_input.source_name().as_str(),
                    &rdf_format,
                    base.as_deref(),
                    reader_mode,
                    true,
                )?;
            }
            Ok(())
        },
        (true, Some(endpoint)) => {
            let (new_endpoint, _sparql) = rudof.get_endpoint(endpoint)?;
            // rudof.add_endpoint(&endpoint, &endpoint, PrefixMap::new())?;
            rudof.use_endpoint(new_endpoint.as_str())?;
            Ok(())
        },
        (false, Some(_)) => Err(RudofError::BothDataAndEndpointSpecified),
    }
}

/// Parses an optional base IRI string into an Option<IriS>.
pub fn parse_optional_base_iri(base_str: Option<String>) -> Result<Option<IriS>, RudofError> {
    base_str
        .map(|s| {
            IriS::from_str(&s).map_err(|e| RudofError::BaseIriError {
                str: s.clone(),
                error: e.to_string(),
            })
        })
        .transpose()
}

/// Converts a case-insensitive image format string ("SVG" or "PNG") into ImageFormat.
pub fn parse_image_format(image_format_str: &str) -> Result<ImageFormat, RudofError> {
    match image_format_str.to_uppercase().as_str() {
        "SVG" => Ok(ImageFormat::Svg),
        "PNG" => Ok(ImageFormat::Png),
        _ => Err(RudofError::InvalidImageFormat {
            format: image_format_str.to_string(),
        }),
    }
}

/// Executes the full visualization and image generation logic.
/// Returns the generated image data as a Vec<u8>.
pub fn export_rdf_to_image(rudof: &Rudof, image_format: ImageFormat) -> Result<Vec<u8>, RudofError> {
    let rdf = rudof.get_rdf_data();
    let config = rudof.config();
    let mut v = Vec::new();

    let uml_converter = VisualRDFGraph::from_rdf(rdf, config.rdf_data_config().rdf_visualization_config())
        .map_err(|e| RudofError::RDF2PlantUmlError { error: e.to_string() })?;

    uml_converter
        .as_image(&mut v, image_format, &UmlGenerationMode::all(), config.plantuml_path())
        .map_err(|e| RudofError::RDF2PlantUmlErrorAsPlantUML { error: e.to_string() })?;

    Ok(v)
}
