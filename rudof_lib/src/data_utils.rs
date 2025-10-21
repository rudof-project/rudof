// Shared core logic for data management
use anyhow::{Result, bail};
use iri_s::IriS;
use iri_s::mime_type::MimeType;
use prefixmap::PrefixMap;
use srdf::{
    ImageFormat, RDFFormat, ReaderMode, UmlConverter, UmlGenerationMode,
    rdf_visualizer::visual_rdf_graph::VisualRDFGraph,
};
use std::str::FromStr;

use crate::{InputSpec, Rudof, RudofConfig, data_format::DataFormat};

// Converts a rudof_lib DataFormat into a srdf RDFFormat.
pub fn data_format2rdf_format(data_format: &DataFormat) -> RDFFormat {
    match data_format {
        DataFormat::N3 => RDFFormat::N3,
        DataFormat::NQuads => RDFFormat::NQuads,
        DataFormat::NTriples => RDFFormat::NTriples,
        DataFormat::RDFXML => RDFFormat::RDFXML,
        DataFormat::TriG => RDFFormat::TriG,
        DataFormat::Turtle => RDFFormat::Turtle,
        DataFormat::JsonLd => RDFFormat::JsonLd,
    }
}

// Helper function to determine the base IRI for reading data.
pub fn get_base(
    input: &InputSpec,
    config: &RudofConfig,
    base: &Option<IriS>,
) -> Result<Option<String>> {
    if let Some(base) = base {
        Ok(Some(base.to_string()))
    } else {
        let base = match config.rdf_data_base() {
            Some(base) => Some(base.to_string()),
            None => {
                if config.automatic_base() {
                    let base = input.guess_base()?;
                    Some(base)
                } else {
                    None
                }
            }
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
) -> Result<()> {
    // Check for the specific "no data, no endpoint" case
    if data.is_empty() && endpoint.is_none() {
        if allow_no_data {
            rudof.reset_data();
            return Ok(());
        } else {
            bail!("None of `data` or `endpoint` parameters have been specified for validation")
        }
    }

    match (data.is_empty(), endpoint) {
        // Case 1: Load from one or more data sources (Files/URLs)
        (false, None) => {
            let rdf_format = data_format2rdf_format(data_format);
            for d in data {
                let data_reader = d.open_read(Some(data_format.mime_type()), "RDF data")?;
                let base = get_base(d, config, base)?;
                rudof.read_data(data_reader, &rdf_format, base.as_deref(), reader_mode)?;
            }
            Ok(())
        }
        // Case 2: Add SPARQL Endpoint
        (true, Some(endpoint)) => {
            let (endpoint_iri, prefixmap) =
                if let Some(endpoint_descr) = config.rdf_data_config().find_endpoint(endpoint) {
                    (
                        endpoint_descr.query_url().clone(),
                        endpoint_descr.prefixmap().clone(),
                    )
                } else {
                    let iri = IriS::from_str(endpoint.as_str())?;
                    (iri, PrefixMap::basic())
                };
            rudof.add_endpoint(&endpoint_iri, &prefixmap)?;
            Ok(())
        }
        // Disallowed cases
        (true, None) => bail!("One of `data` or `endpoint` must be specified"),
        (false, Some(_)) => {
            bail!("Only one of 'data' or 'endpoint' supported at the same time")
        }
    }
}

/// Parses an optional base IRI string into an Option<IriS>.
pub fn parse_optional_base_iri(base_str: Option<String>) -> Result<Option<IriS>> {
    base_str
        .map(|s| IriS::from_str(&s))
        .transpose()
        .map_err(|e| anyhow::anyhow!("Invalid base IRI: {}", e))
}

/// Converts a case-insensitive image format string ("SVG" or "PNG") into ImageFormat.
pub fn parse_image_format(image_format_str: &str) -> Result<ImageFormat> {
    match image_format_str.to_uppercase().as_str() {
        "SVG" => Ok(ImageFormat::SVG),
        "PNG" => Ok(ImageFormat::PNG),
        _ => bail!(
            "Invalid image format: {}. Must be 'SVG' or 'PNG'.",
            image_format_str
        ),
    }
}

/// Executes the full visualization and image generation logic.
/// Returns the generated image data as a Vec<u8>.
pub fn export_rdf_to_image(rudof: &Rudof, image_format: ImageFormat) -> Result<Vec<u8>> {
    let rdf = rudof.get_rdf_data();
    let config = rudof.config();
    let mut v = Vec::new();

    let uml_converter =
        VisualRDFGraph::from_rdf(rdf, config.rdf_data_config().rdf_visualization_config())?;

    uml_converter.as_image(
        &mut v,
        image_format,
        &UmlGenerationMode::all(),
        config.plantuml_path(),
    )?;

    Ok(v)
}
