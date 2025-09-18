// use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;
use std::str::FromStr;

use iri_s::IriS;
use prefixmap::PrefixMap;
use rudof_lib::{InputSpec, Rudof, RudofConfig};
use srdf::rdf_visualizer::visual_rdf_graph::VisualRDFGraph;
use srdf::{ImageFormat, RDFFormat, UmlGenerationMode};

use crate::RDFReaderMode;
use crate::writer::get_writer;
use crate::{data_format::DataFormat, mime_type::MimeType, result_data_format::ResultDataFormat};
use anyhow::{Result, bail};
use srdf::UmlConverter;

pub fn get_data_rudof(
    rudof: &mut Rudof,
    data: &Vec<InputSpec>,
    data_format: &DataFormat,
    endpoint: &Option<String>,
    reader_mode: &RDFReaderMode,
    config: &RudofConfig,
    allow_no_data: bool,
) -> Result<()> {
    match (data.is_empty(), endpoint) {
        (true, None) => {
            if allow_no_data {
                rudof.reset_data();
                Ok(())
            } else {
                bail!("None of `data` or `endpoint` parameters have been specified for validation")
            }
        }
        (false, None) => {
            let rdf_format = data_format2rdf_format(data_format);
            let reader_mode = match &reader_mode {
                RDFReaderMode::Lax => srdf::ReaderMode::Lax,
                RDFReaderMode::Strict => srdf::ReaderMode::Strict,
            };
            for d in data {
                let data_reader = d.open_read(Some(&data_format.mime_type()), "RDF data")?;
                let base = get_base(d, config)?;
                rudof.read_data(data_reader, &rdf_format, base.as_deref(), &reader_mode)?;
            }
            Ok(())
        }
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
        (false, Some(_)) => {
            bail!("Only one of 'data' or 'endpoint' supported at the same time at this moment")
        }
    }
}

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

pub fn get_base(input: &InputSpec, config: &RudofConfig) -> Result<Option<String>> {
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

#[allow(clippy::too_many_arguments)]
pub fn run_data(
    data: &Vec<InputSpec>,
    data_format: &DataFormat,
    debug: u8,
    output: &Option<PathBuf>,
    result_format: &ResultDataFormat,
    force_overwrite: bool,
    reader_mode: &RDFReaderMode,
    config: &RudofConfig,
) -> Result<()> {
    let (mut writer, _color) = get_writer(output, force_overwrite)?;
    let mut rudof = Rudof::new(config);
    if debug > 0 {
        println!("Config: {config:?}")
    }
    get_data_rudof(
        &mut rudof,
        data,
        data_format,
        &None,
        reader_mode,
        config,
        false,
    )?;
    match check_result_format(result_format) {
        CheckResultFormat::RDFFormat(rdf_format) => {
            rudof.get_rdf_data().serialize(&rdf_format, &mut writer)?;
        }
        CheckResultFormat::VisualFormat(VisualFormat::PlantUML) => {
            rudof.data2plant_uml(&mut writer)?;

            /*match visual_format {
                VisualFormat::PlantUML => uml,
                VisualFormat::SVG => todo!(),
                VisualFormat::PNG => todo!(),
            }*/
        }
        CheckResultFormat::VisualFormat(VisualFormat::SVG)
        | CheckResultFormat::VisualFormat(VisualFormat::PNG) => {
            let rdf = rudof.get_rdf_data();
            let uml_converter =
                VisualRDFGraph::from_rdf(rdf, config.rdf_data_config().rdf_visualization_config())?;
            let format = match result_format {
                ResultDataFormat::SVG => ImageFormat::SVG,
                ResultDataFormat::PNG => ImageFormat::PNG,
                _ => unreachable!(),
            };
            uml_converter.as_image(
                &mut writer,
                format,
                &UmlGenerationMode::all(),
                config.plantuml_path(),
            )?;
        }
    }
    Ok(())
}

enum CheckResultFormat {
    RDFFormat(RDFFormat),
    VisualFormat(VisualFormat),
}

#[allow(clippy::upper_case_acronyms)]
enum VisualFormat {
    PlantUML,
    SVG,
    PNG,
}

fn check_result_format(format: &ResultDataFormat) -> CheckResultFormat {
    match format {
        ResultDataFormat::Turtle => CheckResultFormat::RDFFormat(RDFFormat::Turtle),
        ResultDataFormat::N3 => CheckResultFormat::RDFFormat(RDFFormat::N3),
        ResultDataFormat::NTriples => CheckResultFormat::RDFFormat(RDFFormat::NTriples),
        ResultDataFormat::RDFXML => CheckResultFormat::RDFFormat(RDFFormat::RDFXML),
        ResultDataFormat::TriG => CheckResultFormat::RDFFormat(RDFFormat::TriG),
        ResultDataFormat::NQuads => CheckResultFormat::RDFFormat(RDFFormat::NQuads),
        ResultDataFormat::PlantUML => CheckResultFormat::VisualFormat(VisualFormat::PlantUML),
        ResultDataFormat::SVG => CheckResultFormat::VisualFormat(VisualFormat::SVG),
        ResultDataFormat::PNG => CheckResultFormat::VisualFormat(VisualFormat::PNG),
        _ => todo!(),
    }
}
